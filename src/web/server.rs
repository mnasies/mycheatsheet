//! Localhost HTTP server using tiny_http with Host validation,
//! idle shutdown timer, and secure path handling.

use crate::notes::model::NotesDatabase;
use crate::search::index::SearchIndex;
use crate::web::routes::{
    handle_api_note_part, handle_api_search, render_index_html, render_lang_html, APP_JS, STYLE_CSS,
};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub const IDLE_TIMEOUT_DURATION: Duration = Duration::from_secs(15 * 60); // 15 minutes

/// Server configuration and lifecycle controller.
pub struct WebServer {
    server: Arc<Server>,
    db: Arc<NotesDatabase>,
    index: Arc<SearchIndex>,
    bound_addr: SocketAddr,
    shutdown_flag: Arc<AtomicBool>,
}

impl WebServer {
    /// Bind to 127.0.0.1 on the specified port (or a fallback free port).
    pub fn bind(
        db: Arc<NotesDatabase>,
        index: Arc<SearchIndex>,
        preferred_port: u16,
        shutdown_flag: Option<Arc<AtomicBool>>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let bind_target = format!("127.0.0.1:{}", preferred_port);
        let server = match Server::http(&bind_target) {
            Ok(s) => s,
            Err(_) => {
                // If preferred port is busy, fallback to an ephemeral port
                Server::http("127.0.0.1:0")?
            }
        };

        let bound_addr = server
            .server_addr()
            .to_ip()
            .ok_or("Failed to resolve server IP")?;
        let shutdown_flag = shutdown_flag.unwrap_or_else(|| Arc::new(AtomicBool::new(false)));

        Ok(Self {
            server: Arc::new(server),
            db,
            index,
            bound_addr,
            shutdown_flag,
        })
    }

