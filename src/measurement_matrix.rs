use nalgebra::DMatrix;
use rand::{distributions::Bernoulli, prelude::Distribution};

use crate::matrix::{Matrix, RealMatrix};

pub enum MeasurementMatrix {
    Bernoulli,
}

impl MeasurementMatrix {
    pub fn into_matrix(self, nrows: usize, ncolumns: usize) -> Matrix {
        match self {
            MeasurementMatrix::Bernoulli => {
                Matrix::Real(MeasurementMatrix::bernoulli(nrows, ncolumns))
            }
        }
    }

    fn bernoulli(nrows: usize, ncolumns: usize) -> RealMatrix {
        // TODO make this injectable
        let rng = rand::thread_rng();
        let norm = 1.0 / ((ncolumns as f64).sqrt());
        let mut dist =
            Bernoulli::new(0.5)
                .unwrap()
                .sample_iter(rng)
                .map(|v| if v { norm } else { -norm });
        DMatrix::from_fn(nrows, ncolumns, |_, _| dist.next().unwrap()).into()
    }
}

#[cfg(test)]
mod test {
    use super::MeasurementMatrix;

    #[test]
    fn bernoulli() {
        let s = MeasurementMatrix::bernoulli(5, 10);
        println!("Generated bernoulli matrix {:?}", s)
    }
}
