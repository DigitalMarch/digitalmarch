use crate::views::blog::POSTS;
use actix_web::{HttpRequest, HttpResponse, Result};

pub async fn blog(req: HttpRequest) -> Result<HttpResponse> {
    let path = match req.match_info().get("post") {
        Some(p) => p,
        None => {
            return Ok(HttpResponse::NotFound()
                .content_type("text/plain")
                .body("Post not found"))
        }
    };

    let post = match POSTS.get(path) {
        Some(t) => t,
        None => {
            return Ok(HttpResponse::NotFound()
                .content_type("text/plain")
                .body("Post not found"))
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(post.clone()))
}
