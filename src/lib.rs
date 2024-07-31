//! # approximately
//!
//! ApproxEq is used to provide an approximate equality for the type you want, but your type must implement the approx trait itself.
//! By default, [`f32`] and [`f64`] have an implementation with tolerances of `1e-3` and `1e-6`, respectively.
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::nursery)]
#![feature(cfg_match)]
#![feature(portable_simd)]
use std::{borrow::Borrow, fmt::Debug};

#[cfg(feature = "simd")]
use std::simd::{f32x4, f64x4, num::SimdFloat};

/// Evaluate if the values.
#[allow(dead_code)]
pub trait ApproxEq: Debug {
    /// Evaluate if the values are approximately equal.
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool;
    /// Panic when the values are not equal.
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:?} != {other:?}");
    }
}

impl ApproxEq for f32 {
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool {
        (self - other.borrow()).abs() <= 1e-3
    }
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:.3?} != {other:.3?}");
    }
}

impl ApproxEq for f64 {
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool {
        (self - other.borrow()).abs() <= 1e-6
    }
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:.6?} != {other:.6?}");
    }
}

impl<A> ApproxEq for [A]
where
    A: ApproxEq + Debug + Clone,
{
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool {
        if self.len() != other.borrow().len() {
            return false;
        }
        self.iter()
            .zip(other.borrow().iter())
            .all(|(a, b)| a.approx(b))
    }
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:?} != {other:?}");
    }
}

impl<A> ApproxEq for Option<A>
where
    A: ApproxEq + Debug + Clone,
{
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool {
        match (self, other.borrow()) {
            (Some(a), Some(b)) => a.approx(b),
            (None, None) => true,
            _ => false,
        }
    }
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:?} != {other:?}");
    }
}

#[cfg(feature = "simd")]
impl ApproxEq for f32x4 {
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool {
        (*self - other.borrow()).abs() <= Self::splat(1e-3)
    }
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:?} != {other:?}");
    }
}

#[cfg(feature = "simd")]
impl ApproxEq for f64x4 {
    fn approx<T: Borrow<Self>>(&self, other: T) -> bool {
        (*self - other.borrow()).abs() <= Self::splat(1e-6)
    }
    fn assert_approx<T: Borrow<Self> + Debug + Clone>(&self, other: T) {
        assert!(self.approx(other.clone()), "{self:?} != {other:?}");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn test_approx_eq() {
        assert!(1.0000f32.approx(1.0001f32), "1.0000 approx 1.0001");
        assert!(1.000f32.approx(1.001f32).not(), "1.000 not approx 1.001");

        assert!(1.000000f64.approx(1.000001f64), "1.000000 approx 1.000001");
        assert!(
            1.0000f64.approx(1.00001f64).not(),
            "1.000000 not approx 1.00001"
        );
    }

    #[test]
    fn test_assert_approx_eq() {
        1.0000f32.assert_approx(1.0001f32);
        1.000000f64.assert_approx(1.000001f64);

        #[cfg(feature = "simd")]
        {
            f32x4::splat(1.0000).assert_approx(f32x4::splat(1.0001));
            f64x4::splat(1.000000).assert_approx(f64x4::splat(1.000001));
        }
    }

    #[test]
    #[should_panic]
    fn test_assert_not_approx_eq() {
        1.000f32.assert_approx(1.001f32);

        // this is not work
        1.0000f64.assert_approx(1.00001f64);
    }
}
