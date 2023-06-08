use stochastic_processes::processes::{Process, TimeSeries};
use stochastic_processes::rvector::{RandomVector, Sample};
use stochastic_processes::mappings::martingale_strat;
use rand_distr::Uniform;


fn main() {
    let uni = Uniform::new(0, 2);
    let mut rng = rand::thread_rng();
    let n = 10;

    let rv = RandomVector::new(uni, &mut rng, n);
    let a = RandomVector::<f64>::from(rv);
    let p = Process::run_sim(&a, martingale_strat).get_y_values();

    println!("{p:?}");
}
