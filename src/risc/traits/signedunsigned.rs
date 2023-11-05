use num_traits::{Signed, Unsigned};
use super::reg::Reg;


pub trait USMinMax {
    /// `11111..`
    fn umax() -> Self;
    /// `00000..`
    fn umin() -> Self;
    /// `10000..`
    fn smin() -> Self;
    /// `01111..`
    fn smax() -> Self;
}

pub trait SignedUnsigned { 
    type Signed: Signed + Reg; 
    // +Reg included so the returned type of as_signed may have all of the Reg operations used on it
    type Unsigned: Unsigned + Reg;
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
            }
            impl USMinMax for $signed_t {
                fn umax() -> Self { <$t>::MAX as Self}
                fn umin() -> Self { <$t>::MIN as Self}
                fn smax() -> Self { <$signed_t>::MAX  }
                fn smin() -> Self { <$signed_t>::MIN  }
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


// likely not needed

// pub trait RegSize {
//     fn size() -> usize;
// }

// macro_rules! impl_regsize {
//     ($($t:ty, $bytes:literal),*) => {
//         $(
//             impl RegSize for $t {
//                 fn size() -> usize {
//                     $bytes
//                 }
//             }
//         )*
//     };
// }

// impl_regsize! {
//     u16, 16,
//     i16, 16,
//     u32, 32,
//     i32, 32,
//     u64, 64,
//     i64, 64, 
//     u128, 128,
//     i128, 128
// }