use errors::handle_uhd_err;
use starsdr_interface::{SDRDevice, SDRDriver, SDRResult};
use std::fmt::Display;
use uhd_sys::*;
pub(crate) mod errors;
pub(crate) mod structs;
use structs::*;

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
            let r = uhd_usrp_find(args.as_ptr() as _, strings_out.as_mut_ptr());
            handle_uhd_err(r)?;

            for s in strings_out {
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
