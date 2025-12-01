use {
    bigdecimal::{num_bigint::BigInt, BigDecimal, FromPrimitive, ToPrimitive},
    common::{
        LiquidityPoolState, PriceType, TokenDecimalsType, TokenDigitsType, ONE_HUNDRED_OF_BP_DECIMAL,
    },
    ported_from_orca::orca_tick_math,
    whirlpool_interface::Whirlpool,
};

pub(crate) type TickIndexType = i32;
type TickSpacingType = u16;

type SqrtPriceType = u128;

// 2^64. Used for performing the binary shift operations on u128 numbers
static TWO_TO_64: u128 = 18_446_744_073_709_551_616;

#[inline]
fn _rescale_bd_int_value(amount: BigDecimal, new_decimals: TokenDecimalsType) -> Option<BigDecimal> {
    Some(BigDecimal::new(
        BigInt::from_u128(amount.to_u128()?)?,
        new_decimals as i64,
    ))
}

#[inline]
fn sqrt_x64_price_to_price(
    sqrt_x64_price: SqrtPriceType,
    decimals_a: TokenDecimalsType,
    decimals_b: TokenDecimalsType,
) -> Option<BigDecimal> {
    // price = (sqrt_x64_price >> 64)^2 * 10^(decimals_a - decimals_b)
    let decimals_diff = if decimals_a >= decimals_b {
        decimals_a - decimals_b
    } else {
        decimals_b - decimals_a
    };
    let decimal_adjust = 10i64.checked_pow(decimals_diff as u32)?;
    let sqrt_price = BigDecimal::from_u128(sqrt_x64_price)? / TWO_TO_64;
    if decimals_a >= decimals_b {
        Some(&sqrt_price * &sqrt_price * decimal_adjust)
    } else {
        Some(&sqrt_price * &sqrt_price / decimal_adjust)
    }
}

#[allow(warnings)]
#[inline]
fn price_to_sqrt_x64_price(price: &BigDecimal, decimals_a: u32, decimals_b: u32) -> Option<u128> {
    // sqrt_x64_price = (price / 10^(decimals_a - decimals_b)) << 64
    let decimal_adjust = 10u32.pow(decimals_a - decimals_b);
    let result = (price / decimal_adjust).sqrt()? * TWO_TO_64;
    Some(result.round(0).to_u128()?)
}

#[allow(warnings)]
fn find_tick_array_index(
    current_tick_index: TickIndexType,
    tick_spacing: TickSpacingType,
    direction: bool, // true - next tick index, false - prior tick index
) -> Option<TickIndexType> {
    for i in 0..tick_spacing as TickIndexType {
        let result = if direction {
            current_tick_index + i
        } else {
            current_tick_index - i
        };

        if result % tick_spacing as TickIndexType == 0 {
            return Some(result);
        }
    }

    None
}

#[allow(warnings)]
fn find_prior_tick_array_index(
    current_tick_index: TickIndexType,
    tick_spacing: TickSpacingType,
) -> Option<TickIndexType> {
    return find_tick_array_index(current_tick_index - 1, tick_spacing, false);
}

#[allow(warnings)]
fn find_next_tick_array_index(
    current_tick_index: TickIndexType,
    tick_spacing: TickSpacingType,
) -> Option<TickIndexType> {
    return find_tick_array_index(current_tick_index + 1, tick_spacing, true);
}

// The Account Microscope was used for evaluation. https://everlastingsong.github.io/account-microscope/#/whirlpool/whirlpool/FwewVm8u6tFPGewAyHmWAqad9hmF7mvqxK4mJ7iNqqGC
// Token amounts are matching with ones belonging to the upper tick, rather than the curent tick.
// That makes me thinking that the liquidity is applied to the [current_tick, upper_tick) price range.
// That is probably Ok for buying base token at slightly higher price, but might not work for selling at lower price.
pub fn calc_token_amounts(account: &Whirlpool) -> Option<(TokenDigitsType, TokenDigitsType)> {
    let upper_tick_index = find_next_tick_array_index(account.tick_current_index, account.tick_spacing)?;
    let upper_sqrt_x64_price = orca_tick_math::sqrt_price_from_tick_index(upper_tick_index);

    let liquidity_bd = BigDecimal::from_u128(account.liquidity)?;
    let current_price_bd = BigDecimal::from_u128(account.sqrt_price)?;
    let upper_price_bd = BigDecimal::from_u128(upper_sqrt_x64_price)?;
    let price_diff_bd = &upper_price_bd - &current_price_bd;
    let price_mul_bd = &upper_price_bd * &current_price_bd;

    // token_a_amount = (liquidity << 64) * (upper_sqrt_x64_price - current_sqrt_x64_price) / (upper_sqrt_x64_price * current_sqrt_x64_price)
    let token_a_amount: BigDecimal = &liquidity_bd * &TWO_TO_64 * &price_diff_bd / &price_mul_bd;

    // token_b_amount = (liquidity * (upper_sqrt_x64_price - current_sqrt_x64_price)) >> 64
    let token_b_amount: BigDecimal = &liquidity_bd * &price_diff_bd / &TWO_TO_64;

    Some((token_a_amount.to_u64()?, token_b_amount.to_u64()?))
}

