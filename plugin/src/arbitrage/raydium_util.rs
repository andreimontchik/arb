use {
    common::PriceType,
    ported_from_raydium::{AmmState, AmmStatus},
    raydium_amm_interface::AmmInfo,
};

#[inline]
pub fn calc_fee(amm_info: &AmmInfo) -> PriceType {
    amm_info.fees.swap_fee_numerator as PriceType / amm_info.fees.swap_fee_denominator as PriceType
}

// TODO: Consider also evaluating the AmmInfo.reset_flag
pub fn calc_healthy_flag(amm_info: &AmmInfo) -> Option<bool> {
    let status = AmmStatus::from_u64(amm_info.status)?;
    let state = AmmState::from_u64(amm_info.state)?;
    Some(status.swap_permission() && state.swap_permission())
}

#[cfg(test)]
mod tests {
    use {super::*, common::test_util::tests::create_raydium_amm_info};

    #[test]
    fn test_calc_fee() {
        let mut amm_info = create_raydium_amm_info();

        amm_info.fees.swap_fee_numerator = 100;
        amm_info.fees.swap_fee_denominator = 2;
        assert_eq!(calc_fee(&amm_info), 50f64);
    }

    #[test]
    fn test_calc_fee_edge_cases() {
        let mut amm_info = create_raydium_amm_info();

        amm_info.fees.swap_fee_numerator = 0;
        amm_info.fees.swap_fee_denominator = 2;
        assert_eq!(calc_fee(&amm_info), 0f64);

        amm_info.fees.swap_fee_numerator = 100;
        amm_info.fees.swap_fee_denominator = 0;
        assert_eq!(calc_fee(&amm_info), f64::INFINITY);

        amm_info.fees.swap_fee_numerator = 0;
        amm_info.fees.swap_fee_denominator = 0;
        assert!(calc_fee(&amm_info).is_nan());

        amm_info.fees.swap_fee_numerator = u64::MIN;
        amm_info.fees.swap_fee_denominator = u64::MIN;
        assert!(calc_fee(&amm_info).is_nan());

        amm_info.fees.swap_fee_numerator = u64::MAX;
        amm_info.fees.swap_fee_denominator = u64::MAX;
        assert_eq!(calc_fee(&amm_info), 1f64);
    }

    #[test]
    fn test_calc_healthy_flag() {
        // Good status and state
        let mut amm_info = create_raydium_amm_info();
        amm_info.status = AmmStatus::Initialized.into_u64();
        amm_info.state = AmmState::IdleState.into_u64();
        assert!(calc_healthy_flag(&amm_info).unwrap());

        // Bad status, good state
        amm_info.status = AmmStatus::Disabled.into_u64();
        amm_info.state = AmmState::IdleState.into_u64();
        assert!(!calc_healthy_flag(&amm_info).unwrap());

        // Good status, bad state
        amm_info.status = AmmStatus::Initialized.into_u64();
        amm_info.state = AmmState::InvlidState.into_u64();
        assert!(!calc_healthy_flag(&amm_info).unwrap());

        // Bad status and state
        amm_info.status = AmmStatus::Disabled.into_u64();
        amm_info.state = AmmState::InvlidState.into_u64();
        assert!(!calc_healthy_flag(&amm_info).unwrap());
    }
}
