use rand_distr::{Uniform, Standard, Distribution};
use rand::Rng;

#[derive(Debug)]

pub struct CoinFlip(u32);

fn main() {
    let mut rng = rand::thread_rng();
    let angle: CoinFlip = rng.gen();
    println!("Random angle: {angle:?}");
}

impl Distribution<CoinFlip> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoinFlip {
        CoinFlip(Uniform::new(0, 2).sample(rng))
    }
}