#[allow(dead_code)]
pub fn calc_price(whirlpool: &Whirlpool, lp_state: &LiquidityPoolState) -> PriceType {
    match sqrt_x64_price_to_price(
        whirlpool.sqrt_price,
        lp_state.base_token_decimals(),
        lp_state.quote_token_decimals(),
    ) {
        Some(price_bd) => price_bd.to_f64().unwrap_or(f64::NAN),
        None => f64::NAN,
    }
}

#[inline]
pub fn calc_fee(whirlpool: &Whirlpool) -> PriceType {
    whirlpool.fee_rate as PriceType * ONE_HUNDRED_OF_BP_DECIMAL
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        common::{
            test_util::tests::{create_orca_whirlpool_account, create_raydium_amm_lp},
            LiquidityPool,
        },
    };

    #[test]
    fn test_find_prior_tick_index_negative() {
        assert_eq!(find_prior_tick_array_index(-100, 1).unwrap(), -101);
        assert_eq!(find_prior_tick_array_index(-100, 2).unwrap(), -102);
        assert_eq!(find_prior_tick_array_index(-100, 3).unwrap(), -102);
        assert_eq!(find_prior_tick_array_index(-100, 4).unwrap(), -104);
        assert_eq!(find_prior_tick_array_index(-100, 5).unwrap(), -105);
        assert_eq!(find_prior_tick_array_index(-100, 6).unwrap(), -102);
        assert_eq!(find_prior_tick_array_index(-100, 7).unwrap(), -105);
        assert_eq!(find_prior_tick_array_index(-100, 8).unwrap(), -104);
        assert_eq!(find_prior_tick_array_index(-100, 9).unwrap(), -108);

        assert_eq!(find_prior_tick_array_index(-1000, 2).unwrap(), -1002);
        assert_eq!(find_prior_tick_array_index(-1000, 4).unwrap(), -1004);
        assert_eq!(find_prior_tick_array_index(-1000, 8).unwrap(), -1008);
        assert_eq!(find_prior_tick_array_index(-1000, 16).unwrap(), -1008);
        assert_eq!(find_prior_tick_array_index(-1000, 32).unwrap(), -1024);
        assert_eq!(find_prior_tick_array_index(-1000, 64).unwrap(), -1024);
        assert_eq!(find_prior_tick_array_index(-1000, 128).unwrap(), -1024);
        assert_eq!(find_prior_tick_array_index(-1000, 256).unwrap(), -1024);
    }

    #[test]
    fn test_find_prior_tick_positive() {
        assert_eq!(find_prior_tick_array_index(100, 1).unwrap(), 99);
        assert_eq!(find_prior_tick_array_index(100, 2).unwrap(), 98);
        assert_eq!(find_prior_tick_array_index(100, 3).unwrap(), 99);
        assert_eq!(find_prior_tick_array_index(100, 4).unwrap(), 96);
        assert_eq!(find_prior_tick_array_index(100, 5).unwrap(), 95);
        assert_eq!(find_prior_tick_array_index(100, 6).unwrap(), 96);
        assert_eq!(find_prior_tick_array_index(100, 7).unwrap(), 98);
        assert_eq!(find_prior_tick_array_index(100, 8).unwrap(), 96);
        assert_eq!(find_prior_tick_array_index(100, 9).unwrap(), 99);

        assert_eq!(find_prior_tick_array_index(1000, 2).unwrap(), 998);
        assert_eq!(find_prior_tick_array_index(1000, 4).unwrap(), 996);
        assert_eq!(find_prior_tick_array_index(1000, 8).unwrap(), 992);
        assert_eq!(find_prior_tick_array_index(1000, 16).unwrap(), 992);
        assert_eq!(find_prior_tick_array_index(1000, 32).unwrap(), 992);
        assert_eq!(find_prior_tick_array_index(1000, 64).unwrap(), 960);
        assert_eq!(find_prior_tick_array_index(1000, 128).unwrap(), 896);
        assert_eq!(find_prior_tick_array_index(1000, 256).unwrap(), 768);
    }

    #[test]
    fn test_find_next_tick_index_negative() {
        assert_eq!(find_next_tick_array_index(-100, 1).unwrap(), -99);
        assert_eq!(find_next_tick_array_index(-100, 2).unwrap(), -98);
        assert_eq!(find_next_tick_array_index(-100, 3).unwrap(), -99);
        assert_eq!(find_next_tick_array_index(-100, 4).unwrap(), -96);
        assert_eq!(find_next_tick_array_index(-100, 5).unwrap(), -95);
        assert_eq!(find_next_tick_array_index(-100, 6).unwrap(), -96);
        assert_eq!(find_next_tick_array_index(-100, 7).unwrap(), -98);
        assert_eq!(find_next_tick_array_index(-100, 8).unwrap(), -96);
        assert_eq!(find_next_tick_array_index(-100, 9).unwrap(), -99);

        assert_eq!(find_next_tick_array_index(-1000, 2).unwrap(), -998);
        assert_eq!(find_next_tick_array_index(-1000, 4).unwrap(), -996);
        assert_eq!(find_next_tick_array_index(-1000, 8).unwrap(), -992);
        assert_eq!(find_next_tick_array_index(-1000, 16).unwrap(), -992);
        assert_eq!(find_next_tick_array_index(-1000, 32).unwrap(), -992);
        assert_eq!(find_next_tick_array_index(-1000, 64).unwrap(), -960);
        assert_eq!(find_next_tick_array_index(-1000, 128).unwrap(), -896);
        assert_eq!(find_next_tick_array_index(-1000, 256).unwrap(), -768);
    }

    #[test]
    fn test_find_next_tick_index_positive() {
        assert_eq!(find_next_tick_array_index(100, 1).unwrap(), 101);
        assert_eq!(find_next_tick_array_index(100, 2).unwrap(), 102);
        assert_eq!(find_next_tick_array_index(100, 3).unwrap(), 102);
        assert_eq!(find_next_tick_array_index(100, 4).unwrap(), 104);
        assert_eq!(find_next_tick_array_index(100, 5).unwrap(), 105);
        assert_eq!(find_next_tick_array_index(100, 6).unwrap(), 102);
        assert_eq!(find_next_tick_array_index(100, 7).unwrap(), 105);
        assert_eq!(find_next_tick_array_index(100, 8).unwrap(), 104);
        assert_eq!(find_next_tick_array_index(100, 9).unwrap(), 108);

        assert_eq!(find_next_tick_array_index(1000, 2).unwrap(), 1002);
        assert_eq!(find_next_tick_array_index(1000, 4).unwrap(), 1004);
        assert_eq!(find_next_tick_array_index(1000, 8).unwrap(), 1008);
        assert_eq!(find_next_tick_array_index(1000, 16).unwrap(), 1008);
        assert_eq!(find_next_tick_array_index(1000, 32).unwrap(), 1024);
        assert_eq!(find_next_tick_array_index(1000, 64).unwrap(), 1024);
        assert_eq!(find_next_tick_array_index(1000, 128).unwrap(), 1024);
        assert_eq!(find_next_tick_array_index(1000, 256).unwrap(), 1024);
    }

    #[test]
    fn test_sqrt_x64_price_to_price() {
        assert_eq!(
            sqrt_x64_price_to_price(100 * TWO_TO_64, 2, 2),
            Some(BigDecimal::from(10_000))
        );
        assert_eq!(
            sqrt_x64_price_to_price(100 * TWO_TO_64, 3, 2),
            Some(BigDecimal::from(100_000))
        );
        assert_eq!(
            sqrt_x64_price_to_price(100 * TWO_TO_64, 2, 3),
            Some(BigDecimal::from(1_000))
        );

        let src_sqrt_x64_price = 6723192041691824850;
        let price = sqrt_x64_price_to_price(src_sqrt_x64_price, 9, 6)
            .unwrap()
            .with_scale(6);
        println!("price: {}", price);
        let converted_sqrt_x64_price = price_to_sqrt_x64_price(&price, 9, 6).unwrap();
        assert_eq!(src_sqrt_x64_price, converted_sqrt_x64_price);
    }

    #[test]
    fn test_sqrt_x64_price_to_price_edge_cases() {
        let zero_bd: BigDecimal = BigDecimal::from(0);
        assert_eq!(sqrt_x64_price_to_price(0, 4, 2), Some(zero_bd.clone()));
        assert_eq!(
            sqrt_x64_price_to_price(u128::MIN, 4, 2),
            Some(BigDecimal::from(0))
        );
        assert!(sqrt_x64_price_to_price(u128::MAX, 4, 2) > Some(zero_bd.clone()));
        assert!(sqrt_x64_price_to_price(100, 0, 2) > Some(zero_bd.clone()));
        assert!(sqrt_x64_price_to_price(100, u8::MIN, 2) > Some(zero_bd.clone()));
        assert_eq!(sqrt_x64_price_to_price(1, u8::MAX, 2), None);
        assert!(sqrt_x64_price_to_price(100, 4, 0) > Some(zero_bd.clone()));
        assert!(sqrt_x64_price_to_price(100, 4, u8::MIN) > Some(zero_bd.clone()));
        assert_eq!(sqrt_x64_price_to_price(100, 4, u8::MAX), None);
        assert!(sqrt_x64_price_to_price(u128::MAX, u8::MAX, u8::MAX) > Some(zero_bd.clone()));
        assert_eq!(
            sqrt_x64_price_to_price(u128::MIN, u8::MIN, u8::MIN),
            Some(zero_bd.clone())
        );
    }

    #[test]
    fn test_get_token_amounts_from_liquidity() {
        let mut whirlpool = create_orca_whirlpool_account().0;
        whirlpool.liquidity = 93538458143942;
        whirlpool.sqrt_price = 7933124927893604393;
        whirlpool.tick_current_index = -16878;
        whirlpool.tick_spacing = 2;
        let token_a_decimals: TokenDecimalsType = 9;
        let token_b_decimals: TokenDecimalsType = 6;

        if let Some(token_amounts) = calc_token_amounts(&whirlpool) {
            println!(
                "liquidity: {}, sqrtPrice: {}, tickCurrentIndex: {} => TokenAmounts: {:?}",
                whirlpool.liquidity, whirlpool.sqrt_price, whirlpool.tick_current_index, token_amounts
            );
            assert!(token_amounts.0.abs_diff(18062250703) <= token_a_decimals as u64);
            assert!(token_amounts.1.abs_diff(3340850754) <= token_b_decimals as u64);
        } else {
            panic!("Failed to calculate token amounts for {:?}!", whirlpool);
        }
    }

    #[test]
    fn test_calculate_token_amounts() {
        let x64: BigDecimal = BigDecimal::from(BigInt::from_u128(1u128 << 64).unwrap());

        let liquidity = BigDecimal::from_u128(94863249504846).unwrap();
        let current_sqrt_x64_price = BigDecimal::from_u128(7941171036952692651).unwrap();
        let current_tick_index = -16858;
        let tick_spacing = 2;

        let upper_sqrt_x64_price = BigDecimal::from_u128(orca_tick_math::sqrt_price_from_tick_index(
            current_tick_index + tick_spacing,
        ))
        .unwrap();

        let token_a_amount = &liquidity * &x64 * (&upper_sqrt_x64_price - &current_sqrt_x64_price)
            / (&current_sqrt_x64_price * &upper_sqrt_x64_price);
        let adjusted_token_a_amount: f64 = token_a_amount.to_u64().unwrap() as f64 / 10i32.pow(9) as f64;

        let token_b_amount = &liquidity * (&upper_sqrt_x64_price - &current_sqrt_x64_price) / &x64;
        let adjusted_token_b_amount: f64 = token_b_amount.to_u64().unwrap() as f64 / 10i32.pow(6) as f64;

        println!("liquidity: {}, current_tick_index: {}, current_sqrt_x64_price: {}, upper_sqrt_x64_price: {}", 
            &liquidity, &current_tick_index, &current_sqrt_x64_price, &upper_sqrt_x64_price);
        println!(
            "token_a_amount: {}, token_b_amount: {}",
            &adjusted_token_a_amount, &adjusted_token_b_amount
        );
    }

    #[test]
    fn test_calc_price() {
        let mut whirlpool = create_orca_whirlpool_account().0;

        whirlpool.sqrt_price = 100 * TWO_TO_64;

        // Successful calculation
        let lp = create_raydium_amm_lp();
        let lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(lp.clone()));
        assert_eq!(calc_price(&whirlpool, &lp_state), 10_000_000f64);

        // Failed calculation
        /* Couldn't figure out how to repro this :-(
        lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(lp));
        whirlpool.sqrt_price = u128::MAX;
        assert!(calc_price(&whirlpool, &lp_state).is_nan());
        */
    }

    #[test]
    fn test_calc_fee() {
        let mut wp = create_orca_whirlpool_account().0;

        wp.fee_rate = 1_000;
        assert_eq!(calc_fee(&wp), 0.001);

        wp.fee_rate = 0;
        assert_eq!(calc_fee(&wp), 0f64);
    }
}
