use nalgebra::ComplexField;
use simba::scalar::SubsetOf;

use crate::{
    matrix::{AsVectorChunks, Matrix},
    precision::Precision,
};

mod matching_pursuit;
mod orthogonal_matching_pursuit;

#[derive(Clone, Copy)]
pub enum Algorithm {
    MatchingPursuit(matching_pursuit::MatchingPursuitSolver),
    OrthogonalMatchingPursuit(orthogonal_matching_pursuit::OrthogonalMatchingPursuitSolver),
}

impl Algorithm {
    pub fn solve<'a, T, P>(&self, compressed: &'a T, matrix: &nalgebra::DMatrix<P>) -> Vec<P>
    where
        T: AsVectorChunks<'a, f64>,
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
            Algorithm::OrthogonalMatchingPursuit(omp) => {
                let samples_in = matrix.nrows();
                omp.solve(&compressed.as_vec_chuncks(samples_in), matrix)
                    .data
                    .into()
            }
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::OrthogonalMatchingPursuit(
            orthogonal_matching_pursuit::OrthogonalMatchingPursuitSolver::with_parameters(
                1000, 0.1,
            ),
        )
    }
}
