use crate::{
    matrix::{AsChunks, Matrix},
    precision::Precision,
};

use self::matching_pursuit::MatchingPursuitSolver;
pub mod matching_pursuit;

#[derive(Clone, Copy)]
pub enum Algorithm {
    MatchingPursuit(MatchingPursuitSolver),
}

impl Algorithm {
    pub fn solve<'a, T, P>(&self, compressed: &'a T, matrix: &nalgebra::DMatrix<P>) -> Vec<P>
    where
        T: AsChunks<'a> + AsRef<[f64]>,
        P: Precision,
    {
        match self {
            Algorithm::MatchingPursuit(mp) => {
                let samples_in = matrix.nrows();
                mp.solve(&compressed.chunks(samples_in), matrix).data.into()
            }
        }
    }
}
