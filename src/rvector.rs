use rand_distr::{Distribution, Uniform, StandardNormal};
use num_traits::Float;
use rand::{thread_rng, Rng};
use rand::distributions::DistIter;

pub trait Sample<T> {
    fn sample<D, R>(&mut self, dist: D, rng: &mut R, n: usize) where D: Distribution<T>, R: Rng + ?Sized;
}

// N dimension random vector from given distribution
#[derive(Debug)]
pub struct RandomVector<T> 
{
    pub values: Vec<T>,
}

impl<T> Sample<T> for RandomVector<T> {
    fn sample<D, R>(&mut self, dist: D, rng: &mut R, n: usize) 
    where 
    D: Distribution<T>, 
    R: Rng + ?Sized
    {
        self.values = dist.sample_iter(rng).take(n).collect();
    }
}



