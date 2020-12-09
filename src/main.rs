use std::net::TcpListener;
use zero2prod::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(address)?.await
}
