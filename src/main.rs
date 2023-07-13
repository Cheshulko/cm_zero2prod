use std::net::TcpListener;

use cm_zero2prod::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    println!("Port {}", listener.local_addr().unwrap().port());
    run(listener, connection_pool)?.await
}
