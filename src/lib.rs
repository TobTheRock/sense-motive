use algorithm::{matching_pursuit::MatchingPursuit, Algorithm};
use measurement_matrix::MeasurementMatrix;
use sensing_matrix::SensingMatrix;
use transform_matrix::TransformMatrix;

pub mod algorithm;
pub mod measurement_matrix;
pub mod sensing_matrix;
pub mod transform_matrix;

// TODO maybe don't use const generic so one could create models of dynamic sizes
pub struct Model<const M: usize, const N: usize> {
    algorithim: Box<dyn Algorithm<M, N>>,
    measurement: MeasurementMatrix<M, N>,
    transform: TransformMatrix<N>,
    sensing: lazy_init::Lazy<SensingMatrix<M, N>>,
}

impl<const M: usize, const N: usize> Default for Model<M, N> {
    fn default() -> Self {
        Self {
            algorithim: MatchingPursuit::with_parameters(1000, 0.1),
            measurement: MeasurementMatrix::new_bernoulli(),
            transform: TransformMatrix::dct1d_inverse(),
            sensing: lazy_init::Lazy::new(),
        }
    }
}

impl<const M: usize, const N: usize> Model<M, N> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_seed(&mut self, seed: usize) -> &mut Self {
        todo!()
    }

    pub fn transform_matrix(&mut self) -> &mut Self {
        todo!()
    }

    pub fn sensing_matrix(&mut self) -> &mut Self {
        todo!()
    }

    pub fn algorithm(&mut self) -> &mut Self {
        todo!()
    }

    pub fn compress<T>(&self, input: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        // TODO padding
        let uncompressed = nalgebra::DVectorView::from_slice(input.as_ref(), N);
        (self.measurement.as_ref() * uncompressed).data.into()
    }

    pub fn decompress(&self, input: &Vec<f64>) -> Vec<f64> {
        let sensing_matrix = self
            .sensing
            .get_or_create(|| SensingMatrix::from(&self.measurement, &self.transform));
        let compressed = nalgebra::DVectorView::from_slice(input.as_ref(), M);

        let sparse = self.algorithim.solve(&compressed, sensing_matrix);

        // (self.transform.as_ref() * sparse).data.into()
        sparse.data.into()
    }
}
