#[macro_use]
extern crate log;
pub use starsdr_interface::*;
#[cfg(feature="driver-uhd")]
pub use starsdr_uhd::{DriverUHD, DeviceUHD};
pub use num::complex::Complex64;

pub struct SDR<D>
    where D: SDRDriver{
    driver: D ,
}

#[allow(unused)]
impl <D: SDRDriver> SDR <D>{

    pub fn new(driver: D)->Self{
        Self { driver }
    }

    pub fn device_list(&self) -> SDRResult<Vec<D::Item>> {
        self.driver.list()
    }
}

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};
    use log::LevelFilter::Debug;

    use num::{Zero, complex::Complex32};

    use super::*;

    fn init() {
        let _ = env_logger::builder().filter_level(Debug).is_test(true).try_init();
    }
    #[tokio::test]
    async fn test_device_list() {

        let sdr = SDR::new(DriverUHD::new());

        let devices = sdr.device_list().unwrap();

        for d in devices.iter(){
            println!("{}", d);
        }

        assert_ne!(devices.len(), 0);
    }
    #[test]
    fn test_device_open() {

        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter(){
            println!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();

        let rc = d.rx_channel_count().unwrap();
        let tc = d.tx_channel_count().unwrap();

        println!("rx: {}, tx: {}", rc, tc);

        assert!(true );
    }
        #[test]
    fn test_tx() {
        init();
        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter(){
            debug!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channels = vec![0, 1];
      
        let tx  =  d.tx_stream( channels.as_slice()).unwrap();


            debug!("new thread");
        let h = thread::spawn(move||{

            let mut data = vec![Complex32::zero(); tx.sample_num_max];
            for i in 0..tx.sample_num_max{
                data[i] = Complex32::new(0.1, 0.0);
            }
            debug!("start send");

            let c = tx.sample_num_max;
            debug!("max: {}", c);

            let n = tx.send(data.as_slice()).unwrap();
            debug!("send: {}", n);
        });

        h.join().unwrap();

        assert!(true );
    }
}
