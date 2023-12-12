#![no_main]
#![no_std]

use cortex_m_rt::entry;

use panic_halt as _;

use defmt::info;

use defmt_rtt as _;

use stm32ral as ral;

use ral::{adc, adc12_common, gpio, interrupt, nvic, rcc, tim2};
use ral::{modify_reg, read_reg, write_reg};

use g474re_nucleo_ral::gpio::{Gpio, PinAltFunc, PinIndex, PinMode, PinSpeed};
use g474re_nucleo_ral::nvic::{Interrupt, Nvic};

#[entry]
fn main() -> ! {
    let rcc = rcc::RCC::take().unwrap();
    let adc1 = adc::ADC1::take().unwrap();
    let adc_common = adc12_common::ADC12_Common::take().unwrap();
    let gpio_a = Gpio::new(gpio::GPIOA::take().unwrap());
    let tim2 = tim2::TIM2::take().unwrap();
    let nvic = Nvic::new(nvic::NVIC::take().unwrap());

    // Setup adc
    modify_reg!(rcc, rcc, AHB2ENR, ADC12EN: 1);
    modify_reg!(rcc, rcc, CCIPR, ADC12SEL: System);

    // Disable deep power save
    modify_reg!(adc, adc1, CR, DEEPPWD: Disabled);

    // Enable voltage regulator end wait to startup it
    modify_reg!(adc, adc1, CR, ADVREGEN: Enabled);
    while read_reg!(adc, adc1, CR, ADVREGEN == Disabled) {}

    // Enable temperature sensor, vref and set prescaler to divide by 4
    write_reg!(adc12_common, adc_common, CCR, CKMODE: SyncDiv4,VSENSESEL: Enabled, VREFEN: Enabled, PRESC: 0b0);

    // Enable discontinious mode with 3 channels
    // Enable external trigger by rising and event tim2_trgo
    write_reg!(adc, adc1, CFGR, JQDIS: Enabled, DISCEN: Enabled, DISCNUM: 3 - 1, EXTEN: RisingEdge, EXTSEL: TIM2_TRGO);

    // Enable 3 items in sequence vref (18), temperature sensor (16), and channel 1(1)
    // Set sample time for these channels as 6.5 ADC clock cycles (1)
    write_reg!(adc, adc1, SQR1, L: 3 - 1, SQ1: 18, SQ2: 16, SQ3: 1);
    write_reg!(adc, adc1, SMPR1, SMP0: 1);
    write_reg!(adc, adc1, SMPR2, SMP16: 1, SMP18: 1);

    // Enable end of conversion interrupt
    write_reg!(adc, adc1, IER, EOSMPIE: Enabled);

    // Enable and start adc
    modify_reg!(adc, adc1, CR, ADEN: Enabled);
    modify_reg!(adc, adc1, CR, ADSTART: 1);

    adc::ADC1::release(adc1);
    adc12_common::ADC12_Common::release(adc_common);

    modify_reg!(rcc, rcc, AHB2ENR, GPIOAEN: Enabled);

    gpio_a
        .set_mode_alternate(PinIndex::Pin5)
        .set_speed(PinIndex::Pin5, PinSpeed::HighSpeed)
        .set_alt_func(PinIndex::Pin5, PinAltFunc::Af1)
        .set_mode(PinIndex::Pin0, PinMode::Analog);

    modify_reg!(rcc, rcc, APB1ENR1, TIM2EN: Enabled);

    // Setup autoreload
    write_reg!(tim2, tim2, DIER, UIE: 1);
    write_reg!(tim2, tim2, PSC, 1_000 - 1);
    write_reg!(tim2, tim2, ARR, 16_000);
    // Setup pwm output on channel 1 with pwm mode 1
    write_reg!(tim2, tim2, CCR1, 8_000);
    write_reg!(tim2, tim2, CCMR1, OC1PE: 1, OC1M: PwmMode1);
    write_reg!(tim2, tim2, CR1, ARPE: 1);
    write_reg!(tim2, tim2, CCER, CC1P: 0, CC1E: 1);
    // Setup output trigger as update
    write_reg!(tim2, tim2, CR2, MMS: 0b010);

    modify_reg!(tim2, tim2, CR1, CEN: 1);

    tim2::TIM2::release(tim2);

    nvic.enable_interrupt(Interrupt::ADC1_2);
    nvic.enable_interrupt(Interrupt::TIM2);

    loop {
        cortex_m::asm::delay(5_000_000);
    }
}

#[interrupt]
fn TIM2() {
    let tim2 = tim2::TIM2::take().unwrap();
    info!("TIM2 Event occurs");
    modify_reg!(tim2, tim2, SR, UIF: 0);
    tim2::TIM2::release(tim2);
}

#[interrupt]
fn ADC1_2() {
    let adc1 = adc::ADC1::take().unwrap();
    let vref = read_reg!(adc, adc1, DR);
    let temp = read_reg!(adc, adc1, DR);
    let channel1 = read_reg!(adc, adc1, DR);
    let sr = read_reg!(adc, adc1, ISR);
    info!("ADC1_2 Event occurs by sr: {:b}", sr);
    info!(
        "Readed: Vref: {}, Temperature: {}, Channel 1: {}",
        vref, temp, channel1
    );
    modify_reg!(adc, adc1, ISR, EOSMP: Clear);
    adc::ADC1::release(adc1);
}
