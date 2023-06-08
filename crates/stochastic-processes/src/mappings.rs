//! Mappings f: &[X] -> Y for defining stochastic processes.

/// returns a sum of the '[f64]' slice given.
///  
/// # Arguments
/// * 'rv' - reference to a slice.
/// 
/// #Example
/// ```
/// use rand_distr::{Uniform};
/// use stochastic_processes::rvector::RandomVector;
/// use stochastic_processes::processes::{Process};
/// 
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

pub fn cumsum(rv: &[f64]) -> f64 {
    let mut sum = 0.0;
    if rv.len() < 5 {
        for x in 0..rv.len() {
            sum += rv[x];
        }
    } else {
        for x in rv.len()-5..rv.len() {
            sum += rv[x];
        }
    }
    sum / (rv.len() as f64)
}

// todo: fix this.
pub fn quadratic_variation(rv: &[f64]) -> f64 {
    let mut var = 0.0;
    for x in 1..rv.len() {
        var += f64::powi(rv[x] - rv[x-1], 2);
    }
    var 
}

/// Martingale strategy. To be used with coinflip distribution.
pub fn martingale_strat(rv: &[f64]) -> f64 {
    let mut b = 1.0;
    let mut var = 0.0;

    for x in 1..rv.len(){
        if rv[x] == 0.0 {
            // lost, subtract bet
            var -= b;
            // double your bet
            b = b * 2.0;
        } else {
            // win bet
            var += b;
            b = 0.0;
        }
    }
    var
}
