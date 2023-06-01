use stoch::processes::{Process}; 
use stoch::rvector::{RandomVector, Sample};
use rand_distr::Uniform;

fn main() {
    // let p = Process::default();
    // println!{"{p:?}"}
    let mut rv = RandomVector{values: vec![0.0]};
    println!("{rv:?}");
    let dist = Uniform::new(-10.0, 10.0);
    let mut rng = rand::thread_rng();
    let n = 2000000;
    rv.sample(dist, &mut rng, n);
    //println!("{rv:?}");

    //todo - create default for RandomVector

}