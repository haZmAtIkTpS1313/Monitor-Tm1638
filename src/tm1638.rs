//use embassy_stm32::PeripheralType;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::spi::{Spi, Config as SpiConfig};
use embassy_time::Timer;
use embedded_hal_async::spi::SpiDevice;
//use embedded_hal_async::spi::SpiDevice;
//use embedded_hal::digital::OutputPin;

pub trait AsyncCom {
    type Error;
    async fn send_command(&mut self, command: u8) -> Result<(), Self::Error>;
    async fn send_data(&mut self, addr: u8, data: u8) -> Result<(), Self::Error>;
    async fn read_buttons(&mut self) -> Result<u8, Self::Error>;
}

impl<T: SpiDevice> AsyncCom for T {
    type Error = T::Error;
    async fn send_command(&mut self, command: u8) -> Result<(), Self::Error> {
        self.write(&[command]).await
    }
    async fn send_data(&mut self, addr: u8, data: u8) -> Result<(), Self::Error> {
        self.write(&[addr,data]).await
    }
    async fn read_buttons(&mut self) -> Result<u8, Self::Error> {
        self.write(&[0x42, 0x00]).await;
        // self.write(&[]).await;
        let mut buffer = [0u8];
        self.read(&mut buffer).await;
        Ok(buffer[0])
    }
}

pub struct TM1638SPI<Asyn: AsyncCom> {
    asyn: Asyn,
}

impl<Asyn: AsyncCom> TM1638SPI<Asyn> {
    pub fn new(spi: Asyn) -> Self {
        Self { 
            asyn: spi, 
        }
    }
    
    // async fn spi_write(&mut self, data: &[u8]) -> Result<(), SPI::Error> {
    //     self.stb.set_low();  
    //     Timer::after_nanos(100).await;
    //     let result = self.spi.write(data).await;
    //     Timer::after_nanos(100).await;
    //     self.stb.set_high();  
    //     Timer::after_micros(1).await;
    //     result
    // }

    
    // async fn read_buttons_spi(&mut self) -> Result<u8, SPI::Error> {
    //     let mut read_buffer = [0u8; 4];
        
    //     self.stb.set_low();
    //     Timer::after_nanos(100).await;
        
    //     self.spi.write(&[0x42]).await?;
    //     Timer::after_nanos(100).await;
        
    //     for i in 0..4 {
    //         let mut rx_byte = [0u8; 1];
    //         self.spi.transfer(&mut rx_byte, &[0x00]).await?;
    //         read_buffer[i] = rx_byte[0];
    //     }
        
    //     self.stb.set_high();
    //     Timer::after_micros(1).await;
        
    //     Ok(read_buffer[0])

    // }
    
    pub async fn init(&mut self) -> Result<(), Asyn::Error> {
        self.asyn.send_command(0x40).await?;
        Timer::after_micros(100).await;
        self.asyn.send_command(0x8F).await?;
        Timer::after_micros(100).await;
        self.clear().await?;
        Ok(())
    }
    
    // pub async fn set_brightness(&mut self, level: u8) -> Result<(), Asyn::Error> {
    //     let level = if level > 7 { 7 } else { level };
    //     self.asyn.send_command(0x88 | level).await
    // }
    
    pub async fn clear(&mut self) -> Result<(), Asyn::Error> {
        let clear_data = [0x00; 16];
        self.write_display_buffer(&clear_data).await
    }
    
    pub async fn write_display_byte(&mut self, pos: u8, value: u8) -> Result<(), Asyn::Error> {
        if pos > 7 { return Ok(()); }
        let addr = 0xC0 + (pos * 2);
        self.asyn.send_data(addr, value).await
    }
    
    pub async fn write_display_buffer(&mut self, buffer: &[u8; 16]) -> Result<(), Asyn::Error> {
        // self.asyn.send_command(0xC0).await?;
        Timer::after_nanos(100).await;
        for i in 0..16 {
            self.asyn.send_data(0xC0, buffer[i]);
        }
        Ok(())
    }
    
