use sqlx::PgPool;
use std::net::TcpListener;
use zero_trust_auth::configuration::get_configuration;
use zero_trust_auth::startup::run;
use zero_trust_auth::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero_trust_auth".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind address");

    println!("Listening on: {}", address);
    run(listener, connection_pool)?.await
}
