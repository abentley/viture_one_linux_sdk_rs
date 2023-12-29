use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi::c_int;

// Return value indicating success
const ERR_SUCCESS: c_int = viture_one_sdk_sys::ERR_SUCCESS as c_int;

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
/// Recognized error codes for SdkErr
pub enum GeneralErrCode {
    WriteFailure = ERR_WRITE_FAIL,
    RspError = ERR_RSP_ERROR,
    Timeout = ERR_TIMEOUT,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
/// Recognized error codes for SdkErr
pub enum MessageErrCode {
    // Exclude ERR_SUCCESS, because we want to use Result.
    Failure = ERR_FAILURE,
    InvalidArgument = ERR_INVALID_ARGUMENT,
    NotEnoughMemory = ERR_NOT_ENOUGH_MEMORY,
    UnsupportedCommand = ERR_UNSUPPORTED_CMD,
    CrcMismatch = ERR_CRC_MISMATCH,
    VersionMismatch = ERR_VER_MISMATCH,
    MessageIdMismatch = ERR_MSG_ID_MISMATCH,
    MessageStxMismatch = ERR_MSG_STX_MISMATCH,
    CodeNotWritten = ERR_CODE_NOT_WRITTEN,
}

/// Error type
#[derive(Debug)]
pub enum SdkErr {
    /// Negative error codes are general
    GeneralErrCode(GeneralErrCode),

    /// Message Error codes are positive and overlap with other return values
    MessageErrCode(MessageErrCode),

