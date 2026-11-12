//! Tiny loopback HTTP stub for /v1/health and /v1/bank/payments (M13).
//! Not a production server — std::net only, for local smoke tests.

use crate::actions::KnownAction;
use crate::local_api::{blocked_unsupported, health};
use crate::permission::{PermissionGate, PermissionRequest, ResourceRef, Scope};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct LocalApiServer {
    pub addr: SocketAddr,
    shutdown: Arc<AtomicBool>,
}

impl LocalApiServer {
    pub fn spawn_ephemeral() -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        listener.set_nonblocking(true)?;
        let addr = listener.local_addr()?;
        let shutdown = Arc::new(AtomicBool::new(false));
        let flag = shutdown.clone();
        thread::spawn(move || loop {
            if flag.load(Ordering::SeqCst) {
                break;
            }
            match listener.accept() {
                Ok((stream, _)) => {
                    let _ = handle_client(stream);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        });
        // give listener a moment
        thread::sleep(Duration::from_millis(20));
        Ok(Self { addr, shutdown })
    }

    pub fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub fn shutdown(self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf)?;
    let req = String::from_utf8_lossy(&buf[..n]);
    let line = req.lines().next().unwrap_or("");
    let (status, body) = if line.starts_with("GET /v1/health") {
        let h = health();
        (
            "200 OK",
            serde_json::to_string(&h).unwrap_or_else(|_| "{}".into()),
        )
    } else if line.starts_with("POST /v1/bank/payments") {
        let gate = PermissionGate::default();
        let req = PermissionRequest::for_known(
            KnownAction::BankPaymentCreate,
            ResourceRef {
                kind: "bank".into(),
                id: None,
            },
            Scope {
                label: "http".into(),
            },
            "{}",
            "http-stub",
        );
        let _ = gate.authorize(&req); // expect deny
        let err = blocked_unsupported("bank.payment.create is not implemented (REQ-S-004)");
        (
            "403 Forbidden",
            serde_json::to_string(&err).unwrap_or_else(|_| "{}".into()),
        )
    } else {
        ("404 Not Found", "{\"error\":\"not_found\"}".into())
    };
    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(resp.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn http_get(url_path: &str, addr: SocketAddr) -> (u16, String) {
        let mut stream = TcpStream::connect(addr).unwrap();
        let req =
            format!("GET {url_path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
        stream.write_all(req.as_bytes()).unwrap();
        let mut out = String::new();
        stream.read_to_string(&mut out).unwrap();
        let status = out
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        (status, out)
    }

    fn http_post(url_path: &str, addr: SocketAddr) -> u16 {
        let mut stream = TcpStream::connect(addr).unwrap();
        let req = format!(
            "POST {url_path} HTTP/1.1\r\nHost: localhost\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}"
        );
        stream.write_all(req.as_bytes()).unwrap();
        let mut out = String::new();
        stream.read_to_string(&mut out).unwrap();
        out.lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    #[test]
    fn health_ok_and_payments_forbidden() {
        let server = LocalApiServer::spawn_ephemeral().unwrap();
        let (code, body) = http_get("/v1/health", server.addr);
        assert_eq!(code, 200);
        assert!(body.contains("Belegwerk"));
        assert_eq!(http_post("/v1/bank/payments", server.addr), 403);
        server.shutdown();
    }
}
