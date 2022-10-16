use base64;
pub use serde::{self, Deserialize, Serialize};
use syscall_derive::{syscall_response, SysCall, SysCallResp};

pub type SysData = Option<String>;

pub fn encode_buffer(buf: &Vec<u8>, length: i64) -> SysData {
    Some(base64::encode(&buf[0..length as usize]))
}

pub fn decode_buffer(buf: &SysData) -> Vec<u8> {
    if let Some(b) = buf {
        return base64::decode(b).unwrap_or(vec![]);
    }

    vec![]
}

// TODO: SysCalls always return a value (rax) so make it user definable integer,
// something like SysVal::Signed, sysval::unsigned

#[derive(Debug, Copy, Clone)]
#[repr(u64)]
pub enum SysCallNum {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    // NOTE: NoOp is non standard syscall number SHOULD invoke ENOSYS (not implemented)
    NoOp = u64::MAX,
}

impl PartialEq<u64> for SysCallNum {
    fn eq(&self, other: &u64) -> bool {
        *self as u64 == *other
    }
}

impl PartialEq<SysCallNum> for u64 {
    fn eq(&self, other: &SysCallNum) -> bool {
        *self == *other as u64
    }
}

pub fn is_implemented(num: u64) -> bool {
    let num: SysCallNum = unsafe { ::std::mem::transmute(num) };

    match num {
        SysCallNum::Read | SysCallNum::Open | SysCallNum::Close | SysCallNum::Write => true,
        _ => false,
    }
}

pub trait SysCall {
    fn num() -> SysCallNum;
}

pub trait SysCallResp {
    fn ret_value(&self) -> i64;
}

#[derive(SysCall, Debug, Serialize, Deserialize)]
#[syscall(num = "Open")]
pub struct OpenRequest {
    pub path: String,
    pub oflag: u64,
    pub mode: u64,
}

#[syscall_response]
#[derive(SysCall, SysCallResp, Debug, Serialize, Deserialize)]
#[syscall(num = "Open")]
pub struct OpenResp;

#[derive(SysCall, Debug, Serialize, Deserialize)]
#[syscall(num = "Close")]
pub struct CloseRequest {
    pub fd: i64,
}

#[syscall_response]
#[derive(SysCall, SysCallResp, Debug, Serialize, Deserialize)]
#[syscall(num = "Close")]
pub struct CloseResp;

#[derive(SysCall, Debug, Serialize, Deserialize)]
#[syscall(num = "Read")]
pub struct ReadRequest {
    pub fd: i64,
    pub nbytes: u64,
}

#[syscall_response]
#[derive(SysCall, SysCallResp, Debug, Serialize, Deserialize)]
#[syscall(num = "Read")]
pub struct ReadResp {
    pub data: SysData,
}

#[derive(SysCall, Debug, Serialize, Deserialize)]
#[syscall(num = "Write")]
pub struct WriteRequest {
    pub fd: i64,
    pub buf: SysData,
    pub nbytes: u64,
}

#[syscall_response]
#[derive(SysCall, SysCallResp, Debug, Serialize, Deserialize)]
#[syscall(num = "Write")]
pub struct WriteResp;
