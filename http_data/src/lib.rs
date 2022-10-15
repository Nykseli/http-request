use base64;
pub use serde::{self, Deserialize, Serialize};
use syscall_derive::{syscall_response, SysCall, SysCallResp};

pub type SysData = Option<String>;

pub fn encode_buffer(buf: &Vec<u8>, length: i64) -> String {
    base64::encode(&buf[0..length as usize])
}

pub fn decode_buffer(buf: &str) -> Vec<u8> {
    base64::decode(buf).unwrap_or(vec![])
}

// TODO: SysCalls always return a value (rax) so make it user definable integer,
// something like SysVal::Signed, sysval::unsigned

#[derive(Debug)]
#[repr(u64)]
pub enum SysCallNum {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    // NOTE: NoOp is non standard syscall number SHOULD invoke ENOSYS (not implemented)
    NoOp = u64::MAX,
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
