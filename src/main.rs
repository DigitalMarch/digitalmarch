mod logger;
mod views;


use actix_web::*;
use log::{debug, info, LevelFilter};

const LOG_LEVEL: LevelFilter = LevelFilter::Info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    logger::logger::init(LOG_LEVEL);
    debug!("Logger Started");

    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;

    Ok(())
}
