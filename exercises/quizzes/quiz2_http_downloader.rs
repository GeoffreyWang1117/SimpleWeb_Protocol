//! Quiz 2: HTTP File Downloader
//!
//! Build a complete HTTP/1.1 file downloader that:
//! - Parses URLs into components
//! - Constructs HTTP GET requests with proper headers
//! - Handles chunked transfer encoding
//! - Supports Content-Length based downloads
//! - Tracks download progress
//! - Handles redirects
//!
//! This quiz combines: URL parsing, TCP concepts, HTTP protocol, chunked encoding

use std::collections::HashMap;

/// Parsed URL components
#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Option<String>,
}

impl Url {
    /// Parse a URL string into components
    ///
    /// Supported formats:
    /// - http://example.com
    /// - http://example.com:8080
    /// - http://example.com/path
    /// - http://example.com/path?query=value
    /// - https://example.com (port 443)
    pub fn parse(url: &str) -> Result<Self, &'static str> {
        // TODO: Parse the URL into components
        // 1. Extract scheme (http or https)
        // 2. Extract host
        // 3. Extract port (default: 80 for http, 443 for https)
        // 4. Extract path (default: "/")
        // 5. Extract query string if present
        //
        // Hint: Use find() and split() methods on strings

        todo!("Parse URL into components")
    }

    /// Get the host:port string for the Host header
    pub fn host_header(&self) -> String {
        // TODO: Return "host:port" or just "host" if using default port
        // Default ports: 80 for http, 443 for https
        todo!("Generate Host header value")
    }

    /// Get the full request path (path + query)
    pub fn request_path(&self) -> String {
        // TODO: Return path with query string if present
        // e.g., "/path" or "/path?query=value"
        todo!("Generate request path")
    }
}

/// HTTP Response status
#[derive(Debug, Clone)]
pub struct HttpStatus {
    pub code: u16,
    pub reason: String,
}

impl HttpStatus {
    pub fn is_success(&self) -> bool {
        self.code >= 200 && self.code < 300
    }

    pub fn is_redirect(&self) -> bool {
        self.code >= 300 && self.code < 400
    }

    pub fn is_error(&self) -> bool {
        self.code >= 400
    }
}

/// HTTP Response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Get content length from headers
    pub fn content_length(&self) -> Option<usize> {
        // TODO: Parse Content-Length header if present
        // Header names should be case-insensitive
        todo!("Get content length")
    }

    /// Check if response uses chunked transfer encoding
    pub fn is_chunked(&self) -> bool {
        // TODO: Check Transfer-Encoding header for "chunked"
        todo!("Check if chunked")
    }

    /// Get redirect location if this is a redirect response
    pub fn redirect_location(&self) -> Option<&str> {
        // TODO: Return Location header value if this is a redirect
        todo!("Get redirect location")
    }

    /// Get content type
    pub fn content_type(&self) -> Option<&str> {
        // TODO: Return Content-Type header value
        todo!("Get content type")
    }
}

/// HTTP Request builder
pub struct HttpRequest {
    method: String,
    url: Url,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    /// Create a new GET request for a URL
    pub fn get(url: Url) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Host".to_string(), url.host_header());
        headers.insert("Connection".to_string(), "close".to_string());
        headers.insert("User-Agent".to_string(), "RustDownloader/1.0".to_string());

        Self {
            method: "GET".to_string(),
            url,
            headers,
        }
    }

    /// Add a header
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    /// Build the HTTP request as a byte string
    pub fn build(&self) -> Vec<u8> {
        // TODO: Build the complete HTTP/1.1 request
        // Format:
        // GET /path HTTP/1.1\r\n
        // Host: example.com\r\n
        // Header1: Value1\r\n
        // Header2: Value2\r\n
        // \r\n

        todo!("Build HTTP request")
    }
}

/// HTTP Response parser (streaming)
pub struct ResponseParser {
    buffer: Vec<u8>,
    headers_complete: bool,
    content_length: Option<usize>,
    is_chunked: bool,
    body_start: usize,
}

