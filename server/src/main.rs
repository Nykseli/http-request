use actix_web::{post, web, App, HttpResponse, HttpServer};
use http_data;
use std::arch::asm;
use std::ffi::CString;

#[post("/open")]
async fn open(data: web::Json<http_data::OpenRequest>) -> HttpResponse {
    println!("{:#?}", data);
    let path = CString::new(data.path.as_str()).unwrap();
    let mut fp: i64;
    unsafe {
        let pptr = path.as_ptr();
        // TODO: Enumerate syscalls
        asm!(
            "mov rax, 2",
            "syscall",
            in("rdi") pptr,
            in("rsi") data.oflag,
            in("rdx") data.mode,
            lateout("rax") fp
        );
    }

    let open_resp = http_data::OpenResp::new(200, fp);
    HttpResponse::Ok().json(open_resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(open))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
