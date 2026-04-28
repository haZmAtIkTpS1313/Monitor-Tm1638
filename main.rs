#![no_std]
#![no_main]

mod tm1638;

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    gpio::{GpioExt, Output, PushPull, OpenDrain},
    delay::Delay,
};

use tm1638::TM1638;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let delay = Delay::new(cp.SYST, clocks);
    
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    
    let mut clk = gpioa.pA5.into_push_pull_output();
    let mut dio = gpioa.pA7.into_open_drain_output();
    let mut stb = gpiob.pB0.into_push_pull_output();
    
    let mut tm1638 = TM1638::new(clk, dio, stb, delay);
    
    tm1638.init().unwrap();
    tm1638.clear().unwrap();
    tm1638.write_string("HELLO").unwrap();
    
    loop {
        match tm1638.read_buttons() {
            Ok(buttons) => {
                if buttons != 0 {
                    for i in 0..8 {
                        if buttons & (1 << i) != 0 {
                            let digit_char = char::from_digit(i as u32, 10).unwrap();
                            let segment = tm1638::Segment::from_char(digit_char);
                            tm1638.write_display_byte(0, segment.value()).unwrap();
                        }
                    }
                }
            }
            Err(_e) => continue,
        }
    }
}