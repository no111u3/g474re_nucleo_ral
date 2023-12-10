use stm32ral::modify_reg;
use stm32ral::nvic;

pub use stm32ral::Interrupt;

pub struct Nvic {
    p: nvic::Instance,
}

impl<'a> Nvic {
    pub fn new(p: nvic::Instance) -> Self {
        Self { p }
    }

    pub fn enable_interrupt(&'a self, n: Interrupt) -> &Self {
        let number = n as u16;
        let register = number / 32;
        let number = number % 32;
        match register {
            0 => modify_reg!(nvic, self.p, ISER0, |r| r | (1 << number)),
            1 => modify_reg!(nvic, self.p, ISER1, |r| r | (1 << number)),
            2 => modify_reg!(nvic, self.p, ISER2, |r| r | (1 << number)),
            3 => modify_reg!(nvic, self.p, ISER3, |r| r | (1 << number)),
            _ => unreachable!(),
        }
        self
    }
}
