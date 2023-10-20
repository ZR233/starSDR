use std::ffi::c_void;
use std::marker::PhantomData;
use starsdr_interface::{Complex, Rx, SDRError, SDRResult};
use uhd_sys::*;
use crate::errors::handle_uhd_err;
use crate::structs::{RxMetadataHandle, RxStreamerHandle};

pub struct RxUHD<T: Send> {
    pub(crate) streamer: RxStreamerHandle,
    pub sample_num_max: usize,
    pub(crate) md: RxMetadataHandle,
    pub(crate) _t: PhantomData<T>,
}


impl <T:Send> Rx<T> for RxUHD<T> {

    fn recv(&mut self) -> SDRResult<Vec<Complex<T>>> {


        unsafe {
            let num = self.sample_num_max;
            let mut buf: Vec<T> = Vec::with_capacity(num*2);
            let ptr = buf.as_mut_ptr();
            std::mem::forget(buf);

            let mut n = 0;
            handle_uhd_err(uhd_rx_streamer_recv(
                self.streamer.0,
                &mut (ptr as *mut c_void),
                num,
                &mut self.md.0,
                1.0,
                false,
                &mut n
            ) )?;
            let p = ptr as *mut Complex<T>;
            let buf = Vec::from_raw_parts(p, n, num);
            let mut code = 0;
            uhd_rx_metadata_error_code(self.md.0, &mut code);

            #[allow(non_upper_case_globals)]
            match code {
                uhd_rx_metadata_error_code_t_UHD_RX_METADATA_ERROR_CODE_NONE=>Ok(buf),
                uhd_rx_metadata_error_code_t_UHD_RX_METADATA_ERROR_CODE_OVERFLOW=>Err(SDRError::Overflow),
                uhd_rx_metadata_error_code_t_UHD_RX_METADATA_ERROR_CODE_TIMEOUT=>Err(SDRError::TimeOut),
                _=> Err(SDRError::Unknown(format!("recv fail: uhd meta[{code}]")))
            }
        }
    }
}
