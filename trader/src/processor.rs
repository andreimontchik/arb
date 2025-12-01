mod raydium;

use {
    crate::{
        context::Context, gateway::Gateway, ComputedUnitsLimitType, ComputedUnitsPriceType,
        TraderMetricKey,
    },
    anyhow::Result,
    common::{
        message::{ArbitrageMessage, Message, SwapMessage, SwapType},
        metrics::MetricsCollector,
        DecimalPercentageType, LiquidityPool,
    },
    log::{debug, error, info, warn},
    serde_json::Value,
    solana_sdk::{
        compute_budget::ComputeBudgetInstruction, instruction::Instruction, signer::Signer,
        transaction::Transaction,
    },
    std::{
        sync::{Arc, Mutex},
        time::Instant,
    },
};

pub struct Processor<T: Gateway, M: MetricsCollector> {
    gateway: T,
    context: Arc<Mutex<Context>>,
    simulation: bool,
    metrics_collector: M,
}

impl<T: Gateway, M: MetricsCollector> Processor<T, M> {
    pub fn new(
        gateway_config: Value,
        metrics_collector_config: Value,
        simulation: bool,
        context: Arc<Mutex<Context>>,
    ) -> Self {
        Processor {
            gateway: T::new(gateway_config),
            context,
            simulation,
            metrics_collector: M::new(metrics_collector_config),
        }
    }

    pub fn process(&self, msg: &Message) -> Result<()> {
        let (txn, cu_price, cu_limit) = match msg {
            Message::Arbitrage(arb) => create_arbitrage_transaction(&self.context.lock().unwrap(), arb),
            Message::Swap(swap) => create_swap_transaction(&self.context.lock().unwrap(), swap),
            _ => unimplemented!(),
        }?;

        let msg_seq_id = Message::sequence_id(msg);
        let msg_slot = Message::slot(msg);

        if self.simulation {
            let start = Instant::now();
            let result = self.gateway.simulate_transaction(&txn);
            self.metrics_collector
                .duration(&TraderMetricKey::SimulateTransactionDuration, start.elapsed());
            info!(
                "Simulation completed  for the message ({}, slot: {}): CU price ({}), CU limit ({}), Result ({:?}).",
                msg_seq_id, msg_slot, cu_price, cu_limit, result,
            );
        } else {
            let start = Instant::now();
            let result = self.gateway.send_transaction(&txn);
            self.metrics_collector
                .duration(&TraderMetricKey::SubmitTransactionDuration, start.elapsed());
            info!(
                "Transaction submitted for the message ({}, slot: {}): Signature ({:?}), CU price ({}), CU limit ({}).",                
                msg_seq_id, msg_slot,result, cu_price, cu_limit,
            );
        }

        Ok(())
    }
}

pub fn create_arbitrage_transaction(
    context: &Context,
    msg: &ArbitrageMessage,
) -> Result<(Transaction, ComputedUnitsPriceType, ComputedUnitsLimitType)> {
    let mut instructions = Vec::<Instruction>::new();

    // Buy side
    let buy_lp = context.liquidity_pool(&msg.buy_side_info.lp_address)?;
    let buy_instruction = match buy_lp {
        LiquidityPool::RaydiumAmm(lp) => raydium::generate_swap_buy_instruction(
            context,
            msg.swap_base_qty,
            msg.swap_buy_quote_amount,
            lp,
        ),
        _ => unimplemented!(),
    }?;
    instructions.push(buy_instruction);

    // Sell side
    let sell_lp = context.liquidity_pool(&msg.sell_side_info.lp_address)?;
    let sell_instruction = match sell_lp {
        LiquidityPool::RaydiumAmm(lp) => raydium::generate_swap_sell_instruction(
            context,
            msg.swap_base_qty,
            msg.swap_sell_quote_amount,
            lp,
        ),
        _ => unimplemented!(),
    }?;
    instructions.push(sell_instruction);

    // Priority fee
    let cu_limit = context.arb_cu_limit();
    instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(cu_limit));
    let cu_price = calculate_cu_price(context)?;
    instructions.push(ComputeBudgetInstruction::set_compute_unit_price(cu_price));

    Ok((
        Transaction::new_signed_with_payer(
            &instructions,
            Some(&context.wallet().keypair().pubkey()),
            &[context.wallet().keypair()],
            context.latest_blockhash()?,
        ),
        cu_price,
        cu_limit,
    ))
}

