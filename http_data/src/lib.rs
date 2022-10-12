use serde::{Deserialize, Serialize};

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
