use super::traits::reg::Reg;
use core::ops::Deref;

#[derive(Debug, Copy, Clone, Default)]
pub struct RegLock<T: Reg + Default> {
    p: T,
    locked: bool,
}

impl<T:Default + Reg> RegLock<T> {

    pub fn new() -> Self {
        Self {p: T::default(), locked:false}
    }

    pub const fn get(&self) -> T {
        self.p
    }

    pub fn set(&mut self, data: T) {
       self.p = if self.locked { self.p } else { data };
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }
}

impl<T:Reg + Default> Deref for RegLock<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.p
    }
}
