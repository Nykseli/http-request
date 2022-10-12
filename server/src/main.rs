use actix_web::{
    get, App, HttpResponse, HttpServer,
};
use http_data;

#[get("/open")]
async fn open(/* data: web::Json<http_data::OpenRequest> */) -> HttpResponse {
    let open_resp = http_data::OpenResp::new(200, 3);
    HttpResponse::Ok()
        .json(open_resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(open)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
