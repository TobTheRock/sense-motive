use crate::precision::Precision;
use nalgebra::ComplexField;
use num_traits::ToPrimitive;
use simba::scalar::SubsetOf;

#[derive(Clone, Copy)]
pub struct MatchingPursuitSolver {
    max_iter: usize,
    tolerance: f64,
}

impl MatchingPursuitSolver {
    pub fn with_parameters(max_iter: usize, tolerance: f64) -> MatchingPursuitSolver {
        MatchingPursuitSolver {
            max_iter,
            tolerance,
        }
    }
}

impl<'a> MatchingPursuitSolver {
    pub fn solve<P>(
        &self,
        y: &nalgebra::DVectorView<f64>,
        sensing_matrix: &nalgebra::DMatrix<P>,
    ) -> nalgebra::DVector<P>
    where
        P: Precision,
        <P as ComplexField>::RealField: SubsetOf<f64>,
    {
        let original_len = sensing_matrix.ncols();

        let mut sparse = nalgebra::DVector::<P>::zeros(original_len);
        let mut residual = y.map(|e| nalgebra::convert(e));

        for _ in 0..self.max_iter {
            let inner_products = sensing_matrix.tr_mul(&residual);
            let max_idx = inner_products.icamax();
            let max_col = sensing_matrix.column(max_idx);

            sparse[max_idx] += inner_products[max_idx];
            residual -= max_col * inner_products[max_idx];

            if nalgebra::convert::<_, f64>(residual.norm()) < self.tolerance {
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

    use super::MatchingPursuitSolver;

    const ONE_HALF: f64 = 1.0 / 2.0;
    const ONE_THIRD: f64 = 1.0 / 3.0;

    #[test]
    fn selects_best_matching_column() {
        let sensing_matrix = dmatrix![
            1.0, 0.0            , 0.0, ONE_THIRD.sqrt();
            0.0, ONE_HALF.sqrt(), 0.0, ONE_THIRD.sqrt();
            0.0, ONE_HALF.sqrt(), 1.0, ONE_THIRD.sqrt();
        ];
        // TODO SensingMatrix calss should do the normalization maybe (columns need to be normalized, so that l2(column)=1)

        let expected = dvector![0.0, 1.0, 0.0, 0.0];
        let compressed = &sensing_matrix * &expected;

        let algorithm = MatchingPursuitSolver {
            max_iter: 1,
            tolerance: 0.1,
        };

        let decompressed = algorithm.solve(&compressed.column(0), &sensing_matrix.into());

        assert_relative_eq!(expected, decompressed);
    }

    #[test]
    fn should_abort_when_residual_energy_is_below_tolerance() {
        // only one atom/column may match and the residual energy should be 1.5
        let sensing_matrix = dmatrix![
            0.0, 0.0, 0.0, 0.0;
            0.0, 0.0, 0.0, 0.0;
            1.0, 0.0, 0.0, 0.0;
        ];
        let compressed = dvector![1.0, 1.0, 1.0];
        let expected_tolerance = 2.0_f64.sqrt();
        let algorithm = MatchingPursuitSolver {
            max_iter: 10,
            tolerance: expected_tolerance + 0.1,
        };

        let decompressed = algorithm.solve(&compressed.column(0), &sensing_matrix.clone().into());

        assert_eq!(decompressed, dvector![1.0, 0.0, 0.0, 0.0]);

        let residual = compressed - sensing_matrix * decompressed;
        assert_relative_eq!(residual.norm(), expected_tolerance);
        // TODO statistics for algorithms?
    }
}
