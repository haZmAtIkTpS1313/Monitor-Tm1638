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
        let data = [0x00, 16];
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
        self.send_comma
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
#[derive(Debug)]
pub enum OutputDisplayNumbers {
    Zero = 0b00111111,
    One = 0b00000110,
    Two =  0b01011011,
    Three = 0b01001111,
    Four = 0b01100110,
    Five = 0b01101101,
    Six = 0b01111101,
    Seven = 0b00000111,
    Eithg = 0b01111111,
    Nine = 0b01101111,
}
#[repr(u8)]
#[derive(Debug)]
pub enum OutputDisplayLetters {
    PubA = 0b11110111,
    PubB = 0b11111100,
    PubC = 0b10111001,
    PubD = 0b11011110,
    PubE = 0b11111001,
    PubF = 0b11110001,
    PubG = 0b11000000,
    DecPoint = 0b10000000,
}

#[repr(u8)]
#[derive(Debug)]
pub enum CharPoint {
    CharA = 0x77,
    CharC = 0x39,
    CharE = 0x79,
    CharF = 0x71,
    CharG = 0x3D,
    CharH = 0x76,
    CharI = 0x30,
    CharJ = 0x1E,
    CharL = 0x38,
    CharO = 0x3F,
    CharP = 0x73,
}
