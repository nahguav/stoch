//! Generate random vectors.

use rand_distr::{Distribution, weighted_alias::AliasableWeight};
use rand_distr::weighted_alias::{WeightedAliasIndex};
use rand::{Rng};

/// methods for initializing `RandomVector`.
pub trait Sample<T> {
    /// Returns `RandomVector` with `n` random variables sampled from distribution `dist` and source `rng`.
    /// 
    /// # Arguments
    /// 
    /// * 'self'
    /// * 'dist' - object implementing rand::distributions::Distribution trait.
    /// * 'rng' - RNG object that specifices the source of rng. ie, rand::Rng
    /// * 'n' - length of the random vector.
    /// 
    ///  # Example
    /// ```
    /// use stochastic_processes::rvector::{RandomVector, Sample};
    /// use rand_distr::Uniform;
    ///
    /// fn main() {
    ///     let dist = Uniform::new(-10.0, 10.0);
    ///     let mut rng = rand::thread_rng();
    ///     let n = 2;
    ///     let rv = RandomVector::new(dist, &mut rng, n);
    ///     println!("{rv:?}");
    /// }
    /// ```
    fn new<D, R>(dist: D, rng: &mut R, n: usize) -> Self where D: Distribution<T>, R: Rng + ?Sized;

    /// Returns `RandomVector` with `n` random variables drawn with weights corresponding to different choices.
    /// 
    /// # Arguments
    /// 
    /// * 'dist' - rand_distr::weighted_alias::{WeightedAliasIndex} distribution;
    /// * 'rng' - RNG object that specifices the source of rng. ie, rand::Rng
    /// * 'n' - length of the random vector.
    /// * 'choices' - what symbol to substitute for each corresponding weight in `dist`
    /// 
    ///  # Example
    /// ```
    /// use rand_distr::WeightedAliasIndex;
    /// use rand::prelude::*;
    /// 
    /// let choices = vec![2, -1];
    /// let weights = vec![1, 2];
    ///
    /// let dist = WeightedAliasIndex::new(weights).unwrap();
    /// let mut rng = rand::thread_rng();
    /// let n = 10;
    /// 
    /// let rv = RandomVector::new_with_weighted(dist, &mut rng, &choices, n);
    /// ```
    fn new_with_weighted<W, R>(dist: WeightedAliasIndex<W>, rng: &mut R, choices: &Vec<T>, n: usize) -> Self where W: AliasableWeight, R: Rng + ?Sized, T: Copy;
}

/// N dimension random vector from given distribution
#[derive(Debug)]
pub struct RandomVector<T> 
{
    pub values: Vec<T>,
}

impl<T> Sample<T> for RandomVector<T> {
    fn new<D, R>(dist: D, rng: &mut R, n: usize) -> Self where D: Distribution<T>, R: Rng + ?Sized
    {
        let values = dist.sample_iter(rng).take(n+1).collect();
        RandomVector { values }
    }
    fn new_with_weighted<W, R>(dist: WeightedAliasIndex<W>, rng: &mut R, choices: &Vec<T>, n: usize) -> Self where W: AliasableWeight, R: Rng + ?Sized, T: Copy {
        let mut values = Vec::new();
        for _ in 0..n {
            values.push(choices[dist.sample(rng)])
        }
        Self { values }
    }

}

/// Converts a `RandomVector<i32>` into a `RandomVector<f64>` for mapping/computation.
impl From<RandomVector<i32>> for RandomVector<f64> {
    fn from(rv: RandomVector<i32>) -> Self {
        let values = rv.values.iter().map(|&e| e as f64).collect();
        RandomVector { values  }
    }
}