impl ResponseParser {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            headers_complete: false,
            content_length: None,
            is_chunked: false,
            body_start: 0,
        }
    }

    /// Feed data into the parser
    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Try to parse headers (returns true when headers are complete)
    pub fn try_parse_headers(&mut self) -> Result<bool, &'static str> {
        if self.headers_complete {
            return Ok(true);
        }

        // TODO: Look for "\r\n\r\n" to find end of headers
        // Parse status line and headers
        // Set headers_complete, content_length, is_chunked, body_start

        todo!("Parse HTTP headers")
    }

    /// Parse the status line "HTTP/1.1 200 OK"
    fn parse_status_line(line: &str) -> Result<HttpStatus, &'static str> {
        // TODO: Parse the HTTP status line
        // Format: "HTTP/1.x CODE REASON"
        // Return HttpStatus with code and reason

        todo!("Parse status line")
    }

    /// Parse a header line "Name: Value"
    fn parse_header_line(line: &str) -> Option<(String, String)> {
        // TODO: Parse a header line into (name, value)
        // Trim whitespace from value
        // Return None if invalid format

        todo!("Parse header line")
    }

    /// Check if the response body is complete
    pub fn is_body_complete(&self) -> bool {
        if !self.headers_complete {
            return false;
        }

        // TODO: Determine if body is complete based on:
        // 1. Content-Length: body_received >= content_length
        // 2. Chunked: received final chunk (0\r\n\r\n)
        // 3. Neither: wait for connection close

        todo!("Check if body complete")
    }

    /// Get the response body (handles chunked decoding)
    pub fn get_body(&self) -> Result<Vec<u8>, &'static str> {
        if !self.headers_complete {
            return Err("Headers not complete");
        }

        let raw_body = &self.buffer[self.body_start..];

        if self.is_chunked {
            // TODO: Decode chunked body
            // Format: SIZE\r\nDATA\r\nSIZE\r\nDATA\r\n0\r\n\r\n
            todo!("Decode chunked body")
        } else {
            Ok(raw_body.to_vec())
        }
    }

    /// Get parsed response
    pub fn into_response(self) -> Result<HttpResponse, &'static str> {
        // TODO: Assemble the final HttpResponse
        todo!("Build final response")
    }
}

/// Download progress tracker
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total_size: Option<usize>,
    pub downloaded: usize,
    pub start_time: std::time::Instant,
}

impl DownloadProgress {
    pub fn new(total_size: Option<usize>) -> Self {
        Self {
            total_size,
            downloaded: 0,
            start_time: std::time::Instant::now(),
        }
    }

    /// Update downloaded bytes
    pub fn update(&mut self, bytes: usize) {
        self.downloaded += bytes;
    }

    /// Get progress percentage (0.0 - 100.0)
    pub fn percentage(&self) -> Option<f64> {
        // TODO: Calculate percentage if total_size is known
        todo!("Calculate download percentage")
    }

    /// Get download speed in bytes per second
    pub fn speed_bps(&self) -> f64 {
        // TODO: Calculate bytes per second
        // Hint: downloaded / elapsed_seconds
        todo!("Calculate download speed")
    }

    /// Get estimated time remaining
    pub fn eta(&self) -> Option<std::time::Duration> {
        // TODO: Estimate remaining time based on current speed
        // Return None if total_size unknown or speed is zero
        todo!("Calculate ETA")
    }

    /// Format progress as string "1.5 MB / 10.0 MB (15.0%) - 500 KB/s - ETA: 17s"
    pub fn format(&self) -> String {
        // TODO: Format progress string with human-readable sizes
        todo!("Format progress string")
    }
}

/// Format bytes as human-readable string
pub fn format_bytes(bytes: usize) -> String {
    // TODO: Format bytes as KB, MB, GB, etc.
    // e.g., 1024 -> "1.0 KB", 1048576 -> "1.0 MB"
    todo!("Format bytes")
}

