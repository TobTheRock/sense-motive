mod matching_pursuit;

use crate::sensing_matrix::SensingMatrix;
use nalgebra::DVector;

pub trait Algorithm {
    fn solve<const M: usize, const N: usize>(
        &self,
        // TODO: type for compressed signal/ sparse signal
        y: &DVector<f64>,
        sensing_matrix: &SensingMatrix<M, N>,
    ) -> DVector<f64>;
}
