mod api;
mod model;
mod repository;

use std::env::{self, set_var};

use actix_files::Files;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::api_config;
use repository::inmem::Inmem;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "debug");
    set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db_file_name = env::var("DB_FILE_NAME").unwrap_or("".into());
    let db = Inmem::new(db_file_name.into());
    let app_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .configure(api_config)
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
