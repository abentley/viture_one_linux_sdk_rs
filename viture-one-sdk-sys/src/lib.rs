#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use num_enum::TryFromPrimitive;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
pub enum SdkErrCode {
    // Exclude ERR_SUCCESS, because we want to use Result.
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

