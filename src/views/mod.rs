mod blog;
mod site;

use actix_web::web;
use log::debug;


pub fn views_factory(app: &mut web::ServiceConfig) {
    debug!("Loading views factory");

    site::site_factory(app);
    blog::blog_factory(app);

    app.service(actix_files::Files::new("/js", "./static/js"))
        .service(actix_files::Files::new("/css", "./static/css"))
        .service(actix_files::Files::new("/images", "./static/images"));
}
