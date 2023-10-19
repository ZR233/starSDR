pub use starsdr_interface::{SDRDriver, SDRDevice, SDRResult};
#[cfg(feature="driver-uhd")]
pub use starsdr_uhd::{DriverUHD, DeviceUHD};


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
}
