#![allow(unused_imports)]

pub(crate) use std::collections::HashMap;
pub(crate) use std::fs;
pub(crate) use std::io::{self, Read, Write};
pub(crate) use std::process;
pub(crate) use std::thread::sleep;
pub(crate) use std::time::Duration;
pub(crate) use std::net::{TcpStream, ToSocketAddrs};
pub(crate) use url;

use super::USE_LOCAL;
#[derive(Debug)]
pub struct HttpResponse {
    status_line: String,
    headers: HashMap<String, String>,
    body: String,
}
impl HttpResponse {
    pub fn status_line(&self) -> &str {
        &self.status_line
    }
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    pub fn body(&self) -> &str {
        &self.body
    }
}
pub(crate) fn http_request(url: &str, method: &str, include_body: bool, timeout: Option<Duration>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Parse the URL
    let url = url::Url::parse(url)?;
    let host = url.host_str().ok_or("Invalid host")?;
    let path = url.path();
    let port = url.port().unwrap_or(80);

    // Connect to the server with timeout
    let addr = (host, port).to_socket_addrs()?.next().ok_or("Invalid address")?;
    let mut stream = match timeout {
        Some(duration) => TcpStream::connect_timeout(&addr, duration)?,
        None => TcpStream::connect(addr)?,
    };

    // Set read and write timeouts
    if let Some(duration) = timeout {
        stream.set_read_timeout(Some(duration))?;
        stream.set_write_timeout(Some(duration))?;
    }

    // Send the HTTP request
    let request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        method, path, host
    );
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Parse the response
    let mut parts = response.splitn(2, "\r\n\r\n");
    let header_section = parts.next().ok_or("No headers found")?;
    let body = parts.next().unwrap_or("").to_string();

    let mut headers = header_section.lines();
    let status_line = headers.next().ok_or("No status line found")?.to_string();

    let mut header_map = HashMap::new();
    for line in headers {
        if let Some((key, value)) = line.split_once(": ") {
            header_map.insert(key.to_string(), value.to_string());
        }
    }

    Ok(HttpResponse {
        status_line,
        headers: header_map,
        body: if include_body { body } else { String::new() },
    })
}
// Wrapper functions for specific use cases
pub fn http_request_with_head(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = http_request(url, "GET", false, None/* :Option<Duration> */)?;
    Ok(response.status_line)
}
pub(crate) fn http_get(url: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let response = http_request(url, "GET", true, None/* :Option<Duration> */)?;
    Ok(response)
}

pub(crate) fn http_get_head(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = http_request(url, "HEAD", false, None)?;
    Ok(response.headers.get("Content-Length").unwrap().to_string())
}

pub(crate) fn http_get_full(url: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    http_request(url, "GET", true, None/* Option<Duration> */)
}

pub(crate) fn check_internet_connection() {
    if USE_LOCAL {
        println!("Local mode enabled. Skipping internet connection check.");
        return;
    }
    print!("Checking internet connection... ");
    io::stdout().flush().unwrap();

    let url = "http://www.google.com";
    let timeout = Some(Duration::from_secs(5));

    match http_request(url, "HEAD", false, timeout) {
        Ok(response) => {
            if response.status_line.starts_with("HTTP/1.1 200") {
                println!("Connected!");
            } else {
                println!("\x1b[31mFailed to connect. Unexpected response: {}\x1b[0m", response.status_line);
                process::exit(1);
            }
        }
        Err(e) => {
            println!("\x1b[31mFailed to connect. No internet connection detected. Please check your connection.\x1b[0m");
            println!("Error details: {}", e);
            process::exit(1);
        }
    }
}

