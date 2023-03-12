use nalgebra::DVector;

use crate::sensing_matrix::SensingMatrix;

use super::Algorithm;

pub struct MatchingPursuit {
    max_iter: usize,
    // TODO rename
    epsilon: f64,
}

impl MatchingPursuit {
    pub fn with_parameters(max_iter: usize, epsilon: f64) -> Box<MatchingPursuit> {
        Box::new(MatchingPursuit { max_iter, epsilon })
    }
}

impl<const M: usize, const N: usize> Algorithm<M,N> for MatchingPursuit {
    fn solve(
        &self,
        y: &nalgebra::DVectorView<f64>,
        sensing_matrix: &SensingMatrix<M, N>,
    ) -> nalgebra::DVector<f64> {
        let mut sparse = DVector::zeros(N);
        let mut residual = y.clone_owned();

        for _ in 0..self.max_iter {
            let inner_products = sensing_matrix.as_ref().tr_mul(&residual);
            let max_idx = inner_products.iamax();
            sparse[max_idx] = inner_products[max_idx];
            residual -= inner_products[max_idx] * sensing_matrix.as_ref().column(max_idx);
            if residual.norm() < self.epsilon {
                break;
            }
        }

        sparse
    }
}

#[cfg(test)]
mod test {
    use nalgebra::{dmatrix, dvector, DVectorView};
    use rustdct::num_traits::sign;

    use crate::{algorithm::Algorithm, sensing_matrix::SensingMatrix};

    use super::MatchingPursuit;

    #[test]
    fn matching_pursuit() {
        let signal = dvector![1.0, 0.5];
        let matrix = 
             dmatrix![1.0, 0.0; 
            0.5, 3.0_f64.sqrt()/2.0; 
            -1.0/2.0_f64.sqrt(), -1.0/2.0_f64.sqrt()].transpose().into();

        let algo = MatchingPursuit {
            max_iter: 2,
            epsilon: 0.1,
        };

        let sparse = Algorithm::<2,3>::solve(&algo, &signal.column(0), &matrix);
        println!("{}", sparse);
        // TODO expectation

    }
}
