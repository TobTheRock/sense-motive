use nalgebra::DMatrix;

use crate::{measurement_matrix::MeasurementMatrix, transform_matrix::TransformMatrix};

pub struct SensingMatrix {
    matrix: DMatrix<f64>,
}

impl SensingMatrix {
    pub fn from<const M: usize, const N: usize>(
        measurement: &MeasurementMatrix<M, N>,
        transform: &TransformMatrix<N>,
    ) -> Self {
        Self {
            matrix: measurement.as_ref() * transform.as_ref(),
        }
    }
}

impl AsRef<DMatrix<f64>> for SensingMatrix {
    fn as_ref(&self) -> &DMatrix<f64> {
        &self.matrix
    }
}

impl From<DMatrix<f64>> for SensingMatrix {
    fn from(matrix: DMatrix<f64>) -> Self {
        // TODO assert dimensions, own type def for Matrix with VecStorage and fixed! rows, columns
        Self { matrix }
    }
}
