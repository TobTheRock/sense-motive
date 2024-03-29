extern crate derive_more;

use algorithm::Algorithm;
use complex::ComplexFields;
use matrix::Matrix;
use measurement_matrix::MeasurementMatrix;

pub mod algorithm;
mod complex;
pub mod matrix;
pub mod measurement_matrix;
mod precision;

// TODO test crate for signal_utils
// #[cfg(test)]
pub mod signal_utils;
pub mod transform_matrix;

pub use transform_matrix::Transformation;

pub struct ModelBuilder {
    algorithm: Algorithm,
    transform: Transformation,
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self {
            algorithm: Default::default(),
            transform: Transformation::None,
        }
    }
}
pub struct Model {
    algorithm: Algorithm,
    measurement_matrix: Matrix,
    transform: Matrix,
    sensing_matrix: Matrix,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_transformation(&mut self, transform: Transformation) -> &mut Self {
        self.transform = transform;
        self
    }

    pub fn with_sensing_matrix(&mut self) -> &mut Self {
        todo!()
    }

    // TODO this is not accessible from the outside, as MatchingPursuitSolver is private!
    pub fn with_algorithm(&mut self, algorithm: Algorithm) -> &mut Self {
        self.algorithm = algorithm;
        self
    }

    // TODO move dimensions to new method (rename also)
    pub fn build(&self, size_compressed: usize, size_original: usize) -> Model {
        let measurement = MeasurementMatrix::Bernoulli.into_matrix(size_compressed, size_original);
        let transform = self.transform.into_matrix(size_original);
        let sensing = &measurement * &transform;
        Model {
            algorithm: self.algorithm,
            measurement_matrix: measurement,
            transform,
            sensing_matrix: sensing,
        }
    }
}

impl Model {
    pub fn builder() -> ModelBuilder {
        Default::default()
    }

    pub fn compress<T>(&self, orginal: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        &self.measurement_matrix * orginal.as_ref()
    }

    pub fn decompress<T>(&self, compressed: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        match &self.sensing_matrix {
            Matrix::Identity(_) => compressed.as_ref().to_vec(),
            Matrix::Real(m) => {
                let sparse = self.algorithm.solve(&compressed, &m);
                &self.transform * sparse.as_slice()
            }
            Matrix::Complex(m) => {
                let sparse = self.algorithm.solve(&compressed, &m);
                (&self.transform * sparse.as_slice()).real()
            }
        }
    }
}
