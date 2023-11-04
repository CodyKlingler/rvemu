mod reglock;
mod traits;
use reglock::RegLock;
use traits::reg::Reg;
use core::mem::size_of;

pub struct RiscV<T: Reg, const N_BYTES: usize> { 
    x: [RegLock<T>; 32],
    pc: RegLock<T>,
    mem: [u8; N_BYTES],
}

impl<T: Reg, const N_BYTES: usize> RiscV<T, N_BYTES> {
    pub fn new() -> Self {
        let mut new = Self {
            x: [RegLock::<T>::default(); 32],
            pc: RegLock::<T>::default(),
            mem: [0; N_BYTES],
        };
        new.x[0].lock();
        new
    }
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds,
    ConversionFailure
}

impl<T: Reg, const N_BYTES: usize> RiscV<T, N_BYTES> {

    // gets byte from memory. returns u8
    const fn get_byte(&self, addr: usize) -> Result<u8,MemoryError>{
        if addr < self.mem.len() {
            Ok(self.mem[addr])
        }
        else {
            Err(MemoryError::OutOfBounds)
        }
    }

    // load byte unsigned
    pub fn lbu(&self, addr: usize) -> Result<T,MemoryError> {
        let byte = self.get_byte(addr)?;
        if let Some(ret) = T::from_u8(byte) {
            Ok(ret)
        } else {
            Err(MemoryError::ConversionFailure)
        }
    }

    /// load word
    pub fn lw(&self, addr: usize) -> Result<T,MemoryError>{
        self.load_n_bytes_u(addr, size_of::<T>())
    }

    /// load half-word (unsigned)
    pub fn lhu(&self, addr: usize) -> Result<T,MemoryError>{
        self.load_n_bytes_u(addr, size_of::<T>()/2)
    }

    /// load `n` bytes (unsigned)
    fn load_n_bytes_u(&self, addr: usize, n: usize) -> Result<T,MemoryError>{
        let mut r = T::umin();
        for i in 0.. n{
            let byte = self.get_byte(addr+i)?;
            let byte_t = T::from_u8(byte)
                .ok_or_else(|| MemoryError::ConversionFailure)?;
            let n_shifts = T::from_usize(8*i)
                .ok_or_else(|| MemoryError::ConversionFailure)?;
            r = r | (byte_t << n_shifts);
        }
        Ok(r)
    }

    /// load byte (signed) 
    pub fn lb(&self, addr:usize) -> Result<T,MemoryError>{
        let byte = self.get_byte(addr)?;
        let ret = T::from_u8(byte).ok_or_else(|| MemoryError::ConversionFailure)?;
        let loooooo = T::one() << T::from_u8(7) // binary 1000 0000
            .ok_or_else(|| MemoryError::ConversionFailure)?;
        let byte_is_negative = ret & loooooo == loooooo;
        if byte_is_negative {
            let ooollllllll = T::from_u8(u8::MAX)
                .ok_or_else(|| MemoryError::ConversionFailure)?;
            let llloooooooo = ooollllllll ^ T::umax();
            Ok(llloooooooo |  ret)
        }
        else {
            Ok(ret)
        }
    }

    fn load_n_bytes(&self, addr: usize, n_bytes: usize) -> Result<T,MemoryError>{

        let mut r = T::umin(); // r = 0

        // load each byte into r
        for i in 0.. n_bytes{
            let byte = self.get_byte(addr+i)?;
            let byte_t = T::from_u8(byte)
                .ok_or_else(|| MemoryError::ConversionFailure)?;
            let n_shifts = T::from_usize(8*i)
                .ok_or_else(|| MemoryError::ConversionFailure)?;
            r = r | (byte_t << n_shifts)
        }

        // If msb of half-word is 1, the leading bits should be 1 to maintain the negative sign 
        let loooooo = T::one() << T::from_usize(n_bytes*8-1)
            .ok_or_else(|| MemoryError::ConversionFailure)?;// MSB mask
        let is_negative = r & loooooo == loooooo;
        if is_negative {
            r = r | T::umax() << T::from_usize(n_bytes*8)
                .ok_or_else(|| MemoryError::ConversionFailure)?;
        }

        Ok(r)
    }

    /// load half-word (signed)
    pub fn lh(&self, addr: usize) -> Result<T,MemoryError> {
        self.load_n_bytes(addr, size_of::<T>()/2)
    }


    /// store byte
    pub fn sb(&mut self, b: u8, addr: usize) -> Result<(), MemoryError> {
        if addr < self.mem.len() {
            self.mem[addr] = b;
            Ok(())
        }
        else {
            Err(MemoryError::OutOfBounds)
        }
    }
}



#[cfg(test)]
pub mod test {
    
    use super::*;
    #[test]
    fn memory_read_write() -> Result<(), MemoryError> {
        let mut cpu = RiscV::<u32, 1024>::new();

        // Test out of bounds
        assert!(cpu.sb(0,cpu.mem.len()).is_err());
        assert!(cpu.lb(cpu.mem.len()).is_err());
        assert!(cpu.lbu(cpu.mem.len()).is_err());
        assert!(cpu.lh(cpu.mem.len()).is_err());
        assert!(cpu.lhu(cpu.mem.len()).is_err());
        assert!(cpu.lw(cpu.mem.len()).is_err());

        // taste basic read and write
        cpu.sb(1_u8, 0)?;
        assert_eq!(cpu.lbu(0)?, 1);
        cpu.sb(0_u8, 0)?;
        assert_eq!(cpu.lbu(0)?, 0);

        // write 0xFF..FF in mem[1.. 3]
        for i in 0.. (32/8) {
            cpu.sb(0xFF, i+1)?;
        }

        assert_eq!(cpu.lb(1)?,  0xFFFF_FFFF);
        assert_eq!(cpu.lbu(1)?, 0x0000_00FF);
        assert_eq!(cpu.lh(1)?,  0xFFFF_FFFF);
        assert_eq!(cpu.lhu(1)?, 0x0000_FFFF);
        assert_eq!(cpu.lw(1)?,  0xFFFF_FFFF);

        assert_eq!(cpu.lb(0)?,  0x0);
        assert_eq!(cpu.lbu(0)?, 0x0);
        assert_eq!(cpu.lh(0)?,  0xFFFF_FF00);
        assert_eq!(cpu.lhu(0)?, 0x0000_FF00);
        assert_eq!(cpu.lw(0)?,  0xFFFF_FF00);

        cpu.sb(i8::MIN as u8, 100)?;  // 0x80 at addr[100]
        assert_eq!(cpu.lb(100)?,  0xFFFF_FF80);
        assert_eq!(cpu.lbu(100)?, 0x0000_0080);
        assert_eq!(cpu.lh(100)?,  0x0000_0080);
        assert_eq!(cpu.lhu(100)?, 0x0000_0080);
        assert_eq!(cpu.lw(100)?,  0x0000_0080);

        assert_eq!(cpu.lh(99)?,  0xFFFF8000);
        assert_eq!(cpu.lhu(99)?, 0x00008000);     
        assert_eq!(cpu.lw(99)?, 0x00008000);    
        assert_eq!(cpu.lw(100)?, 0x00000080); 

        Ok(())
    }
}

