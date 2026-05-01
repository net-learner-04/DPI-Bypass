mod proxy;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    proxy::conn_accept().await
}
