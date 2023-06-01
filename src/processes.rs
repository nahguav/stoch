pub trait TimeSeries {
    fn init() -> Process<TimePoint>;

    /// accepts a function and a random vector, the function maps the rv -> yn
    fn run_sim<T>(&mut self, rv: &Vec<T>, f: fn(&[T]) -> f64);
} 

#[derive(Debug)]
pub struct TimePoint {
    pub t: f64,
    pub y: f64,
}
#[derive(Debug)]
pub struct Process<TimePoint> {
    pub data: Vec<TimePoint>,
}

impl TimeSeries for Process<TimePoint> {
    fn init() -> Process<TimePoint> {
        let mut data: Vec<TimePoint> = Vec::new();

        for x in 0..=19 {
            data.push(TimePoint { t: (x as f64)*0.001, y: 0.0 });
        }
        Process { data }
    }


    fn run_sim<T>(&mut self, rv: &Vec<T>, f: fn(&[T]) -> f64){
        assert!(self.data.len() == rv.len(), "array size mismatch");
        for x in 0..self.data.len() {
            self.data[x].y = f(&rv[0..x]);
        }
    }
}



//impl Default for Process<TimePoint> {
    // fn default() -> Process<TimePoint> {
    //     // Basic martingale, sum of uniformly distributed independent RVs 

    //     let rv = RandomVector::new()
    //     let data = (0..100).map(|i| { 
    //         let x = i  as f64 * 0.01;
    //         TimePoint {t: x, y: rng.sample(u)}
    //     }).collect();
        
    //     Process { data }
    // }
//}
