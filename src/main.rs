use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Result};

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
