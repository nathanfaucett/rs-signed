#![no_std]


pub trait Signum {
    fn signum(self) -> Self;
}

macro_rules! trait_signum_unsigned {
    ($t:ident) => (
        impl Signum for $t {
            #[inline(always)]
            fn signum(self) -> Self { if self > 0 {1} else {0} }
        }
    );
}

macro_rules! trait_signum_signed {
    ($t:ident) => (
        impl Signum for $t {
            #[inline(always)]
            fn signum(self) -> Self { if self > 0 {1} else if self < 0 {-1} else {0} }
        }
    );
}

macro_rules! trait_signum_float {
    ($t:ident) => (
        impl Signum for $t {
            #[inline(always)]
            fn signum(self) -> Self { if self > 0.0 {1.0} else if self < 0.0 {-1.0} else {0.0} }
        }
    );
}

trait_signum_unsigned!(usize);
trait_signum_unsigned!(u8);
trait_signum_unsigned!(u16);
trait_signum_unsigned!(u32);
trait_signum_unsigned!(u64);

trait_signum_signed!(isize);
trait_signum_signed!(i8);
trait_signum_signed!(i16);
trait_signum_signed!(i32);
trait_signum_signed!(i64);

trait_signum_float!(f32);
trait_signum_float!(f64);

#[test]
fn signum() {
    assert_eq!((0u32).signum(), 0u32);
    assert_eq!((1u32).signum(), 1u32);

    assert_eq!((0i32).signum(), 0i32);
    assert_eq!((-1i32).signum(), -1i32);
    assert_eq!((1i32).signum(), 1i32);

    assert_eq!((-1f32).signum(), -1f32);
    assert_eq!((1f32).signum(), 1f32);
}
