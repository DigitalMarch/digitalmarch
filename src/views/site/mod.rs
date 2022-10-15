use actix_web::web;
use actix_web::web::ServiceConfig;
use log::debug;

mod about;
mod index;
mod projects;
mod publickey;

pub fn site_factory(app: &mut ServiceConfig) {
    debug!("Loading site factory");

    app.route("/", web::get().to(index::index))
        .route("/about", web::get().to(about::about))
        .route("/projects", web::get().to(projects::projects))
        .route("/publickey", web::get().to(publickey::publickey));
}
