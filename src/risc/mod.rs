mod reglock;
mod traits;
use reglock::RegLock;
use traits::Reg;


pub struct RiscV<T: Reg> { 
    x: [RegLock<T>; 32],
    pc: RegLock<T>,
}


impl RiscV<u32> {
    pub fn new() -> Self {
        let mut new = Self {
            x: [RegLock::<u32>::default(); 32],
            pc: RegLock::<u32>::default(),
        };
        new.x[0].lock();
        new
    }
}
impl RiscV<u64> {
    pub fn new() -> Self {
        let mut new = Self {
            x: [RegLock::<u64>::default(); 32],
            pc: RegLock::<u64>::default(),
        };
        new.x[0].lock();
        new
    }
}
impl RiscV<u128> {
    pub fn new() -> Self {
        let mut new = Self {
            x: [RegLock::<u128>::default(); 32],
            pc: RegLock::<u128>::default(),
        };
        new.x[0].lock();
        new
    }
}

impl<T: Reg> RiscV<T> {
    pub fn add(&self, v1: T, v2: T) -> T {
        v1.wrapping_add(&v2)
    }

    pub fn sub(&self, v1: T, v2: T) -> T {
        v1.wrapping_sub(&v2)
    }

    pub fn mul(&self, v1: T, v2: T) -> T {
        v1.wrapping_mul(&v2)
    }

    pub fn xor(&self, v1: T, v2: T) -> T {
        v1 ^ v2
    }

    pub fn or(&self, v1: T, v2: T) -> T {
        v1 | v2
    }

    pub fn and(&self, v1: T, v2: T) -> T {
        v1 & v2
    }

    /// Shift Left Logical
    pub fn sll(&self, v1: T, v2: T) -> T {
        v1 << v2
    }

    /// Shift Right Logical
    pub fn srl(&self, v1: T, v2: T) -> T {
        v1 >> v2
    }

    /// Shift Right Arithmetic
    pub fn sra(&self, v1: T, v2: T) -> T {
        let srl = v1 >> v2;
        let loooo = T::smin();

        if srl & loooo == loooo {   // if the msb in srl is 1 (neg #)
            let ooolll = T::umax() >> v2;
            let lllooo = ooolll ^ T::umax(); // NOT
            srl | lllooo    // set the leading 0's to 1
        } else {
            srl
        }
    }

    /// Set Less Than (Signed)
    pub fn slt(&self, v1: T, v2: T) -> T {
        // could alternatively do with bit manipulation and remove the SignedUnsigned trait
        if v1.as_signed() < v2.as_signed() {
            T::one()
        }
        else {
            T::umin()
        }
    }

    /// Set Less Than (Unsigned)
    pub fn sltu(&self, v1: T, v2: T) -> T {
        // could alternatively do with bit manipulation and remove the SignedUnsigned trait
        if v1.as_unsigned() < v2.as_unsigned() {
            T::one()
        }
        else {
            T::umin()
        }
    }
}
