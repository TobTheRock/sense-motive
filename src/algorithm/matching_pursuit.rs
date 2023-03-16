use nalgebra::DVector;

use crate::sensing_matrix::SensingMatrix;

use super::Algorithm;

pub struct MatchingPursuit {
    max_iter: usize,
    tolerance: f64,
}

impl MatchingPursuit {
    pub fn with_parameters(max_iter: usize, tolerance: f64) -> Box<MatchingPursuit> {
        Box::new(MatchingPursuit {
            max_iter,
            tolerance,
        })
    }
}

impl<const M: usize, const N: usize> Algorithm<M, N> for MatchingPursuit {
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

            sparse[max_idx] += inner_products[max_idx];
            residual -= inner_products[max_idx] * sensing_matrix.as_ref().column(max_idx);

            if residual.norm() < self.tolerance {
                break;
            }
        }

        sparse
    }
}

#[cfg(test)]
mod test {

    use approx::assert_relative_eq;
    use nalgebra::{dmatrix, dvector};

    use crate::algorithm::Algorithm;

    use super::MatchingPursuit;

    const ONE_HALF: f64 = 1.0 / 2.0;
    const ONE_THIRD: f64 = 1.0 / 3.0;

    #[test]
    fn selects_best_matching_column() {
        let sensing_matrix = dmatrix![
            1.0, 0.0, 0.0, ONE_THIRD;
            0.0, 1.0, 0.0, ONE_THIRD;
            0.0, 0.0, 1.0, ONE_THIRD;
        ];
        // TODO SensingMatrix calss should do the normalization maybe (columns need to be normalized)

        let expected = dvector![0.0, 1.0, 0.0, 0.0];
        let compressed = &sensing_matrix * &expected;

        let algorithm = MatchingPursuit {
            max_iter: 1,
            tolerance: 0.1,
        };

        let decompressed =
            Algorithm::<3, 4>::solve(&algorithm, &compressed.column(0), &sensing_matrix.into());

        assert_relative_eq!(expected, decompressed);
    }

    #[test]
    fn should_abort_when_residual_energy_is_below_tolerance() {
        // only one atom/column may match and the residual energy should be 1.5
        let sensing_matrix = dmatrix![
            0.0, 0.0, 0.0, 0.0;
            ONE_HALF, 0.0, 0.0, 0.0;
            ONE_HALF, 0.0, 0.0, 0.0;
        ];

        let compressed = dvector![1.0, 1.0, 1.0];

        let algorithm = MatchingPursuit {
            max_iter: 10,
            tolerance: 1.1,
        };

        let decompressed =
            Algorithm::<3, 4>::solve(&algorithm, &compressed.column(0), &sensing_matrix.into());
        assert_eq!(decompressed, dvector![1.5, 0.0, 0.0, 0.0]);
        // TODO statistics for algorithms?
    }
}
