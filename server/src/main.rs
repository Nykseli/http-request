use actix_web::{post, web, App, HttpResponse, HttpServer};
use http_data;
use std::arch::asm;
use std::ffi::CString;

#[post("/open")]
async fn open(data: web::Json<http_data::OpenRequest>) -> HttpResponse {
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

#[post("/read")]
async fn read(data: web::Json<http_data::ReadRequest>) -> HttpResponse {
    let mut data_buf: Vec<u8> = vec![0; data.nbytes as usize];
    let mut read_length: i64;

    unsafe {
        let dptr = data_buf.as_mut_ptr();
        // TODO: Enumerate syscalls
        asm!(
            "mov rax, 0",
            "syscall",
            in("rdi") data.fd,
            in("rsi") dptr,
            in("rdx") data.nbytes,
            lateout("rax") read_length
        );
    }

    let resp_data = if read_length > 0 {
        http_data::encode_buffer(&data_buf, read_length)
    } else {
        None
    };

    let open_resp = http_data::ReadResp::new(200, read_length, resp_data);
    HttpResponse::Ok().json(open_resp)
}

#[post("/close")]
async fn close(data: web::Json<http_data::CloseRequest>) -> HttpResponse {
    let mut ret: i64;

    unsafe {
        // TODO: Enumerate syscalls
        asm!(
            "mov rax, 3",
            "syscall",
            in("rdi") data.fd,
            lateout("rax") ret
        );
    }

    let open_resp = http_data::CloseResp::new(200, ret);
    HttpResponse::Ok().json(open_resp)
}

#[post("/write")]
async fn write(data: web::Json<http_data::WriteRequest>) -> HttpResponse {
    let mut ret: i64;
    let mut write_data = http_data::decode_buffer(&data.buf);
    unsafe {
        let wptr = write_data.as_mut_ptr();
        asm!(
            "syscall",
            in("rax") http_data::SysCallNum::Write as u64,
            in("rdi") data.fd,
            in("rsi") wptr,
            in("rdx") data.nbytes,
            lateout("rax") ret
        );
    }

    let write_resp = http_data::WriteResp::new(200, ret);
    HttpResponse::Ok().json(write_resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(open)
            .service(read)
            .service(close)
            .service(write)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
