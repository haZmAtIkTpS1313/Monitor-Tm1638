#![no_std]
#![no_main]

mod tm1638; 

use panic_halt as _; // 

use cortex_m_rt::entry;
use embedded_hal_async::spi::SpiDevice;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    spi::{Spi, Mode, Phase, Polarity},
    gpio::{GpioExt, Output, PushPull},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();
    
    let spi_mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnFirstTransition,
    };
    
    let mut spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        1.mhz(),
        clocks,
    );
    
    let mut tm1638 = TM1638::new(spi);
    
    tm1638.init().unwrap();
    tm1638.clear().unwrap();
    
    let text = "HELLO";
    for (i, ch) in text.chars().enumerate() {
        if i < 8 {
            let segment = Segment::from_char(ch);
            tm1638.write_display_byte(i as u8, segment.value()).unwrap();
        }
    }
    
    loop {
        match tm1638.read_buttons() {
            Ok(buttons) => {
                if buttons != 0 {
                    for i in 0..8 {
                        if buttons & (1 << i) != 0 {
                            let segment = Segment::from_char(char::from_digit(i as u32, 10).unwrap());
                            tm1638.write_display_byte(0, segment.value()).unwrap();
                        }
                    }
                }
            }
            Err(e) => {
            }
        }
    }
}