/// Complete download manager
pub struct Downloader {
    max_redirects: u32,
    follow_redirects: bool,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            max_redirects: 5,
            follow_redirects: true,
        }
    }

    /// Set maximum redirects to follow
    pub fn max_redirects(mut self, n: u32) -> Self {
        self.max_redirects = n;
        self
    }

    /// Resolve relative redirect URL against base URL
    pub fn resolve_redirect(&self, base: &Url, location: &str) -> Result<Url, &'static str> {
        // TODO: Resolve redirect location
        // Handle both absolute URLs and relative paths
        // - "http://other.com/path" -> parse as new URL
        // - "/new/path" -> same host, new path
        // - "relative/path" -> append to current path directory

        todo!("Resolve redirect URL")
    }

    /// Extract filename from URL or Content-Disposition header
    pub fn get_filename(&self, url: &Url, response: &HttpResponse) -> String {
        // TODO: Determine filename for download
        // 1. Check Content-Disposition header for filename
        // 2. Extract from URL path
        // 3. Default to "download"

        todo!("Get filename")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parse_simple() {
        let url = Url::parse("http://example.com").unwrap();
        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, 80);
        assert_eq!(url.path, "/");
        assert_eq!(url.query, None);
    }

    #[test]
    fn test_url_parse_with_port() {
        let url = Url::parse("http://example.com:8080/path").unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, 8080);
        assert_eq!(url.path, "/path");
    }

    #[test]
    fn test_url_parse_https() {
        let url = Url::parse("https://secure.example.com/api").unwrap();
        assert_eq!(url.scheme, "https");
        assert_eq!(url.port, 443);
    }

    #[test]
    fn test_url_parse_with_query() {
        let url = Url::parse("http://example.com/search?q=rust&page=1").unwrap();
        assert_eq!(url.path, "/search");
        assert_eq!(url.query, Some("q=rust&page=1".to_string()));
    }

    #[test]
    fn test_url_host_header() {
        let url1 = Url::parse("http://example.com").unwrap();
        assert_eq!(url1.host_header(), "example.com");

        let url2 = Url::parse("http://example.com:8080").unwrap();
        assert_eq!(url2.host_header(), "example.com:8080");
    }

    #[test]
    fn test_url_request_path() {
        let url1 = Url::parse("http://example.com/path").unwrap();
        assert_eq!(url1.request_path(), "/path");

        let url2 = Url::parse("http://example.com/path?q=test").unwrap();
        assert_eq!(url2.request_path(), "/path?q=test");
    }

    #[test]
    fn test_http_request_build() {
        let url = Url::parse("http://example.com/file.txt").unwrap();
        let request = HttpRequest::get(url)
            .header("Accept", "*/*")
            .build();

        let request_str = String::from_utf8(request).unwrap();
        assert!(request_str.starts_with("GET /file.txt HTTP/1.1\r\n"));
        assert!(request_str.contains("Host: example.com\r\n"));
        assert!(request_str.contains("Accept: */*\r\n"));
        assert!(request_str.ends_with("\r\n\r\n"));
    }

    #[test]
    fn test_response_parser_headers() {
        let mut parser = ResponseParser::new();

        let response = b"HTTP/1.1 200 OK\r\n\
            Content-Length: 13\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            Hello, World!";

        parser.feed(response);
        assert!(parser.try_parse_headers().unwrap());
        assert!(parser.is_body_complete());

        let body = parser.get_body().unwrap();
        assert_eq!(body, b"Hello, World!");
    }

    #[test]
    fn test_response_parser_chunked() {
        let mut parser = ResponseParser::new();

        let response = b"HTTP/1.1 200 OK\r\n\
            Transfer-Encoding: chunked\r\n\
            \r\n\
            5\r\nHello\r\n\
            7\r\n, World\r\n\
            0\r\n\r\n";

        parser.feed(response);
        assert!(parser.try_parse_headers().unwrap());
        assert!(parser.is_body_complete());

        let body = parser.get_body().unwrap();
        assert_eq!(body, b"Hello, World");
    }

    #[test]
    fn test_http_status() {
        let ok = HttpStatus { code: 200, reason: "OK".to_string() };
        assert!(ok.is_success());

        let redirect = HttpStatus { code: 302, reason: "Found".to_string() };
        assert!(redirect.is_redirect());

        let error = HttpStatus { code: 404, reason: "Not Found".to_string() };
        assert!(error.is_error());
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_download_progress() {
        let mut progress = DownloadProgress::new(Some(1000));
        progress.update(250);

        assert_eq!(progress.percentage(), Some(25.0));
        assert!(progress.speed_bps() > 0.0);
        assert!(progress.eta().is_some());
    }

    #[test]
    fn test_resolve_redirect_absolute() {
        let downloader = Downloader::new();
        let base = Url::parse("http://example.com/old/path").unwrap();

        let resolved = downloader.resolve_redirect(&base, "http://other.com/new").unwrap();
        assert_eq!(resolved.host, "other.com");
        assert_eq!(resolved.path, "/new");
    }

    #[test]
    fn test_resolve_redirect_relative() {
        let downloader = Downloader::new();
        let base = Url::parse("http://example.com/dir/file").unwrap();

        let resolved = downloader.resolve_redirect(&base, "/new/path").unwrap();
        assert_eq!(resolved.host, "example.com");
        assert_eq!(resolved.path, "/new/path");
    }
}
