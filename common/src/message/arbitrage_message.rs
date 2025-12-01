use {
    super::SequenceId,
    crate::{AmountType, LiquidityPool, LiquidityPoolState, PriceType},
    solana_sdk::{clock::Slot, pubkey::Pubkey, signature::Signature},
};

#[derive(Debug)]
pub struct ArbitrageSide {
    pub lp_name: String,
    pub lp_address: Pubkey,
    pub base_qty: AmountType,
    pub quote_amount: AmountType,
    pub price: PriceType,
    pub fee: PriceType,
    pub lp_last_upd_slot: Slot,
    pub lp_last_upd_txn_sig: Option<Signature>,
}

#[derive(Debug)]
pub struct ArbitrageMessage {
    pub sequence_id: SequenceId,
    pub slot: Slot,
    pub buy_side_info: ArbitrageSide,
    pub sell_side_info: ArbitrageSide,
    pub swap_base_qty: AmountType,
    pub swap_buy_quote_amount: AmountType,
    pub swap_sell_quote_amount: AmountType,
}

impl ArbitrageMessage {
    pub fn new(
        sequence_id: SequenceId,
        slot: Slot,
        buy_lp_state: &LiquidityPoolState,
        sell_lp_state: &LiquidityPoolState,
        swap_base_qty: AmountType,
        swap_buy_quote_amount: AmountType,
        swap_sell_quote_amount: AmountType,
    ) -> Self {
        ArbitrageMessage {
            sequence_id,
            slot,
            buy_side_info: ArbitrageSide {
                lp_name: LiquidityPool::name(&buy_lp_state.liquidity_pool).to_string(),
                lp_address: *LiquidityPool::address(&buy_lp_state.liquidity_pool),
                base_qty: buy_lp_state.base_token_amount(),
                quote_amount: buy_lp_state.quote_token_amount(),
                price: buy_lp_state.calc_price_to_buy(),
                fee: buy_lp_state.fee_ratio,
                lp_last_upd_slot: buy_lp_state.latest_upd_slot,
                lp_last_upd_txn_sig: buy_lp_state.latest_upd_txn_sig,
            },
            sell_side_info: ArbitrageSide {
                lp_name: LiquidityPool::name(&sell_lp_state.liquidity_pool).to_string(),
                lp_address: *LiquidityPool::address(&sell_lp_state.liquidity_pool),
                base_qty: sell_lp_state.base_token_amount(),
                quote_amount: sell_lp_state.quote_token_amount(),
                price: sell_lp_state.calc_price_for_selling(),
                fee: sell_lp_state.fee_ratio,
                lp_last_upd_slot: sell_lp_state.latest_upd_slot,
                lp_last_upd_txn_sig: sell_lp_state.latest_upd_txn_sig,
            },
            swap_base_qty,
            swap_buy_quote_amount,
            swap_sell_quote_amount,
        }
    }

    pub fn swap_quote_amount_margin(&self) -> AmountType {
        self.swap_sell_quote_amount - self.swap_buy_quote_amount
    }
}

impl PartialEq for ArbitrageMessage {
    fn eq(&self, other: &Self) -> bool {
        // Purposely comparing on the selected set of fields
        self.sequence_id == other.sequence_id
            && self.slot == other.slot
            && self.buy_side_info.lp_address == other.buy_side_info.lp_address
            && self.sell_side_info.lp_address == other.sell_side_info.lp_address
            && self.buy_side_info.lp_last_upd_slot == other.buy_side_info.lp_last_upd_slot
            && self.sell_side_info.lp_last_upd_slot == other.sell_side_info.lp_last_upd_slot
            && self.buy_side_info.lp_last_upd_txn_sig == other.buy_side_info.lp_last_upd_txn_sig
            && self.sell_side_info.lp_last_upd_txn_sig == other.sell_side_info.lp_last_upd_txn_sig
            && self.swap_base_qty == other.swap_base_qty
            && self.swap_buy_quote_amount == other.swap_buy_quote_amount
            && self.swap_sell_quote_amount == other.swap_sell_quote_amount
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{ArbitrageMessage, ArbitrageSide},
        crate::{
            calc_token_amount,
            message::SequenceId,
            test_util::tests::{create_liquidity_pool, create_raydium_amm_lp},
            AccountType, AmountType, LiquidityPool, LiquidityPoolState, PriceType, TokenDigitsType,
        },
        solana_sdk::clock::Slot,
    };

    const SLOT: Slot = 123;
    const BUY_BASE_AMOUNT: TokenDigitsType = 1000;
    const BUY_QUOTE_AMOUNT: TokenDigitsType = 200000;
    const BUY_FEE: PriceType = 1.2;
    const BUY_LP_SLOT: Slot = 123;
    const SELL_BASE_AMOUNT: TokenDigitsType = 3000;
    const SELL_QUOTE_AMOUNT: TokenDigitsType = 400000;
    const SELL_FEE: PriceType = 3.4;
    const SELL_LP_SLOT: Slot = 456;
    const SWAP_BASE_QTY: AmountType = 0.7;
    const SWAP_BUY_QUOTE_AMOUNT: AmountType = 7.9;
    const SWAP_SELL_QUOTE_AMOUNT: AmountType = 8.0;

