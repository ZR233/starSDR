use std::ffi::CStr;

use starsdr_interface::*;
use uhd_sys::*;

#[allow(non_upper_case_globals)]
pub(crate) fn handle_uhd_err(r: i32)->SDRResult<()>{
    match r {
        uhd_error_UHD_ERROR_NONE=> Ok(()),
        uhd_error_UHD_ERROR_INVALID_DEVICE=> Err(SDRError::NotFound),
        _=> Err(SDRError::Unknown(format!("UHD Err({r}): {}", uhd_get_error_str())))
    }
}

fn uhd_get_error_str()->String{
    unsafe{
        let mut buffer = vec![0u8; 2048];
        uhd_get_last_error(buffer.as_mut_ptr() as _, buffer.len());
        let out = CStr::from_bytes_until_nul(&buffer).unwrap();
        let out = out.to_string_lossy();
        out.to_string()
    }
}