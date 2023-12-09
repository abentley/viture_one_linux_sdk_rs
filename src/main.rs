use viture_rs::init;

unsafe extern "C" fn imu_callback (_: *mut u8, _: u16, _: u32) {
}

unsafe extern "C" fn mcu_callback (_: u16, _: *mut u8, _: u16, _: u32) {
}


fn main() {
    println!("Hello, world!");
    unsafe {
        init(Some(imu_callback), Some(mcu_callback));
    }
}