    /// Error codes outside the range supported by the Sdk (and also 0 for success)
    UnknownCode(c_int),
}

impl From<MessageErrCode> for SdkErr {
    fn from(code: MessageErrCode) -> SdkErr {
        SdkErr::MessageErrCode(code)
    }
}

impl From<GeneralErrCode> for SdkErr {
    fn from(code: GeneralErrCode) -> SdkErr {
        SdkErr::GeneralErrCode(code)
    }
}

use viture_one_sdk_sys::{
    ERR_CODE_NOT_WRITTEN, ERR_CRC_MISMATCH, ERR_FAILURE, ERR_INVALID_ARGUMENT, ERR_MSG_ID_MISMATCH,
    ERR_MSG_STX_MISMATCH, ERR_NOT_ENOUGH_MEMORY, ERR_RSP_ERROR, ERR_TIMEOUT, ERR_UNSUPPORTED_CMD,
    ERR_VER_MISMATCH, ERR_WRITE_FAIL, IMU_FREQUENCE_120, IMU_FREQUENCE_240, IMU_FREQUENCE_60,
    IMU_FREQUENCE_90,
};

#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
/// Frequencies that the IMU can run at.
pub enum ImuFrequency {
    Hz60 = IMU_FREQUENCE_60 as c_int,
    Hz90 = IMU_FREQUENCE_90 as c_int,
    Hz120 = IMU_FREQUENCE_120 as c_int,
    Hz240 = IMU_FREQUENCE_240 as c_int,
}

/// Note: You should normally implement CallbackImu.  This is for cases where even converting the
/// data is too expensive.
pub trait RawCallbackImu {
    /// # Safety
    /// None
    unsafe extern "C" fn raw_imu_message(data: *mut u8, len: u16, ts: u32);
}

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

/// Callback that receives IMU data as f32 floats
pub trait CallbackImu {
    /**
     * A function that will be called for every IMU message received.
     *
     * data: an ImuData object
     * ts: milliseconds since connected?  Monotonic?
     */
    fn imu_message(data: ImuData, ts: u32);
}

/// Callback that receives raw IMU data
impl<T: CallbackImu> RawCallbackImu for T {
    /// data: 12 32-bit floats in big-endian format: roll, pitch, yaw.  All remaining bytes
    /// reserved.
    ///
    /// len: The length of data
    /// ts: A timestamp.  (Since connected?  Monotonic?)
    ///
    /// # Safety
    /// We copy the contents of data, we don't pass it down to safe functions
    /// We check that len >= MIN_SIZE.
    /// We check that data is not null.
    /// It is possible that data is invalid in some other way, but we can't know in advance.
    unsafe extern "C" fn raw_imu_message(data: *mut u8, len: u16, ts: u32) {
        const PITCH_OFFSET: usize = size_of::<f32>();
        const YAW_OFFSET: usize = PITCH_OFFSET * 2;
        const MIN_SIZE: usize = PITCH_OFFSET * 3;
        if data.is_null() || (len as usize) < MIN_SIZE {
            return;
        }
        let data = ImuData {
            roll: f32::from_be_bytes(*data.cast::<[u8; 4]>()),
            pitch: f32::from_be_bytes(*data.add(PITCH_OFFSET).cast::<[u8; 4]>()),
            yaw: f32::from_be_bytes(*data.add(YAW_OFFSET).cast::<[u8; 4]>()),
        };
        Self::imu_message(data, ts);
    }
}

/// Callback that should receive events, but currently doesn't.
pub trait RawCallbackMcu {
    /// # Safety
    /// Does nothing.
    unsafe extern "C" fn raw_mcu_message(msgid: u16, _data: *mut u8, len: u16, ts: u32);
}

/// No-op callback for MCU
struct Noop {}

impl RawCallbackMcu for Noop {
    /// # Safety
    /// Does nothing.
    unsafe extern "C" fn raw_mcu_message(msgid: u16, _data: *mut u8, len: u16, ts: u32) {
        eprintln!("msgid: {msgid} len: {len} ts: {ts}")
    }
}

impl From<c_int> for SdkErr {
    fn from(discriminant: c_int) -> SdkErr {
        if let Ok(err) = GeneralErrCode::try_from(discriminant) {
            err.into()
        } else if let Ok(err) = MessageErrCode::try_from(discriminant as u32) {
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
     * Use raw callbacks.
     */
    pub fn raw_init<I: RawCallbackImu, M: RawCallbackMcu>() -> Result<Self, SdkErr> {
        use viture_one_sdk_sys::init;
        unsafe {
            match init(Some(I::raw_imu_message), Some(M::raw_mcu_message)) {
                true => Ok(Self {}),
                false => Err(MessageErrCode::Failure.into()),
            }
        }
    }

    /**
     * Initialize the usblib and return an Sdk object to interact with the glasses.
     * Use a safe Imu callback.
     */
    pub fn init<I: CallbackImu>() -> Result<Self, SdkErr> {
        Self::raw_init::<I, Noop>()
    }

    /**
     * Set IMU state.  true: on, false: off
     */
    pub fn set_imu(&mut self, on_off: bool) -> Result<(), SdkErr> {
        use viture_one_sdk_sys::set_imu;
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
        use viture_one_sdk_sys::get_imu_state;
        match unsafe { get_imu_state() } {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(x.into()),
        }
    }

    pub fn set_imu_fq(&mut self, frequency: ImuFrequency) -> Result<(), SdkErr> {
        use viture_one_sdk_sys::set_imu_fq;
        let result = unsafe { set_imu_fq(frequency.into()) };
        if result == ERR_SUCCESS {
            Ok(())
        } else {
            Err(result.into())
        }
    }

    /**
     * Get IMU state.  true: on, false: off
     */
    pub fn get_imu_fq(&mut self) -> Result<ImuFrequency, SdkErr> {
        use viture_one_sdk_sys::get_imu_fq;
        let fq = unsafe { get_imu_fq() };
        if let Ok(freq) = ImuFrequency::try_from(fq) {
            Ok(freq)
        } else if let Ok(e) = GeneralErrCode::try_from(fq) {
            Err(SdkErr::from(e))
        } else {
            Err(SdkErr::UnknownCode(fq))
        }
    }

    /**
     * Set 3d state.  true: on (resolution 3840x1080), false: off (resolution 1920x1080)
     */
    pub fn set_3d(&mut self, on_off: bool) -> Result<(), SdkErr> {
        use viture_one_sdk_sys::set_3d;
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
        use viture_one_sdk_sys::get_3d_state;
        match unsafe { get_3d_state() } {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(x.into()),
        }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        use viture_one_sdk_sys::deinit;
        unsafe { deinit() };
    }
}
