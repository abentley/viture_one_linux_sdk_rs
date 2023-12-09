#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    #[derive(Debug)]
    #[repr(i32)]
    pub enum SdkErr {
        Success = ERR_SUCCESS as i32,
        Failure = ERR_FAILURE as i32,
        InvalidArgument = ERR_INVALID_ARGUMENT as i32,
        NotEnoughMemory = ERR_NOT_ENOUGH_MEMORY as i32,
        UnsupportedCommand = ERR_UNSUPPORTED_CMD as i32,
        CrcMismatch = ERR_CRC_MISMATCH as i32,
        VersionMismatch = ERR_VER_MISMATCH as i32,
        MessageIdMismatch = ERR_MSG_ID_MISMATCH as i32,
        MessageStxMismatch = ERR_MSG_STX_MISMATCH as i32,
        CodeNotWritten = ERR_CODE_NOT_WRITTEN as i32,
        WriteFailure = ERR_WRITE_FAIL,
        RspError = ERR_RSP_ERROR,
        Timeout = ERR_TIMEOUT,
        UnknownErr(i32) = 255,
    }
    impl From<i32> for SdkErr {
        fn from(discriminant: i32) -> Self {
            use SdkErr::*;
            match discriminant {
                ERR_WRITE_FAIL => WriteFailure,
                ERR_RSP_ERROR => RspError,
                ERR_TIMEOUT => Timeout,
                _ => match discriminant as u32 {
                    ERR_FAILURE => Failure,
                    ERR_INVALID_ARGUMENT => InvalidArgument,
                    ERR_NOT_ENOUGH_MEMORY => NotEnoughMemory,
                    ERR_UNSUPPORTED_CMD => UnsupportedCommand,
                    ERR_CRC_MISMATCH => CrcMismatch,
                    ERR_VER_MISMATCH => VersionMismatch,
                    ERR_MSG_ID_MISMATCH => MessageIdMismatch,
                    ERR_MSG_STX_MISMATCH => MessageStxMismatch,
                    ERR_CODE_NOT_WRITTEN => CodeNotWritten,
                    _ => UnknownErr(discriminant),
                },
            }
        }
    }
}

pub use sys::SdkErr;

unsafe extern "C" fn imu_callback(_: *mut u8, _: u16, _: u32) {}

unsafe extern "C" fn mcu_callback(_: u16, _: *mut u8, _: u16, _: u32) {}

fn result_from_err(discriminant: i32) -> Result<(), SdkErr> {
    let err: SdkErr = discriminant.into();
    if let SdkErr::UnknownErr(0) = err {
        Ok(())
    } else {
        Err(err)
    }
}

#[derive(Debug)]
pub struct Sdk {}

impl Sdk {
    pub fn safe_init() -> Option<Self> {
        use self::sys::init;
        unsafe {
            match init(Some(imu_callback), Some(mcu_callback)) {
                true => Some(Self {}),
                false => None,
            }
        }
    }
    pub fn set_imu(&self, on_off: bool) -> Result<(), SdkErr> {
        use self::sys::set_imu;
        unsafe { result_from_err(set_imu(on_off)) }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        use self::sys::deinit;
        unsafe { deinit() };
    }
}
