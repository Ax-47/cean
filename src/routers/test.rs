use actix_web::{get, HttpResponse};
#[get("/")]
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}
