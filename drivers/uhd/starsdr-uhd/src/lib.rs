use errors::handle_uhd_err;

use log::{info, debug};
use starsdr_interface::*;
use std::{
    ffi::CString,
    fmt::Display,
    marker::PhantomData,
    ptr::null_mut,
    sync::{Arc, RwLock},
};

use uhd_sys::*;

pub(crate) mod errors;
pub(crate) mod structs;

pub use crate::rx::RxUHD;
pub use crate::tx::TxUHD;
pub use starsdr_interface::CreateTx;
use structs::*;

pub mod rx;
pub mod tx;
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

            let args = CString::new("").unwrap();
            let r = uhd_usrp_find(args.as_ptr(), strings_out.as_mut_ptr());
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

    fn new_tx_streamer<T: Send>(
        &self,
        cpu_fmt: &str,
        otw_fmt: &str,
        args: &str,
        channels: &[usize],
    ) -> SDRResult<TxUHD<T>> {
        unsafe {
            let streamer = TxStreamerHandle::new()?;
            let mut stream_args = get_stream_args(cpu_fmt, otw_fmt, args, channels);
            let mut sample_num_max = 0;
            self.use_usrp(|h| {
                handle_uhd_err(uhd_usrp_get_tx_stream(h, &mut stream_args.0, streamer.0))?;
                handle_uhd_err(uhd_tx_streamer_max_num_samps(
                    streamer.0,
                    &mut sample_num_max,
                ))?;

                Ok(())
            })?;

            Ok(TxUHD {
                streamer,
                sample_num_max,
                _t: PhantomData,
            })
        }
    }
    fn new_rx_streamer<T: Send>(
        &self,
        cpu_fmt: &str,
        otw_fmt: &str,
        args: &str,
        channels: &[usize],
    ) -> SDRResult<RxUHD<T>> {
        unsafe {
            let streamer = RxStreamerHandle::new()?;
            let mut stream_args = get_stream_args(cpu_fmt, otw_fmt, args, channels);
            let mut sample_num_max = 0;
            let md = RxMetadataHandle::new()?;
            self.use_usrp(|h| {
                handle_uhd_err(uhd_usrp_get_rx_stream(h, &mut stream_args.0, streamer.0))?;
                handle_uhd_err(uhd_rx_streamer_max_num_samps(
                    streamer.0,
                    &mut sample_num_max,
                ))?;
                let cmd = uhd_stream_cmd_t {
                    stream_mode: uhd_stream_mode_t_UHD_STREAM_MODE_START_CONTINUOUS,
                    num_samps: sample_num_max,
                    stream_now: true,
                    time_spec_full_secs: 0,
                    time_spec_frac_secs: 0.0,
                };
                handle_uhd_err(uhd_rx_streamer_issue_stream_cmd(streamer.0, &cmd))?;
                Ok(())
            })?;

            Ok(RxUHD {
                streamer,
                sample_num_max,
                md,
                _t: PhantomData,
            })
        }
    }
}
struct StreamArgs(uhd_stream_args_t, CString, CString, CString);
fn get_stream_args(cpu_fmt: &str, otw_fmt: &str, args: &str, channels: &[usize]) -> StreamArgs {
    let cpu_fmt = CString::new(cpu_fmt).unwrap();
    let otw_fmt = CString::new(otw_fmt).unwrap();
    let args = CString::new(args).unwrap();
    StreamArgs(
        uhd_stream_args_t {
            cpu_format: cpu_fmt.as_ptr() as _,
            otw_format: otw_fmt.as_ptr() as _,
            args: args.as_ptr() as _,
            channel_list: channels.as_ptr() as _,
            n_channels: channels.len() as _,
        },
        cpu_fmt,
        otw_fmt,
        args,
    )
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

    fn set_tx_rate(&self, rate: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| handle_uhd_err(unsafe { uhd_usrp_set_tx_rate(h, rate, channel) }))
    }

    fn get_tx_rate(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut rate = 0.0;
            unsafe { handle_uhd_err(uhd_usrp_get_tx_rate(h, channel, &mut rate)) }?;
            Ok(rate)
        })
    }
    fn set_tx_freq(&self, freq: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| unsafe {
            let mut request = new_uhd_tune_request_t(freq);
            let mut result = new_uhd_tune_result_t();
            let r = handle_uhd_err(uhd_usrp_set_tx_freq(h, &mut request, channel, &mut result));
            debug!("result: {:?}", result);
            r
        })
    }

    fn get_tx_freq(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut freq = 0.0;
            unsafe { handle_uhd_err(uhd_usrp_get_tx_freq(h, channel, &mut freq)) }?;
            Ok(freq)
        })
    }
    fn set_tx_gain(&self, gain: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| {
            let name = CString::new("").unwrap();
            handle_uhd_err(unsafe { uhd_usrp_set_tx_gain(h, gain, channel, name.as_ptr()) })
        })
    }

    fn get_tx_gain(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut gain = 0.0;
            let name = CString::new("").unwrap();
            unsafe { handle_uhd_err(uhd_usrp_get_tx_gain(h, channel, name.as_ptr(), &mut gain)) }?;
            Ok(gain)
        })
    }

    fn set_tx_bandwidth(&self, bw: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| handle_uhd_err(unsafe { uhd_usrp_set_tx_bandwidth(h, bw, channel) }))
    }

    fn get_tx_bandwidth(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut bw = 0.0;
            unsafe { handle_uhd_err(uhd_usrp_get_tx_bandwidth(h, channel, &mut bw)) }?;
            Ok(bw)
        })
    }

    fn set_rx_rate(&self, rate: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| handle_uhd_err(unsafe { uhd_usrp_set_rx_rate(h, rate, channel) }))
    }

    fn get_rx_rate(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut rate = 0.0;
            unsafe { handle_uhd_err(uhd_usrp_get_rx_rate(h, channel, &mut rate)) }?;
            Ok(rate)
        })
    }
    fn set_rx_freq(&self, freq: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| unsafe {
            let mut request = new_uhd_tune_request_t(freq);
            let mut result = new_uhd_tune_result_t();
            let r = handle_uhd_err(uhd_usrp_set_rx_freq(h, &mut request, channel, &mut result));
            debug!("result: {:?}", result);
            r
        })
    }

    fn get_rx_freq(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut freq = 0.0;
            unsafe { handle_uhd_err(uhd_usrp_get_rx_freq(h, channel, &mut freq)) }?;
            Ok(freq)
        })
    }
    fn set_rx_gain(&self, gain: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| {
            let name = CString::new("").unwrap();
            handle_uhd_err(unsafe { uhd_usrp_set_rx_gain(h, gain, channel, name.as_ptr()) })
        })
    }

    fn get_rx_gain(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut gain = 0.0;
            let name = CString::new("").unwrap();
            unsafe { handle_uhd_err(uhd_usrp_get_rx_gain(h, channel, name.as_ptr(), &mut gain)) }?;
            Ok(gain)
        })
    }

    fn set_rx_bandwidth(&self, bw: f64, channel: usize) -> SDRResult<()> {
        self.use_usrp(|h| handle_uhd_err(unsafe { uhd_usrp_set_rx_bandwidth(h, bw, channel) }))
    }

    fn get_rx_bandwidth(&self, channel: usize) -> SDRResult<f64> {
        self.use_usrp(|h| {
            let mut bw = 0.0;
            unsafe { handle_uhd_err(uhd_usrp_get_rx_bandwidth(h, channel, &mut bw)) }?;
            Ok(bw)
        })
    }
}

