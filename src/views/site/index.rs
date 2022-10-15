use actix_web::{HttpResponse, Result};
use std::fs;

pub async fn index() -> Result<HttpResponse> {
    let index_file: String = match fs::read_to_string("./static/index.html") {
        Ok(i) => i,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Unable to open file, sorry about that."))
        }
    };

    return Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(index_file));
}
