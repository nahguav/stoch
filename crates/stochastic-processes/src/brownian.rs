use rand_distr::{StandardNormal};
use crate::{processes::{Process, TimeSeries, TimePoint}, rvector::{RandomVector, Sample}, mappings::sum};

pub trait BrownianMotion {
    /// create a new brownian motion. 
    fn new(n: usize) -> Self;

    /// create `paths` number of brownian motion and return a vector of Box<Self>
    fn many_new(step: usize, paths: usize) -> Vec<Box<Self>>;
}

impl BrownianMotion for Process<TimePoint> {
    fn new(n: usize) -> Self {
        let dist = StandardNormal;
        let mut rng = rand::thread_rng();
        let rv = RandomVector::new(dist, &mut rng, n);
        Process::run_sim(&rv, sum)
    }
    fn many_new(step: usize, paths: usize) -> Vec<Box<Self>> {
        let mut r = Vec::new();
        for i in 0..paths {
            r.push(Box::new(Self::new(step)));
        }
        r
    }

}