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

    use super::*;
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

        let sdr = SDR::new(DriverUHD::new());

        let mut devices = sdr.device_list().unwrap();

        for d in devices.iter(){
            println!("{}", d);
        }

        let mut d = devices.pop().unwrap();
        d.open().unwrap();
        let channels = vec![0, 1];
      
        let tx  =  d.tx_stream( channels.as_slice()).unwrap();
        let rc = d.rx_channel_count().unwrap();
        let tc = d.tx_channel_count().unwrap();

        println!("rx: {}, tx: {}", rc, tc);

        assert!(true );
    }
}
