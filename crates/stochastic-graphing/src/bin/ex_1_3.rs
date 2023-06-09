use stochastic_processes::processes::{Process, TimeSeries};
use stochastic_processes::rvector::{RandomVector, Sample};
use stochastic_processes::mappings::sum;
use rand::rngs::SmallRng;
use rand::{thread_rng, SeedableRng};

use rand_distr::WeightedAliasIndex;

fn main() {
    let choices = vec![2, -1];
    let weights = vec![1, 2];

    // weighted distribution, in this case, 1/3 chance to hit 2, 2/3 chance to hit -1.
    let dist = WeightedAliasIndex::new(weights).unwrap();
    // typical fast source of rng
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    // in order to have moments converge, use large n.
    let n = 200000;

    // create the random vector.
    let rv = RandomVector::new_with_weighted(dist, &mut rng, &choices, n);
    let a = RandomVector::<f64>::from(rv);

    // apply the sum function to make into a martingale.
    let p = Process::run_sim(&a, sum);

    let mean = p.mean();
    println!("{mean}");
}