pub fn create_swap_transaction(
    context: &Context,
    msg: &SwapMessage,
) -> Result<(Transaction, ComputedUnitsPriceType, ComputedUnitsLimitType)> {
    let mut instructions = Vec::<Instruction>::new();

    let lp = context.liquidity_pool(&msg.lp_address)?;
    let swap_instruction = match lp {
        LiquidityPool::RaydiumAmm(lp) => match msg.swap_type {
            SwapType::Buy => {
                raydium::generate_swap_buy_instruction(context, msg.base_qty, msg.quote_amount, lp)
            }
            SwapType::Sell => {
                raydium::generate_swap_sell_instruction(context, msg.base_qty, msg.quote_amount, lp)
            }
        },
        _ => unimplemented!(),
    }?;
    instructions.push(swap_instruction);

    // Priority fee
    let cu_limit = context.swap_cu_limit();
    instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(cu_limit));
    let cu_price = calculate_cu_price(context)?;
    instructions.push(ComputeBudgetInstruction::set_compute_unit_price(cu_price));

    Ok((
        Transaction::new_signed_with_payer(
            &instructions,
            Some(&context.wallet().keypair().pubkey()),
            &[context.wallet().keypair()],
            context.latest_blockhash()?,
        ),
        cu_price,
        cu_limit,
    ))
}

#[inline]
fn calculate_cu_price(context: &Context) -> Result<ComputedUnitsPriceType> {
    let result = if let Some(cu_price) = context.recent_cu_price(&context.spl_token_program_id) {
        // Bump up CU price for a bit to make the transaction more attractive
        let mut result = bump_cu_price(cu_price);

        if result > context.max_cu_price {
            warn!(
                "The calculated Arbitrage CU price {} is greater than the configured MAX CU Price {}. The latter will be used instead.",
                  result,context.max_cu_price);
            result = context.max_cu_price;
        }

        if result < context.min_cu_price {
            debug!(
                "The calculated Arbitrage CU price {} is less than the configured MIN CU Price {}. The latter will be used instead.",
                  result,context.min_cu_price);
            result = context.min_cu_price
        }

        result
    } else {
        error!(
            "Recent CU price is missing for the SPL Token Program ({})! Using the configured MAX CU Price ({}).",
            &context.spl_token_program_id, context.max_cu_price
        );
        context.max_cu_price
    };

    Ok(result)
}

const PRICE_BUMP: DecimalPercentageType = 0.5;
fn bump_cu_price(price: ComputedUnitsPriceType) -> ComputedUnitsPriceType {
    price + (price as DecimalPercentageType * PRICE_BUMP) as ComputedUnitsPriceType
}

#[cfg(test)]
mod tests {
    use {
        super::{bump_cu_price, calculate_cu_price, create_swap_transaction},
        crate::{
            processor::create_arbitrage_transaction,
            test_util::tests::{generate_context, MOCK_CU_PRICE1, MOCK_CU_PRICE2},
            ComputedUnitsPriceType,
        },
        common::{
            message::{ArbitrageMessage, SequenceId, SwapMessage, SwapType},
            AmountType, LiquidityPool, LiquidityPoolState,
        },
        solana_sdk::{clock::Slot, signer::Signer},
    };

    const SLOT: Slot = 1234;
    const SWAP_BASE_QTY: AmountType = 100.0;
    const SWAP_BUY_QUOTE_AMOUNT: AmountType = 23456.78;
    const SWAP_SELL_QUOTE_AMOUNT: AmountType = 34567.89;

