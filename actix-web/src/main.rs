use actix_web::{get, App, HttpServer, HttpRequest, middleware, HttpResponse, web};
use actix_files::NamedFile;
use std::path::PathBuf;
use std::io::Error;
use actix_web::http::ContentEncoding;
use serde::Deserialize;
use std::fs;

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let mut path: PathBuf = PathBuf::from(r"public\static");
    let file: PathBuf = req.match_info().query("filename").parse().unwrap();
    path.push(file);
    Ok(NamedFile::open(path)?)
}

#[derive(Deserialize)]
struct Operands {
    pub a: i64,
    pub b: i64
}

/*
#[get("/dynamic")]
async fn dynamic(operands: web::Query<Operands>) -> HttpResponse {
    let mut data = fs::read_to_string(r"public\test.html")
        .expect("File not found!");
    data = data.replace("{{}}", &format!("{}", operands.a + operands.b));
    return HttpResponse::Ok().body(data);
}
*/

#[get("/dynamic")]
async fn dynamic(operands: web::Query<Operands>) -> HttpResponse {
    return HttpResponse::Ok().body(format!("The result of a + b is: {}", operands.a + operands.b));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new().wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(dynamic)
            .service(index))
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
        .await
}