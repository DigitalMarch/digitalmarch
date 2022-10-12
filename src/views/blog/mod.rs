use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;
use actix_web::{HttpRequest, HttpResponse, web, Result};
use chrono::{Date, DateTime, NaiveDate};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use actix_web::web::Path;
use serde::Deserialize;
use actix_files::NamedFile;
use log::debug;

pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub path: PathBuf
}

lazy_static! {
    pub static ref POSTS: HashMap<String, BlogPost> = {
        let mut posts: HashMap<String, BlogPost> = HashMap::new();
        let path = PathBuf::from_str("./static/blog/").unwrap();

        for post_file in path.read_dir().unwrap().flatten() {
            let f = post_file.file_name();
            let p = f.into_string().unwrap();
            let plist: Vec<&str> = p.split('_').collect();
            let pdatesplit: Vec<&str> = plist[1].split('.').collect();

            let pdate = NaiveDate::parse_from_str(pdatesplit[0], "%Y-%m-%d").unwrap();
            let ptitle = plist[0].replace('-', " ");
            let pshort = plist[0].replace('-', "");

            debug!("Loaded blog post:\n\
                short: {}\n\
                title: {}\n\
                date: {}\n\
                path: {}\n", &pshort, &ptitle, &pdate, &post_file.path().display());

            let post = BlogPost{
                title: ptitle,
                date: pdate.format("%A, %-d %B, %C%y").to_string(),
                path: post_file.path()
            };
            posts.insert(pshort, post);
        }
        posts
    };
}

pub fn blog_factory(app: &mut web::ServiceConfig) {
    debug!("Loading blog factory");

    app.route("/blog/{post}", web::get().to(blog));
}

pub async fn blog(req: HttpRequest) -> Result<HttpResponse> {
    let path = match req.match_info().get("post") {
        Some(p) => p,
        None => return Ok(HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Post not found"))
    };

    let post = match POSTS.get(path) {
        Some(p) => p,
        None => return Ok(HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Post not found"))
    };

    let post_data = match fs::read(&post.path) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("Unable to read file, sorry about that."))
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(post_data))

}