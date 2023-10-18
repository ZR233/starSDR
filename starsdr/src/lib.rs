use std::error::Error;

use starsdr_interface::{Driver, SDRDevice};

pub struct SDR{
    drivers: Vec<Box<dyn Driver>> ,
}
#[allow(unused)]
impl SDR{

    async fn new() -> Result<Self, Box<dyn Error>> {
        let mut drivers: Vec<Box<dyn Driver>> = vec![];

        #[cfg(feature = "driver-uhd")]
        {
            drivers.push(Box::new(starsdr_uhd::DriverUHD::new()));
        }


        Ok(Self { drivers })
    }

    async fn device_list(&self) -> Vec<Box<dyn SDRDevice>> {
        let mut out: Vec<Box<dyn SDRDevice>> = vec![];
        for driver in self.drivers.iter() {
            out.append(&mut driver.list().await);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_device_list() {

        let sdr = SDR::new().await.unwrap();

        let devices = sdr.device_list().await;
        assert_ne!(devices.len(), 0);
    }
}
