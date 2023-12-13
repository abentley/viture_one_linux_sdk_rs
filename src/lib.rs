#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod sys {
    use num_enum::TryFromPrimitive;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    #[derive(Debug, TryFromPrimitive)]
    #[repr(i32)]
    pub enum SdkErrCode {
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
    }
}

/// Return value indicating success
const ERR_SUCCESS: i32 = sys::ERR_SUCCESS as i32;

#[derive(Debug)]
pub enum SdkErr {
    SdkErrCode(SdkErrCode),
    UnknownCode(i32),
}

impl From<SdkErrCode> for SdkErr {
    fn from(code: SdkErrCode) -> SdkErr {
        SdkErr::SdkErrCode(code)
    }
}

pub use sys::SdkErrCode;

use std::mem::size_of;

#[derive(Debug)]
pub struct ImuData {
    // +- 90 (past 90 degrees, pitch and yaw invert)
    pub roll: f32,
    // +- 180
    pub pitch: f32,
    // +- 180 (zero at connection time)
    pub yaw: f32,
}

/// Note: You should normally implement CallbackImu.  This is for cases where even converting the
/// data is too expensive.
pub trait RawCallbackImu {
    /// # Safety
    /// None
    unsafe extern "C" fn raw_imu_message(data: *mut u8, len: u16, ts: u32);
}

pub trait CallbackImu {
    /**
     * A function that will be called for every IMU message received.
     *
     * data: an IMUData object
     * ts: milliseconds since connected?  Monotonic?
     */
    fn imu_message(data: &ImuData, ts: u32);
}

impl<T: CallbackImu> RawCallbackImu for T {
    /// data: 12 32-bit floats in big-endian format: roll, pitch, yaw.  All remaining bytes
    /// reserved.
    ///
    /// len: The length of data
    /// ts: A timestamp.  (Since connected?  Monotonic?)
    ///
    /// # Safety
    /// We copy the contents of data, we don't pass it down to safe functions
    /// We check that len >= min_size.
    /// We check that data is not null.
    /// It is possible that data is invalid in some other way, but we can't know in advance.
    unsafe extern "C" fn raw_imu_message(data: *mut u8, len: u16, ts: u32) {
        const pitch_offset: usize = size_of::<f32>();
        const yaw_offset: usize = pitch_offset * 2;
        const min_size: usize = pitch_offset * 3;
        eprintln!("len: {} ts: {}", len, ts);
        if data.is_null() || (len as usize) < min_size {
            return;
        }
        let data = ImuData {
            roll: f32::from_be_bytes(*data.cast::<[u8; 4]>()),
            pitch: f32::from_be_bytes(*data.add(pitch_offset).cast::<[u8; 4]>()),
            yaw: f32::from_be_bytes(*data.add(yaw_offset).cast::<[u8; 4]>()),
        };
        Self::imu_message(&data, ts);
    }
}

pub trait CallbackMcu {
    /// # Safety
    /// Does nothing.
    unsafe extern "C" fn raw_mcu_message(msgid: u16, _data: *mut u8, len: u16, ts: u32) {
        eprintln!("msgid: {msgid} len: {len} ts: {ts}")
    }
}

impl From<i32> for SdkErr {
    fn from(discriminant: i32) -> SdkErr {
        if let Ok(err) = SdkErrCode::try_from(discriminant) {
            err.into()
        } else {
            SdkErr::UnknownCode(discriminant)
        }
    }
}

/**
 * Must be initialized to access functionality.  Will deinitialize when dropped.
 */
#[derive(Debug)]
pub struct Sdk {}

impl Sdk {
    /**
     * Initialize the usblib and return an Sdk object to interact with the glasses.
     */
    pub fn init<I: RawCallbackImu, M: CallbackMcu>() -> Result<Self, SdkErr> {
        use self::sys::init;
        unsafe {
            match init(Some(I::raw_imu_message), Some(M::raw_mcu_message)) {
                true => Ok(Self {}),
                false => Err(SdkErrCode::Failure.into()),
            }
        }
    }
    /**
     * Set IMU state.  true: on, false: off
     */
    pub fn set_imu(&mut self, on_off: bool) -> Result<(), SdkErr> {
        use self::sys::set_imu;
        let result = unsafe { set_imu(on_off) };
        if result == ERR_SUCCESS {
            Ok(())
        } else {
            Err(result.into())
        }
    }
    /**
     * Get IMU state.  true: on, false: off
     */
    pub fn get_imu_state(&mut self) -> Result<bool, SdkErr> {
        use self::sys::get_imu_state;
        match unsafe { get_imu_state() } {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(x.into()),
        }
    }
    /**
     * Set 3d state.  true: on (resolution 3840x1080), false: off (resolution 1920x1080)
     */
    pub fn set_3d(&mut self, on_off: bool) -> Result<(), SdkErr> {
        use self::sys::set_3d;
        let result = unsafe { set_3d(on_off) };
        if result == ERR_SUCCESS {
            Ok(())
        } else {
            Err(result.into())
        }
    }
    /**
     * Get 3d state.  true: on (resolution 3840x1080), false: off (resolution 1920x1080)
     */
    pub fn get_3d_state(&mut self) -> Result<bool, SdkErr> {
        use self::sys::get_3d_state;
        match unsafe { get_3d_state() } {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(x.into()),
        }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        use self::sys::deinit;
        unsafe { deinit() };
    }
}
