use portfolio_v3::{app::Application, config, error::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::get_config().expect("Failed to read configuration.");
    let port = config.application.port;
    let app = Application::build(config).await?;
    println!("Listening on port: {}", port);
    let _ = app.run_until_stop().await;
    Ok(())
}
