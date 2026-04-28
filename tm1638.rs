
use embedded_hal_async::spi::SpiDevice;
use stm32f4xx_hal::gpio::{Output, PushPull, OpenDrain, Pin};
use stm32f4xx_hal::delay::Delay;
use core::fmt::Write;

#[derive(Debug)]
pub enum Tm1638Error {
    WriteError,
    DataWriteFailed,
    CommunicationError(String),
}


pub trait Tm1638Operations {
    async fn send_command(&mut self, cmd: u8) -> Result<(), Tm1638Error>;
    async fn send_command_with_data(&mut self, cmd: u8, data: u8) -> Result<(), Tm1638Error>;
    async fn send_data(&mut self, data: u8) -> Result<(), Tm1638Error>;
    async fn send_data_slice(&mut self, data: &[u8]) -> Result<(), Tm1638Error>;
    async fn read_data(&mut self, bytes: usize) -> Result<Vec<u8>, Tm1638Error>;
}

pub struct TM1638<CLK, DIO, STB> {
    clk: CLK,
    dio: DIO,
    stb: STB,
    delay: Delay,
}

impl<CLK, DIO, STB> TM1638<CLK, DIO, STB>
where
    CLK: Output<Pin = Pin<Output<PushPull>>>,
    DIO: Output<Pin = Pin<Output<OpenDrain>>>,
    STB: Output<Pin = Pin<Output<PushPull>>>,
{
    pub fn new(mut clk: CLK, mut dio: DIO, mut stb: STB, delay: Delay) -> Self {
        stb.set_high();
        clk.set_high();
        dio.set_high();
        TM1638 { clk, dio, stb, delay }
    }

    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            self.clk.set_low();
            self.delay.delay_us(1);
            
            if data & 0x01 != 0 {
                self.dio.set_high();
            } else {
                self.dio.set_low();
            }
            self.delay.delay_us(1);
            
            self.clk.set_high();
            self.delay.delay_us(1);
            data >>= 1;
        }
        
        self.clk.set_low();
        self.delay.delay_us(1);
        self.dio.set_high();
        self.delay.delay_us(1);
        self.clk.set_high();
        self.delay.delay_us(1);
        self.dio.set_low();
    }

    fn send_command(&mut self, cmd: u8) {
        self.stb.set_low();
        self.delay.delay_us(1);
        self.write_byte(cmd);
        self.stb.set_high();
        self.delay.delay_us(1);
    }

    fn send_command_with_data(&mut self, cmd: u8, data: u8) {
        self.stb.set_low();
        self.delay.delay_us(1);
        self.write_byte(cmd);
        self.write_byte(data);
        self.stb.set_high();
        self.delay.delay_us(1);
    }

    pub fn init(&mut self) -> Result<(), Tm1638Error> {
        self.send_command(0x40);
        self.send_command(0x8F); 
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Tm1638Error> {
        self.send_command(0xC0);
        
        self.stb.set_low();
        self.delay.delay_us(1);
        for _ in 0..16 {
            self.write_byte(0x00);
        }
        self.stb.set_high();
        self.delay.delay_us(1);
        
        Ok(())
    }

    pub fn write_display_byte(&mut self, pos: u8, value: u8) -> Result<(), Tm1638Error> {
        let addr = 0xC0 | (pos << 1);
        self.send_command_with_data(addr, value);
        Ok(())
    }

    pub fn write_string(&mut self, text: &str) -> Result<(), Tm1638Error> {
        for (i, ch) in text.chars().enumerate() {
            if i < 8 {
                let segment = Segment::from_char(ch);
                self.write_display_byte(i as u8, segment.value())?;
            }
        }
        Ok(())
    }

    pub fn read_buttons(&mut self) -> Result<u8, Tm1638Error> {
        self.stb.set_low();
        self.delay.delay_us(1);
        self.write_byte(0x42); 
        let mut buttons = 0u8;
        
        for i in 0..4 {
            for bit in 0..8 {
                self.clk.set_low();
                self.delay.delay_us(1);
                self.clk.set_high();
                self.delay.delay_us(1);
            }
        }
        
        self.stb.set_high();
        self.delay.delay_us(1);
        
        Ok(buttons)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Segment {
    Zero = 0b00111111,
    One = 0b00000110,
    Two = 0b01011011,
    Three = 0b01001111,
    Four = 0b01100110,
    Five = 0b01101101,
    Six = 0b01111101,
    Seven = 0b00000111,
    Eight = 0b01111111,
    Nine = 0b01101111,
    A = 0b01110111,
    B = 0b01111100,
    C = 0b00111001,
    D = 0b01011110,
    E = 0b01111001,
    F = 0b01110001,
    G = 0b00111101,
    H = 0b01110110,
    I = 0b00110000,
    J = 0b00011110,
    L = 0b00111000,
    O = 0b00111111,
    P = 0b01110011,
    U = 0b00111110,
    S = 0b00110110,
    Minus = 0b01000000,
    Dot = 0b10000000,
    Space = 0b00000000,
}

impl Segment {
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => Segment::Zero,
            '1' => Segment::One,
            '2' => Segment::Two,
            '3' => Segment::Three,
            '4' => Segment::Four,
            '5' => Segment::Five,
            '6' => Segment::Six,
            '7' => Segment::Seven,
            '8' => Segment::Eight,
            '9' => Segment::Nine,
            'A' | 'a' => Segment::A,
            'B' | 'b' => Segment::B,
            'C' | 'c' => Segment::C,
            'D' | 'd' => Segment::D,
            'E' | 'e' => Segment::E,
            'F' | 'f' => Segment::F,
            'G' | 'g' => Segment::G,
            'H' | 'h' => Segment::H,
            'I' | 'i' => Segment::I,
            'J' | 'j' => Segment::J,
            'L' | 'l' => Segment::L,
            'O' | 'o' => Segment::O,
            'P' | 'p' => Segment::P,
            'U' | 'u' => Segment::U,
            'S' | 's' => Segment::S,
            '-' => Segment::Minus,
            '.' => Segment::Dot,
            ' ' => Segment::Space,
            _ => Segment::Space,
        }
    }
    
    pub fn value(&self) -> u8 {
        *self as u8
    }
}