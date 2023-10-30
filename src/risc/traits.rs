use num_traits::{Signed, Unsigned};
use num_traits::ops::wrapping::*;
use core::ops::{BitXor, BitAnd, BitOr, Shl, Shr};
use core::cmp::{PartialEq, PartialOrd};

pub trait Reg: 
  Clone +
  Default +
  Copy +
  WrappingAdd + 
  WrappingSub + 
  WrappingMul + 
  BitXor<Output = Self> + 
  BitAnd<Output = Self> + 
  BitOr<Output = Self> +
  Shl<Output = Self>  + 
  Shr<Output = Self>  +
  PartialEq +
  PartialOrd +
  USMinMax + // implemented here
  SignedUnsigned + //implemented here
{}



pub trait USMinMax {
    /// `11111..`
    fn umax() -> Self;
    /// `00000..`
    fn umin() -> Self;
    /// `10000..`
    fn smin() -> Self;
    /// `01111..`
    fn smax() -> Self;
    /// `..0001`
    fn one() -> Self;
}

pub trait SignedUnsigned { 
    type Signed: Reg + Signed;
    type Unsigned: Reg + Unsigned;
    fn as_signed(&self) -> Self::Signed;
    fn as_unsigned(&self) -> Self::Unsigned;
}


macro_rules! impl_usminmax_and_signedunsigned {
    ($($t:ty, $signed_t:ty),*) => {
        $(
            impl USMinMax for $t {
                fn umax() -> Self { <$t>::MAX }
                fn umin() -> Self { <$t>::MIN }
                fn smax() -> Self { <$signed_t>::MAX as Self }
                fn smin() -> Self { <$signed_t>::MIN as Self }
                fn one()  -> Self { 1 as $t }
            }
            impl USMinMax for $signed_t {
                fn umax() -> Self { <$t>::MAX as Self}
                fn umin() -> Self { <$t>::MIN as Self}
                fn smax() -> Self { <$signed_t>::MAX  }
                fn smin() -> Self { <$signed_t>::MIN  }
                fn one()  -> Self { 1 as $signed_t }
            }
            impl SignedUnsigned for $t {
                type Signed = $signed_t;
                type Unsigned = $t;
                fn as_signed(&self) -> Self::Signed {
                    *self as $signed_t
                }
                fn as_unsigned(&self) -> Self::Unsigned {
                    *self
                }
            }
            impl SignedUnsigned for $signed_t {
                type Signed = $signed_t;
                type Unsigned = $t;
                fn as_signed(&self) -> Self::Signed {
                    *self
                }
                fn as_unsigned(&self) -> Self::Unsigned {
                    *self as $t
                }
            }
        )*
    };
}

impl_usminmax_and_signedunsigned ! {
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128
}


impl Reg for u16 {}
impl Reg for u32 {}
impl Reg for u64 {}
impl Reg for u128 {}
impl Reg for i16 {}
impl Reg for i32 {}
impl Reg for i64 {}
impl Reg for i128 {}
