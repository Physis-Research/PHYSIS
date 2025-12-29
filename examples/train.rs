use aether::data;
use aether::engine;
use aether::{EvolutionConfig, Performance, RegimeAwareIndividual};

fn main() -> aether::Result<()> {
    let universe = data::prepare_universe("data/market_data.csv")?;
    let config = EvolutionConfig::default();

    let (b_m, b_t) = data::filter_by_regime(&universe, true);
    let bull = engine::evolve(&b_m, &b_t, config.clone(), true);

    let (r_m, r_t) = data::filter_by_regime(&universe, false);
    let bear = engine::evolve(&r_m, &r_t, config.clone(), false);

    let mut ind = RegimeAwareIndividual {
        bull,
        bear,
        performance: Performance::default(),
    };
    engine::evaluate_regime_aware(&mut ind, &universe, config.sharpe_annualization);

    println!("ROI: {:.2}%", ind.performance.roi * 100.0);
    println!("Sharpe: {:.2}", ind.performance.sharpe);
    println!("MDD: {:.2}%", ind.performance.mdd * 100.0);
    Ok(())
}
