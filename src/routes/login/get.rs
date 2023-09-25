//! src/routes/login/get.rs
use actix_web::{HttpResponse, http::header::ContentType, HttpRequest};
use actix_web::cookie::{Cookie};


pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html: String = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };
    let mut response = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(error_html);
    response
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();
    response

}