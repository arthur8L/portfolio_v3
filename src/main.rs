use portfolio_v3::{
    app::Application,
    config,
    error::AppError,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_subscriber(get_subscriber("portfolio_v3", "info", std::io::stdout));
    let config = config::get_config().expect("Failed to read configuration.");
    let port = config.application.port;
    let app = Application::build(config).await?;
    println!("Listening on port: {}", port);
    let _ = app.run_until_stop().await;
    Ok(())
}
