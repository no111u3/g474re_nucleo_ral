#![no_main]
#![no_std]

use cortex_m_rt::entry;

use panic_halt as _;

use defmt::info;

use defmt_rtt as _;

use stm32ral as ral;

use ral::{gpio, interrupt, nvic, rcc, tim2};
use ral::{modify_reg, write_reg};

use g474re_nucleo_ral::gpio::{Gpio, PinAltFunc, PinIndex, PinSpeed};
use g474re_nucleo_ral::nvic::{Interrupt, Nvic};

#[entry]
fn main() -> ! {
    let rcc = rcc::RCC::take().unwrap();
    let gpio_a = Gpio::new(gpio::GPIOA::take().unwrap());
    let tim2 = tim2::TIM2::take().unwrap();
    let nvic = Nvic::new(nvic::NVIC::take().unwrap());

    modify_reg!(rcc, rcc, AHB2ENR, GPIOAEN: Enabled);

    gpio_a
        .set_mode_alternate(PinIndex::Pin5)
        .set_speed(PinIndex::Pin5, PinSpeed::HighSpeed)
        .set_alt_func(PinIndex::Pin5, PinAltFunc::Af1);

    modify_reg!(rcc, rcc, APB1ENR1, TIM2EN: Enabled);

    write_reg!(tim2, tim2, DIER, UIE: 1);
    write_reg!(tim2, tim2, PSC, 1_000 - 1);
    write_reg!(tim2, tim2, ARR, 16_000);

    write_reg!(tim2, tim2, CCR1, 8_000);
    write_reg!(tim2, tim2, CCMR1, OC1PE: 1, OC1M: PwmMode1);
    write_reg!(tim2, tim2, CR1, ARPE: 1);
    write_reg!(tim2, tim2, CCER, CC1P: 0, CC1E: 1);

    modify_reg!(tim2, tim2, CR1, CEN: 1);

    tim2::TIM2::release(tim2);

    nvic.enable_interrupt(Interrupt::TIM2);

    loop {
        cortex_m::asm::delay(5_000_000);
    }
}

#[interrupt]
fn TIM2() {
    info!("TIM2 Event occurs");
    let tim2 = tim2::TIM2::take().unwrap();
    modify_reg!(tim2, tim2, SR, UIF: 0);
    tim2::TIM2::release(tim2);
}
