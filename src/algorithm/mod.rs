pub mod matching_pursuit;

use crate::sensing_matrix::SensingMatrix;

pub trait Algorithm<const M: usize, const N: usize> {
    fn solve(
        &self,
        // TODO: type for compressed signal/ sparse signal
        y: &nalgebra::DVectorView<f64>,
        sensing_matrix: &SensingMatrix,
    ) -> nalgebra::DVector<f64>;
}
