//! Generate stochastic processes. 
//! 
use crate::rvector::RandomVector;

/// methods for creating stochastic processes 
pub trait TimeSeries {
    /// return a empty n-sized process bounded [0, 1] with 1/n increments
    /// 
    /// # Arguments
    /// 
    /// * 'n' - usize determining how many increments between [0,1].
    /// 
    /// # Example
    /// ```
    /// let mut process = Process::init(50);
    /// ```
    fn init(n: usize) -> Process<TimePoint>;

    /// Return a sampled `Process`.
    /// 
    /// # Arguments
    /// 
    /// * 'rv' - &RandomVector<T> populated with random variables.
    /// * 'f' - function defining the process.  
    /// 
    /// # Example
    /// ```
    /// let n = 2000;
    /// let dist = Uniform::new(-1.0, 1.0);
    /// let mut rng = rand::thread_rng();
    /// 
    /// let rv = RandomVector::new(dist, &mut rng, n);
    /// let p = Process::run_sim(&rv, sum_5);
    /// ```
    fn run_sim<T>(rv: &RandomVector<T>, f: fn(&[T]) -> f64) -> Process<TimePoint>;
} 

/// Used in `Process<TimePoint>` holds a `t` time value and a `y` value. 
#[derive(Debug, Copy, Clone)]
pub struct TimePoint {
    pub t: f64,
    pub y: f64,
}

/// Holds stochastic process data
#[derive(Debug, Clone)]
pub struct Process<TimePoint> {
    pub data: Vec<TimePoint>,
}

impl TimeSeries for Process<TimePoint> {
    fn init(n: usize) -> Process<TimePoint> {
        let mut data: Vec<TimePoint> = Vec::new();

        // =n because we want a point at t = 1.0.
        // initialize all time values to y = 0.0.
        for x in 0..=n {
            // rust ensures no division by zero if multiplied by zero?
            data.push(TimePoint { t: (x as f64)*(1.0/n as f64), y: 0.0 });
        }
        Process { data }
    }


    fn run_sim<T>(rv: &RandomVector<T>, f: fn(&[T]) -> f64) -> Process<TimePoint>{
        let mut p = Self::init(rv.values.len());
        for x in 0..p.data.len() {
            // Assume that at timepoint N, the process only has knowledge of events X_1 .. X_n,
            // thus feed the slice X_1 to X_n. Let the mappings do the logic.
            p.data[x].y = f(&rv.values[0..x]);
        }
        p
    }
}

