use std::ops::Div;

use nalgebra::{dimension, DMatrix};
use rustdct::DctPlanner;
use rustfft::{FftDirection, FftPlanner};

use crate::{
    matrix::{ComplexConversion, ComplexMatrix, Dimension, Matrix, RealMatrix},
    precision::Complex64,
};

#[derive(Clone, Copy)]
pub enum Transformation {
    None,
    Dct1dInverse,
    Dct1d,
    Fourier1dInverse,
    Fourier1d,
}

impl Transformation {
    pub fn into_matrix(self, dimension: usize) -> Matrix {
        match self {
            Transformation::None => Matrix::Identity(Dimension {
                nrows: dimension,
                ncols: dimension,
            }),
            Transformation::Dct1dInverse => Transformation::dct1d_inverse(dimension).into(),
            Transformation::Dct1d => Transformation::dct1d(dimension).into(),
            Transformation::Fourier1dInverse => Transformation::fft1d_inverse(dimension).into(),
            Transformation::Fourier1d => Transformation::fft1d(dimension).into(),
        }
    }

    // DCT 2, 1D
    fn dct1d(dimension: usize) -> RealMatrix {
        // TODO reuse planner
        let mut planner = DctPlanner::<f64>::new();
        let dct = planner.plan_dct2(dimension);
        let mut scratch = vec![0.0; dct.get_scratch_len()];

        let mut matrix = DMatrix::<f64>::identity(dimension, dimension);
        for mut col in matrix.column_iter_mut() {
            dct.process_dct2_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        // normalize
        matrix = matrix.unscale(f64::sqrt(dimension as f64 / 2.0));

        matrix.into()
    }
    // DCT 2 inverse, 1D
    // TODO consolidate methose
    fn dct1d_inverse(dimension: usize) -> RealMatrix {
        // TODO reuse planner
        let mut planner = DctPlanner::<f64>::new();

        // Inverse of DCT 2 is DCT3
        let dct = planner.plan_dct3(dimension);
        let mut scratch = vec![0.0; dct.get_scratch_len()];

        let mut matrix = DMatrix::<f64>::identity(dimension, dimension);
        for mut col in matrix.column_iter_mut() {
            dct.process_dct3_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        // normalize
        matrix = matrix.unscale(f64::sqrt(dimension as f64 / 2.0));

        matrix.into()
    }

    fn fft1d(dimension: usize) -> ComplexMatrix {
        Transformation::fft(dimension, FftDirection::Forward).into_complex()
    }
    fn fft1d_inverse(dimension: usize) -> ComplexMatrix {
        let mut matrix = Transformation::fft(dimension, FftDirection::Inverse);

        let norm = dimension as f64;
        matrix = matrix.unscale(norm);

        matrix.into_complex()
    }

    fn fft(dimension: usize, direction: FftDirection) -> DMatrix<num_complex::Complex64> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(dimension, direction);

        let mut scratch =
            vec![num_complex::Complex64::new(0.0, 0.0); fft.get_inplace_scratch_len()];
        let mut matrix = DMatrix::<num_complex::Complex64>::identity(dimension, dimension);

        for mut col in matrix.column_iter_mut() {
            fft.process_with_scratch(col.as_mut_slice(), &mut scratch);
        }

        matrix
    }
}

#[cfg(test)]
mod test {
    use crate::{matrix::ComplexGetter, precision::Complex64};
    use approx::assert_relative_eq;
    use nalgebra::{DMatrix, DVector};

    use crate::Transformation;

    const TOLERANCE: f64 = 0.01;
    const N: usize = 4;

    #[test]
    fn dct1d() {
        let t: DMatrix<f64> = Transformation::dct1d(N).into();
        println!("DCT1D {}", t);

        let inv: DMatrix<f64> = Transformation::dct1d_inverse(N).into();
        println!("DCT1D inverse {}", inv);

        let x = DVector::<f64>::from_fn(4, |i, _| i as f64);

        let x_trans = &t * &x;
        let x2 = &inv * x_trans;
        assert_relative_eq!(x, x2, epsilon = TOLERANCE);

        assert_relative_eq!(
            DMatrix::<f64>::identity(4, 4),
            (inv * t),
            epsilon = TOLERANCE
        );
    }

    #[test]
    fn fft1d() {
        let t = Transformation::Fourier1d.into_matrix(N);
        println!("FFT1D {}", t);

        let inv = Transformation::fft1d_inverse(N);
        println!("FFT1D inverse {}", inv);

        let x = DVector::<Complex64>::from_fn(4, |i, _| Complex64::new(i as f64, i as f64));

        let x_trans = &t * &x;
        let x2 = &inv * x_trans;

        assert_relative_eq!(x.real(), x2.real(), epsilon = TOLERANCE);
        assert_relative_eq!(x.imag(), x2.imag(), epsilon = TOLERANCE);

        let unit = inv * t;
        assert_relative_eq!(
            DMatrix::<f64>::identity(4, 4),
            unit.real(),
            epsilon = TOLERANCE
        );
        assert_relative_eq!(
            DMatrix::<f64>::zeros(4, 4),
            unit.imag(),
            epsilon = TOLERANCE
        );
    }
}
