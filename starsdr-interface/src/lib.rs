mod error;
pub use error::{SDRError, SDRResult};
use std::{fmt::Display};
pub use num::{Complex};



pub trait SDRDriver: Send {
    type Item: SDRDevice;
    fn list(&self) -> SDRResult<Vec<Self::Item>>;
}

pub trait SDRDevice: Send + Display {
    fn open(&mut self) -> SDRResult<()>;
    fn tx_channel_count(&self) -> SDRResult<usize>;
    fn rx_channel_count(&self) -> SDRResult<usize>;
    fn set_tx_rate(&self, rate: f64, channel: usize)-> SDRResult<()>; 
    fn get_tx_rate(&self, channel: usize)->SDRResult<f64>;
    fn set_tx_freq(&self, freq: f64, channel: usize)-> SDRResult<()>; 
    fn get_tx_freq(&self, channel: usize)->SDRResult<f64>;
    fn set_tx_gain(&self, gain: f64, channel: usize)->SDRResult<()>;
    fn get_tx_gain(&self, channel: usize)->SDRResult<f64>;
    fn set_tx_bandwidth(&self, bw: f64, channel: usize)->SDRResult<()>;
    fn get_tx_bandwidth(&self, channel: usize)->SDRResult<f64>;
    fn set_rx_rate(&self, rate: f64, channel: usize)-> SDRResult<()>; 
    fn get_rx_rate(&self, channel: usize)->SDRResult<f64>;
    fn set_rx_freq(&self, freq: f64, channel: usize)-> SDRResult<()>; 
    fn get_rx_freq(&self, channel: usize)->SDRResult<f64>;
    fn set_rx_gain(&self, gain: f64, channel: usize)->SDRResult<()>;
    fn get_rx_gain(&self, channel: usize)->SDRResult<f64>;
    fn set_rx_bandwidth(&self, bw: f64, channel: usize)->SDRResult<()>;
    fn get_rx_bandwidth(&self, channel: usize)->SDRResult<f64>;
    
}

pub trait CreateTx<I: Send, T: Tx<I>> {
   fn tx_stream(&self, channels: &[usize]) -> SDRResult<T> ;
}

pub trait Tx<Item: Send>: Send{
   fn send(&self, v: &[Complex<Item>])->SDRResult<usize>;
}

pub trait CreateRx<I: Send, T: Rx<I>> {
    fn rx_stream(&self, channels: &[usize]) -> SDRResult<T> ;
}

pub trait Rx<Item: Send>: Send{
    fn recv(&mut self)->SDRResult<Vec<Complex<Item>>>;
}

