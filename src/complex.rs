use crate::precision::Precision;

pub trait ComplexFields {
    type RealField;
    fn real(&self) -> Self::RealField;
    fn imag(&self) -> Self::RealField;
}

impl<P> ComplexFields for Vec<P>
where
    P: Precision,
{
    type RealField = Vec<P::RealField>;

    fn real(&self) -> Vec<P::RealField> {
        self.iter().map(|e| e.real()).collect()
    }

    fn imag(&self) -> Vec<P::RealField> {
        self.iter().map(|e| e.real()).collect()
    }
}
