use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero_trust_auth::configuration::get_configuration;
use zero_trust_auth::startup::run;
use zero_trust_auth::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero_trust_auth".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(&address).expect("Failed to bind address");

    println!("Listening on: {}", address);
    run(listener, connection_pool)?.await
}
