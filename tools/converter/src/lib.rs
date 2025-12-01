use {
    chrono::Utc,
    common::message::{ArbitrageMessage, SequenceId, SwapMessage, SwapType},
    std::env,
};

pub fn create_swap_file_name() -> String {
    let timestamp = Utc::now().format("%Y-%m-%dT%H%M%S").to_string();

    let dir_name = env::current_dir().unwrap();
    let swap_file_path = dir_name.join(format!("swap.{}", timestamp));
    swap_file_path.to_string_lossy().to_string()
}

pub fn convert_arbitrage(seq_id: &mut SequenceId, msg: &ArbitrageMessage) -> Vec<SwapMessage> {
    let mut result: Vec<SwapMessage> = vec![];

    result.push(SwapMessage {
        sequence_id: seq_id.increment_and_get(),
        lp_name: msg.buy_side_info.lp_name.clone(),
        lp_address: msg.buy_side_info.lp_address,
        swap_type: SwapType::Buy,
        base_qty: msg.swap_base_qty,
        quote_amount: msg.swap_buy_quote_amount,
    });
    result.push(SwapMessage {
        sequence_id: seq_id.increment_and_get(),
        lp_name: msg.sell_side_info.lp_name.clone(),
        lp_address: msg.sell_side_info.lp_address,
        swap_type: SwapType::Sell,
        base_qty: msg.swap_base_qty,
        quote_amount: msg.swap_sell_quote_amount,
    });

    result
}
