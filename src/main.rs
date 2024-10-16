use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("There should be an available config file.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("We should be able to bind a random port.˝");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("We should be able to connect to the database.");
    run(listener, connection_pool)?.await

}
