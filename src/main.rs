use std::io::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("Failed to handle client: {}", e);
            }
        });
    }
}

async fn handle_client(mut stream: tokio::net::TcpStream) -> Result<(), Error> {
    match timeout(Duration::from_secs(1), process_request(&mut stream)).await {
        Ok(result) => result,
        Err(_) => {
            // If the timeout occurs, we just drop the connection by doing nothing
            Ok(())
        }
    }
}

async fn process_request(stream: &mut tokio::net::TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    
    if bytes_read == 0 {
        // Connection was closed by the client
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or("");
    
    if request_line.starts_with("GET /MySecretPath ") {
        let ip = stream.peer_addr()?.ip().to_string();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            ip.len(),
            ip
        );
        stream.write_all(response.as_bytes()).await?;
    } else {
        // For unauthorized requests, we do nothing and let the function end
        // This will cause the connection to be dropped without any response
    }
    
    Ok(())
}