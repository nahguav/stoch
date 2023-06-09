use stochastic_processes::processes::{Process, TimeSeries};

#[test]
fn test_create_length() {
    let p = Process::init(20);
    assert_eq!(p.len(), 20);
} 