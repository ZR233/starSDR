use errors::handle_uhd_err;
use starsdr_interface::{SDRDevice, SDRDriver, SDRResult};
use std::fmt::Display;
use uhd_sys::*;
mod errors;

pub struct DriverUHD {}

impl DriverUHD {
    pub fn new() -> Self {
        DriverUHD {}
    }
}

impl SDRDriver for DriverUHD {
    type Item = DeviceUHD;

    fn list(&self) -> SDRResult<Vec<DeviceUHD>> {
        let mut out = vec![];

        unsafe {
            let mut strings_out = UHDStringVector::new();

            let args = "";
            let r = uhd_usrp_find(args.as_ptr() as _, &mut strings_out.ptr);
            handle_uhd_err(r)?;

            let list = strings_out.list();

            for s in list {
                out.push(s.into());
            }
        }

        Ok(out)
    }
}

pub struct DeviceUHD {
    args: String,
}

impl From<String> for DeviceUHD {
    fn from(value: String) -> Self {
        Self { args: value }
    }
}

impl DeviceUHD {}
impl Display for DeviceUHD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.args)
    }
}

impl SDRDevice for DeviceUHD {
    fn open(&mut self) -> SDRResult<()> {
        todo!()
    }
}

struct UHDStringVector {
    ptr: uhd_string_vector_handle,
}
impl UHDStringVector {
    fn new() -> Self {
        unsafe {
            let mut tmp: Vec<uhd_string_vector_t> = vec![];
            let mut ptr = tmp.as_mut_ptr();
            uhd_string_vector_make(&mut ptr);
            Self { ptr }
        }
    }

    fn list(&self) -> Vec<String> {
        unsafe {
            let mut size = 0;
            uhd_string_vector_size(self.ptr, &mut size as _);
            let mut out = Vec::with_capacity(size);
            for i in 0..size {
                let mut buffer: Vec<u8> = Vec::with_capacity(1024);
                buffer.resize(buffer.capacity(), 0);
                uhd_string_vector_at(self.ptr, i, buffer.as_mut_ptr() as _, buffer.len());

                out.push(String::from_utf8(buffer).unwrap());
            }

            out
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
