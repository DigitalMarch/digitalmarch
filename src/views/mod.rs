mod index;
mod about;
mod projects;

use actix_web::web;
use log::debug;


pub fn views_factory(app: &mut web::ServiceConfig) {
    debug!("Loading views factory");

    // factory for index, about, projects page
    app.route("/", web::get()
        .to(index::index))
        .route("/about", web::get()
            .to(about::about))
        .route("/projects", web::get()
            .to(projects::projects));
}