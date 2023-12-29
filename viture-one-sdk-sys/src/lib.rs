#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use num_enum::TryFromPrimitive;
use std::ffi::c_int;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
pub enum SdkErrCode {
    // Exclude ERR_SUCCESS, because we want to use Result.
    Failure = ERR_FAILURE as c_int,
    InvalidArgument = ERR_INVALID_ARGUMENT as c_int,
    NotEnoughMemory = ERR_NOT_ENOUGH_MEMORY as c_int,
    UnsupportedCommand = ERR_UNSUPPORTED_CMD as c_int,
    CrcMismatch = ERR_CRC_MISMATCH as c_int,
    VersionMismatch = ERR_VER_MISMATCH as c_int,
    MessageIdMismatch = ERR_MSG_ID_MISMATCH as c_int,
    MessageStxMismatch = ERR_MSG_STX_MISMATCH as c_int,
    CodeNotWritten = ERR_CODE_NOT_WRITTEN as c_int,
    WriteFailure = ERR_WRITE_FAIL,
    RspError = ERR_RSP_ERROR,
    Timeout = ERR_TIMEOUT,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
/// Frequencies that the IMU can run at.
pub enum ImuFrequency {
    Hz60 = IMU_FREQUENCE_60 as c_int,
    Hz90 = IMU_FREQUENCE_90 as c_int,
    Hz120 = IMU_FREQUENCE_120 as c_int,
    Hz240 = IMU_FREQUENCE_240 as c_int,
}
