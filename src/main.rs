#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use {defmt_rtt as _, panic_probe as _};

use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;

mod tm1638;
use tm1638::{TM1638SPI, AsyncCom, Segment};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    
    let mut spi_config = SpiConfig::default();
    spi_config.frequency = Hertz(1_000_000);
    let spi = Spi::new(
        p.SPI1,
        p.PA5,
        p.PA7,
        p.PA6,
        p.DMA2_CH3,
        p.DMA2_CH2,
        spi_config,
    );
    
    let stb = Output::new(p.PB0, Level::High, Speed::Low);
    
    let bus: Mutex<
        CriticalSectionRawMutex,
        Spi<'_, Async, Master>
    > = Mutex::new(spi);

    let dev = SpiDevice::new(&bus, stb);
    let mut display = TM1638SPI::new(dev);

    loop {
            display.write_string("HELLO   ").await.unwrap();
            Timer::after_secs(2).await;
            
            display.write_number(12345, 2).await.unwrap();
            Timer::after_secs(2).await;
            
            display.write_string("    GOOD").await.unwrap();
            Timer::after_secs(2).await;
            
            display.write_number(25, 0).await.unwrap();
            Timer::after_secs(2).await;
            
            display.write_number(-52, 0).await.unwrap();
            Timer::after_secs(2).await;
            
            display.clear().await.unwrap();
            Timer::after_secs(1).await;
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    loop {}
}