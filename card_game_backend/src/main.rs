use actix_web::{App, HttpServer};
use actix_web_lab::web::spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
