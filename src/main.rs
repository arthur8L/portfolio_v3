use portfolio_v3::{config, error::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::get_config().expect("Failed to read configuration.");
    println!("{}", config.database.database_name);
    Ok(())
}
