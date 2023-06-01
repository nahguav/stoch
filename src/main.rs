//use stoch::processes::{Process}; 
use stoch::rvector::{RandomVector, Sample};
use rand_distr::Uniform;

fn main() {
    // let p = Process::default();
    // println!{"{p:?}"}
    
    let nm: f32 = 0.1;
    let dist = Uniform::new(nm, 10.0);
    let mut rng = rand::thread_rng();
    let n = 20;
    let rv = RandomVector::new(dist, &mut rng, n);
    println!("{rv:?}");

}