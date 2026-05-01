use tokio::net::{ TcpListener, TcpStream };
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    conn_accept().await
}

async fn conn_accept() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        //result -> socket : connected socket, addr : client address
        let (socket, addr) = listener.accept().await?;
        println!("{addr} connected");

        tokio::spawn(async move {
            conn_goal(socket).await;
        });
    }
}

async fn conn_goal(mut socket: TcpStream) {
    //max bytes = 1024
    let mut buf = [0u8; 1024];
    let n = match socket.read(&mut buf).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from socket {e}");
            return;
        }
    };

    // change byte to utf8
    let request = String::from_utf8_lossy(&buf[..n]);
    
    let host_addr = request
        .lines()
        .find(|line| line.starts_with("Host:"))
        .map(|line| line["Host: ".len()..].trim())
        .unwrap_or("unknown");
    
    if host_addr == "unknown" {
        eprintln!("Failed to find the host header");
        return;
    }

    let target = if host_addr.contains(':') {
        host_addr.to_string()
    } else {
        format!("{}:80", host_addr)
    };

    if target.contains("127.0.0.1:8080") || target.contains("localhost:8080") {
        eprintln!("Connection is being terminated due to a loop detection");
        return;
    }

    let mut dst_server = match TcpStream::connect(&target).await {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to connect to the target server {e}");
            return;
        }
    };
    
    if let Err(e) = dst_server.write_all(&buf[..n]).await {
        eprintln!("Failed to write data to the target server: {e}");
        return;
    }

    match tokio::io::copy_bidirectional(&mut socket, &mut dst_server).await {
        Ok((client_bytes, server_bytes)) => {
            println!("Transfer completed: Client({} bytes) <-> Server({} bytes)", client_bytes, server_bytes);
        }
        Err(e) => {
            eprintln!("Network Error: {}", e);
        }
    }
}
