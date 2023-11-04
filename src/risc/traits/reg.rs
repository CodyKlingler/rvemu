use num_traits::ops::wrapping::*;
use core::ops::{BitXor, BitAnd, BitOr, Shl, Shr};
use core::cmp::{PartialEq, PartialOrd};
use super::signedunsigned::*;
use num_traits::{FromBytes, ToBytes, ToPrimitive};
use num_traits::cast::FromPrimitive;

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
  FromPrimitive +
  FromBytes +
  //ToBytes<Bytes = >>
{
    fn add(&self, v2: Self) -> Self {
        self.wrapping_add(&v2)
    }

    fn sub(&self, v2: Self) -> Self {
        self.wrapping_sub(&v2)
    }

    fn mul(&self, v2: Self) -> Self {
        self.wrapping_mul(&v2)
    }

    fn xor(&self, v2: Self) -> Self {
        *self ^ v2
    }

    fn or(&self, v2: Self) -> Self {
        *self | v2
    }

    fn and(&self, v2: Self) -> Self {
        *self & v2
    }

    /// Shift Left Logical
    fn sll(&self, v2: Self) -> Self {
        *self << v2
    }

    /// Shift Right Logical
    fn srl(&self, v2: Self) -> Self {
        *self >> v2
    }

    /// Shift Right Arithmetic
    fn sra(&self, v2: Self) -> Self {
        // doing >> on signed integers will do an arithmetic shift,
        // but would likely be difficult to reconcile with the compiler while maintaining generics
        let srl = *self >> v2;
        let loooo = Self::smin();

        if *self & loooo == loooo {   // if the msb in srl is 1 (neg #)
            let ooolll = Self::umax() >> v2;
            let lllooo = ooolll ^ Self::umax(); // NOSelf
            srl | lllooo    // set the leading 0's to 1
        } else {
            srl
        }
    }

    /// Set Less Selfhan (Signed)
    fn slt(&self, v2: Self) -> Self {
        // could alternatively do with bit manipulation and remove the SignedUnsigned trait
        if self.as_signed() < v2.as_signed() {
            Self::one()
        }
        else {
            Self::umin()
        }
    }

    /// Set Less Selfhan (Unsigned)
    fn sltu(&self, v2: Self) -> Self {
        // could alternatively do with bit manipulation and remove the SignedUnsigned trait
        if self.as_unsigned() < v2.as_unsigned() {
            Self::one()
        }
        else {
            Self::umin()
        }
    }
}



impl Reg for u16 {}
impl Reg for u32 {}
impl Reg for u64 {}
impl Reg for u128 {}
impl Reg for i16 {}
impl Reg for i32 {}
impl Reg for i64 {}
impl Reg for i128 {}
