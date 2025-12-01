// Ported from https://github.com/raydium-io/raydium-amm/blob/18bed23de5a29b038a93db493fafd5ff4ee5386b/program/src/state.rs#L208
#[repr(u64)]
pub enum AmmStatus {
    Uninitialized = 0u64,
    Initialized = 1u64,
    Disabled = 2u64,
    WithdrawOnly = 3u64,
    // pool only can add or remove liquidity, can't swap and plan orders
    LiquidityOnly = 4u64,
    // pool only can add or remove liquidity and plan orders, can't swap
    OrderBookOnly = 5u64,
    // pool only can add or remove liquidity and swap, can't plan orders
    SwapOnly = 6u64,
    // pool status after created and will auto update to SwapOnly during swap after open_time
    WaitingTrade = 7u64,
}

impl AmmStatus {
    pub fn from_u64(status: u64) -> Option<Self> {
        match status {
            0u64 => Some(AmmStatus::Uninitialized),
            1u64 => Some(AmmStatus::Initialized),
            2u64 => Some(AmmStatus::Disabled),
            3u64 => Some(AmmStatus::WithdrawOnly),
            4u64 => Some(AmmStatus::LiquidityOnly),
            5u64 => Some(AmmStatus::OrderBookOnly),
            6u64 => Some(AmmStatus::SwapOnly),
            7u64 => Some(AmmStatus::WaitingTrade),
            _ => None,
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmStatus::Uninitialized => 0u64,
            AmmStatus::Initialized => 1u64,
            AmmStatus::Disabled => 2u64,
            AmmStatus::WithdrawOnly => 3u64,
            AmmStatus::LiquidityOnly => 4u64,
            AmmStatus::OrderBookOnly => 5u64,
            AmmStatus::SwapOnly => 6u64,
            AmmStatus::WaitingTrade => 7u64,
        }
    }

    pub fn swap_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => false,
            AmmStatus::OrderBookOnly => false,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }
}

// Ported from https://github.com/raydium-io/raydium-amm/blob/18bed23de5a29b038a93db493fafd5ff4ee5386b/program/src/state.rs#L310
#[repr(u64)]
pub enum AmmState {
    InvlidState = 0u64,
    IdleState = 1u64,
    CancelAllOrdersState = 2u64,
    PlanOrdersState = 3u64,
    CancelOrderState = 4u64,
    PlaceOrdersState = 5u64,
    PurgeOrderState = 6u64,
}

impl AmmState {
    pub fn from_u64(state: u64) -> Option<Self> {
        match state {
            0u64 => Some(AmmState::InvlidState),
            1u64 => Some(AmmState::IdleState),
            2u64 => Some(AmmState::CancelAllOrdersState),
            3u64 => Some(AmmState::PlanOrdersState),
            4u64 => Some(AmmState::CancelOrderState),
            5u64 => Some(AmmState::PlaceOrdersState),
            6u64 => Some(AmmState::PurgeOrderState),
            _ => None,
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmState::InvlidState => 0u64,
            AmmState::IdleState => 1u64,
            AmmState::CancelAllOrdersState => 2u64,
            AmmState::PlanOrdersState => 3u64,
            AmmState::CancelOrderState => 4u64,
            AmmState::PlaceOrdersState => 5u64,
            AmmState::PurgeOrderState => 6u64,
        }
    }

    // Done this way to emphasize which statuses are Okey for swap.
    pub fn swap_permission(&self) -> bool {
        match self {
            AmmState::IdleState
            | AmmState::CancelAllOrdersState
            | AmmState::PlanOrdersState
            | AmmState::CancelOrderState
            | AmmState::PlaceOrdersState
            | AmmState::PurgeOrderState => return true,
            _ => return false,
        }
    }
}
