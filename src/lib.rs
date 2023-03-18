use algorithm::{matching_pursuit::MatchingPursuit, Algorithm};
use measurement_matrix::MeasurementMatrix;
use sensing_matrix::SensingMatrix;
use transform_matrix::TransformMatrix;

pub mod algorithm;
pub mod measurement_matrix;
pub mod sensing_matrix;
pub mod transform_matrix;

pub use transform_matrix::Transformation;

pub struct ModelBuilder {
    transform: Transformation,
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self {
            transform: Transformation::None,
        }
    }
}
// TODO maybe don't use const generic so one could store models of dynamic sizes
pub struct Model<const M: usize, const N: usize> {
    algorithim: Box<dyn Algorithm<M, N>>,
    measurement_matrix: MeasurementMatrix<M, N>,
    transform: TransformMatrix<N>,
    sensing_matrix: SensingMatrix<M, N>,
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

    pub fn with_algorithm(&mut self) -> &mut Self {
        todo!()
    }

    pub fn build<const M: usize, const N: usize>(&self) -> Model<M, N> {
        let measurement = MeasurementMatrix::new_bernoulli();
        let transform = self.transform.into();
        let sensing = SensingMatrix::from(&measurement, &transform);
        Model {
            algorithim: MatchingPursuit::with_parameters(1000, 0.1),
            measurement_matrix: measurement,
            transform,
            sensing_matrix: sensing,
        }
    }
}

impl<const M: usize, const N: usize> Model<M, N> {
    pub fn compress<T>(&self, input: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        // TODO padding
        let uncompressed = nalgebra::DVectorView::from_slice(input.as_ref(), N);
        (self.measurement_matrix.as_ref() * uncompressed)
            .data
            .into()
    }

    pub fn decompress(&self, input: &Vec<f64>) -> Vec<f64> {
        let compressed = nalgebra::DVectorView::from_slice(input.as_ref(), M);
        let sparse = self.algorithim.solve(&compressed, &self.sensing_matrix);

        (self.transform.as_ref() * sparse).data.into()
    }
}
