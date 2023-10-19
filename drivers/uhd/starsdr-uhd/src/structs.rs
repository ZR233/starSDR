use uhd_sys::*;

pub(crate) struct UHDStringVector {
    size: Option<usize>,
    index: usize,
    ptr: uhd_string_vector_handle,
}
impl UHDStringVector {
    pub fn new() -> Self {
        unsafe {
            let mut tmp: Vec<uhd_string_vector_t> = vec![];
            let mut ptr = tmp.as_mut_ptr();
            uhd_string_vector_make(&mut ptr);
            Self {
                index: 0,
                ptr,
                size: None,
            }
        }
    }
    pub fn as_mut_ptr(&mut self) -> &mut uhd_string_vector_handle {
        &mut self.ptr
    }
}
impl Iterator for UHDStringVector {
    fn next(&mut self) -> Option<Self::Item> {
        let size = self.size.unwrap_or_else(|| {
            let s = unsafe {
                let mut size = 0;
                uhd_string_vector_size(self.ptr, &mut size as _);
                size
            };
            self.size = Some(s);
            s
        });
        if self.index < size {
            let mut buffer: Vec<u8> = vec![0u8; 1024];
            unsafe {
                uhd_string_vector_at(self.ptr, self.index, buffer.as_mut_ptr() as _, buffer.len());
            }
            self.index += 1;
            Some(String::from_utf8(buffer).unwrap())
        } else {
            None
        }
    }

    type Item = String;
}

impl Drop for UHDStringVector {
    fn drop(&mut self) {
        unsafe {
            uhd_string_vector_free(&mut self.ptr);
        }
    }
}
