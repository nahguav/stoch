use rand_distr::{Distribution};
use rand::{Rng};

/// trait Sample implements a method to initiate/resample RandomVectors with generic distributions and rng.
pub trait Sample<T> {
    /// Returns a 'RandomVector' with 'n' random variables drawn from specified distribution 'dist' and source rng 'rng'.
    /// 
    /// # Arguments
    /// 
    /// * 'self'
    /// * 'dist' - A distribution object that implements the Distribution trait rand::distributions::Distribution.
    /// * 'rng' - A rng object that specifices the source of rng. ie, rand::Rng
    /// * 'n' - desired length of the random vector.
    ///  # Example
    /// 
    /// ```
    /// use stoch::rvector::{RandomVector, Sample};
    /// use rand_distr::Uniform;
    ///
    /// fn main() {
    ///     let dist = Uniform::new(-10.0, 10.0);
    ///     let mut rng = rand::thread_rng();
    ///     let n = 2;
    ///     let rv = RandomVector::new(dist, &mut rng, n);
    ///     println!("{rv:?}");
    ///
    /// }
    /// ```
    fn new<D, R>(dist: D, rng: &mut R, n: usize) -> Self where D: Distribution<T>, R: Rng + ?Sized;
}

// N dimension random vector from given distribution
#[derive(Debug)]
pub struct RandomVector<T> 
{
    pub values: Vec<T>,
}

impl<T> Sample<T> for RandomVector<T> {
    fn new<D, R>(dist: D, rng: &mut R, n: usize) -> Self where D: Distribution<T>, R: Rng + ?Sized
    {
        let values = dist.sample_iter(rng).take(n).collect();
        RandomVector { values }
    }
}



