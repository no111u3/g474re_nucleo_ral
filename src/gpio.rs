use stm32ral::gpio;
use stm32ral::modify_reg;

pub struct Gpio {
    p: gpio::Instance,
}

impl<'a> Gpio {
    pub fn new(p: gpio::Instance) -> Self {
        Self { p }
    }

    #[inline]
    pub fn set_mode(&'a self, n: PinIndex, mode: PinMode) -> &Self {
        let offset = (n as u8) * 2;
        let mask = 0b11 << offset;
        let val = ((mode as u32) << offset) & mask;

        modify_reg!(gpio, self.p, MODER, |r| (r & !mask) | val);

        self
    }

    #[inline]
    pub fn set_mode_alternate(&'a self, n: PinIndex) -> &Self {
        self.set_mode(n, PinMode::Alternate)
    }

    #[inline]
    pub fn set_speed(&'a self, n: PinIndex, mode: PinSpeed) -> &Self {
        let offset = (n as u8) * 2;
        let mask = 0b11 << offset;
        let val = ((mode as u32) << offset) & mask;

        modify_reg!(gpio, self.p, OSPEEDR, |r| (r & !mask) | val);

        self
    }

    #[inline]
    pub fn set_alt_func(&'a self, n: PinIndex, func: PinAltFunc) -> &Self {
        let n = n as u8;
        if n < 8 {
            let offset = n * 4;
            let mask = 0b1111 << offset;
            let val = ((func as u32) << offset) & mask;
            modify_reg!(gpio, self.p, AFRL, |r| (r & !mask) | val);
        } else {
            let offset = (n - 8) * 4;
            let mask = 0b1111 << offset;
            let val = ((func as u32) << offset) & mask;
            modify_reg!(gpio, self.p, AFRH, |r| (r & !mask) | val);
        }

        self
    }
}

pub enum PinIndex {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
    Pin6 = 6,
    Pin7 = 7,
    Pin8 = 8,
    Pin9 = 9,
    Pin10 = 10,
    Pin11 = 11,
    Pin12 = 12,
    Pin13 = 13,
    Pin14 = 14,
    Pin15 = 15,
}

pub enum PinMode {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10,
    Analog = 0b11,
}

pub enum PinSpeed {
    LowSpeed = 0b00,
    MediumSpeed = 0b01,
    HighSpeed = 0b10,
    VeryHighSpeed = 0b11,
}

pub enum PinAltFunc {
    /// I2C4/SYS_AF
    Af0 = 0b0000,
    /// LPTIM1/TIM2/5/15/16/17
    Af1 = 0b0001,
    /// I2C1/3/TIM1/2/3/4/5/8/20/15/COMP1
    Af2 = 0b0010,
    /// QUADSPI1/I2C3/4/SAI1/USB/HRTIM1/TIM8/20/15/COMP3
    Af3 = 0b0011,
    /// I2C1/2/3/4/TIM1/8/16/17
    Af4 = 0b0100,
    /// QUADSPI1/SPI1/2/3/4/I2S2/3/I2C4/UART4/5/TIM8/Infrared
    Af5 = 0b0101,
    /// QUADSPI1/SPI2/3/I2S2/3/TIM1/5/8/20/Infrared
    Af6 = 0b0110,
    /// USART1/2/3/FDCAN/COMP7/5/6
    Af7 = 0b0111,
    /// I2C3/4/UART4/5/LPUUART1/COMP1/2/7/4/5/6/3
    Af8 = 0b1000,
    /// FDCAN/TIM1/8/15/FDCAN1/2
    Af9 = 0b1001,
    /// QADSPI1/TIM2/3/4/8/17
    Af10 = 0b1010,
    /// LPTIM1/TIM1/8/FDCAN1/3
    Af11 = 0b1011,
    /// FMC/LPUART1/SAI1/HRTIM1/TIM1
    Af12 = 0b1100,
    /// SAI1/HRTIM1/OPAMP2
    Af13 = 0b1101,
    /// UART4/5/SAI1/TIM2/15/UCPD1
    Af14 = 0b1110,
    /// Event
    Af15 = 0b1111,
}
