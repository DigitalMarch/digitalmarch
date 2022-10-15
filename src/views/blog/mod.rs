mod blog;

use actix_web::web;
use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

lazy_static! {
    pub static ref POSTS: HashMap<String, String> = {
        let mut posts: HashMap<String, String> = HashMap::new();
        let path = PathBuf::from_str("./static/blog/").unwrap();

        for post_file in path.read_dir().unwrap().flatten() {
            let post_content = fs::read_to_string(post_file.path()).unwrap();

            let post_short: String = post_file
                .file_name()
                .into_string()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();

            posts.insert(post_short, post_content);
        }
        posts
    };
}

pub fn blog_factory(app: &mut web::ServiceConfig) {
    debug!("Loading blog factory");

    app.route("/blog/{post}", web::get().to(blog::blog));
}
