use std::ops::Div;

use nalgebra::{dimension, DMatrix};
use rustdct::DctPlanner;

use crate::matrix::{Dimension, Matrix, RealMatrix};

#[derive(Clone, Copy)]
pub enum Transformation {
    None,
    Dct1dInverse,
    Dct1d,
}

impl Transformation {
    pub fn into_matrix(self, dimension: usize) -> Matrix {
        match self {
            Transformation::None => Matrix::Identity(Dimension {
                nrows: dimension,
                ncols: dimension,
            }),
            Transformation::Dct1dInverse => Transformation::dct1d_inverse(dimension).into(),
            Transformation::Dct1d => Transformation::dct1d(dimension).into(),
        }
    }

    // DCT 2, 1D
    fn dct1d(dimension: usize) -> RealMatrix {
        // TODO reuse planner
        let mut planner = DctPlanner::<f64>::new();
        let dct = planner.plan_dct2(dimension);
        let mut scratch = vec![0.0; dct.get_scratch_len()];

        let mut matrix = DMatrix::<f64>::identity(dimension, dimension);
        for mut col in matrix.column_iter_mut() {
            dct.process_dct2_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        // normalize
        matrix = matrix.div(f64::sqrt(dimension as f64 / 2.0));

        matrix.into()
    }
    // DCT 2 inverse, 1D
    // TODO consolidate methose
    pub fn dct1d_inverse(dimension: usize) -> RealMatrix {
        // TODO reuse planner
        let mut planner = DctPlanner::<f64>::new();

        // Inverse of DCT 2 is DCT3
        let dct = planner.plan_dct3(dimension);
        let mut scratch = vec![0.0; dct.get_scratch_len()];

        let mut matrix = DMatrix::<f64>::identity(dimension, dimension);
        for mut col in matrix.column_iter_mut() {
            dct.process_dct3_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        // normalize
        matrix = matrix.div(f64::sqrt(dimension as f64 / 2.0));

        matrix.into()
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use nalgebra::{DMatrix, DVector};

    use crate::Transformation;

    const TOLERANCE: f64 = 0.01;
    const N: usize = 4;

    #[test]
    fn dct1d() {
        let t: DMatrix<f64> = Transformation::dct1d(N).into();
        println!("DCT1D {}", t);

        let inv: DMatrix<f64> = Transformation::dct1d_inverse(N).into();
        println!("DCT1D inverse {}", inv);

        let x = DVector::<f64>::from_fn(4, |i, _| i as f64);

        let x_trans = &t * &x;
        let x2 = &inv * x_trans;
        assert_relative_eq!(x, x2, epsilon = TOLERANCE);

        assert_relative_eq!(
            DMatrix::<f64>::identity(4, 4),
            (inv * t),
            epsilon = TOLERANCE
        );
    }
}
