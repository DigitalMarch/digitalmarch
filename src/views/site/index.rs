use std::fs;
use actix_web::{HttpResponse, Result};
use crate::views::blog::{BlogPost, POSTS};

pub async fn index() -> Result<HttpResponse> {
    let index_file: String = match fs::read_to_string("./static/index.html") {
        Ok(i) => i,
        Err(_) => return Ok(HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("Unable to open file, sorry about that."))
    };

    let mut post_list = String::new();

    for (k, v) in POSTS.iter().clone() {
        post_list += &*format!(
            "<div class=\"article\">\
                <span class=\"date\">{}</span>\
                <a href=\"/blog/{}\">{}</a>\
            </div>\n", v.date, k, v.title)
    }

    let formatted_file = index_file.replace("{{ARTICLES}}", &*post_list);


    return Ok(HttpResponse::Ok().content_type("text/html").body(formatted_file))
}