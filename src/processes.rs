use rand_distr::{Uniform};
use rand::{Rng, thread_rng};

#[derive(Debug)]
pub struct TimePoint {
    pub t: f64,
    pub y: f64,
}
#[derive(Debug)]
pub struct Process<TimePoint> {
    pub data: Vec<TimePoint>,
}

impl Default for Process<TimePoint> {
    fn default() -> Process<TimePoint> {
        // Basic martingale, sum of uniformly distributed independent RVs 
        let u = Uniform::new(-10.0, 10.0);
        let mut rng = thread_rng();

        let data = (0..100).map(|i| { 
            let x = i  as f64 * 0.01;
            TimePoint {t: x, y: rng.sample(u)}
        }).collect();
        
        // for _ in start..end {
        //     // sample a point from the square with sides -10 - 10 in two dimensions
        //     data.push(rng.sample(u));
        // }
        Process { data }
    }
}
