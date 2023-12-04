#![no_main]
#![no_std]

use cortex_m_rt::entry;

use panic_halt as _;

use stm32ral::stm32g4::peripherals;

#[entry]
fn main() -> ! {
    loop {}
}
