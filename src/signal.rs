use derive_more::{AsRef, From, Into};
use nalgebra::{DVector, DVectorView};

#[derive(From)]
pub enum Signal<'a> {
    RealOwned(RealOwnedSignal),
    RealView(RealViewSignal<'a>),
}

#[derive(From, Into, AsRef)]
pub struct RealOwnedSignal(Vec<f64>);

impl<'a> RealOwnedSignal {
    pub fn chunks(&'a self, size: usize) -> DVectorView<'a, f64> {
        // TODO slicing, padding, return Iter over DVectorViews
        DVectorView::from_slice(self.0.as_slice(), size)
    }
}

impl<'a> From<DVector<f64>> for Signal<'a> {
    fn from(value: DVector<f64>) -> Self {
        let s = RealOwnedSignal(value.data.into());
        Self::RealOwned(s)
    }
}

#[derive(From, Into, AsRef)]
pub struct RealViewSignal<'a>(&'a [f64]);

impl<'a> RealViewSignal<'a> {
    pub fn chunks(&self, size: usize) -> DVectorView<'a, f64> {
        // TODO slicing, padding, return Iter over DVectorViews
        DVectorView::from_slice(self.0, size)
    }
}

impl<'a> Into<RealOwnedSignal> for RealViewSignal<'a> {
    fn into(self) -> RealOwnedSignal {
        self.0.to_vec().into()
    }
}
