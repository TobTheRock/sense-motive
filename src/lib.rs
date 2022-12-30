pub mod measurement_matrix;
pub mod transform_matrix;

pub struct Model<const M: usize, const N: usize> {}

impl<const M: usize, const N: usize> Model<M, N> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_seed(seed: usize) -> Self {
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

    pub fn compress(&self, input: Vec<f64>) -> Vec<f64> {
        todo!();
    }

    pub fn decompress(&self, input: Vec<f64>) -> Vec<f64> {
        todo!();
    }
}
