use std::ops::Mul;

use nalgebra::{DMatrix, DVectorView};

use crate::signal::{RealOwnedSignal, RealViewSignal, Signal};

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

#[derive(Debug)]
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

impl<'a> Mul<Signal<'a>> for &Matrix {
    type Output = RealOwnedSignal;

    fn mul(self, rhs: Signal) -> Self::Output {
        match (self, rhs) {
            (Matrix::Identity(_), Signal::RealOwned(s)) => s,
            (Matrix::Identity(_), Signal::RealView(s)) => s.into(),
            (Matrix::Real(m), Signal::RealOwned(s)) => m * s.chunks(m.dimension().ncols),
            (Matrix::Real(m), Signal::RealView(s)) => m * s.chunks(m.dimension().ncols),
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

impl From<nalgebra::DMatrix<f64>> for Matrix {
    fn from(matrix: nalgebra::DMatrix<f64>) -> Self {
        Matrix::Real(RealMatrix::from(matrix))
    }
}

impl From<Vec<f64>> for Matrix {
    fn from(vec: Vec<f64>) -> Self {
        Matrix::Real(RealMatrix::from(vec))
    }
}

impl Matrix {
    fn dimension(&self) -> Dimension {
        match self {
            Matrix::Identity(dim) => *dim,
            Matrix::Real(matrix) => matrix.dimension(),
        }
    }
}

// TODO use a tuple and derive AsRef
#[derive(Clone, Debug, PartialEq)]
pub struct RealMatrix {
    inner: nalgebra::DMatrix<f64>,
}

impl From<nalgebra::DMatrix<f64>> for RealMatrix {
    fn from(matrix: nalgebra::DMatrix<f64>) -> Self {
        Self { inner: matrix }
    }
}

impl Into<nalgebra::DMatrix<f64>> for RealMatrix {
    fn into(self) -> nalgebra::DMatrix<f64> {
        self.inner
    }
}

impl From<Vec<f64>> for RealMatrix {
    fn from(vec: Vec<f64>) -> Self {
        let nrows = vec.len();
        Self {
            inner: DMatrix::from_vec(nrows, 1, vec),
        }
    }
}

impl AsRef<nalgebra::DMatrix<f64>> for RealMatrix {
    fn as_ref(&self) -> &nalgebra::DMatrix<f64> {
        &self.inner
    }
}

impl Into<Matrix> for RealMatrix {
    fn into(self) -> Matrix {
        Matrix::Real(self)
    }
}

impl Into<Vec<f64>> for RealMatrix {
    fn into(self) -> Vec<f64> {
        self.inner.data.into()
    }
}

impl Mul for &RealMatrix {
    type Output = RealMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self.inner * &rhs.inner).into()
    }
}

impl<'a> Mul<DVectorView<'a, f64>> for &RealMatrix {
    type Output = RealOwnedSignal;

    fn mul(self, rhs: DVectorView<'a, f64>) -> Self::Output {
        let result = &self.inner * rhs;
        let vec: Vec<f64> = result.data.into();
        RealOwnedSignal::from(vec)
    }
}

impl RealMatrix {
    pub fn dimension(&self) -> Dimension {
        Dimension {
            nrows: self.inner.nrows(),
            ncols: self.inner.ncols(),
        }
    }
}
