use errors::handle_uhd_err;
use num::complex::{Complex32};
use starsdr_interface::*;
use std::{
    ffi::{CString, c_void},
    fmt::Display,
    marker::PhantomData,
    ptr::{null_mut, slice_from_raw_parts_mut, slice_from_raw_parts},
    sync::{
        mpsc::{SendError, Sender},
        Arc, RwLock,
    }, cell::{Cell, UnsafeCell},
};
use log::debug;
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
        if g.0.is_null() {
            return Err(SDRError::NotOpen);
        }
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
impl CreateTx<Complex32, TxUHD<Complex32>> for DeviceUHD {
    fn tx_stream(&self, channels: &[usize]) -> SDRResult<TxUHD<Complex32>> {
        unsafe {
            let streamer = TxStreamerHandle::new()?;
            let cpu_fmt = CString::new("fc32").unwrap();
            let otw_fmt = CString::new("sc16").unwrap();

            let mut stream_args = uhd_stream_args_t {
                cpu_format: cpu_fmt.as_ptr() as _,
                otw_format: otw_fmt.as_ptr() as _,
                args: "".as_ptr() as _,
                channel_list: channels.as_ptr() as _,
                n_channels: channels.len() as _,
            };
            let mut sample_num_max = 0;
            self.use_usrp(|h| {
                handle_uhd_err(uhd_usrp_get_tx_stream(h, &mut stream_args, streamer.0))?;
                handle_uhd_err(uhd_tx_streamer_max_num_samps(streamer.0, &mut sample_num_max))?;
                Ok(())
            })?;

            Ok(TxUHD {
                streamer,
                sample_num_max,
                _t: PhantomData,
            })
        }
    }
}

pub struct TxUHD<T: Send> {
    streamer: TxStreamerHandle,
    pub sample_num_max: usize,
    _t: PhantomData<T>,
}

impl <T: Send> Tx<T> for TxUHD<T> {
    fn send(&self, v: &[T]) -> SDRResult<usize> {
        if v.len() > self.sample_num_max {
            return Err(SDRError::Param {
                key: "v".into(),
                value: format!("len()={}", v.len()),
                msg: format!("> max: {}", self.sample_num_max),
            });
        }
        let mut items_sent = 0;
        let mut md = TxMetadataHandle::new()?;
        unsafe{
            let buf1 = v.as_ptr() as *const c_void;
            let buf2 = &*buf1;
            let buf3 = buf2 as *const c_void;
            let buf4 = &buf3 as *const *const c_void;
            let buf5 = buf4 as *mut *const c_void;
            // let md = &self.md.0 as *const uhd_tx_metadata_handle;
            handle_uhd_err(uhd_tx_streamer_send(
                self.streamer.0,  buf5,
                 v.len(), &mut md.0 , 0.1, &mut items_sent))?;
        }

        Ok(items_sent)
    }
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
