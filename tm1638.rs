#![no_std]
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_time::Timer;

pub struct TM1638 {
    clk: Output<'static>,
    dio: Output<'static>,
    stb: Output<'static>,
}

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
    G = 0b00111101,  
    H = 0b01110110,  
    I = 0b00110000,  
    J = 0b00011110,  
    K = 0b01110101,  
    L = 0b00111000,  
    M = 0b01010101,  
    N = 0b01010100,  
    O = 0b10111111,  
    P = 0b01110011,  
    Q = 0b01100111,  
    R = 0b01010000,  
    S = 0b11101101,  
    T = 0b01111000,  
    U = 0b00111110,  
    V = 0b00101110,  
    W = 0b01101010,  
    X = 0b11110110,  
    Y = 0b01101110,  
    Z = 0b11011011,
    Minus = 0b01000000,  
    Space = 0b00000000,
    Dot = 0b10000000,
}

impl Segment {
    pub fn from_char(c: char) -> Self {
        match c {
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
            'G' => Segment::G,
            'H' => Segment::H,
            'I' => Segment::I,
            'J' => Segment::J,
            'K' => Segment::K,
            'L' => Segment::L,
            'M' => Segment::M,
            'N' => Segment::N,
            'O' => Segment::O,
            'P' => Segment::P,
            'Q' => Segment::Q,
            'R' => Segment::R,
            'S' => Segment::S,
            'T' => Segment::T,
            'U' => Segment::U,
            'V' => Segment::V,
            'W' => Segment::W,
            'X' => Segment::X,
            'Y' => Segment::Y,
            'Z' => Segment::Z,
            '-' => Segment::Minus, 
            '.' => Segment::Dot,
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

impl TM1638 {
    pub fn new(mut clk: Output<'static>, mut dio: Output<'static>, mut stb: Output<'static>) -> Self {
        stb.set_high();
        clk.set_high();
        dio.set_high();
        TM1638 { clk, dio, stb }
    }
    
    pub fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            self.clk.set_low();
            Timer::after_micros(1).await;
            
            if data & 0x01 != 0 {
                self.dio.set_high();
            } else {
                self.dio.set_low();
            }
            
            Timer::after_micros(1).await;
            self.clk.set_high();
            Timer::after_micros(1).await;
            data >>= 1;
        }
        
        self.clk.set_low();
        Timer::after_micros(1).await;
        self.dio.set_high();
        Timer::after_micros(1).await;
        self.clk.set_high();
        Timer::after_micros(1).await;
        self.dio.set_low(); 
        Timer::after_micros(1).await;
    }
    
    async fn send_command(&mut self, cmd: u8) {
        self.stb.set_low();
        Timer::after_micros(1).await;
        self.write_byte(cmd).await;
        self.stb.set_high();
        Timer::after_micros(1).await;
    }
    
    async fn send_data(&mut self, addr: u8, data: u8) {
        self.stb.set_low();
        Timer::after_micros(1).await;
        self.write_byte(addr).await;
        self.write_byte(data).await;
        self.stb.set_high();
        Timer::after_micros(1).await;
    }
    
    pub async fn init(&mut self) {
        self.send_command(0x40).await; 
        self.send_command(0x8F).await; 
    }
    
    pub async fn set_brightness(&mut self, level: u8) {
        let level = if level > 7 { 7 } else { level };
        self.send_command(0x88 | level).await;
    }
    
    pub async fn clear(&mut self) {
        self.send_command(0xC0).await; 
        self.stb.set_low();
        Timer::after_micros(1).await;
        for _ in 0..16 {
            self.write_byte(0x00).await;
        }
        self.stb.set_high();
        Timer::after_micros(1).await;
    }
    
    pub async fn write_display_byte(&mut self, pos: u8, value: u8) {
        if pos > 7 { return; }
        let addr = 0xC0 | (pos << 1);
        self.send_data(addr, value).await;
    }
    
    pub async fn write_display_buffer(&mut self, buffer: &[u8; 16]) {
        self.send_command(0xC0).await;
        self.stb.set_low();
        Timer::after_micros(1).await;
        for &byte in buffer {
            self.write_byte(byte).await;
        }
        self.stb.set_high();
        Timer::after_micros(1).await;
    }
    
    // pub async fn write_string(&mut self, text: &str) {
    //     let chars: Vec<char> = text;
    //     for (i, ch) in chars.iter().enumerate() {
    //         if i < 8 {
    //             let segment = Segment::from_char(*ch);
    //             self.write_display_byte(i as u8, segment.value()).await;
    //         }
    //     }
    // }
    
    pub async fn write_number(&mut self, mut num: i32, decimal_places: usize, leading_zeros: bool) {
        let mut digits = [Segment::Space; 8];
        let is_negative = num < 0;
        if is_negative {
            num = -num;
        }
        
        let mut pos = 7;
        for i in 0..8 {
            if num == 0 && i >= decimal_places && !leading_zeros {
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
            let dot_pos = 8 - decimal_places;
            if dot_pos < 8 {
                digits[dot_pos] = digits[dot_pos].with_dot();
            }
        }
    
        if is_negative && pos > 0 {
            digits[pos - 1] = Segment::Minus;
        }
        
        for i in 0..8 {
            self.write_display_byte(i as u8, digits[i].value()).await;
        }
    }
    
    pub async fn read_buttons(&mut self) -> Result<u8, ()> {
        self.stb.set_low();
        Timer::after_micros(1).await;
        self.write_byte(0x42).await; 
        
        let mut buttons = 0u8;
        
        for i in 0..4 {
            let mut byte = 0u8;
            for bit in 0..8 {
                self.clk.set_low();
                Timer::after_micros(1).await;
                self.clk.set_high();
                Timer::after_micros(1).await;
            }
            buttons |= byte << (i * 8);
        }
        
        self.stb.set_high();
        Timer::after_micros(1).await;
        
        Ok(buttons)
    }
    
    // pub async fn test_display(&mut self) {
    //     let test_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
    //                       'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    //                       'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    //                       'U', 'V', 'W', 'X', 'Y', 'Z', '-', '.', ' '];
        
    //     for &ch in test_chars.iter() {
    //         self.clear().await;
    //         Timer::after_millis(50).await;
    //         self.write_string(&ch.to_string()).await;
    //         Timer::after_millis(200).await;
    //     }
        
    //     self.clear().await;
    // }
}