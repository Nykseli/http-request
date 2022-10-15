use http_data::{serde, Serialize, SysCall, SysCallResp};
use reqwest;

pub fn unchecked_request<Resp, Req>(endpoint: &str, request: &Req) -> Resp
where
    Req: SysCall + Serialize,
    Resp: SysCall + SysCallResp + serde::de::DeserializeOwned,
{
    let client = reqwest::blocking::Client::new();
    client
        .post(&format!("http://localhost:8081/{}", endpoint))
        .json(request)
        .send()
        .unwrap()
        .json::<Resp>()
        .unwrap()
}