    #[test]
    fn test_bump_price() {
        assert_eq!(bump_cu_price(100), 150);
        assert_eq!(bump_cu_price(200), 300);
        assert_eq!(bump_cu_price(123), 184);
        assert_eq!(bump_cu_price(456), 684);
    }

    #[test]
    fn test_process_arbitrate() {
        let mut context = generate_context();
        assert!(context.liquidity_pools().len() >= 2);

        let lps = context.liquidity_pools();
        let buy_lp = lps.values().next().unwrap().clone();
        let buy_lp_state = LiquidityPoolState::new(buy_lp.clone());
        let sell_lp = lps.values().next().unwrap().clone();
        let sell_lp_state = LiquidityPoolState::new(sell_lp.clone());

        context.update_recent_cu_price(&LiquidityPool::program_id(&buy_lp), MOCK_CU_PRICE1);
        context.update_recent_cu_price(&LiquidityPool::program_id(&sell_lp), MOCK_CU_PRICE2);

        let msg = ArbitrageMessage::new(
            SequenceId::new(),
            SLOT,
            &buy_lp_state,
            &sell_lp_state,
            SWAP_BASE_QTY,
            SWAP_BUY_QUOTE_AMOUNT,
            SWAP_SELL_QUOTE_AMOUNT,
        );

        let (txn, _, _) = create_arbitrage_transaction(&context, &msg).unwrap();
        assert_eq!(txn.message().account_keys.len(), 10);
        assert_eq!(txn.message().instructions.len(), 4);
        assert_eq!(txn.signatures.len(), 1);
        assert_eq!(txn.message.signer_keys()[0], &context.wallet().keypair().pubkey());
    }

    #[test]
    fn test_process_swap() {
        let mut context = generate_context();
        assert!(context.liquidity_pools().len() >= 2);
        let lp = context.liquidity_pools().values().next().unwrap().clone();
        context.update_recent_cu_price(&LiquidityPool::program_id(&lp), MOCK_CU_PRICE1);

        let msg = SwapMessage {
            sequence_id: SequenceId::new(),
            lp_name: LiquidityPool::name(&lp).to_string(),
            lp_address: *LiquidityPool::address(&lp),
            swap_type: SwapType::Buy,
            base_qty: SWAP_BASE_QTY,
            quote_amount: SWAP_BUY_QUOTE_AMOUNT,
        };
        let (txn, _, _) = create_swap_transaction(&context, &msg).unwrap();
        assert_eq!(txn.message().instructions.len(), 3);
        assert_eq!(txn.signatures.len(), 1);
        assert_eq!(txn.message.signer_keys()[0], &context.wallet().keypair().pubkey());
        assert_eq!(txn.message().account_keys.len(), 10);
    }

    #[test]
    fn test_calculate_arbitrage_cu_price() {
        let mut context = generate_context();
        let spl_token_program_id = context.spl_token_program_id;

        // No recent CU price. Should return the configured MAX CU price.
        assert_eq!(calculate_cu_price(&context,).unwrap(), context.max_cu_price);

        // Bump is less than the configured MAX CU price. Should return the recent CU price
        let mut cu_price: ComputedUnitsPriceType =
            (context.max_cu_price * 2).abs_diff(bump_cu_price(context.max_cu_price) - 1);
        context.update_recent_cu_price(&spl_token_program_id, cu_price);
        assert_eq!(calculate_cu_price(&context).unwrap(), bump_cu_price(cu_price));

        // Bump is greater than the configured MAC CU price. Should return the configured MAX CU price.
        cu_price = bump_cu_price(context.max_cu_price);
        context.update_recent_cu_price(&spl_token_program_id, cu_price);
        assert_eq!(calculate_cu_price(&context,).unwrap(), context.max_cu_price);

        // Calculated price is less than the configured MIN CU price.
        cu_price = (context.min_cu_price / 2) as ComputedUnitsPriceType;
        context.update_recent_cu_price(&spl_token_program_id, cu_price);
        assert_eq!(calculate_cu_price(&context,).unwrap(), context.min_cu_price);
    }
}
