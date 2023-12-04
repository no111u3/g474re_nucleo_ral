#![no_main]
#![no_std]

use cortex_m_rt::entry;

use panic_halt as _;

use defmt::info;

use defmt_rtt as _;

use stm32ral as ral;

use ral::{gpio, interrupt, nvic, rcc, tim2};
use ral::{modify_reg, write_reg};

#[entry]
fn main() -> ! {
    let rcc = rcc::RCC::take().unwrap();
    let gpio_a = gpio::GPIOA::take().unwrap();
    let tim2 = tim2::TIM2::take().unwrap();
    let nvic = nvic::NVIC::take().unwrap();

    modify_reg!(rcc, rcc, AHB2ENR, GPIOAEN: Enabled);

    modify_reg!(gpio, gpio_a, MODER, MODER5: Alternate);
    modify_reg!(gpio, gpio_a, OSPEEDR, OSPEEDR5: HighSpeed);
    modify_reg!(gpio, gpio_a, AFRL, AFRL5: AF1);

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

    write_reg!(nvic, nvic, ISER0, 1 << (interrupt::TIM2 as u8));

    nvic::NVIC::release(nvic);

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
