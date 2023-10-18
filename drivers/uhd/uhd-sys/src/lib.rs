#[allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings;

pub use bindings::*;


#[allow(unused)]
fn version() -> String {
    unsafe {
        let mut buffer = Vec::with_capacity(512);
        buffer.resize(512, 0);
        let buffer_len = buffer.len();
        let ptr = buffer.as_mut_ptr();
        let r = bindings::uhd_get_version_string(ptr as _, buffer_len);
        if r != 0 {
            panic!("uhd_get_version_string failed {}", r);
        }
        String::from_utf8(buffer).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        println!("UHD Version: {}", v);
        assert!(v.len() > 0);
    }
}
