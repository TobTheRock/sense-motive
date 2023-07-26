use nalgebra::ComplexField;
use simba::scalar::SubsetOf;

use crate::{
    matrix::{AsVectorChunks, Matrix},
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
        T: AsVectorChunks<'a, f64> + AsRef<[f64]>,
        P: Precision,
        <P as ComplexField>::RealField: SubsetOf<f64>,
    {
        match self {
            Algorithm::MatchingPursuit(mp) => {
                let samples_in = matrix.nrows();
                mp.solve(&compressed.as_vec_chuncks(samples_in), matrix)
                    .data
                    .into()
            }
        }
    }
}
