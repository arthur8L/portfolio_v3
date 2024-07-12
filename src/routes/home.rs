use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok().body("hello")
}
