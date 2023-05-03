use std::ops::Mul;

use derive_more::From;
use nalgebra::DVectorView;
use rustfft::num_complex::Complex64;

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
    Complex(ComplexMatrix),
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Matrix::Identity(ldim), Matrix::Identity(rdim)) => Matrix::Identity(ldim.merge(&rdim)),
            (Matrix::Identity(_), Matrix::Real(matrix)) => matrix.into(),
            (Matrix::Real(matrix), Matrix::Identity(_)) => matrix.into(),
            (Matrix::Real(lhs), Matrix::Real(rhs)) => (&lhs * &rhs).into(),
            (Matrix::Identity(_), Matrix::Complex(_)) => todo!(),
            (Matrix::Real(_), Matrix::Complex(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Identity(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Real(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Complex(_)) => todo!(),
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
            (Matrix::Identity(_), Matrix::Complex(_)) => todo!(),
            (Matrix::Real(_), Matrix::Complex(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Identity(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Real(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Complex(_)) => todo!(),
        }
    }
}

// multiplication with a real vector
impl<'a, T> Mul<&'a T> for &Matrix
where
    T: AsChunks<'a> + AsRef<[f64]>,
{
    type Output = Vec<f64>;

    fn mul(self, rhs: &'a T) -> Self::Output {
        match self {
            Matrix::Identity(_) => rhs.as_ref().into(),
            Matrix::Real(matrix) => (matrix * rhs.chunks(matrix.ncols())).data.into(),
            Matrix::Complex(_) => todo!(),
        }
    }
}

pub type RealMatrix = nalgebra::DMatrix<f64>;
pub type ComplexMatrix = nalgebra::DMatrix<Complex64>;

pub trait ComplexGetter<R, C>
where
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    nalgebra::DefaultAllocator: nalgebra::allocator::Allocator<f64, R, C>,
{
    fn real(&self) -> nalgebra::OMatrix<f64, R, C>;
    fn imag(&self) -> nalgebra::OMatrix<f64, R, C>;
}

// TODO return view instead of copy
impl<R, C, S> ComplexGetter<R, C> for nalgebra::Matrix<Complex64, R, C, S>
where
    S: nalgebra::RawStorage<Complex64, R, C>,
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    nalgebra::DefaultAllocator: nalgebra::allocator::Allocator<f64, R, C>,
{
    fn real(&self) -> nalgebra::OMatrix<f64, R, C> {
        self.map(|e| e.re)
    }

    fn imag(&self) -> nalgebra::OMatrix<f64, R, C> {
        self.map(|e| e.im)
    }
}
