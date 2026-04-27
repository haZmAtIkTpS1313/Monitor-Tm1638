use core::ptr::Pointee;

use embedded_hal_async::spi::{SpiDevice, Operation};

use crate::tm1638;


#[derive(thiserror::Error)]

pub enum Tm1638Error {
    #[error("Error in writing commands")]
    WriteError,
    #[error ("Error entering data")]
    DataWriteFailed,
    #[error("Communication error: {0}")]
    CommunicationError(String),

}

trait Tm1638Operations {
    async fn send_command(&mut self, cmd: u8) -> Result<(), Tm1638Error>;
    async fn send_command_with_data(&mut self, cmd: u8, data: u8) -> Result<(), Tm1638Error>;
    async fn send_data(&mut self, data: u8) -> Result<(), Tm1638Error>;
    async fn send_data_slice(&mut self, data: &[u8]) -> Result<(), Tm1638Error>;
    async fn read_data(&mut self, bytes: usize) -> Result<Vec<u8>, Tm1638Error>;
}

impl<T: SpiDevice> Tm1638device for T {
    async fn send_command_raw(&mut self, cmd: u8, data:u8) -> Result<(), Tm1638Error> {
        trace!("Write byte {} to add {} ", data, cmd); 
        match self.write(&[cmd, data]).await {
            Ok(_) => Ok(()),
            Err(_) =>Err(Tm1638Error::WriteError)?,
        }
    }

    async fn send_command_with_data(&mut self, cmd: u8, data: u8) ->Result<(), Tm1638Error> {
        trace!("{} command and {} datta recording" cmd, data);
        match self.write(cmd, data).await {
            Ok(_) => Ok(()),
            Err(_)=> Err(Tm1638Error::WriteError)?,
        } 
    }
    async fn send_data(&mut self, data: u8) -> Result<(), Tm1638Error> {
        trace!("Send data in {0x02}": data);
        match self.write(&[data]).await {
            Ok(_) => Ok(()),
            Err(_) =>Err(Tm1638Error::CommunictionError)?,
        }
    }
    async fn send_data_slice(&mut self, data: &[u8]) -> Result<(), Tm1638Error> {
        trace!("Send data slice: " [data]);
        match self.write(&[data]).await {
            Ok(_) => Ok(()),
            Err(_) =>Err(Tm1638Error::CommunictionError)?,
        }

    }
    async fn read_data(&mut self, bytes: usize) -> Result<Vec<u8>, Tm1638Error> {
        trace!("Read {} bytes!", bytes);
        let mut buffer = [0x00, bytes];
        match seld.read(&[buffer]).await {
            Ok(_) => Ok(buffer),
            Err(_) =>Err(Tm1638Error::CommunictionError)?,
        }
    }
}

pub struct TM1638<'a, D: SpiDevice> {
    device: D,
}

impl<D:SpiDevice> TM1638<D> {
    pub fn new(device: D) -> Self {
        Self {
            device
        }
    }

    pub fn init(self, defice: D) {
        trace!("init: command executon reset!");
        let data = [0x40, 16];
        self.write_raw(&data, 0x00);

        match self.read() {
            Ok(keys) => {
                trace!("The data has been recorded");
            }
            Err(e) => {
                Tm1638Error::DataWriteFailed;
            }
        }
    }
    pub fn command_raw(&mut self, cmd: u8) {
        self.send_command_raw(cmd);
    }
    pub fn command_with_data(&mut self, cmd: u8, data: u8) {
        self.send_command;
        nd_with_data(cmd, data);
    }
    pub fn write_data(&mut self, data: u8) {
       self.send_data(data);
    }
    pub fn write_data_silce(&mut self, data: &[u8]) {
       self.send_data_silce(&[data]);
    }
    pub fn reads(&mut self, bytes: usize) {
       self.read_data(bytes);
    } 
}
pub trait IntoSegments {

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
    Space = 0b00000000
}

impl Segment {
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => Segment::Zero,
            '1' => Segment::One,
            '2' => Swgment::Two,
            '3' => Segment::Three,
            '4' => Segment::Four,
            '5' => Segment::Five,
            '6' => Segment::Six,
            '7' => Segment::Seven,
            '8' => Segment::Eigth,
            '9' => Segment::Nine,
            'A'| 'a' => Segment::A,
            'B' | 'b' => Segment::B,
            'C'| 'c' => Segment::C,
            'D' | 'd' => Segment::D,
            'E'| 'e' => Segment::E,
            'F' | 'f' => Segment::F,
            'G'| 'g' => Segment::G,
            'H' | 'h' => Segment::H, 
            'I'| 'i' => Segment::I,
            'J' | 'j' => Segment::J,
            'L'| 'l' => Segment::L,
            'O' | 'o' => Segment::O,  
            'P'| 'p' => Segment::P,
            'U' | 'u' => Segment::U,
            'S' | 's' => Segment::S,
            '-' => Segment::Minus,
            '.' => Segment::Dot,
            ' ' => Segment::Space
          }
    }
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
