use egui::plot::{PlotPoint, PlotPoints};
use crate::rvector::RandomVector;

/// methods for creating stochastic processes 
pub trait TimeSeries {
    /// return a empty n-sized process bounded [0, 1] with 1/n increments
    /// 
    /// # Arguments
    /// 
    /// * 'n' - usize determining how many increments between [0,1].
    fn init(n: usize) -> Process<TimePoint>;

    /// Return a populated Process object 
    /// 
    /// # Arguments
    /// 
    /// * 'rv' - &RandomVector<T> populated with random variables.
    /// * 'f' - function defining the process.  
    fn run_sim<T>(rv: &RandomVector<T>, f: fn(&[T]) -> f64) -> Process<TimePoint>;

    /// turns &mut reference to self into PlotPoints for visualizing with egui.
    /// A &mut reference is present when Process<TimePoint> objects are being referenced from a vector. 
    /// 
    /// # Arguments
    /// 
    /// *'&mut self'
    /// 
    fn mut_into(&mut self) -> PlotPoints;
} 

/// Used in `Process<TimePoint>` holds a `t` time value and a `y` value. 
/// Useful for plotting.
#[derive(Debug, Copy, Clone)]
pub struct TimePoint {
    pub t: f64,
    pub y: f64,
}

/// struct containing the stochastic proccess' data. 
#[derive(Debug, Clone)]
pub struct Process<TimePoint> {
    pub data: Vec<TimePoint>,
}

impl TimeSeries for Process<TimePoint> {
    fn init(n: usize) -> Process<TimePoint> {
        let mut data: Vec<TimePoint> = Vec::new();

        for x in 0..=n {
            // rust ensures no division by zero if multiplied by zero?
            data.push(TimePoint { t: (x as f64)*(1.0/n as f64), y: 0.0 });
        }
        Process { data }
    }


    fn run_sim<T>(rv: &RandomVector<T>, f: fn(&[T]) -> f64) -> Process<TimePoint>{
        let mut p = Self::init(rv.values.len());
        for x in 0..p.data.len() {
           p.data[x].y = f(&rv.values[0..x]);
        }
        p
    }

    fn mut_into(&mut self) -> PlotPoints {
        let mut v = Vec::new();
        for p in self.data.as_slice(){
            v.push( PlotPoint {x: p.t, y: p.y})
        }
        PlotPoints::Owned(v)
    }
}

impl Into<PlotPoints> for Process<TimePoint> {
    fn into(self) -> PlotPoints {
        let mut v = Vec::new();
        for p in self.data{
            v.push( PlotPoint {x: p.t, y: p.y})
        }
        PlotPoints::Owned(v)
    }
}

