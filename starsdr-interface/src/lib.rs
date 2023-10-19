mod error;
pub use error::{SDRError, SDRResult};
use std::{fmt::Display, sync::mpsc::{Sender, SendError}};



pub trait SDRDriver: Send {
    type Item: SDRDevice;
    fn list(&self) -> SDRResult<Vec<Self::Item>>;
}

pub trait SDRDevice: Send + Display {
    fn open(&mut self) -> SDRResult<()>;
    fn tx_channel_count(&self) -> SDRResult<usize>;
    fn rx_channel_count(&self) -> SDRResult<usize>;
    
}

pub trait CreateTx<I, T: Tx<I>> {
   fn tx_stream(&self, channels: &[usize]) -> SDRResult<T>;
}

pub trait Tx<Item>: Send{
   fn send(v: &[Item])->Result<(), SendError<&[Item]>>;
}