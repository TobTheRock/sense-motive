use crate::precision::Precision;
use nalgebra::ComplexField;
use simba::scalar::{SubsetOf, SupersetOf};

#[derive(Clone, Copy)]
pub struct OrthogonalMatchingPursuitSolver {
    max_iter: usize,
    tolerance: f64,
}

impl OrthogonalMatchingPursuitSolver {
    pub fn with_parameters(max_iter: usize, tolerance: f64) -> OrthogonalMatchingPursuitSolver {
        OrthogonalMatchingPursuitSolver {
            max_iter,
            tolerance,
        }
    }
}

impl<'a> OrthogonalMatchingPursuitSolver {
    pub fn solve<P>(
        &self,
        // TODO should y also be type of P? => convert earlier
        y: &nalgebra::DVectorView<f64>,
        sensing_matrix: &nalgebra::DMatrix<P>,
    ) -> nalgebra::DVector<P>
    where
        P: Precision,
        <P as ComplexField>::RealField: SubsetOf<f64>,
    {
        let original_len = sensing_matrix.ncols();

        let mut sparse_solution = nalgebra::DVector::<P>::zeros(original_len);
        let mut compressed_signal = y.map(|e| nalgebra::convert(e));
        let mut residual = compressed_signal.clone();
        let mut selected_column_idxs = Vec::<usize>::new();

        // TODO this is actually sparse_solution, try the nalgebra type
        let mut selected_basis =
            nalgebra::DMatrix::zeros(sensing_matrix.nrows(), sensing_matrix.ncols());

        let max_iter = std::cmp::min(self.max_iter, sensing_matrix.ncols());
        for _ in 0..max_iter {
            let inner_products: nalgebra::DVector<P> = sensing_matrix.tr_mul(&residual);

            // filter out already used indices
            //TODO : function to find max idx/ product
            let view: nalgebra::DMatrixView<P> = inner_products.as_view();
            let filter_view = view
                .iter()
                .enumerate()
                .filter(|(idx, product)| !selected_column_idxs.contains(&idx));
            let max_idx = filter_view
                .map(|(idx, product)| (idx, product.norm1()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Can't compare, probably nan"))
                .map(|(index, _)| index)
                .unwrap();

            // TODO handle None
            // update support
            selected_column_idxs.push(max_idx);
            let selected_column = sensing_matrix.column(max_idx);
            selected_basis.set_column(max_idx, &selected_column);

            // Least square
            let svd = nalgebra::linalg::SVD::new(selected_basis.clone(), true, true);
            let eps = 0.1; // TODO make configurable
            sparse_solution = svd.solve(&residual, nalgebra::convert(eps)).unwrap();

            // calculate residual
            residual = &compressed_signal - (sensing_matrix * &sparse_solution);

            // abort
            if nalgebra::convert::<_, f64>(residual.norm()) < self.tolerance {
                break;
            }
        }

        sparse_solution
    }
}

#[cfg(test)]
mod test {

    use approx::assert_relative_eq;
    use nalgebra::{dmatrix, dvector};

    use super::OrthogonalMatchingPursuitSolver;

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

        let algorithm = OrthogonalMatchingPursuitSolver {
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
        let algorithm = OrthogonalMatchingPursuitSolver {
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
