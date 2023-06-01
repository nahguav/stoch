use stoch::processes::{Process, TimeSeries}; 
use stoch::rvector::{RandomVector, Sample};
use rand_distr::Uniform;

fn main() {
    // let p = Process::default();
    // println!{"{p:?}"}
    
    let dist = Uniform::new(-10.0, 10.0);
    let mut rng = rand::thread_rng();
    let n = 20;
    let rv = RandomVector::new(dist, &mut rng, n);
    println!("{rv:?}");

    let mut p = Process::init();

    // define the martingale
    // may want to abstract 
    fn martingale(rv: &[f64]) -> f64 {
        let mut sum = 0.0;
        
        for x in 0..rv.len() {
            sum += rv[x];
        }
        sum
    }

    p.run_sim(&rv.values, martingale);

    println!("{p:?}");

}