use std::ops::Mul;

use nalgebra::SMatrix;

use crate::{measurement_matrix::MeasurementMatrix, transform_matrix::TransformMatrix};

pub struct SensingMatrix<const M: usize, const N: usize> {
    matrix: SMatrix<f64, M, N>,
}

impl<const M: usize, const N: usize> From<MeasurementMatrix<M, N>> for SensingMatrix<M, N> {
    fn from(matrix: MeasurementMatrix<M, N>) -> Self {
        Self {
            matrix: *matrix.as_ref(),
        }
    }
}

impl<const M: usize, const N: usize> Mul<TransformMatrix<N>> for MeasurementMatrix<M, N> {
    type Output = SensingMatrix<M, N>;

    fn mul(self, rhs: TransformMatrix<N>) -> Self::Output {
        let matrix = self.as_ref() * rhs.as_ref();
        SensingMatrix { matrix }
    }
}

impl<const M: usize, const N: usize> AsRef<SMatrix<f64, M, N>> for SensingMatrix<M, N> {
    fn as_ref(&self) -> &SMatrix<f64, M, N> {
        &self.matrix
    }
}
