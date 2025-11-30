//! # 练习 02: HTTP 客户端
//!
//! 使用 TCP 实现一个简单的 HTTP/1.1 客户端。
//!
//! ## 基本流程
//!
//! 1. 解析 URL
//! 2. DNS 解析（获取 IP）
//! 3. 建立 TCP 连接
//! 4. 发送 HTTP 请求
//! 5. 接收 HTTP 响应
//! 6. 解析响应
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test http_client
//! ```

use super::ex00_request::HttpRequest;
use super::ex01_response::HttpResponse;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

/// 解析 URL
#[derive(Debug, Clone)]
pub struct ParsedUrl {
    /// 协议 (http 或 https)
    pub scheme: String,
    /// 主机名
    pub host: String,
    /// 端口号
    pub port: u16,
    /// 路径
    pub path: String,
    /// 查询字符串
    pub query: Option<String>,
}

impl ParsedUrl {
    /// 解析 URL 字符串
    ///
    /// # 示例
    /// ```
    /// let url = ParsedUrl::parse("http://example.com:8080/path?query=1").unwrap();
    /// assert_eq!(url.host, "example.com");
    /// assert_eq!(url.port, 8080);
    /// assert_eq!(url.path, "/path");
    /// ```
    pub fn parse(url: &str) -> Option<Self> {
        // TODO: 解析 URL
        //
        // 格式: scheme://host[:port][/path][?query]
        //
        // 步骤:
        // 1. 提取 scheme（http 或 https）
        // 2. 提取 host 和可选的 port
        // 3. 提取 path（默认为 "/"）
        // 4. 提取可选的 query
        //
        // 默认端口: http=80, https=443
        todo!("解析 URL")
    }

    /// 获取 host:port 格式的地址
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// 获取请求路径（包含查询字符串）
    pub fn request_path(&self) -> String {
        match &self.query {
            Some(q) => format!("{}?{}", self.path, q),
            None => self.path.clone(),
        }
    }
}

/// 简单的 HTTP 客户端
pub struct HttpClient {
    /// 连接超时
    timeout: Duration,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new() -> Self {
        // TODO: 创建客户端，默认 30 秒超时
        todo!("创建客户端")
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 发送请求并获取响应
    pub fn send(&self, request: &HttpRequest, url: &ParsedUrl) -> io::Result<HttpResponse> {
        // TODO: 发送 HTTP 请求
        //
        // 步骤:
        // 1. 建立 TCP 连接
        // 2. 设置超时
        // 3. 发送请求数据
        // 4. 接收响应数据
        // 5. 解析响应
        todo!("发送请求")
    }

    /// 执行 GET 请求
    pub fn get(&self, url: &str) -> io::Result<HttpResponse> {
        // TODO: 构建并发送 GET 请求
        //
        // 步骤:
        // 1. 解析 URL
        // 2. 构建请求
        // 3. 添加必要的头部 (Host, User-Agent)
        // 4. 发送请求
        todo!("执行 GET 请求")
    }

    /// 执行 POST 请求
    pub fn post(&self, url: &str, body: Vec<u8>, content_type: &str) -> io::Result<HttpResponse> {
        // TODO: 构建并发送 POST 请求
        todo!("执行 POST 请求")
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// 读取 HTTP 响应（处理 Content-Length 和 Transfer-Encoding）
fn read_response(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    // TODO: 读取完整的 HTTP 响应
    //
    // 这是一个复杂的任务，需要:
    // 1. 读取头部（直到 \r\n\r\n）
    // 2. 解析 Content-Length 或 Transfer-Encoding
    // 3. 读取正确长度的正文
    //
    // 简化版本: 读取固定大小或直到连接关闭
    todo!("读取响应")
}

/// 发送简单的 HTTP GET 请求（便捷函数）
pub fn simple_get(url: &str) -> io::Result<String> {
    // TODO: 发送 GET 请求并返回响应正文
    todo!("简单 GET 请求")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_simple() {
        let url = ParsedUrl::parse("http://example.com/path").unwrap();

        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, 80);
        assert_eq!(url.path, "/path");
        assert!(url.query.is_none());
    }

    #[test]
    fn test_parse_url_with_port() {
        let url = ParsedUrl::parse("http://example.com:8080/path").unwrap();

        assert_eq!(url.port, 8080);
    }

    #[test]
    fn test_parse_url_with_query() {
        let url = ParsedUrl::parse("http://example.com/path?foo=bar&baz=qux").unwrap();

        assert_eq!(url.path, "/path");
        assert_eq!(url.query, Some("foo=bar&baz=qux".to_string()));
    }

    #[test]
    fn test_parse_url_https() {
        let url = ParsedUrl::parse("https://secure.example.com/").unwrap();

        assert_eq!(url.scheme, "https");
        assert_eq!(url.port, 443);
    }

    #[test]
    fn test_parse_url_no_path() {
        let url = ParsedUrl::parse("http://example.com").unwrap();

        assert_eq!(url.path, "/");
    }

    #[test]
    fn test_url_address() {
        let url = ParsedUrl::parse("http://example.com:8080/path").unwrap();
        assert_eq!(url.address(), "example.com:8080");
    }

    #[test]
    fn test_url_request_path() {
        let url1 = ParsedUrl::parse("http://example.com/path").unwrap();
        assert_eq!(url1.request_path(), "/path");

        let url2 = ParsedUrl::parse("http://example.com/path?query=1").unwrap();
        assert_eq!(url2.request_path(), "/path?query=1");
    }

    #[test]
    fn test_http_client_new() {
        let client = HttpClient::new();
        assert_eq!(client.timeout, Duration::from_secs(30));
    }

    // 注意: 网络测试通常需要真实的服务器
    // 这些测试在 CI 环境中可能会失败
    #[test]
    #[ignore] // 忽略需要网络的测试
    fn test_http_client_get() {
        let client = HttpClient::new().with_timeout(Duration::from_secs(5));

        // 这需要一个真实的服务器
        // let response = client.get("http://example.com/").unwrap();
        // assert!(response.is_success());
    }
}
