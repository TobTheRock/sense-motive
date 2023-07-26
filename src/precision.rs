use simba::scalar::SupersetOf;

// TODO: naming
pub trait Precision:
    num_traits::Num
    + nalgebra::Scalar
    + nalgebra::ClosedAdd
    + nalgebra::ClosedMul
    + nalgebra::ClosedSub
    + nalgebra::ComplexField
    + SupersetOf<f64>
    + Copy
{
}
// available precisions
impl Precision for f64 {}

pub type Complex64 = num_complex::Complex<f64>;
impl Precision for Complex64 {}
