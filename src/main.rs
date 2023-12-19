use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::io::Error;

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            if let Err(e) = handle_client(stream) {
                eprintln!("Failed to handle client: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");

    if request_line.starts_with("GET /MySecretPath") {
        let ip = stream.peer_addr()?.ip().to_string();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            ip.len(),
            ip
        );
        stream.write_all(response.as_bytes())?;
    } else {
        // If the request is not the specific GET request, reject the connection
        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes())?;
    }

    stream.flush()?;
    Ok(())
}
