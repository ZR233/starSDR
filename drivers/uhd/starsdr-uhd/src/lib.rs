use async_trait::async_trait;
use starsdr_interface::{Driver, SDRDevice};
use uhd_sys::*;

pub struct DriverUHD {}

impl DriverUHD {
    pub fn new() -> Self {
        DriverUHD {}
    }
}

#[async_trait]
impl Driver for DriverUHD {
    async fn list(&self) -> Vec<Box<dyn SDRDevice>> {
        let mut out = vec![];

        unsafe{
            let mut strings_out = UHDStringVector::new();

            let args = "";
            let r = uhd_usrp_find(args.as_ptr() as _, &mut strings_out.ptr);
            if r != 0{
                return vec![];
            }

            let list = strings_out.list();

            for s in list {
                let one : Box<dyn SDRDevice>=Box::new(Device{args: s});
                out.push(one);
            }
        }

        out
    }
}

struct Device{
    args: String,
}

impl Device {
    
}

impl SDRDevice for Device {
    
}

struct UHDStringVector{
    ptr: uhd_string_vector_handle,
}
impl UHDStringVector {
    fn new()->Self{
        unsafe{
            let mut tmp: Vec<uhd_string_vector_t> = vec![];
            let mut ptr=  tmp.as_mut_ptr();
            uhd_string_vector_make(&mut ptr);
            Self {  ptr  }
        }
    }

    fn list(&self)->Vec<String>{
        unsafe{
           let mut size = 0;
            uhd_string_vector_size(self.ptr, &mut size as _);
            let mut out = Vec::with_capacity(size);
            for i in 0..size{
                let mut buffer:Vec<u8> = Vec::with_capacity(1024);
                buffer.resize(buffer.capacity(), 0);
                uhd_string_vector_at(self.ptr, i, buffer.as_mut_ptr() as _, buffer.len());

                out.push(String::from_utf8(buffer).unwrap());
            }


            out
        }
    }
}


impl Drop  for UHDStringVector {
    fn drop(&mut self) {
        unsafe{
            uhd_string_vector_free(&mut self.ptr);
        }
    }
}
