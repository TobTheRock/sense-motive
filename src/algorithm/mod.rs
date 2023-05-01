use crate::{
    matrix::Matrix,
    signal::{RealViewSignal, Signal},
};

use self::matching_pursuit::MatchingPursuitSolver;
pub mod matching_pursuit;

#[derive(Clone, Copy)]
pub enum Algorithm {
    MatchingPursuit(MatchingPursuitSolver),
}

impl Algorithm {
    pub fn solve(
        &self,
        // TODO: type for compressed signal/ sparse signal
        y: &RealViewSignal,
        sensing_matrix: &Matrix,
    ) -> Signal {
        match (self, sensing_matrix) {
            // TODO better comparison, error handling
            (Algorithm::MatchingPursuit(mp), Matrix::Identity(_)) => panic!(),
            (Algorithm::MatchingPursuit(mp), Matrix::Real(matrix)) => {
                let samples_in = matrix.dimension().nrows;
                mp.solve(&y.chunks(samples_in), matrix.as_ref()).into()
            }
        }
    }
}
