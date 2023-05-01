extern crate derive_more;

use algorithm::{matching_pursuit::MatchingPursuitSolver, Algorithm};
use matrix::Matrix;
use measurement_matrix::MeasurementMatrix;

pub mod algorithm;
pub mod matrix;
pub mod measurement_matrix;
pub mod signal;
pub mod transform_matrix;

use signal::{RealViewSignal, Signal};
pub use transform_matrix::Transformation;

pub struct ModelBuilder {
    algorithm: Algorithm,
    transform: Transformation,
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self {
            // TODO impl default for matching pursuit
            algorithm: Algorithm::MatchingPursuit(MatchingPursuitSolver::with_parameters(
                1000, 0.1,
            )),
            transform: Transformation::None,
        }
    }
}
// TODO maybe don't use const generic so one could store models of dynamic sizes
pub struct Model<const M: usize, const N: usize> {
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

    pub fn with_algorithm(&mut self, algorithm: Algorithm) -> &mut Self {
        self.algorithm = algorithm;
        self
    }

    pub fn build<const M: usize, const N: usize>(&self) -> Model<M, N> {
        let measurement = MeasurementMatrix::Bernoulli.into_matrix(M, N);
        let transform = self.transform.into_matrix(N);
        let sensing = &measurement * &transform;
        Model {
            algorithm: self.algorithm,
            measurement_matrix: measurement,
            transform,
            sensing_matrix: sensing,
        }
    }
}

impl<const M: usize, const N: usize> Model<M, N> {
    pub fn builder() -> ModelBuilder {
        Default::default()
    }

    pub fn compress<T>(&self, input: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        // TODO padding
        let uncompressed: Signal = RealViewSignal::from(input.as_ref()).into();
        (&self.measurement_matrix * uncompressed).into()
    }

    pub fn decompress<T>(&self, input: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        let compressed = RealViewSignal::from(input.as_ref());
        let sparse = self.algorithm.solve(&compressed, &self.sensing_matrix);

        (&self.transform * sparse).into()
    }
}
