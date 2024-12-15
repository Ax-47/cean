use crate::routers::send::send_message;
use crate::{config::configuration::Config, routers::test};
use actix_web::{web, App, HttpServer};
use serenity::http::Http;
use std::sync::Arc;
pub async fn start_http_server(con: Config) -> std::io::Result<()> {
    let token = Arc::new(Http::new(con.discord_token.as_str()));
    if con.debug {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(token.clone()))
            .app_data(web::Data::new(con.auth.clone()))
            .service(send_message)
            .service(test::hello_world)
    })
    .bind((con.ip.as_str(), con.port))?
    .workers(con.worker as usize)
    .run()
    .await
}
