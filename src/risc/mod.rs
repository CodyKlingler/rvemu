mod reglock;
mod traits;
use reglock::RegLock;
use traits::reg::Reg;


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
    
}


