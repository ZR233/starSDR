mod bindings;

#[cfg(test)]
mod test{
    use std::ffi::CString;
    use std::ffi::CStr;

    use super::bindings::*;
    #[test]
    fn test_version(){
        unsafe{
            
            let mut buffer = [0;20];
            
            let err = uhd_get_version_string(buffer.as_mut_ptr() , buffer.len());
            println!("{:?}", err);

            let version = CStr::from_ptr(buffer.as_ptr()) .to_str().unwrap();

            println!("{:?}",  version);


        }
    }
}