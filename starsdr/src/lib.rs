pub use num::complex::Complex64;
pub use starsdr_interface::*;
#[cfg(feature = "driver-uhd")]
pub use starsdr_uhd::*;

pub struct SDR<D>
where
    D: SDRDriver,
{
    driver: D,
}

#[allow(unused)]
impl<D: SDRDriver> SDR<D> {
    pub fn new(driver: D) -> Self {
        Self { driver }
    }

    pub fn device_list(&self) -> SDRResult<Vec<D::Item>> {
        self.driver.list()
    }
}

#[cfg(test)]
mod tests {
    use log::debug;

    use log::LevelFilter::Debug;

    use super::*;
    use num::{complex::Complex32, Complex, Zero};

    fn init() {
        let _ = env_logger::builder()
            .filter_level(Debug)
            .is_test(true)
            .try_init();
    }

    #[tokio::test]
    async fn test_device_list() {
        let sdr = SDR::new(DriverUHD::new());

        let devices = sdr.device_list().unwrap();

        for d in devices.iter() {
            println!("{}", d);
        }

        assert!(true);
    }

    #[test]
    fn test_device_open() {
        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter() {
            println!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();

        let rc = d.rx_channel_count().unwrap();
        let tc = d.tx_channel_count().unwrap();

        println!("rx: {}, tx: {}", rc, tc);

        assert!(true);
    }

    #[test]
    fn test_tx_f32() {
        init();
        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter() {
            debug!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channels = vec![0, 1];

        let tx: TxUHD<f32> = d.tx_stream(channels.as_slice()).unwrap();
        let mut data = vec![Complex32::zero(); tx.sample_num_max];
        for datum in &mut data {
            *datum = Complex32::new(0.1, 0.0);
        }
        debug!("start send");

        let c = tx.sample_num_max;
        debug!("max: {}", c);

        for _ in 0..1000 {
            let n = tx.send(data.as_slice()).unwrap();
            debug!("send: {}", n);
        }
    }
    #[test]
    fn test_tx_i16() {
        init();
        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter() {
            debug!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channels = vec![0, 1];

        let tx: TxUHD<i16> = d.tx_stream(channels.as_slice()).unwrap();
        let mut data = Vec::with_capacity(tx.sample_num_max);
        for _ in 0..data.capacity() {
            data.push(Complex::new(1, 0));
        }
        debug!("start send");

        let c = tx.sample_num_max;
        debug!("max: {}", c);

        for _ in 0..1000 {
            let n = tx.send(data.as_slice()).unwrap();
            debug!("send: {}", n);
        }
    }
    #[test]
    fn test_rx_f32() {
        init();
        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter() {
            debug!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channels = vec![0];

        let mut rx: RxUHD<f32> = d.rx_stream(channels.as_slice()).unwrap();
        for _ in 0..100 {
            let r = rx.recv().unwrap();
            debug!("rcv: {}", r.len());
        }
    }
    #[test]
    fn test_rx_i16() {
        init();
        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter() {
            debug!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channels = vec![0];

        let mut rx: RxUHD<i16> = d.rx_stream(channels.as_slice()).unwrap();
        for _ in 0..100 {
            let r = rx.recv().unwrap();
            debug!("rcv: {}", r.len());
        }

        assert!(true);
    }

    #[test]
    fn test_tx_rx_params() {
        init();
        let sdr = SDR::new(DriverUHD::new());
        let mut devices = sdr.device_list().unwrap();
        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channel = 0;
        let rate = 30e6;
        d.set_tx_rate(rate, channel).unwrap();

        assert_eq!(rate, d.get_tx_rate(channel).unwrap());

        d.set_rx_rate(rate, channel).unwrap();

        assert_eq!(rate, d.get_rx_rate(channel).unwrap());

        let gain = 76.0;
        d.set_tx_gain(gain, channel).unwrap();
        assert_eq!(gain, d.get_tx_gain(channel).unwrap());
        d.set_rx_gain(gain, channel).unwrap();
        assert_eq!(gain, d.get_tx_gain(channel).unwrap());

        let bw = 20e6;
        d.set_tx_bandwidth(bw, channel).unwrap();
        assert_eq!(bw, d.get_tx_bandwidth(channel).unwrap());


        let freq = 88.7e6;
        d.set_tx_freq(freq, channel).unwrap();
        assert_eq!(freq, d.get_tx_freq(channel).unwrap().round());
        d.set_rx_freq(freq, channel).unwrap();
        assert_eq!(freq, d.get_rx_freq(channel).unwrap().round());
    }
}