    pub fn bound_addr(&self) -> SocketAddr {
        self.bound_addr
    }

    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.bound_addr.port())
    }

    pub fn shutdown_flag(&self) -> Arc<AtomicBool> {
        self.shutdown_flag.clone()
    }

    /// Run the server loop. Shuts down after 15 minutes of inactivity or when shutdown_flag is set.
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut last_activity = Instant::now();

        while !self.shutdown_flag.load(Ordering::Relaxed) {
            // Check for idle shutdown
            if last_activity.elapsed() >= IDLE_TIMEOUT_DURATION {
                break;
            }

            // Non-blocking wait for next request with 1-second timeout
            match self.server.recv_timeout(Duration::from_secs(1)) {
                Ok(Some(request)) => {
                    last_activity = Instant::now();
                    self.handle_request(request);
                }
                Ok(None) => {
                    // Timeout with no request; continue loop to inspect idle and shutdown flags
                }
                Err(err) => {
                    eprintln!("HTTP server receive error: {}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_request(&self, request: tiny_http::Request) {
        // 1. Validate Host header against DNS rebinding
        if !validate_host_header(&request) {
            let _ = request.respond(
                Response::from_string("Forbidden: Invalid Host header")
                    .with_status_code(StatusCode(403)),
            );
            return;
        }

        let raw_url = request.url().to_string();

        // 2. Prevent path traversal attacks
        if raw_url.contains("..") || raw_url.to_lowercase().contains("%2e%2e") {
            let _ = request.respond(
                Response::from_string("Bad Request: Path traversal prohibited")
                    .with_status_code(StatusCode(400)),
            );
            return;
        }

        let (path, query_str) = split_url(&raw_url);
        let method = request.method().clone();

        match (&method, path.as_str()) {
            (&Method::Get, "/") => {
                let html = render_index_html(&self.db);
                self.respond_html(request, html);
            }
            (&Method::Get, path) if path.starts_with("/lang/") => {
                let lang_id = path.trim_start_matches("/lang/").trim_end_matches('/');
                match render_lang_html(&self.db, lang_id) {
                    Some(html) => self.respond_html(request, html),
                    None => self.respond_not_found(request),
                }
            }
            (&Method::Get, "/api/search") => {
                let params = parse_query_params(&query_str);
                let q = params.get("q").cloned().unwrap_or_default();
                let lang_filter = params.get("lang").map(|s| s.as_str());
                let json = handle_api_search(&self.index, &q, lang_filter);
                self.respond_json(request, json);
            }
            (&Method::Get, path) if path.starts_with("/api/notes/") => {
                // Format: /api/notes/:lang/:part
                let sub = path.trim_start_matches("/api/notes/");
                let parts: Vec<&str> = sub.split('/').collect();
                if parts.len() == 2 {
                    if let Ok(part_num) = parts[1].parse::<u32>() {
                        if let Some(json) = handle_api_note_part(&self.db, parts[0], part_num) {
                            self.respond_json(request, json);
                            return;
                        }
                    }
                }
                self.respond_not_found(request);
            }
            (&Method::Post, "/api/keepalive") => {
                self.respond_json(request, r#"{"status":"ok"}"#.to_string());
            }
            (&Method::Get, "/health") => {
                let body = format!(
                    r#"{{"app":"mycheatsheet","version":"{}"}}"#,
                    env!("CARGO_PKG_VERSION")
                );
                self.respond_json(request, body);
            }
            (&Method::Get, "/assets/style.css") => {
                let header =
                    Header::from_bytes(&b"Content-Type"[..], &b"text/css; charset=utf-8"[..])
                        .unwrap();
                let _ = request.respond(Response::from_string(STYLE_CSS).with_header(header));
            }
            (&Method::Get, "/assets/app.js") => {
                let header = Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"application/javascript; charset=utf-8"[..],
                )
                .unwrap();
                let _ = request.respond(Response::from_string(APP_JS).with_header(header));
            }
            _ => {
                self.respond_not_found(request);
            }
        }
    }

    fn respond_html(&self, request: tiny_http::Request, html: String) {
        let header =
            Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
        let _ = request.respond(Response::from_string(html).with_header(header));
    }

    fn respond_json(&self, request: tiny_http::Request, json: String) {
        let header = Header::from_bytes(
            &b"Content-Type"[..],
            &b"application/json; charset=utf-8"[..],
        )
        .unwrap();
        let _ = request.respond(Response::from_string(json).with_header(header));
    }

    fn respond_not_found(&self, request: tiny_http::Request) {
        let header =
            Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
        let body = "<!DOCTYPE html><html><head><title>404 Not Found</title></head>\
                    <body style=\"font-family: serif; text-align: center; padding: 4rem;\">\
                    <h1>404 Not Found</h1><p>The requested note or resource does not exist.</p>\
                    <a href=\"/\">&larr; Return to Home</a></body></html>";
        let _ = request.respond(
            Response::from_string(body)
                .with_status_code(StatusCode(404))
                .with_header(header),
        );
    }
}

/// Validates Host header against localhost or 127.0.0.1.
pub fn validate_host_header(request: &tiny_http::Request) -> bool {
    let host_header = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Host"))
        .map(|h| h.value.as_str());

    match host_header {
        Some(host) => {
            let h = host.to_lowercase();
            h == "localhost"
                || h == "127.0.0.1"
                || h.starts_with("localhost:")
                || h.starts_with("127.0.0.1:")
        }
        None => false,
    }
}

fn split_url(url: &str) -> (String, String) {
    if let Some(pos) = url.find('?') {
        (url[..pos].to_string(), url[pos + 1..].to_string())
    } else {
        (url.to_string(), String::new())
    }
}

fn parse_query_params(query: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if query.is_empty() {
        return map;
    }

    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let Some(key) = parts.next() {
            let val = parts.next().unwrap_or("");
            map.insert(url_decode(key), url_decode(val));
        }
    }
    map
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            let h1 = bytes.next();
            let h2 = bytes.next();
            if let (Some(h1), Some(h2)) = (h1, h2) {
                if let Ok(hex_str) = std::str::from_utf8(&[h1, h2]) {
                    if let Ok(byte_val) = u8::from_str_radix(hex_str, 16) {
                        result.push(byte_val as char);
                        continue;
                    }
                }
            }
        } else if b == b'+' {
            result.push(' ');
        } else {
            result.push(b as char);
        }
    }
    result
}
