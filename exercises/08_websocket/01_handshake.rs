//! # 练习 01: WebSocket 握手
//!
//! WebSocket 使用 HTTP Upgrade 机制建立连接。
//!
//! ## 客户端握手请求
//!
//! ```text
//! GET /chat HTTP/1.1
//! Host: server.example.com
//! Upgrade: websocket
//! Connection: Upgrade
//! Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
//! Sec-WebSocket-Version: 13
//! ```
//!
//! ## 服务器握手响应
//!
//! ```text
//! HTTP/1.1 101 Switching Protocols
//! Upgrade: websocket
//! Connection: Upgrade
//! Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=
//! ```
//!
//! ## 密钥计算
//!
//! Accept = Base64(SHA1(Key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"))
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test websocket_handshake
//! ```

use std::collections::HashMap;

/// WebSocket 握手 GUID (RFC 6455)
pub const WS_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

/// 生成随机的 Sec-WebSocket-Key
///
/// Key 是 16 字节随机数的 Base64 编码
pub fn generate_key() -> String {
    // TODO: 生成 WebSocket Key
    //
    // 步骤:
    // 1. 生成 16 字节随机数
    // 2. Base64 编码
    //
    // 简化版本: 可以使用固定的随机值进行测试
    todo!("生成 WebSocket Key")
}

/// 计算 Sec-WebSocket-Accept 值
///
/// # 参数
/// * `key` - 客户端发送的 Sec-WebSocket-Key
///
/// # 返回值
/// 服务器应该返回的 Sec-WebSocket-Accept 值
///
/// # 算法
/// Accept = Base64(SHA1(Key + GUID))
pub fn compute_accept_key(key: &str) -> String {
    // TODO: 计算 Accept Key
    //
    // 这需要 SHA1 和 Base64 编码
    // 由于标准库不包含这些，这里可以:
    // 1. 使用简化的实现
    // 2. 或者返回预计算的值用于测试
    //
    // 完整实现需要添加 sha1 和 base64 依赖
    todo!("计算 Accept Key")
}

/// WebSocket 握手请求
#[derive(Debug, Clone)]
pub struct WsHandshakeRequest {
    /// 请求路径
    pub path: String,
    /// Host
    pub host: String,
    /// Sec-WebSocket-Key
    pub key: String,
    /// Sec-WebSocket-Version
    pub version: u8,
    /// 可选的子协议
    pub protocols: Vec<String>,
    /// 可选的扩展
    pub extensions: Vec<String>,
}

impl WsHandshakeRequest {
    /// 创建新的握手请求
    pub fn new(host: &str, path: &str) -> Self {
        // TODO: 创建握手请求
        // 自动生成 key
        // version = 13
        todo!("创建握手请求")
    }

    /// 添加子协议
    pub fn with_protocol(mut self, protocol: &str) -> Self {
        self.protocols.push(protocol.to_string());
        self
    }

    /// 构建 HTTP 请求字符串
    pub fn to_http_request(&self) -> String {
        // TODO: 构建 HTTP 请求
        //
        // 必需的头部:
        // - GET <path> HTTP/1.1
        // - Host: <host>
        // - Upgrade: websocket
        // - Connection: Upgrade
        // - Sec-WebSocket-Key: <key>
        // - Sec-WebSocket-Version: 13
        //
        // 可选:
        // - Sec-WebSocket-Protocol: <protocols>
        // - Sec-WebSocket-Extensions: <extensions>
        todo!("构建 HTTP 请求")
    }

    /// 从 HTTP 请求解析
    pub fn parse(request: &str) -> Option<Self> {
        // TODO: 解析 HTTP 请求
        todo!("解析握手请求")
    }
}

/// WebSocket 握手响应
#[derive(Debug, Clone)]
pub struct WsHandshakeResponse {
    /// 状态码 (应该是 101)
    pub status_code: u16,
    /// Sec-WebSocket-Accept
    pub accept: String,
    /// 选定的子协议
    pub protocol: Option<String>,
    /// 选定的扩展
    pub extensions: Vec<String>,
}

impl WsHandshakeResponse {
    /// 创建成功的握手响应
    pub fn accept(client_key: &str) -> Self {
        // TODO: 创建响应
        // 计算 accept key
        todo!("创建握手响应")
    }

    /// 构建 HTTP 响应字符串
    pub fn to_http_response(&self) -> String {
        // TODO: 构建 HTTP 响应
        //
        // HTTP/1.1 101 Switching Protocols
        // Upgrade: websocket
        // Connection: Upgrade
        // Sec-WebSocket-Accept: <accept>
        todo!("构建 HTTP 响应")
    }

