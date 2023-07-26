use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use derive_more::{Display, From};
use nalgebra::{
    constraint::AreMultipliable, ClosedMul, Complex, ComplexField, DMatrix, DVectorView, Dyn,
    RealField, VectorView, U1,
};
use simba::scalar::{SubsetOf, SupersetOf};

use crate::precision::{Complex64, Precision};

pub trait AsVectorChunks<'a, P>: AsRef<[P]>
where
    P: Precision,
{
    fn as_vec_chuncks(&'a self, size: usize) -> DVectorView<'a, P>;
}

impl<'a, T, P> AsVectorChunks<'a, P> for T
where
    T: AsRef<[P]>,
    P: Precision,
{
    fn as_vec_chuncks(&'a self, size: usize) -> DVectorView<'a, P> {
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

impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("(r: {}, c: {})", self.nrows, self.ncols));
        Ok(())
    }
}

#[derive(Debug, From, Display)]
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
            (Matrix::Identity(_), Matrix::Complex(matrix)) => matrix.into(),
            (Matrix::Real(_), Matrix::Complex(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Identity(matrix)) => matrix.into(),
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
            (Matrix::Identity(_), Matrix::Real(matrix)) => matrix.clone().into(),
            (Matrix::Real(matrix), Matrix::Identity(_)) => matrix.clone().into(),
            (Matrix::Real(lhs), Matrix::Real(rhs)) => (lhs * rhs).into(),
            (Matrix::Identity(_), Matrix::Complex(matrix)) => matrix.clone().into(),
            (Matrix::Real(_), Matrix::Complex(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Identity(matrix)) => matrix.clone().into(),
            (Matrix::Complex(_), Matrix::Real(_)) => todo!(),
            (Matrix::Complex(_), Matrix::Complex(_)) => todo!(),
        }
    }
}

// multiplication with a real vector
impl Mul<&[f64]> for &Matrix {
    type Output = Vec<f64>;

    fn mul(self, rhs: &[f64]) -> Self::Output {
        match self {
            Matrix::Identity(_) => rhs.as_ref().into(),
            Matrix::Real(matrix) => matrix.mul_chunked(rhs),
            Matrix::Complex(matrix) => matrix.real().mul_chunked(rhs),
        }
    }
}
impl Mul<&[Complex64]> for &Matrix {
    type Output = Vec<Complex64>;

    fn mul(self, rhs: &[Complex64]) -> Self::Output {
        match self {
            Matrix::Identity(_) => rhs.into(),
            Matrix::Real(matrix) => {
                let cmatrix: DMatrix<Complex64> = matrix.to_superset();
                cmatrix.mul_chunked(rhs)
            }
            Matrix::Complex(matrix) => matrix.mul_chunked(rhs),
        }
    }
}

pub type RealMatrix = nalgebra::DMatrix<f64>;
pub type ComplexMatrix = nalgebra::DMatrix<Complex64>;

pub trait MatrixComplexFields<P, R, C>
where
    P: Precision,
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    nalgebra::DefaultAllocator: nalgebra::allocator::Allocator<P::RealField, R, C>,
{
    fn real(&self) -> nalgebra::OMatrix<P::RealField, R, C>;
    fn imag(&self) -> nalgebra::OMatrix<P::RealField, R, C>;
}

// TODO return view instead of copy
impl<P, R, C, S> MatrixComplexFields<P, R, C> for nalgebra::Matrix<P, R, C, S>
where
    P: Precision,
    S: nalgebra::RawStorage<P, R, C> + nalgebra::Storage<P, R, C>,
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    nalgebra::DefaultAllocator: nalgebra::allocator::Allocator<P::RealField, R, C>,
{
    fn real(&self) -> nalgebra::OMatrix<P::RealField, R, C> {
        self.map(|e| e.real())
    }

    fn imag(&self) -> nalgebra::OMatrix<P::RealField, R, C> {
        self.map(|e| e.imaginary())
    }
}

pub trait ChunkedMultiplication<P>
where
    P: Precision,
{
    fn mul_chunked(&self, signal: &[P]) -> Vec<P>;
}

impl<P> ChunkedMultiplication<P> for nalgebra::DMatrix<P>
where
    P: Precision,
{
    fn mul_chunked(&self, signal: &[P]) -> Vec<P> {
        // TODO use asVecChuncks
        let size = self.ncols();
        let vec = DVectorView::from_slice(signal, size);

        (self * vec).data.into()
    }
}

// trait VectorHelper<P, R>
// where
//     P: Precision,
//     R: nalgebra::Dim,
// {
//     //! equivalent to iamax
//     fn max_abs_idx() -> usize;
// }

// impl<P, R, S> VectorHelper<P, R> for nalgebra::Matrix<P, R, nalgebra::U1, S>
// where
//     P: Precision,
//     R: nalgebra::Dim,
// {
//     fn max_abs_idx() -> usize {
//         todo!()
//     }
// }
