use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use starsdr_interface::SDRResult;
use uhd_sys::*;

use crate::errors::handle_uhd_err;

pub(crate) struct UHDStringVector {
    size: Option<usize>,
    index: usize,
    ptr: uhd_string_vector_handle,
}
impl UHDStringVector {
    pub fn new() -> Self {
        unsafe {
            let mut tmp: Vec<uhd_string_vector_t> = vec![];
            let mut ptr = tmp.as_mut_ptr();
            uhd_string_vector_make(&mut ptr);
            Self {
                index: 0,
                ptr,
                size: None,
            }
        }
    }
    pub fn as_mut_ptr(&mut self) -> &mut uhd_string_vector_handle {
        &mut self.ptr
    }
}
impl Iterator for UHDStringVector {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.size.unwrap_or_else(|| {
            let s = unsafe {
                let mut size = 0;
                uhd_string_vector_size(self.ptr, &mut size as _);
                size
            };
            self.size = Some(s);
            s
        });
        if self.index < size {
            let mut buffer= [0u8; 1024];
            unsafe {
                uhd_string_vector_at(self.ptr, self.index, buffer.as_mut_ptr() as _, buffer.len());
            }
            self.index += 1;
            let c= CStr::from_bytes_until_nul(&buffer).unwrap();
            Some(c.to_string_lossy().to_string())
            // Some(String::from_utf8(buffer).unwrap())
        } else {
            None
        }
    }
}

impl Drop for UHDStringVector {
    fn drop(&mut self) {
        unsafe {
            uhd_string_vector_free(&mut self.ptr);
        }
    }
}

pub(crate) struct TxStreamerHandle(pub uhd_tx_streamer_handle);
unsafe impl Send for TxStreamerHandle {}

impl TxStreamerHandle {
    pub fn new() -> SDRResult<Self> {
        unsafe {
            let mut streamer = null_mut();
            handle_uhd_err(uhd_tx_streamer_make(&mut streamer))?;

            Ok(Self(streamer))
        }
    }
}
impl Drop for TxStreamerHandle {
    fn drop(&mut self) {
        unsafe {
            uhd_tx_streamer_free(&mut self.0);
        }
    }
}

pub(crate) struct TxMetadataHandle(pub uhd_tx_metadata_handle);
unsafe impl Send for TxMetadataHandle {}

impl TxMetadataHandle {
    pub fn new() -> SDRResult<Self> {
        unsafe {
            let mut md = null_mut();
            handle_uhd_err(uhd_tx_metadata_make(
                &mut md, 
                false, 0, 
                0.1, true, false))?;

            Ok(Self(md))
        }
    }
}
impl Drop for TxMetadataHandle {
    fn drop(&mut self) {
        unsafe {
            uhd_tx_metadata_free(&mut self.0);
        }
    }
}
