use mycheatsheet::launcher::check_existing_instance;
use mycheatsheet::notes::load_notes_dir;
use mycheatsheet::search::SearchIndex;
use mycheatsheet::web::server::WebServer;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn raw_http_request(port: u16, method: &str, path: &str, host: Option<&str>) -> (u16, String) {
    let host_header = host.unwrap_or("127.0.0.1");
    let req = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        method, path, host_header
    );

    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
        .expect("Failed to connect to test server");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    stream.write_all(req.as_bytes()).unwrap();

    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);

    let status_code = if response.starts_with("HTTP/1.1 ") {
        response[9..12].parse::<u16>().unwrap_or(0)
    } else {
        0
    };

    (status_code, response)
}

#[test]
fn test_web_server_endpoints() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let notes_dir = manifest_dir.join("notes");
    let (db, _) = load_notes_dir(&notes_dir);
    let index = SearchIndex::from_database(&db);

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let server = Arc::new(
        WebServer::bind(
            Arc::new(db),
            Arc::new(index),
            0,
            Some(shutdown_flag.clone()),
        )
        .expect("Failed to bind"),
    );
    let port = server.bound_addr().port();

    let s = server.clone();
    let handle = thread::spawn(move || {
        let _ = s.run();
    });

    thread::sleep(Duration::from_millis(60));

    // 1. Health check & instance reuse signature
    let (code, res) = raw_http_request(port, "GET", "/health", None);
    assert_eq!(code, 200);
    assert!(res.contains("\"app\":\"mycheatsheet\""));

    let instance_url = check_existing_instance(port);
    assert_eq!(instance_url, Some(format!("http://127.0.0.1:{}", port)));

    // 2. Index page
    let (code, res) = raw_http_request(port, "GET", "/", None);
    assert_eq!(code, 200);
    assert!(res.contains("mycheatsheet"));
    assert!(res.contains("Editorial Edition"));
    assert!(res.contains("Programming Syntax Reference"));

    // 3. Language page
    let (code, res) = raw_http_request(port, "GET", "/lang/go", None);
    assert_eq!(code, 200);
    assert!(res.contains("Go Cheatsheet"));
    assert!(res.contains("Program structure &amp; entry point"));

    // Non-existent language returns 404
    let (code, _) = raw_http_request(port, "GET", "/lang/nonexistent", None);
    assert_eq!(code, 404);

    // 4. Search API
    let (code, res) = raw_http_request(port, "GET", "/api/search?q=package", None);
    assert_eq!(code, 200);
    assert!(res.contains("package"));

    // 5. Keepalive endpoint
    let (code, res) = raw_http_request(port, "POST", "/api/keepalive", None);
    assert_eq!(code, 200);
    assert!(res.contains("\"status\":\"ok\""));

    // 6. Security: Host header validation (reject malicious host)
    let (code, _) = raw_http_request(port, "GET", "/", Some("evil-site.com"));
    assert_eq!(code, 403);

    // 7. Security: Path traversal rejection
    let (code, _) = raw_http_request(port, "GET", "/lang/../../etc/passwd", None);
    assert_eq!(code, 400);

    // 8. Static assets
    let (code, res) = raw_http_request(port, "GET", "/assets/style.css", None);
    assert_eq!(code, 200);
    assert!(res.contains("--font-serif"));

    let (code, res) = raw_http_request(port, "GET", "/assets/app.js", None);
    assert_eq!(code, 200);
    assert!(res.contains("initSyntaxHighlighting"));

    // Shut down cleanly
    shutdown_flag.store(true, Ordering::Relaxed);
    let _ = handle.join();
}