    fn deep_copy(src: &ArbitrageMessage) -> ArbitrageMessage {
        let buy_side_info = ArbitrageSide {
            lp_name: src.buy_side_info.lp_name.clone(),
            lp_address: src.buy_side_info.lp_address,
            base_qty: src.buy_side_info.base_qty,
            quote_amount: src.buy_side_info.quote_amount,
            price: src.buy_side_info.price,
            fee: src.buy_side_info.fee,
            lp_last_upd_slot: src.buy_side_info.lp_last_upd_slot,
            lp_last_upd_txn_sig: src.buy_side_info.lp_last_upd_txn_sig,
        };
        let sell_side_info = ArbitrageSide {
            lp_name: src.sell_side_info.lp_name.clone(),
            lp_address: src.sell_side_info.lp_address,
            base_qty: src.sell_side_info.base_qty,
            quote_amount: src.sell_side_info.quote_amount,
            price: src.sell_side_info.price,
            fee: src.sell_side_info.fee,
            lp_last_upd_slot: src.sell_side_info.lp_last_upd_slot,
            lp_last_upd_txn_sig: src.sell_side_info.lp_last_upd_txn_sig,
        };
        ArbitrageMessage {
            sequence_id: src.sequence_id,
            slot: src.slot,
            buy_side_info,
            sell_side_info,
            swap_base_qty: src.swap_base_qty,
            swap_buy_quote_amount: src.swap_buy_quote_amount,
            swap_sell_quote_amount: src.swap_sell_quote_amount,
        }
    }

    #[test]
    fn test_create_arbitrage() {
        let buy_lp = create_raydium_amm_lp();
        let mut buy_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(buy_lp));
        buy_lp_state.base_token_digits = BUY_BASE_AMOUNT;
        buy_lp_state.quote_token_digits = BUY_QUOTE_AMOUNT;
        buy_lp_state.fee_ratio = BUY_FEE;
        buy_lp_state.latest_upd_slot = BUY_LP_SLOT;

        let sell_lp = create_raydium_amm_lp();
        let mut sell_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(sell_lp));
        sell_lp_state.base_token_digits = SELL_BASE_AMOUNT;
        sell_lp_state.quote_token_digits = SELL_QUOTE_AMOUNT;
        sell_lp_state.fee_ratio = SELL_FEE;
        sell_lp_state.latest_upd_slot = SELL_LP_SLOT;

        let sequence_id = SequenceId::new();
        let arb = ArbitrageMessage::new(
            sequence_id,
            SLOT,
            &buy_lp_state,
            &sell_lp_state,
            SWAP_BASE_QTY,
            SWAP_BUY_QUOTE_AMOUNT,
            SWAP_SELL_QUOTE_AMOUNT,
        );
        assert_eq!(arb.sequence_id, sequence_id);

        assert_eq!(
            arb.buy_side_info.base_qty,
            calc_token_amount(
                BUY_BASE_AMOUNT,
                LiquidityPool::base_token(&buy_lp_state.liquidity_pool).decimals()
            )
        );
        assert_eq!(
            arb.buy_side_info.quote_amount,
            calc_token_amount(
                BUY_QUOTE_AMOUNT,
                LiquidityPool::quote_token(&buy_lp_state.liquidity_pool).decimals()
            )
        );
        assert_eq!(arb.buy_side_info.fee, BUY_FEE);
        assert_eq!(arb.buy_side_info.lp_last_upd_slot, BUY_LP_SLOT);
        assert_eq!(
            arb.buy_side_info.lp_last_upd_txn_sig,
            buy_lp_state.latest_upd_txn_sig
        );

        assert_eq!(arb.sell_side_info.fee, SELL_FEE);
        assert_eq!(
            arb.sell_side_info.base_qty,
            calc_token_amount(
                SELL_BASE_AMOUNT,
                LiquidityPool::base_token(&sell_lp_state.liquidity_pool).decimals()
            )
        );
        assert_eq!(
            arb.sell_side_info.quote_amount,
            calc_token_amount(
                SELL_QUOTE_AMOUNT,
                LiquidityPool::quote_token(&sell_lp_state.liquidity_pool).decimals()
            )
        );
        assert_eq!(arb.sell_side_info.lp_last_upd_slot, SELL_LP_SLOT);
        assert_eq!(
            arb.sell_side_info.lp_last_upd_txn_sig,
            sell_lp_state.latest_upd_txn_sig
        );

        assert_eq!(arb.swap_base_qty, SWAP_BASE_QTY);
        assert_eq!(arb.swap_buy_quote_amount, SWAP_BUY_QUOTE_AMOUNT);
        assert_eq!(arb.swap_sell_quote_amount, SWAP_SELL_QUOTE_AMOUNT);
    }

    #[test]
    fn test_copy_and_equal() {
        let buy_lp_state =
            LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount));
        let sell_lp_state =
            LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount));

        let arb1 = ArbitrageMessage::new(
            SequenceId::new(),
            SLOT,
            &buy_lp_state,
            &sell_lp_state,
            SWAP_BASE_QTY,
            SWAP_BUY_QUOTE_AMOUNT,
            SWAP_SELL_QUOTE_AMOUNT,
        );

        let mut arb2 = deep_copy(&arb1);
        assert_eq!(arb1, arb2);

        arb2.buy_side_info.lp_name = "dummy".to_string();
        assert_eq!(arb1, arb2);

        arb2.sell_side_info.lp_last_upd_slot = 1;
        assert_ne!(arb1, arb2);

        // Same arb, diff slot
        arb2 = deep_copy(&arb1);
        assert_eq!(arb1, arb2);
        arb2.slot = arb1.slot + 1;
        assert_ne!(arb1, arb2);

        // Same arb, diff sequence id.
        arb2 = deep_copy(&arb1);
        assert_eq!(arb1, arb2);
        arb2.sequence_id.minor = arb1.sequence_id.minor + 1;
        assert_ne!(arb1, arb2);
    }
}
