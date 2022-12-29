use nalgebra::SMatrix;
use rand::{distributions::Bernoulli, prelude::Distribution};

pub struct MeasurementMatrix<const M: usize, const N: usize> {
    matrix: SMatrix<f64, M, N>,
}

impl<const M: usize, const N: usize> MeasurementMatrix<M, N> {
    pub fn new_bernoulli() -> Self {
        // TODO make this injectable
        let rng = rand::thread_rng();
        // pseudo normalization
        let norm = (1 as f64) / (M as f64);
        let mut dist =
            Bernoulli::new(0.5)
                .unwrap()
                .sample_iter(rng)
                .map(|v| if v { norm } else { -norm });
        Self {
            matrix: SMatrix::from_fn(|_, _| dist.next().unwrap()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::MeasurementMatrix;

    #[test]
    fn bernoulli() {
        let s = MeasurementMatrix::<5, 10>::new_bernoulli();
        println!("Generated bernoulli matrix {}", s.matrix)
    }
}
