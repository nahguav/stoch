pub fn martingale(rv: &[f64]) -> f64 {
    let mut sum = 0.0;
    
    for x in 0..rv.len() {
        sum += rv[x];
    }
    sum
}