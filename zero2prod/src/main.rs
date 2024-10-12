use zero2prod::startup::run;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use sqlx::postgres::PgPool;
use env_logger::Env;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Starting HTTP server at http://127.0.0.1:8080");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;    
    run(listener, connection_pool).await?.await?;
    Ok(())
}
