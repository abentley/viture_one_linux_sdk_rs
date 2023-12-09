#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod sys {
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

unsafe extern "C" fn imu_callback (_: *mut u8, _: u16, _: u32) {
}

unsafe extern "C" fn mcu_callback (_: u16, _: *mut u8, _: u16, _: u32) {
}

#[derive(Debug)]
pub struct Sdk {
}

impl Sdk {
    pub fn safe_init() -> Option<Self> {
        use self::sys::init;
        unsafe {
            match init(Some(imu_callback), Some(mcu_callback)) {
                true => Some(Self{}),
                false => None,
            }
        }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        use self::sys::deinit;
        unsafe {deinit()};
    }
}
