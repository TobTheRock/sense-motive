use std::ops::Mul;

use nalgebra::DMatrix;

use crate::{measurement_matrix::MeasurementMatrix, transform_matrix::TransformMatrix};

pub struct SensingMatrix<const M: usize, const N: usize> {
    matrix: DMatrix<f64>,
}

impl<const M: usize, const N: usize> SensingMatrix<M, N> {
    pub fn from(measurement: &MeasurementMatrix<M, N>, transfrom: &TransformMatrix<N>) -> Self {
        Self {
            matrix: measurement.as_ref() * transfrom.as_ref(),
        }
    }
}

impl<const M: usize, const N: usize> AsRef<DMatrix<f64>> for SensingMatrix<M, N> {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.matrix
    }
}

impl<const M: usize, const N: usize> From<DMatrix<f64>> for SensingMatrix<M, N> {
    fn from(matrix: DMatrix<f64>) -> Self {
        // TODO assert dimensions, own type def for Matrix with VecStorage and fixed! rows, columns
        Self { matrix }
    }
}
