use portfolio_v3::{
    app::Application,
    config::{self, DatabaseSettings},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub pool: PgPool,
    pub db_config: DatabaseSettings,
}
impl TestApp {
    pub async fn clean_up(&self) {
        // self.pool.close().await;
        let mut conn = PgConnection::connect_with(&self.db_config.without_db())
            .await
            .expect("Failed createing pg connection for clean_up ");

        conn.execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                AND pid <> pg_backend_pid()"#,
                self.db_config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to terminate current connection to test db");

        conn.execute(
            format!(
                r#"DROP DATABASE IF EXISTS "{}";"#,
                self.db_config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed drop database created for test");
        println!(
            "Database with id = {} dropped successfully",
            self.db_config.database_name
        );
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        //with sqlx::test we don't need this but keeping it for now
        std::thread::scope(|s| {
            s.spawn(|| {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("FAiled getting tokio runtime");
                rt.block_on(self.clean_up());
            });
        })
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
        .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
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
