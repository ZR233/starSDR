use errors::handle_uhd_err;
use num::complex::Complex64;
use starsdr_interface::*;
use std::{
    fmt::Display,
    marker::PhantomData,
    ptr::null_mut,
    sync::{
        mpsc::{SendError, Sender},
        Arc, RwLock,
    },
};
use uhd_sys::*;
pub(crate) mod errors;
pub(crate) mod structs;
pub use starsdr_interface::CreateTx;
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
pub struct USRPHandle(uhd_usrp_handle);
unsafe impl Send for USRPHandle {}
unsafe impl Sync for USRPHandle {}
pub type USRPInner = Arc<RwLock<USRPHandle>>;
pub struct DeviceUHD {
    args: String,
    usrp: USRPInner,
}

impl From<String> for DeviceUHD {
    fn from(value: String) -> Self {
        Self {
            args: value,
            usrp: Arc::new(RwLock::new(USRPHandle(null_mut()))),
        }
    }
}

impl DeviceUHD {
    fn use_usrp<R, F>(&self, f: F) -> SDRResult<R>
    where
        F: FnOnce(uhd_usrp_handle) -> SDRResult<R>,
    {
        let g = self.usrp.read().unwrap();
        f(g.0)
    }
}

impl Display for DeviceUHD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.args)
    }
}

impl SDRDevice for DeviceUHD {
    fn open(&mut self) -> SDRResult<()> {
        unsafe {
            let mut g = self.usrp.write().unwrap();
            if g.0.is_null() {
                handle_uhd_err(uhd_usrp_make(&mut g.0, self.args.as_ptr() as _))?;
            }
        }
        Ok(())
    }

    fn tx_channel_count(&self) -> SDRResult<usize> {
        self.use_usrp(|h| {
            let mut count = 0;
            handle_uhd_err(unsafe { uhd_usrp_get_tx_num_channels(h, &mut count as _) })?;
            Ok(count)
        })
    }

    fn rx_channel_count(&self) -> SDRResult<usize> {
        self.use_usrp(|h| {
            let mut count = 0;
            handle_uhd_err(unsafe { uhd_usrp_get_rx_num_channels(h, &mut count as _) })?;
            Ok(count)
        })
    }
}
impl CreateTx<Complex64, TxUHD<Complex64>> for DeviceUHD {
    fn tx_stream(&self, channels: &[usize]) -> SDRResult<TxUHD<Complex64>> {
        todo!()
    }
}

pub struct TxUHD<T> {
    inner: USRPInner,
    streamer: uhd_tx_streamer_handle,
    md: uhd_tx_metadata_handle,
    _t: PhantomData<T>,
}
unsafe impl Send for TxUHD<Complex64> {}

impl Tx<Complex64> for TxUHD<Complex64> {
    fn send(v: &[Complex64]) -> Result<(), SendError<&[Complex64]>> {
        todo!()
    }

    // fn new(channels: &[usize], device: Self::D)->SDRResult<Self> {
    //     unsafe{
    //         let mut streamer = null_mut();
    //         handle_uhd_err(uhd_tx_streamer_make(&mut streamer))?;
    //         let mut md = null_mut();
    //         handle_uhd_err(uhd_tx_metadata_make(&mut md, false, 0, 0.1, true, false))?;

    //         Ok(Self { inner: device, streamer, md, _t: PhantomData })
    //     }
    // }
}

impl Drop for DeviceUHD {
    fn drop(&mut self) {
        let mut g = self.usrp.write().unwrap();

        if !g.0.is_null() {
            unsafe {
                uhd_usrp_free(&mut g.0);
            }
        }
    }
}