    pub async fn write_string(&mut self, text: &str) -> Result<(), Asyn::Error> {
        let chars = text.as_bytes();
        let mut display_buffer = [0u8; 16];
        
        for i in 0..8 {
            let segment = if i < chars.len() {
                Segment::from_char(chars[i] as char)
            } else {
                Segment::Space
            };
            display_buffer[i * 2] = segment.value();
            display_buffer[i * 2 + 1] = 0x00;
        }
        
        self.write_display_buffer(&display_buffer).await
    }
    
    pub async fn write_number(&mut self, mut num: i32, decimal_places: usize) -> Result<(), Asyn::Error> {
        let mut display_buffer = [0u8; 16];
        let mut digits = [Segment::Space; 8];
        let is_negative = num < 0;
        
        if is_negative {
            num = -num;
        }
        
        let mut pos = 7;
        for _ in 0..8 {
            if num == 0 && (7 - pos) >= decimal_places {
                break;
            }
            let digit = (num % 10) as u8;
            digits[pos] = match digit {
                0 => Segment::Digit0,
                1 => Segment::Digit1,
                2 => Segment::Digit2,
                3 => Segment::Digit3,
                4 => Segment::Digit4,
                5 => Segment::Digit5,
                6 => Segment::Digit6,
                7 => Segment::Digit7,
                8 => Segment::Digit8,
                9 => Segment::Digit9,
                _ => Segment::Space,
            };
            num /= 10;
            if pos > 0 { pos -= 1; }
        }
        
        if decimal_places > 0 && decimal_places <= 8 {
            let dot_pos = 7 - decimal_places;
            if dot_pos <= 7 {
                digits[dot_pos] = digits[dot_pos].with_dot();
            }
        }
    
        if is_negative {
            digits[0] = Segment::Minus;
        }
        
        for i in 0..8 {
            display_buffer[i * 2] = digits[i].value();
            display_buffer[i * 2 + 1] = 0x00;
        }
        
        self.write_display_buffer(&display_buffer).await
    }
    
    // pub async fn read_buttons(&mut self) -> Result<u8, Asyn::Error> {
    //     self.asyn.read_buttons_spi().await
    // }
}

// impl<'a, Asyn: AsyncCom> AsyncCom for TM1638SPI<'a, Asyn> {
//     type Error = Asyn::Error;
    
//     async fn send_command(&mut self, command: u8) -> Result<(), Self::Error> {
//         self.asyn.send_command(command).await
//     }
    
//     async fn send_data(&mut self, addr: u8, data: u8) -> Result<(), Self::Error> {
//         self.asyn.send_data(addr, data).await
//     }
    
//     async fn read_buttons(&mut self) -> Result<u8, Self::Error> {
//         self.asyn.read_buttons().await
//     }
// }

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Segment {
    Digit0 = 0b00111111,
    Digit1 = 0b00000110,
    Digit2 = 0b01011011,
    Digit3 = 0b01001111,
    Digit4 = 0b01100110,
    Digit5 = 0b01101101,
    Digit6 = 0b01111101,
    Digit7 = 0b00000111,
    Digit8 = 0b01111111,
    Digit9 = 0b01101111,
    A = 0b01110111,  
    B = 0b01111100,  
    C = 0b00111001,  
    D = 0b01011110,  
    E = 0b01111001,  
    F = 0b01110001,  
    Minus = 0b01000000,  
    Space = 0b00000000,
    Dot = 0b10000000,
}

impl Segment {
    pub fn from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            '0' => Segment::Digit0,
            '1' => Segment::Digit1,
            '2' => Segment::Digit2,
            '3' => Segment::Digit3,
            '4' => Segment::Digit4,
            '5' => Segment::Digit5,
            '6' => Segment::Digit6,
            '7' => Segment::Digit7,
            '8' => Segment::Digit8,
            '9' => Segment::Digit9,
            'A' => Segment::A,
            'B' => Segment::B,
            'C' => Segment::C,
            'D' => Segment::D,
            'E' => Segment::E,
            'F' => Segment::F,
            '-' => Segment::Minus,
            _ => Segment::Space,
        }
    }
    
    pub fn with_dot(self) -> Self {
        let val = self as u8;
        Segment::from_u8(val | 0b10000000)
    }
    
    fn from_u8(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
    }
    
    pub fn value(&self) -> u8 {
        *self as u8
    }
}