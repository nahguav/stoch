//! Mappings f: &[X] -> Y for defining stochastic processes.

/// returns a sum of the '[f64]' slice given.
///  
/// # Arguments
/// * 'rv' - reference to a slice.
/// 
/// #Example
/// ```
/// let n = 2000;
/// let dist = Uniform::new(-1.0, 1.0);
/// let mut rng = rand::thread_rng();
///
/// let rv = RandomVector::new(dist, &mut rng, n);
/// let p = Process::run_sim(&rv, sum); // <- used here to specify the mapping f: X -> Y. 
/// ```
pub fn sum(rv: &[f64]) -> f64 {
    let mut sum = 0.0;
    
    for x in 0..rv.len() {
        sum += rv[x];
    }
    sum
}

pub fn sum_5(rv: &[f64]) -> f64 {
    let mut sum = 0.0;
    if (rv.len() < 5) {
        for x in 0..rv.len() {
            sum += rv[x];
        }
    } else {
        for x in rv.len()-5..rv.len() {
            sum += rv[x];
        }
    }
    sum
}