use std::marker::PhantomData;
use starsdr_interface::*;
use crate::structs::{TxMetadataHandle, TxStreamerHandle};
use std::ffi::c_void;
use uhd_sys::uhd_tx_streamer_send;
use crate::errors::handle_uhd_err;

pub struct TxUHD<T: Send> {
    pub(crate) streamer: TxStreamerHandle,
    pub sample_num_max: usize,
    pub(crate) _t: PhantomData<T>,
}

impl<T: Send> Tx<T> for TxUHD<T> {
    fn send(&self, v: &[Complex<T>]) -> SDRResult<usize> {
        let data_len = v.len();
        if data_len > self.sample_num_max {
            return Err(SDRError::Param {
                key: "v".into(),
                value: format!("len()={}", data_len),
                msg: format!("> max: {}", self.sample_num_max),
            });
        }
        let mut items_sent = 0;
        let mut md = TxMetadataHandle::new()?;
        unsafe {
            let buf = v.as_ptr() as *const c_void;
            let buf = &*buf;

            handle_uhd_err(uhd_tx_streamer_send(
                self.streamer.0, &mut (buf as *const c_void),
                v.len(), &mut md.0, 0.1, &mut items_sent))?;
        }

        Ok(items_sent)
    }
}
