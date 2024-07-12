use std::net::TcpListener;

use actix_web::dev::Server;
use sqlx::PgPool;

use crate::{config::Settings, error::AppError};

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, AppError> {
        todo!()
    }
}

pub async fn generate_server(listener: TcpListener, db_pool: PgPool) -> Result<Server, AppError> {
    todo!()
}
