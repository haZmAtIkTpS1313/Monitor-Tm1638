use core::ptr::Pointee;

use embedded_hal_async::spi::{SpiDevice, Operation};


#[derive(thiserror::Error)]

pub enum Tm1638Error {
    #[error("Error in writing commands")]
    WriteError,
    #[error ("Error entering data")]
    DataWriteFailed
}


trait Tm1638device {
    async fn send_command_raw(&mut self, cmd: u8, data: u8) -> Result<(), Tm1638Error>;
    async fn send_data_raw(&mut self, data: u8) -> Result<(), Tm1638Error>;
}

impl<T: SpiDevice> Tm1638device for T {
    async fn send_command_raw(&mut self, cmd: u8, data:u8) -> Result<(), Tm1638Error> {
        trace!("Write byte {} to add {} ", data, cmd); 
        self.write(&[cmd, data]);
        Tm1638Error::WriteErrorp;
    }
    async fn send_data_raw(&mut self, data: u8) -> Result<(), Tm1638Error> {
        trace !("Entering data {} ", data);
        self.write(&[data]);
        Tm1638Error::DataWriteFailed;
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
                trace!("Th data has been recorded");
            }
            Err(e) => {
                Tm1638Error::DataWriteFailed;
            }
        }
    }
}
