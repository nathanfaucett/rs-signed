use core::{f32, f64};
use core::intrinsics;
use core::ops::Neg;


use num::Num;


pub trait Signed: Num + Neg<Output=Self> {
    /// # Example
    /// ```rust
    /// use signed::Signed;
    ///
    /// assert_eq!((-1).abs(), 1);
    /// assert_eq!((-1.0).abs(), 1.0);
    /// ```
    fn abs(&self) -> Self;
    /// # Example
    /// ```rust
    /// use signed::Signed;
    ///
    /// assert_eq!(2.abs_sub(&1), 1);
    /// assert_eq!((-1.0).abs_sub(&-2.0), 1.0);
    /// ```
    fn abs_sub(&self, other: &Self) -> Self;
    /// # Example
    /// ```rust
    /// use signed::Signed;
    ///
    /// assert_eq!(0.signum(), 0);
    /// assert_eq!((-1).signum(), -1);
    /// assert_eq!(1.signum(), 1);
    /// ```
    fn signum(&self) -> Self;
    /// # Example
    /// ```rust
    /// use signed::Signed;
    ///
    /// assert_eq!(1.is_positive(), true);
    /// assert_eq!((-1).is_positive(), false);
    /// ```
    fn is_positive(&self) -> bool;
    /// # Example
    /// ```rust
    /// use signed::Signed;
    ///
    /// assert_eq!(1.is_negative(), false);
    /// assert_eq!((-1).is_negative(), true);
    /// ```
    fn is_negative(&self) -> bool;
}


macro_rules! trait_signed {
    ($t:ty) => (
        impl Signed for $t {
            #[inline]
            fn abs(&self) -> Self {
                if self.is_negative() { -*self } else { *self }
            }
            #[inline]
            fn abs_sub(&self, other: &Self) -> Self {
                if *self <= *other { 0 } else { *self - *other }
            }
            #[inline]
            fn signum(&self) -> Self {
                match *self {
                    n if n > 0 => 1,
                    0 => 0,
                    _ => -1,
                }
            }
            #[inline]
            fn is_positive(&self) -> bool { *self > 0 }
            #[inline]
            fn is_negative(&self) -> bool { *self < 0 }
        }
    );
}

trait_signed!(isize);
trait_signed!(i8);
trait_signed!(i16);
trait_signed!(i32);
trait_signed!(i64);


macro_rules! trait_float {
    ($t:ty, $nan:expr, $inf:expr, $neg_inf:expr, $fabs:path, $fcopysign:path, $fdim:ident) => (
        impl Signed for $t {
            #[inline]
            fn abs(&self) -> Self {
                unsafe { $fabs(*self) }
            }
            #[inline]
            fn abs_sub(&self, other: &Self) -> Self {
                extern { fn $fdim(a: $t, b: $t) -> $t; }
                unsafe { $fdim(*self, *other) }
            }
            #[inline]
            fn signum(&self) -> Self {
                if self != self { $nan } else {
                    unsafe { $fcopysign(1.0, *self) }
                }
            }
            #[inline]
            fn is_positive(&self) -> bool { *self > 0.0 || (1.0 / *self) == $inf }
            #[inline]
            fn is_negative(&self) -> bool { *self < 0.0 || (1.0 / *self) == $neg_inf }
        }
    );
}

trait_float!(
    f32, f32::NAN, f32::INFINITY, f32::NEG_INFINITY,
    intrinsics::fabsf32, intrinsics::copysignf32, fdimf
);
trait_float!(
    f64, f64::NAN, f64::INFINITY, f64::NEG_INFINITY,
    intrinsics::fabsf64, intrinsics::copysignf64, fdim
);
