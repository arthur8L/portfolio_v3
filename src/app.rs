use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    config::{DatabaseSettings, Settings},
    error::AppError,
    routes::{health_check::health_check, home::home},
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, AppError> {
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.application.host, config.application.port
        ))
        .map_err(AppError::new)?;
        let db_pool = get_connection_pool(&config.database);
        Ok(Application {
            port: listener.local_addr().unwrap().port(),
            server: run_server(listener, db_pool).await.map_err(AppError::new)?,
        })
    }
    pub async fn run_until_stop(self) -> Result<(), std::io::Error> {
        self.server.await
    }
    pub fn get_port(&self) -> u16 {
        self.port
    }
}

pub async fn run_server(listener: TcpListener, db_pool: PgPool) -> Result<Server, AppError> {
    let db_conn = web::Data::new(db_pool);
    let app = move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(home))
            .app_data(db_conn.clone())
    };
    let server = HttpServer::new(app)
        .listen(listener)
        .map_err(AppError::new)?
        .run();
    Ok(server)
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
