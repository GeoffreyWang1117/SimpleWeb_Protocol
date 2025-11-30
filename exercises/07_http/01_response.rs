//! # 练习 01: HTTP 响应
//!
//! HTTP 响应由状态行、头部和正文组成。
//!
//! ## 响应格式
//!
//! ```text
//! HTTP/1.1 200 OK\r\n
//! Content-Type: text/html\r\n
//! Content-Length: 1234\r\n
//! \r\n
//! <html>...</html>
//! ```
//!
//! ## 状态行
//!
//! ```text
//! <Version> <Status-Code> <Reason-Phrase>\r\n
//! ```
//!
//! ## 常见状态码
//!
//! - 1xx: 信息
//! - 2xx: 成功 (200 OK, 201 Created, 204 No Content)
//! - 3xx: 重定向 (301 Moved, 302 Found, 304 Not Modified)
//! - 4xx: 客户端错误 (400 Bad Request, 404 Not Found, 403 Forbidden)
//! - 5xx: 服务器错误 (500 Internal Server Error, 502 Bad Gateway)
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test http_response
//! ```

use super::ex00_request::HttpVersion;
use std::collections::HashMap;

/// HTTP 状态码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(pub u16);

impl StatusCode {
    // 2xx Success
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const NO_CONTENT: StatusCode = StatusCode(204);

    // 3xx Redirection
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301);
    pub const FOUND: StatusCode = StatusCode(302);
    pub const NOT_MODIFIED: StatusCode = StatusCode(304);

    // 4xx Client Error
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);

    // 5xx Server Error
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
    pub const BAD_GATEWAY: StatusCode = StatusCode(502);
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);

    /// 获取状态码的默认原因短语
    pub fn reason_phrase(&self) -> &'static str {
        // TODO: 返回状态码对应的原因短语
        todo!("获取原因短语")
    }

    /// 检查是否是成功状态
    pub fn is_success(&self) -> bool {
        // TODO: 检查是否是 2xx
        todo!("检查成功状态")
    }

    /// 检查是否是重定向状态
    pub fn is_redirect(&self) -> bool {
        // TODO: 检查是否是 3xx
        todo!("检查重定向状态")
    }

    /// 检查是否是客户端错误
    pub fn is_client_error(&self) -> bool {
        // TODO: 检查是否是 4xx
        todo!("检查客户端错误")
    }

    /// 检查是否是服务器错误
    pub fn is_server_error(&self) -> bool {
        // TODO: 检查是否是 5xx
        todo!("检查服务器错误")
    }
}

/// HTTP 响应
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP 版本
    pub version: HttpVersion,
    /// 状态码
    pub status: StatusCode,
    /// 原因短语
    pub reason: String,
    /// 响应头部
    pub headers: HashMap<String, String>,
    /// 响应正文
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// 创建一个新的 HTTP 响应
    pub fn new(status: StatusCode) -> Self {
        // TODO: 创建响应
        // 默认版本为 HTTP/1.1
        // 原因短语使用状态码的默认值
        todo!("创建响应")
    }

    /// 添加头部
    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        // TODO: 添加头部
        todo!("添加头部")
    }

    /// 设置正文
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        // TODO: 设置正文
        // 自动添加 Content-Length
        todo!("设置正文")
    }

    /// 设置 HTML 正文
    pub fn with_html(self, html: &str) -> Self {
        // TODO: 设置 HTML 正文
        // Content-Type: text/html; charset=utf-8
        todo!("设置 HTML 正文")
    }

    /// 设置 JSON 正文
    pub fn with_json(self, json: &str) -> Self {
        // TODO: 设置 JSON 正文
        // Content-Type: application/json
        todo!("设置 JSON 正文")
    }

    /// 将响应序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化响应
        //
        // 格式:
        // <Version> <Status-Code> <Reason>\r\n
        // <Header-Name>: <Header-Value>\r\n
        // ...
        // \r\n
        // <Body>
        todo!("序列化响应")
    }

    /// 从字节切片解析响应
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 HTTP 响应
        //
        // 步骤:
        // 1. 将字节转换为字符串
        // 2. 按 \r\n\r\n 分割头部和正文
        // 3. 解析状态行
        // 4. 解析头部
        // 5. 提取正文
        todo!("解析响应")
    }

    /// 获取头部值
    pub fn get_header(&self, name: &str) -> Option<&String> {
        // TODO: 获取头部值（不区分大小写）
        todo!("获取头部")
    }

    /// 获取 Content-Type
    pub fn content_type(&self) -> Option<&String> {
        self.get_header("Content-Type")
    }

    /// 获取 Content-Length
    pub fn content_length(&self) -> Option<usize> {
        // TODO: 解析 Content-Length 头部
        todo!("获取 Content-Length")
    }

    /// 检查是否是成功响应
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// 获取正文作为字符串
    pub fn body_text(&self) -> Option<String> {
        // TODO: 将正文转换为 UTF-8 字符串
        todo!("获取正文文本")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_reason() {
        assert_eq!(StatusCode::OK.reason_phrase(), "OK");
        assert_eq!(StatusCode::NOT_FOUND.reason_phrase(), "Not Found");
        assert_eq!(
            StatusCode::INTERNAL_SERVER_ERROR.reason_phrase(),
            "Internal Server Error"
        );
    }

    #[test]
    fn test_status_code_categories() {
        assert!(StatusCode::OK.is_success());
        assert!(StatusCode::CREATED.is_success());

        assert!(StatusCode::MOVED_PERMANENTLY.is_redirect());
        assert!(StatusCode::FOUND.is_redirect());

        assert!(StatusCode::NOT_FOUND.is_client_error());
        assert!(StatusCode::BAD_REQUEST.is_client_error());

        assert!(StatusCode::INTERNAL_SERVER_ERROR.is_server_error());
        assert!(StatusCode::BAD_GATEWAY.is_server_error());
    }

    #[test]
    fn test_http_response_new() {
        let resp = HttpResponse::new(StatusCode::OK);

        assert_eq!(resp.status, StatusCode::OK);
        assert_eq!(resp.version, HttpVersion::Http11);
        assert_eq!(resp.reason, "OK");
    }

    #[test]
    fn test_http_response_with_html() {
        let html = "<html><body>Hello</body></html>";
        let resp = HttpResponse::new(StatusCode::OK).with_html(html);

        assert!(resp
            .content_type()
            .unwrap()
            .contains("text/html"));
        assert_eq!(resp.body, html.as_bytes());
    }

    #[test]
    fn test_http_response_to_bytes() {
        let resp = HttpResponse::new(StatusCode::OK)
            .with_header("Content-Type", "text/plain")
            .with_body(b"Hello".to_vec());

        let bytes = resp.to_bytes();
        let text = String::from_utf8_lossy(&bytes);

        assert!(text.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(text.contains("Content-Type: text/plain\r\n"));
        assert!(text.contains("Content-Length: 5\r\n"));
        assert!(text.ends_with("\r\n\r\nHello"));
    }

    #[test]
    fn test_http_response_parse() {
        let data = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nHello";

        let resp = HttpResponse::parse(data).unwrap();

        assert_eq!(resp.status, StatusCode::OK);
        assert_eq!(resp.version, HttpVersion::Http11);
        assert_eq!(resp.reason, "OK");
        assert_eq!(resp.body, b"Hello");
        assert_eq!(resp.content_length(), Some(5));
    }

    #[test]
    fn test_http_response_body_text() {
        let resp = HttpResponse::new(StatusCode::OK).with_body(b"Hello, World!".to_vec());

        assert_eq!(resp.body_text(), Some("Hello, World!".to_string()));
    }
}
