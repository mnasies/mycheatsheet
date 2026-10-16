//! Single-instance detection and health check ping.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

/// Probes the specified localhost port to check if an existing `mycheatsheet`
/// instance is running.
///
/// Returns `Some(url)` if the healthcheck returns a valid application signature,
/// otherwise returns `None`.
pub fn check_existing_instance(port: u16) -> Option<String> {
    let addr = format!("127.0.0.1:{}", port);
    let socket_addr = addr.parse().ok()?;

    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(300)).ok()?;
    stream
        .set_read_timeout(Some(Duration::from_millis(400)))
        .ok()?;
    stream
        .set_write_timeout(Some(Duration::from_millis(300)))
        .ok()?;

    let request = format!(
        "GET /health HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nConnection: close\r\n\r\n",
        port
    );
    stream.write_all(request.as_bytes()).ok()?;

    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;

    if response.contains("\"app\":\"mycheatsheet\"") {
        Some(format!("http://127.0.0.1:{}", port))
    } else {
        None
    }
}
