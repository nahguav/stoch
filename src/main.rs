use stoch::processes::{Process, TimeSeries}; 
use stoch::rvector::{RandomVector, Sample};
use stoch::rv_functions::martingale;
use rand_distr::Uniform;

fn main() {
    let dist = Uniform::new(-10.0, 10.0);
    let mut rng = rand::thread_rng();
    let n = 20;
    let rv = RandomVector::new(dist, &mut rng, n);

    let mut p = Process::init(n);

    p.run_sim(&rv, martingale);

    println!("{p:#?}");

}