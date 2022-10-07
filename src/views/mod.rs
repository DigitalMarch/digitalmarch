mod site;


use actix_web::web;
use crate::views::site::site_factory;

pub fn views_factory(app: &mut web::ServiceConfig) {
    // factory for index, about, projects page
    site_factory(app);
}