use portfolio_v3::{
    app::Application,
    config::{self, DatabaseSettings},
    error::AppError,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::runtime::Handle;
use uuid::Uuid;

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub pool: PgPool,
    pub db_config: DatabaseSettings,
}
impl TestApp {
    async fn clean_up_db(&self) -> Result<(), AppError> {
        PgConnection::connect_with(&self.db_config.without_db())
            .await
            .map_err(AppError::new)?
            .execute(
                format!(
                    r#"DROP DATABASE IF EXISTS {};"#,
                    self.db_config.database_name
                )
                .as_str(),
            )
            .await
            .map_err(AppError::new)?;
        Ok(())
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        let handle = Handle::current();
        handle
            .block_on(self.clean_up_db())
            .expect("Failed to cleanup db");
    }
}

pub async fn spawn_app() -> TestApp {
    let config = {
        let mut c = config::get_config().expect("Failed to get config");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };
    let app = Application::build(config.clone())
        .await
        .expect("Failed to build application");
    TestApp {
        port: app.get_port(),
        pool: configure_test_database(&config.database).await,
        address: format!("http://127.0.0.1:{}", app.get_port()),
        db_config: config.database,
    }
}

async fn configure_test_database(config: &DatabaseSettings) -> PgPool {
    // Create empty conenction to genereate database
    PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to postgres")
        .execute(format!(r#"CREATE DATABSE "{}""#, config.database_name).as_str())
        .await
        .expect("Failed to send query to create database");
    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    pool
}
