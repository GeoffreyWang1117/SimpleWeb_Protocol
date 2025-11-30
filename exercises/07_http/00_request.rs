//! # 练习 00: HTTP 请求
//!
//! HTTP 请求由请求行、头部和正文组成。
//!
//! ## 请求格式
//!
//! ```text
//! GET /path HTTP/1.1\r\n
//! Host: example.com\r\n
//! User-Agent: my-client/1.0\r\n
//! Accept: */*\r\n
//! \r\n
//! [body]
//! ```
//!
//! ## 请求行
//!
//! ```text
//! <Method> <Path> <Version>\r\n
//! ```
//!
//! ## 常见方法
//!
//! - GET: 获取资源
//! - POST: 提交数据
//! - PUT: 更新资源
//! - DELETE: 删除资源
//! - HEAD: 获取头部（不包含正文）
//! - OPTIONS: 获取支持的方法
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test http_request
//! ```

use std::collections::HashMap;

/// HTTP 方法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    CONNECT,
    TRACE,
}

impl HttpMethod {
    /// 从字符串解析方法
    pub fn from_str(s: &str) -> Option<Self> {
        // TODO: 将字符串转换为方法
        todo!("解析 HTTP 方法")
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        // TODO: 返回方法的字符串表示
        todo!("转换为字符串")
    }
}

/// HTTP 版本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
}

impl HttpVersion {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "HTTP/1.0" => Some(HttpVersion::Http10),
            "HTTP/1.1" => Some(HttpVersion::Http11),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
        }
    }
}

/// HTTP 请求
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// HTTP 方法
    pub method: HttpMethod,
    /// 请求路径
    pub path: String,
    /// HTTP 版本
    pub version: HttpVersion,
    /// 请求头部
    pub headers: HashMap<String, String>,
    /// 请求正文
    pub body: Vec<u8>,
}

impl HttpRequest {
    /// 创建一个新的 HTTP 请求
    pub fn new(method: HttpMethod, path: &str) -> Self {
        // TODO: 创建 HTTP 请求
        // 默认版本为 HTTP/1.1
        todo!("创建 HTTP 请求")
    }

    /// 添加头部
    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        // TODO: 添加头部
        // 注意：头部名称应该标准化（首字母大写）
        todo!("添加头部")
    }

    /// 设置 Host 头部
    pub fn with_host(self, host: &str) -> Self {
        self.with_header("Host", host)
    }

    /// 设置 User-Agent 头部
    pub fn with_user_agent(self, user_agent: &str) -> Self {
        self.with_header("User-Agent", user_agent)
    }

    /// 设置正文
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        // TODO: 设置正文
        // 自动添加 Content-Length 头部
        todo!("设置正文")
    }

    /// 设置 JSON 正文
    pub fn with_json(self, json: &str) -> Self {
        // TODO: 设置 JSON 正文
        // 添加 Content-Type: application/json
        todo!("设置 JSON 正文")
    }

    /// 将请求序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化请求
        //
        // 格式:
        // <Method> <Path> <Version>\r\n
        // <Header-Name>: <Header-Value>\r\n
        // ...
        // \r\n
        // <Body>
        todo!("序列化请求")
    }

    /// 从字节切片解析请求
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 HTTP 请求
        //
        // 步骤:
        // 1. 将字节转换为字符串
        // 2. 按 \r\n\r\n 分割头部和正文
        // 3. 解析请求行
        // 4. 解析头部
        // 5. 提取正文
        todo!("解析请求")
    }

    /// 获取头部值
    pub fn get_header(&self, name: &str) -> Option<&String> {
        // TODO: 获取头部值（不区分大小写）
        todo!("获取头部")
    }

    /// 获取 Content-Length
    pub fn content_length(&self) -> Option<usize> {
        // TODO: 解析 Content-Length 头部
        todo!("获取 Content-Length")
    }
}

/// 创建一个 GET 请求
pub fn get(path: &str) -> HttpRequest {
    HttpRequest::new(HttpMethod::GET, path)
}

/// 创建一个 POST 请求
pub fn post(path: &str) -> HttpRequest {
    HttpRequest::new(HttpMethod::POST, path)
}

/// 标准化头部名称（每个单词首字母大写）
///
/// # 示例
/// ```
/// assert_eq!(normalize_header_name("content-type"), "Content-Type");
/// assert_eq!(normalize_header_name("ACCEPT"), "Accept");
/// ```
pub fn normalize_header_name(name: &str) -> String {
    // TODO: 标准化头部名称
    todo!("标准化头部名称")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_method_parse() {
        assert_eq!(HttpMethod::from_str("GET"), Some(HttpMethod::GET));
        assert_eq!(HttpMethod::from_str("POST"), Some(HttpMethod::POST));
        assert_eq!(HttpMethod::from_str("INVALID"), None);
    }

    #[test]
    fn test_http_request_method_str() {
        assert_eq!(HttpMethod::GET.as_str(), "GET");
        assert_eq!(HttpMethod::POST.as_str(), "POST");
    }

    #[test]
    fn test_http_request_new() {
        let req = HttpRequest::new(HttpMethod::GET, "/index.html");

        assert_eq!(req.method, HttpMethod::GET);
        assert_eq!(req.path, "/index.html");
        assert_eq!(req.version, HttpVersion::Http11);
    }

    #[test]
    fn test_http_request_with_header() {
        let req = HttpRequest::new(HttpMethod::GET, "/")
            .with_host("example.com")
            .with_user_agent("test/1.0");

        assert_eq!(req.get_header("Host"), Some(&"example.com".to_string()));
        assert_eq!(req.get_header("User-Agent"), Some(&"test/1.0".to_string()));
    }

    #[test]
    fn test_http_request_to_bytes() {
        let req = HttpRequest::new(HttpMethod::GET, "/")
            .with_host("example.com")
            .with_header("Accept", "*/*");

        let bytes = req.to_bytes();
        let text = String::from_utf8_lossy(&bytes);

        assert!(text.starts_with("GET / HTTP/1.1\r\n"));
        assert!(text.contains("Host: example.com\r\n"));
        assert!(text.contains("Accept: */*\r\n"));
        assert!(text.ends_with("\r\n\r\n"));
    }

    #[test]
    fn test_http_request_parse() {
        let data = b"GET /path HTTP/1.1\r\nHost: example.com\r\nAccept: */*\r\n\r\n";

        let req = HttpRequest::parse(data).unwrap();

        assert_eq!(req.method, HttpMethod::GET);
        assert_eq!(req.path, "/path");
        assert_eq!(req.version, HttpVersion::Http11);
        assert_eq!(req.get_header("Host"), Some(&"example.com".to_string()));
    }

    #[test]
    fn test_http_request_with_body() {
        let req = HttpRequest::new(HttpMethod::POST, "/api/data")
            .with_host("example.com")
            .with_body(b"hello".to_vec());

        assert_eq!(req.body, b"hello");
        assert_eq!(req.content_length(), Some(5));
    }

    #[test]
    fn test_normalize_header_name() {
        assert_eq!(normalize_header_name("content-type"), "Content-Type");
        assert_eq!(normalize_header_name("ACCEPT"), "Accept");
        assert_eq!(normalize_header_name("x-custom-header"), "X-Custom-Header");
    }
}
