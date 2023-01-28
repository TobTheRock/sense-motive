use measurement_matrix::MeasurementMatrix;
use nalgebra::{SVector, SVectorSlice};

pub mod measurement_matrix;
pub mod sensing_matrix;
pub mod transform_matrix;

pub struct Model<const M: usize, const N: usize> {}

impl<const M: usize, const N: usize> Model<M, N> {
    pub fn new() -> Self {
        // todo!()
        Self {}
    }

    pub fn with_seed(&mut self, seed: usize) -> &mut Self {
        todo!()
    }

    pub fn transform_matrix(&mut self) -> &mut Self {
        // todo!()
        self
    }

    pub fn sensing_matrix(&mut self) -> &mut Self {
        // todo!()
        self
    }

    pub fn algorithm(&mut self) -> &mut Self {
        // todo!()
        self
    }

    pub fn compress<T>(&self, input: T) -> Vec<f64>
    where
        T: AsRef<[f64]>,
    {
        let m = MeasurementMatrix::<M, N>::new_bernoulli();
        // TODO padding
        let uncompressed = SVectorSlice::from_slice(input.as_ref());
        let compressed: [f64; M] = (m.as_ref() * uncompressed).into();
        compressed.to_vec()
    }

    pub fn decompress(&self, input: &Vec<f64>) -> Vec<f64> {
        todo!();
    }
}