impl CreateTx<f32, TxUHD<f32>> for DeviceUHD {
    fn tx_stream(&self, channels: &[usize]) -> SDRResult<TxUHD<f32>> {
        self.new_tx_streamer("fc32", "sc16", "", channels)
    }
}

impl CreateTx<f64, TxUHD<f64>> for DeviceUHD {
    fn tx_stream(&self, channels: &[usize]) -> SDRResult<TxUHD<f64>> {
        self.new_tx_streamer("fc64", "sc16", "", channels)
    }
}
impl CreateTx<i16, TxUHD<i16>> for DeviceUHD {
    fn tx_stream(&self, channels: &[usize]) -> SDRResult<TxUHD<i16>> {
        self.new_tx_streamer("sc16", "sc16", "", channels)
    }
}

impl CreateRx<f32, RxUHD<f32>> for DeviceUHD {
    fn rx_stream(&self, channels: &[usize]) -> SDRResult<RxUHD<f32>> {
        self.new_rx_streamer("fc32", "sc16", "", channels)
    }
}
impl CreateRx<i16, RxUHD<i16>> for DeviceUHD {
    fn rx_stream(&self, channels: &[usize]) -> SDRResult<RxUHD<i16>> {
        self.new_rx_streamer("sc16", "sc16", "", channels)
    }
}

impl CreateRx<f64, RxUHD<f64>> for DeviceUHD {
    fn rx_stream(&self, channels: &[usize]) -> SDRResult<RxUHD<f64>> {
        self.new_rx_streamer("sc64", "sc16", "", channels)
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

fn new_uhd_tune_result_t() -> uhd_tune_result_t {
    uhd_tune_result_t {
        clipped_rf_freq: 0.0,
        target_rf_freq: 0.0,
        actual_rf_freq: 0.0,
        target_dsp_freq: 0.0,
        actual_dsp_freq: 0.0,
    }
}

fn new_uhd_tune_request_t(freq: f64) -> uhd_tune_request_t {
    let args = null_mut();
    uhd_tune_request_t {
        target_freq: freq,
        rf_freq_policy: uhd_tune_request_policy_t_UHD_TUNE_REQUEST_POLICY_AUTO,
        rf_freq: 0.0,
        dsp_freq_policy: uhd_tune_request_policy_t_UHD_TUNE_REQUEST_POLICY_AUTO,
        dsp_freq: 0.0,
        args,
    }
}
