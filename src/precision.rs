use derive_more::{Add, AsMut, AsRef, Constructor, Display, From, Into, Mul};
use nalgebra::DMatrix;

pub trait Precision:
    num_traits::Num
    + num_traits::Signed
    + nalgebra::Scalar
    + nalgebra::ClosedAdd
    + nalgebra::ClosedMul
    + nalgebra::ClosedSub
    + nalgebra::ComplexField
    + std::ops::Mul<nalgebra::DMatrix<Self>>
    + PartialOrd
{
}

// available precision
impl Precision for f64 {}

#[derive(Display, Debug, Clone, From, Into, AsRef, PartialEq, AsMut, Add, Mul)]
pub struct Complex64(pub num_complex::Complex<f64>);
// impl Precision for Complex64 {}

impl Complex64 {
    pub fn new(re: f64, im: f64) -> Self {
        num_complex::Complex::<f64>::new(re, im).into()
    }
}

// impl num_traits::Zero for Complex64 {
//     fn zero() -> Self {
//         num_complex::Complex::<f64>::zero().into()
//     }

//     fn is_zero(&self) -> bool {
//         self.0.is_zero()
//     }
// }

// impl num_traits::One for Complex64 {
//     fn one() -> Self {
//         num_complex::Complex::<f64>::one().into()
//     }
// }

// impl std::ops::Mul for Complex64 {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         (self.0 * rhs.0).into()
//     }
// }

impl PartialOrd for Complex64 {
    //! compare `a + b*i`with `c + d*i`
    //! partially ordering holds when testing
    //! a <= b && c <= d
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.re.partial_cmp(&other.0.re) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.0.im.partial_cmp(&other.0.im)
    }
}
