mod error;
use std::fmt::Display;

pub use error::{SDRError, SDRResult};

pub trait SDRDriver: Send {
   type Item : SDRDevice;
   fn list(&self)->SDRResult<Vec<Self::Item>>; 
}



pub trait SDRDevice: Send + Display {
   fn open(&mut self)->SDRResult<()>;
}