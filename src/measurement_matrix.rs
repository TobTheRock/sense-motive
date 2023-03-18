use std::ops::Mul;

use nalgebra::{DMatrix, DVector};
use rand::{distributions::Bernoulli, prelude::Distribution};

pub struct MeasurementMatrix<const M: usize, const N: usize> {
    matrix: DMatrix<f64>,
}

impl<const M: usize, const N: usize> MeasurementMatrix<M, N> {
    pub fn new_bernoulli() -> Self {
        // TODO make this injectable
        let rng = rand::thread_rng();
        let norm = 1.0 / ((M as f64).sqrt());
        let mut dist =
            Bernoulli::new(0.5)
                .unwrap()
                .sample_iter(rng)
                .map(|v| if v { norm } else { -norm });
        Self {
            matrix: DMatrix::from_fn(M, N, |_, _| dist.next().unwrap()),
        }
    }
}

impl<const M: usize, const N: usize> AsRef<DMatrix<f64>> for MeasurementMatrix<M, N> {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.matrix
    }
}

// impl<const M: usize, const N: usize> Mul<&DVector<f64, N>> for MeasurementMatrix<M, N> {
//     type Output = DVector<f64, M>;

//     fn mul(self, rhs: &DVector<f64, N>) -> Self::Output {
//         self.matrix * rhs
//     }
// }

#[cfg(test)]
mod test {
    use super::MeasurementMatrix;

    #[test]
    fn bernoulli() {
        let s = MeasurementMatrix::<5, 10>::new_bernoulli();
        println!("Generated bernoulli matrix {}", s.matrix)
    }
}
