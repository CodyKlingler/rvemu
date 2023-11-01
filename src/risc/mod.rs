mod reglock;
mod traits;
use reglock::RegLock;
use traits::reg::Reg;


pub struct RiscV<T: Reg, const N_BYTES: usize> { 
    x: [RegLock<T>; 32],
    pc: RegLock<T>,
    mem: [u8; N_BYTES],
}


impl<T: Reg, const N_BYTES: usize> RiscV<T, N_BYTES> {
    pub fn new(n_bytes: usize) -> Self {
        let mut new = Self {
            x: [RegLock::<T>::default(); 32],
            pc: RegLock::<T>::default(),
            mem: [0; N_BYTES],
        };
        new.x[0].lock();
        new
    }
}


enum MemoryError {
    Unitialized,
    OutOfBounds,
    AlreadyInitialized,
}

impl<T: Reg, const N_BYTES: usize> RiscV<T, N_BYTES> {
    pub const fn mem(&self, elem: usize) -> Result<u8,MemoryError>{

                if elem < self.mem.len() {
                    Ok(self.mem[elem])
                }
                else {
                    Err(MemoryError::OutOfBounds)
                }
    }
}


