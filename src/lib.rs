extern crate ash;

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

// A set of strings, prepared for a C FFI
pub struct CStringSet {
    // This is the pointer you pass to C
    pub pp: *const *const c_char,
    cstrings: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl CStringSet {
    pub fn new(s: Vec<&str>) -> CStringSet {
        let mut ss = CStringSet {
            pp: ptr::null(),
            cstrings: s.iter().map(|s| CString::new(&**s).unwrap()).collect(),
            ptrs: vec![],
        };

        ss.ptrs = ss.cstrings.iter().map(|cs| cs.as_ptr()).collect();
        ss.ptrs.push(ptr::null()); // C uses null ptr as terminator of ptr list
        ss.pp = ss.ptrs.as_ptr();
        ss
    }
}
