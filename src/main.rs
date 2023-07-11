use std::net::TcpListener;

use cm_zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    println!("Port {}", listener.local_addr().unwrap().port());
    run(listener)?.await
}
