use aether::engine;

fn main() -> aether::Result<()> {
    let summary = engine::walk_forward_validation("data/market_data.csv", 3, 0.7, 42)?;
    println!("Mean Test ROI: {:.2}%", summary.mean_test_roi * 100.0);
    println!("Baseline B&H:  {:.2}%", summary.baseline_buy_hold * 100.0);
    Ok(())
}
