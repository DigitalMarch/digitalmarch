use actix_web::web;

mod index;
mod about;
mod projects;

pub fn site_factory(app: &mut web::ServiceConfig) {
    app.route("/", web::get()
            .to(index::index))
        .route("/about", web::get()
            .to(about::about))
        .route("/projects", web::get()
            .to(projects::projects));
}