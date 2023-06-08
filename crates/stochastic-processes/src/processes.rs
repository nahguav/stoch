//! Generate stochastic processes. 
//! 
use crate::rvector::RandomVector;
use core::f64::INFINITY;
use core::f64::NEG_INFINITY;
use crate::mappings::sum;

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
    /// use stochastic_processes::processes::{Process, TimeSeries};
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
    /// use stochastic_processes::mappings::sum;
    /// use stochastic_processes::processes::{Process, TimeSeries};
    /// use rand_distr::Uniform;
    /// use stochastic_processes::rvector::{RandomVector, Sample};
    /// 
    /// let n = 2000;
    /// let dist = Uniform::new(-1.0, 1.0);
    /// let mut rng = rand::thread_rng();
    /// 
    /// let rv = RandomVector::new(dist, &mut rng, n);
    /// let p = Process::run_sim(&rv, sum);
    /// ```
    fn run_sim<T>(rv: &RandomVector<T>, f: fn(&[T]) -> f64) -> Process<TimePoint>;
    
    /// Return the supremum of the timeseries.
    fn sup(&self) -> f64;

    /// Return the infimum of the timeseries.
    fn inf(&self) -> f64;

    /// Return the mean of the timeseries.
    fn mean(&self) -> f64;

    /// Return the quadratic variation of the timeseries.
    fn quadratic_variation(&self) -> f64;

    /// return a copy of &self.data.
    fn get_y_values(&self) -> Vec<f64>;
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
            // This code assumes that the process only has knowledge of RVs X_1 .. X_n
            p.data[x].y = f(&rv.values[0..x]);
        }
        p
    }

    fn sup(&self) -> f64 {
        let mut s = NEG_INFINITY; 
        for x in &self.data {
            if x.y > s { s = x.y}
        }
        s
    } 

    /// Return the infinum of the process.
    fn inf(&self) -> f64 {
        let mut i = INFINITY; 
        for x in &self.data {
            if x.y < i { i = x.y}
        }
        i
    }

    /// Return the mean value of the process.
    fn mean(&self) -> f64 {
        sum(self.get_y_values().as_slice())
    }

    /// Returns a Vec<f64> of Y values.
    fn get_y_values(&self) -> Vec<f64> {
        let mut v = Vec::new();  
        for x in &self.data {
            v.push(x.y);
        }
        v
    }

    /// Return quadratic variance for process
    fn quadratic_variation(&self) -> f64 {
        let mut var = 0.0;
        for x in 1..=self.data.len() {
            var += f64::powi(&self.data[x].y - &self.data[x-1].y, 2);
        }
        var
    }
}

impl Process<TimePoint> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

