use crate::app::{echo, index, index_x, turnaround};
use actix_web::{web, App, HttpServer};
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(app::AppState {
                app_name: String::from("David's First Actix Web Experiment"),
            }))
            .service(index)
            .service(index_x)
            .service(web::resource("/turnaround").route(web::post().to(turnaround)))
            .service(web::resource("/echo").route(web::post().to(echo)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
