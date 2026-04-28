#![no_std]
#![no_main]

mod tm1638;

use embassy_executor::Spawner;
use embassy_stm32::{Config, peripherals};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_halt as _;
use tm1638::TM1638;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.sysclk = Some(84.mhz());
    let p = embassy_stm32::init(config);
    static mut CLK: Option<Output<'static>> = None;
    static mut DIO: Option<Output<'static>> = None;
    static mut STB: Option<Output<'static>> = None;
    
    let clk = Output::new(p.PA5, Level::High, Speed::Low);
    let dio = Output::new(p.PA7, Level::High, Speed::Low);
    let stb = Output::new(p.PB0, Level::High, Speed::Low);
    
    let mut tm1638 = TM1638::new(clk, dio, stb);
    
    tm1638.init().await;
    tm1638.clear().await;
    
    loop {
        tm1638.write_byte("H").await;
        Timer::after_millis(1500).await;
        
        tm1638.write_byte("W").await;
        Timer::after_millis(1500).await;
        
        tm1638.write_byte("1").await;
        Timer::after_millis(1500).await;
    
        tm1638.clear().await;
        for num in -99..=99 {
            tm1638.write_number(num, 0, false).await;
            Timer::after_millis(50).await;
        }
        
        // tm1638.test_display().await;
    }
}