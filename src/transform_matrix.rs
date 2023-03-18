use std::ops::Div;

use nalgebra::DMatrix;
use rustdct::DctPlanner;

#[derive(Clone, Copy)]
pub enum Transformation {
    None,
    Dct1dInverse,
}

impl<const N: usize> Into<TransformMatrix<N>> for Transformation {
    fn into(self) -> TransformMatrix<N> {
        match self {
            Transformation::None => TransformMatrix::none(),
            Transformation::Dct1dInverse => TransformMatrix::dct1d_inverse(),
        }
    }
}

pub struct TransformMatrix<const N: usize> {
    matrix: DMatrix<f64>,
}

impl<const N: usize> TransformMatrix<N> {
    pub fn none() -> Self {
        Self {
            // TODO: we actually can skip the calculation in this case, to be optimized
            // Nonethe less it make sense for other cases to store the transform matrix to be faster when decompressing
            matrix: DMatrix::identity(N, N),
        }
    }

    // DCT 2, 1D
    pub fn dct1d() -> Self {
        // TODO reuse planner
        let mut planner = DctPlanner::<f64>::new();
        let dct = planner.plan_dct2(N);
        let mut scratch = vec![0.0; dct.get_scratch_len()];

        let mut matrix = DMatrix::<f64>::identity(N, N);
        for mut col in matrix.column_iter_mut() {
            dct.process_dct2_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        // normalize
        matrix = matrix.div(f64::sqrt(N as f64 / 2.0));

        Self { matrix }
    }
    // DCT 2 inverse, 1D
    // TODO consolidate methose
    pub fn dct1d_inverse() -> Self {
        // TODO reuse planner
        let mut planner = DctPlanner::<f64>::new();

        // Inverse of DCT 2 is DCT3
        let dct = planner.plan_dct3(N);
        let mut scratch = vec![0.0; dct.get_scratch_len()];

        let mut matrix = DMatrix::<f64>::identity(N, N);
        for mut col in matrix.column_iter_mut() {
            dct.process_dct3_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        // normalize
        matrix = matrix.div(f64::sqrt(N as f64 / 2.0));

        Self { matrix }
    }
}

impl<const N: usize> AsRef<DMatrix<f64>> for TransformMatrix<N> {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.matrix
    }
}

#[cfg(test)]
mod test {
    use approx::{assert_relative_eq, relative_eq};
    use nalgebra::{DMatrix, DVector};

    use super::TransformMatrix;

    const TOLERANCE: f64 = 0.01;

    #[test]
    fn dct1d() {
        let t = TransformMatrix::<4>::dct1d();
        println!("DCT1D {}", t.matrix);

        let inv = TransformMatrix::<4>::dct1d_inverse();
        println!("DCT1D inverse {}", inv.matrix);

        let x = DVector::<f64>::from_fn(4, |i, _| i as f64);
        let x_trans = &t.matrix * &x;
        let x2 = &inv.matrix * x_trans;
        assert_relative_eq!(x, x2, epsilon = TOLERANCE);

        assert_relative_eq!(
            DMatrix::<f64>::identity(4, 4),
            inv.matrix * t.matrix,
            epsilon = TOLERANCE
        );
    }
}
