use crate::matrix::{AsChunks, Matrix};

use self::matching_pursuit::MatchingPursuitSolver;
pub mod matching_pursuit;

#[derive(Clone, Copy)]
pub enum Algorithm {
    MatchingPursuit(MatchingPursuitSolver),
}

impl Algorithm {
    pub fn solve<'a, T>(&self, compressed: &'a T, sensing_matrix: &Matrix) -> Vec<f64>
    where
        T: AsChunks<'a> + AsRef<[f64]>,
    {
        match (self, sensing_matrix) {
            // TODO better comparison, error handling
            (Algorithm::MatchingPursuit(mp), Matrix::Identity(_)) => panic!(),
            (Algorithm::MatchingPursuit(mp), Matrix::Real(matrix)) => {
                let samples_in = matrix.dimension().nrows;
                mp.solve(&compressed.chunks(samples_in), matrix.as_ref())
                    .data
                    .into()
            }
        }
    }
}
