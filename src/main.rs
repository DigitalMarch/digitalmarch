mod logger;
mod views;

use actix_web::*;
use log::{info, LevelFilter};

const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    logger::logger::init(LOG_LEVEL);
    info!("Logger Started");

    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
