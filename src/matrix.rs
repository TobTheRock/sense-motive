use std::ops::Mul;

use derive_more::{AsRef, From};
use nalgebra::DVectorView;

pub trait AsChunks<'a> {
    fn chunks(&'a self, size: usize) -> DVectorView<'a, f64>;
}

impl<'a, T> AsChunks<'a> for T
where
    T: AsRef<[f64]>,
{
    fn chunks(&'a self, size: usize) -> DVectorView<'a, f64> {
        // TODO slicing, padding, return Iter over DVectorViews
        DVectorView::from_slice(self.as_ref(), size)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension {
    pub nrows: usize,
    pub ncols: usize,
}

impl Dimension {
    fn merge(&self, rhs: &Dimension) -> Dimension {
        debug_assert!(self.ncols == rhs.nrows);
        Dimension {
            nrows: self.nrows,
            ncols: rhs.ncols,
        }
    }
}

#[derive(Debug, From)]
pub enum Matrix {
    Identity(Dimension),
    Real(RealMatrix),
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Matrix::Identity(ldim), Matrix::Identity(rdim)) => Matrix::Identity(ldim.merge(&rdim)),
            (Matrix::Identity(_), Matrix::Real(matrix)) => matrix.into(),
            (Matrix::Real(matrix), Matrix::Identity(_)) => matrix.into(),
            (Matrix::Real(lhs), Matrix::Real(rhs)) => (&lhs * &rhs).into(),
        }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Matrix::Identity(ldim), Matrix::Identity(rdim)) => Matrix::Identity(ldim.merge(rdim)),
            (Matrix::Identity(_), Matrix::Real(matrix)) => Matrix::Real(matrix.clone()),
            (Matrix::Real(matrix), Matrix::Identity(_)) => Matrix::Real(matrix.clone()),
            (Matrix::Real(lhs), Matrix::Real(rhs)) => (lhs * rhs).into(),
        }
    }
}

impl<'a, T> Mul<&'a T> for &Matrix
where
    T: AsChunks<'a> + AsRef<[f64]>,
{
    type Output = Vec<f64>;

    fn mul(self, rhs: &'a T) -> Self::Output {
        match self {
            Matrix::Identity(_) => rhs.as_ref().into(),
            Matrix::Real(matrix) => (matrix * rhs.chunks(matrix.ncols())).data.into(),
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Real(l0), Self::Real(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub type RealMatrix = nalgebra::DMatrix<f64>;
