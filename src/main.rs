mod config;
mod model;
mod routers;
mod setup;
use crate::config::configuration::Config;
use crate::setup::http;
use crate::setup::init;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let con = Config::from_env();
    init::init(con.ip.clone(), con.port, con.debug, con.worker);
    http::start_http_server(con).await
}
