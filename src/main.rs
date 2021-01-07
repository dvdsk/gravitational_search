use grav_search::{GSA, Minimize};

use grav_search::Number;
use noisy_float::NoisyFloat as NoisyFloatInner;
use noisy_float::checkers::FiniteChecker;

mod gsa_paper {
    pub fn f1(x: &[f32]) -> f32 {
        x.iter().map(|x| x*x).sum()
    }
}

struct NoisyFloat(NoisyFloatInner<f32, FiniteChecker>);

fn main() {
    let g0 = 100f32;
    let alpha = 20f32;
    let t0 = 20f32;
    let max_n = 5;
    const SEED: u64 = 0;
    const POPULATION: usize = 2;
    const DIMENSION: usize = 2;

    let stop = |n: usize, _| n > max_n;
    let mut gsa: GSA<F, _, Minimize, _, DIMENSION> = GSA::new(g0, t0, alpha, max_n, SEED, gsa_paper::f1, stop);
    gsa.search(-5f32..=5f32, POPULATION);
}
