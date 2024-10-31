use actix_cors::Cors;
use actix_web::{App, HttpServer};
use poke_api::handler;
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set a subscriber");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .configure(handler::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
