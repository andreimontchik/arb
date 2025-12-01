use {
    crate::{context::Context, processor},
    common::{AmountType, RaydiumAmmLp},
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signer::Signer,
    },
};

const SWAP_CODE_BASE_IN: u8 = 9; // Hardcoded in the Raydium AMM Program: https://github.com/raydium-io/raydium-amm/blob/ae039d21cd49ef670d76b3a1cf5485ae0213dc5e/program/src/instruction.rs#L491
const SWAP_CODE_BASE_OUT: u8 = 11; //Hardcoded in the Raydium AMM Program: https://github.com/raydium-io/raydium-amm/blob/ae039d21cd49ef670d76b3a1cf5485ae0213dc5e/program/src/instruction.rs#L503

fn pack_swap_data(swap_code: u8, amount_in: u64, amount_out: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.push(swap_code);
    result.extend_from_slice(&amount_in.to_le_bytes());
    result.extend_from_slice(&amount_out.to_le_bytes());

    result
}

pub(crate) fn generate_swap_buy_instruction(
    context: &Context,
    base_qty: AmountType,
    quote_amount: AmountType,
    lp: &RaydiumAmmLp,
) -> processor::Result<Instruction> {
    // Round down to prevent going over the remaining base tokens.
    let normalized_base_qty: u64 =
        (base_qty * 10u64.pow(lp.base_token.decimals() as u32) as f64).floor() as u64;
    // Round up to prevent paying less than the calculated quote amount
    let normalized_quote_amount: u64 =
        (quote_amount * 10u64.pow(lp.quote_token.decimals() as u32) as f64).ceil() as u64;
    let data = pack_swap_data(SWAP_CODE_BASE_OUT, normalized_quote_amount, normalized_base_qty);

    let base_token_address = context.wallet().token_account(&lp.base_token.code())?.address();
    let quote_token_address = context.wallet().token_account(&lp.quote_token.code())?.address();

    let accounts = vec![
        // spl token program
        AccountMeta::new_readonly(context.spl_token_program_id, false),
        // AMM
        AccountMeta::new(lp.address, false),
        AccountMeta::new_readonly(lp.authority, false),
        AccountMeta::new(lp.address, false),
        // The AMM target orders account is no longer needed.
        AccountMeta::new(lp.base_token_vault, false),
        AccountMeta::new(lp.quote_token_vault, false),
        // Market
        AccountMeta::new_readonly(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new_readonly(lp.address, false),
        // User
        AccountMeta::new(*quote_token_address, false), // my quote token
        AccountMeta::new(*base_token_address, false),  // my base token
        AccountMeta::new_readonly(context.wallet().keypair().pubkey(), true), // My quote token account owner
    ];

    Ok(Instruction {
        program_id: lp.program_id,
        accounts,
        data,
    })
}

pub(crate) fn generate_swap_sell_instruction(
    context: &Context,
    base_qty: AmountType,
    quote_amount: AmountType,
    lp: &RaydiumAmmLp,
) -> processor::Result<Instruction> {
    // Round down to prevent going over the base token amount to sell.
    let normalized_base_qty: u64 =
        (base_qty * 10u64.pow(lp.base_token.decimals() as u32) as f64).floor() as u64;
    // Round down to prevent over charging.
    let normalized_quote_amount: u64 =
        (quote_amount * 10u64.pow(lp.quote_token.decimals() as u32) as f64).floor() as u64;
    let data = pack_swap_data(SWAP_CODE_BASE_IN, normalized_base_qty, normalized_quote_amount);

    let base_token_address = context.wallet().token_account(&lp.base_token.code())?.address();
    let quote_token_address = context.wallet().token_account(&lp.quote_token.code())?.address();

    let accounts = vec![
        // spl token program
        AccountMeta::new_readonly(context.spl_token_program_id, false),
        // amm
        AccountMeta::new(lp.address, false),
        AccountMeta::new_readonly(lp.authority, false),
        AccountMeta::new(lp.address, false),
        // The AMM target orders account is no longer needed.
        AccountMeta::new(lp.base_token_vault, false),
        AccountMeta::new(lp.quote_token_vault, false),
        // Market
        AccountMeta::new_readonly(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new(lp.address, false),
        AccountMeta::new_readonly(lp.address, false),
        // User
        AccountMeta::new(*base_token_address, false), // my base token
        AccountMeta::new(*quote_token_address, false), // my quote token
        AccountMeta::new_readonly(context.wallet().keypair().pubkey(), true), // My quote token account owner
    ];

    Ok(Instruction {
        program_id: lp.program_id,
        accounts,
        data,
    })
}

#[cfg(test)]
mod tests {
    use {
        super::pack_swap_data,
        crate::{
            processor::raydium::{
                generate_swap_buy_instruction, generate_swap_sell_instruction, SWAP_CODE_BASE_IN,
                SWAP_CODE_BASE_OUT,
            },
            test_util::tests::{create_liquidity_pool, create_raydium_amm_lp, generate_context},
        },
        common::{
            message::{ArbitrageMessage, SequenceId},
            AccountType, AmountType, LiquidityPool, LiquidityPoolState,
        },
        solana_sdk::{clock::Slot, signer::Signer},
    };

    const SLOT: Slot = 123456789;
    const SWAP_BASE_QTY: AmountType = 56.7;
    const SWAP_BUY_QUOTE_AMOUNT: AmountType = 1234.5;
    const SWAP_SELL_QUOTE_AMOUNT: AmountType = 2345.6;

    #[test]
    fn test_pack_swap_data() {
        let swap_code = 255u8;
        let amount_in = 1234u64;
        let amount_out = 5678u64;

        let data = pack_swap_data(swap_code, amount_in, amount_out);
        assert_eq!(swap_code, data[0]);
        assert_eq!(amount_in, u64::from_le_bytes(data[1..9].try_into().unwrap()));
        assert_eq!(amount_out, u64::from_le_bytes(data[9..17].try_into().unwrap()));
    }

    #[test]
    fn test_generate_swap_buy_instruction() {
        let context = generate_context();
        let wallet = context.wallet();

        let buy_lp = create_raydium_amm_lp();
        let buy_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(buy_lp.clone()));
        let sell_lp_state =
            LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount));

        let arb_msg = ArbitrageMessage::new(
            SequenceId::new(),
            SLOT,
            &buy_lp_state,
            &sell_lp_state,
            SWAP_BASE_QTY,
            SWAP_BUY_QUOTE_AMOUNT,
            SWAP_SELL_QUOTE_AMOUNT,
        );

        let instruction = generate_swap_buy_instruction(
            &context,
            arb_msg.swap_base_qty,
            arb_msg.swap_buy_quote_amount,
            &buy_lp,
        )
        .unwrap();

        // Program
        assert_eq!(&instruction.program_id, &buy_lp.program_id);

        // Data
        assert_eq!(instruction.data[0], SWAP_CODE_BASE_OUT);

        let quote_amount_in = u64::from_le_bytes(instruction.data[1..9].try_into().unwrap());
        assert_eq!(
            (SWAP_BUY_QUOTE_AMOUNT * 10u64.pow(sell_lp_state.quote_token_decimals() as u32) as f64)
                as u64,
            quote_amount_in
        );

        let base_qty_out = u64::from_le_bytes(instruction.data[9..17].try_into().unwrap());
        assert_eq!(
            (SWAP_BASE_QTY * 10u64.pow(sell_lp_state.base_token_decimals() as u32) as f64) as u64,
            base_qty_out
        );

        let base_token_address = *wallet.token_account(&buy_lp.base_token.code()).unwrap().address();
        let quote_token_address = *wallet
            .token_account(&buy_lp.quote_token.code())
            .unwrap()
            .address();

        //Accounts
        assert_eq!(instruction.accounts.len(), 17);
        assert_eq!(instruction.accounts[0].pubkey, context.spl_token_program_id);
        assert_eq!(instruction.accounts[1].pubkey, buy_lp.address);
        assert_eq!(instruction.accounts[2].pubkey, buy_lp.authority);
        assert_eq!(instruction.accounts[4].pubkey, buy_lp.base_token_vault);
        assert_eq!(instruction.accounts[5].pubkey, buy_lp.quote_token_vault);
        assert_eq!(instruction.accounts[14].pubkey, quote_token_address);
        assert_eq!(instruction.accounts[15].pubkey, base_token_address);
        assert_eq!(instruction.accounts[16].pubkey, wallet.keypair().pubkey());
    }
    #[test]
    fn test_generate_swap_sell_instruction() {
        let context = generate_context();
        let wallet = context.wallet();

        let buy_lp_state =
            LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount));
        let sell_lp = create_raydium_amm_lp();
        let sell_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(sell_lp.clone()));

        let arb_msg = ArbitrageMessage::new(
            SequenceId::new(),
            SLOT,
            &buy_lp_state,
            &sell_lp_state,
            SWAP_BASE_QTY,
            SWAP_BUY_QUOTE_AMOUNT,
            SWAP_SELL_QUOTE_AMOUNT,
        );
        let instruction = generate_swap_sell_instruction(
            &context,
            arb_msg.swap_base_qty,
            arb_msg.swap_sell_quote_amount,
            &sell_lp,
        )
        .unwrap();

        // Program
        assert_eq!(&instruction.program_id, &sell_lp.program_id);

        // Data
        assert_eq!(instruction.data[0], SWAP_CODE_BASE_IN);

        let base_qty_in = u64::from_le_bytes(instruction.data[1..9].try_into().unwrap());
        assert_eq!(
            (SWAP_BASE_QTY * 10u64.pow(sell_lp_state.base_token_decimals() as u32) as f64) as u64,
            base_qty_in
        );

        let quote_amount_out = u64::from_le_bytes(instruction.data[9..17].try_into().unwrap());
        assert_eq!(
            (SWAP_SELL_QUOTE_AMOUNT * 10u64.pow(sell_lp_state.quote_token_decimals() as u32) as f64)
                as u64,
            quote_amount_out
        );

        let base_token_address = *wallet
            .token_account(&sell_lp.base_token.code())
            .unwrap()
            .address();
        let quote_token_address = *wallet
            .token_account(&sell_lp.quote_token.code())
            .unwrap()
            .address();

        //Accounts
        assert_eq!(instruction.accounts.len(), 17);
        assert_eq!(instruction.accounts[0].pubkey, context.spl_token_program_id);
        assert_eq!(instruction.accounts[1].pubkey, sell_lp.address);
        assert_eq!(instruction.accounts[2].pubkey, sell_lp.authority);
        assert_eq!(instruction.accounts[4].pubkey, sell_lp.base_token_vault);
        assert_eq!(instruction.accounts[5].pubkey, sell_lp.quote_token_vault);
        assert_eq!(instruction.accounts[14].pubkey, base_token_address);
        assert_eq!(instruction.accounts[15].pubkey, quote_token_address);
        assert_eq!(instruction.accounts[16].pubkey, wallet.keypair().pubkey());
    }
}