    /// 从 HTTP 响应解析
    pub fn parse(response: &str) -> Option<Self> {
        // TODO: 解析 HTTP 响应
        todo!("解析握手响应")
    }

    /// 验证服务器响应
    ///
    /// # 参数
    /// * `client_key` - 客户端发送的 Key
    pub fn validate(&self, client_key: &str) -> bool {
        // TODO: 验证响应
        //
        // 检查:
        // 1. 状态码是 101
        // 2. Accept 值正确
        todo!("验证响应")
    }
}

/// 执行 WebSocket 握手（客户端）
///
/// # 参数
/// * `host` - 服务器主机名
/// * `path` - 请求路径
/// * `send_fn` - 发送数据的函数
/// * `recv_fn` - 接收数据的函数
///
/// # 返回值
/// 成功返回选定的子协议（如果有）
pub fn perform_handshake<S, R>(
    host: &str,
    path: &str,
    mut send_fn: S,
    mut recv_fn: R,
) -> Result<Option<String>, &'static str>
where
    S: FnMut(&[u8]) -> Result<(), &'static str>,
    R: FnMut(&mut [u8]) -> Result<usize, &'static str>,
{
    // TODO: 执行握手
    //
    // 步骤:
    // 1. 创建握手请求
    // 2. 发送请求
    // 3. 接收响应
    // 4. 解析响应
    // 5. 验证响应
    todo!("执行握手")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_generate_key() {
        let key = generate_key();

        // Base64 编码的 16 字节应该是 24 个字符（包含填充）
        // 但由于 Base64 的特性，可能是 22-24 个字符
        assert!(key.len() >= 20);
    }

    #[test]
    fn test_ws_compute_accept_key() {
        // RFC 6455 示例
        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        let expected = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

        let accept = compute_accept_key(key);
        assert_eq!(accept, expected);
    }

    #[test]
    fn test_ws_handshake_request_new() {
        let req = WsHandshakeRequest::new("example.com", "/chat");

        assert_eq!(req.host, "example.com");
        assert_eq!(req.path, "/chat");
        assert_eq!(req.version, 13);
        assert!(!req.key.is_empty());
    }

    #[test]
    fn test_ws_handshake_request_to_http() {
        let req = WsHandshakeRequest::new("example.com", "/chat");
        let http = req.to_http_request();

        assert!(http.starts_with("GET /chat HTTP/1.1\r\n"));
        assert!(http.contains("Host: example.com\r\n"));
        assert!(http.contains("Upgrade: websocket\r\n"));
        assert!(http.contains("Connection: Upgrade\r\n"));
        assert!(http.contains("Sec-WebSocket-Key:"));
        assert!(http.contains("Sec-WebSocket-Version: 13\r\n"));
        assert!(http.ends_with("\r\n\r\n"));
    }

    #[test]
    fn test_ws_handshake_response_accept() {
        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        let resp = WsHandshakeResponse::accept(key);

        assert_eq!(resp.status_code, 101);
        assert!(!resp.accept.is_empty());
    }

    #[test]
    fn test_ws_handshake_response_to_http() {
        let resp = WsHandshakeResponse::accept("dGhlIHNhbXBsZSBub25jZQ==");
        let http = resp.to_http_response();

        assert!(http.starts_with("HTTP/1.1 101 Switching Protocols\r\n"));
        assert!(http.contains("Upgrade: websocket\r\n"));
        assert!(http.contains("Connection: Upgrade\r\n"));
        assert!(http.contains("Sec-WebSocket-Accept:"));
    }

    #[test]
    fn test_ws_handshake_validate() {
        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        let resp = WsHandshakeResponse::accept(key);

        assert!(resp.validate(key));
        assert!(!resp.validate("wrong_key"));
    }

    #[test]
    fn test_ws_handshake_request_parse() {
        let http = "GET /chat HTTP/1.1\r\n\
                    Host: example.com\r\n\
                    Upgrade: websocket\r\n\
                    Connection: Upgrade\r\n\
                    Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
                    Sec-WebSocket-Version: 13\r\n\
                    \r\n";

        let req = WsHandshakeRequest::parse(http).unwrap();

        assert_eq!(req.path, "/chat");
        assert_eq!(req.host, "example.com");
        assert_eq!(req.key, "dGhlIHNhbXBsZSBub25jZQ==");
        assert_eq!(req.version, 13);
    }
}
