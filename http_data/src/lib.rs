use base64;
use serde::{Deserialize, Serialize};

// pub type SysData = Option<String>;

pub fn encode_buffer(buf: &Vec<u8>, length: i64) -> String {
    base64::encode(&buf[0..length as usize])
}

pub fn decode_buffer(buf: &str) -> Vec<u8> {
    base64::decode(buf).unwrap_or(vec![])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysCallResp<T> {
    pub status: u64,
    pub response: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenRequest {
    pub path: String,
    pub oflag: u64,
    pub mode: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenResp {
    pub fd: i64,
}

impl OpenResp {
    pub fn new(status: u64, fd: i64) -> SysCallResp<Self> {
        SysCallResp::<Self> {
            status: status,
            response: Self { fd: fd },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    pub fd: i64,
    pub nbytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResp {
    pub read_length: i64,
    pub data: Option<String>,
}

impl ReadResp {
    pub fn new(status: u64, read_length: i64, data: Option<String>) -> SysCallResp<Self> {
        SysCallResp::<Self> {
            status: status,
            response: Self {
                read_length: read_length,
                data: data,
            },
        }
    }
}
