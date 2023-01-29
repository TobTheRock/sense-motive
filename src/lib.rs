use measurement_matrix::MeasurementMatrix;
use nalgebra::DVectorSlice;

pub mod algorithm;
pub mod measurement_matrix;
pub mod sensing_matrix;
pub mod transform_matrix;

pub struct Model<const M: usize, const N: usize> {
    measurement: MeasurementMatrix<M, N>,
}

impl<const M: usize, const N: usize> Model<M, N> {
    pub fn new() -> Self {
        Self {
            measurement: MeasurementMatrix::<M, N>::new_bernoulli(),
        }
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
        let uncompressed = DVectorSlice::from_slice(input.as_ref(), N);
        (self.measurement.as_ref() * uncompressed).data.into()
    }

    pub fn decompress(&self, input: &Vec<f64>) -> Vec<f64> {
        todo!();
    }
}
