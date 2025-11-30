//! # 练习 00: WebSocket 帧
//!
//! WebSocket 使用帧（Frame）来传输数据。
//!
//! ## 帧格式 (RFC 6455)
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-------+-+-------------+-------------------------------+
//! |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
//! |I|S|S|S|  (4)  |A|     (7)     |             (16/64)           |
//! |N|V|V|V|       |S|             |   (if payload len==126/127)   |
//! | |1|2|3|       |K|             |                               |
//! +-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
//! |     Extended payload length continued, if payload len == 127  |
//! + - - - - - - - - - - - - - - - +-------------------------------+
//! |                               |Masking-key, if MASK set to 1  |
//! +-------------------------------+-------------------------------+
//! | Masking-key (continued)       |          Payload Data         |
//! +-------------------------------- - - - - - - - - - - - - - - - +
//! :                     Payload Data continued ...                :
//! + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
//! |                     Payload Data continued ...                |
//! +---------------------------------------------------------------+
//! ```
//!
//! ## 操作码
//!
//! - 0x0: 继续帧
//! - 0x1: 文本帧
//! - 0x2: 二进制帧
//! - 0x8: 关闭帧
//! - 0x9: Ping
//! - 0xA: Pong
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test websocket_frame
//! ```

/// WebSocket 操作码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsOpcode {
    /// 继续帧
    Continuation = 0x0,
    /// 文本帧
    Text = 0x1,
    /// 二进制帧
    Binary = 0x2,
    /// 关闭连接
    Close = 0x8,
    /// Ping
    Ping = 0x9,
    /// Pong
    Pong = 0xA,
}

impl WsOpcode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x0 => Some(WsOpcode::Continuation),
            0x1 => Some(WsOpcode::Text),
            0x2 => Some(WsOpcode::Binary),
            0x8 => Some(WsOpcode::Close),
            0x9 => Some(WsOpcode::Ping),
            0xA => Some(WsOpcode::Pong),
            _ => None,
        }
    }

    /// 检查是否是控制帧
    pub fn is_control(&self) -> bool {
        matches!(self, WsOpcode::Close | WsOpcode::Ping | WsOpcode::Pong)
    }
}

/// WebSocket 帧
#[derive(Debug, Clone)]
pub struct WsFrame {
    /// FIN 位：是否是消息的最后一帧
    pub fin: bool,
    /// 操作码
    pub opcode: WsOpcode,
    /// 是否使用掩码（客户端发送必须为 true）
    pub masked: bool,
    /// 掩码键（4 字节）
    pub mask_key: Option<[u8; 4]>,
    /// 载荷数据
    pub payload: Vec<u8>,
}

impl WsFrame {
    /// 创建文本帧
    pub fn text(data: &str) -> Self {
        // TODO: 创建文本帧
        // fin=true, opcode=Text, masked=true (客户端)
        todo!("创建文本帧")
    }

    /// 创建二进制帧
    pub fn binary(data: Vec<u8>) -> Self {
        // TODO: 创建二进制帧
        todo!("创建二进制帧")
    }

    /// 创建关闭帧
    pub fn close(code: Option<u16>, reason: Option<&str>) -> Self {
        // TODO: 创建关闭帧
        //
        // 关闭帧的载荷格式:
        // - 2 字节状态码（大端序）
        // - 可选的原因字符串
        todo!("创建关闭帧")
    }

    /// 创建 Ping 帧
    pub fn ping(data: Vec<u8>) -> Self {
        // TODO: 创建 Ping 帧
        todo!("创建 Ping 帧")
    }

    /// 创建 Pong 帧
    pub fn pong(data: Vec<u8>) -> Self {
        // TODO: 创建 Pong 帧
        todo!("创建 Pong 帧")
    }

    /// 设置掩码
    pub fn with_mask(mut self, mask_key: [u8; 4]) -> Self {
        self.masked = true;
        self.mask_key = Some(mask_key);
        self
    }

    /// 生成随机掩码键
    pub fn generate_mask_key() -> [u8; 4] {
        // TODO: 生成随机掩码键
        // 简化版本：使用固定值或简单随机
        todo!("生成掩码键")
    }

    /// 应用/移除掩码
    ///
    /// XOR 操作是对称的，所以同一个函数用于加密和解密
    pub fn apply_mask(data: &mut [u8], mask_key: [u8; 4]) {
        // TODO: 应用掩码
        //
        // 对于每个字节 i: data[i] ^= mask_key[i % 4]
        todo!("应用掩码")
    }

    /// 将帧序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: 序列化 WebSocket 帧
        //
        // 步骤:
        // 1. 构建第一个字节: FIN(1) + RSV(3) + Opcode(4)
        // 2. 构建第二个字节: MASK(1) + Payload len(7)
        // 3. 如果 payload > 125: 添加扩展长度
        // 4. 如果 masked: 添加掩码键
        // 5. 添加（可能被掩码的）载荷
        todo!("序列化帧")
    }

    /// 从字节切片解析帧
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        // TODO: 解析 WebSocket 帧
        //
        // 步骤:
        // 1. 解析第一个字节获取 FIN 和 opcode
        // 2. 解析第二个字节获取 MASK 和 payload length
        // 3. 处理扩展长度
        // 4. 如果有掩码，读取掩码键
        // 5. 读取载荷
        // 6. 如果有掩码，应用掩码
        //
        // 返回: (帧, 消耗的字节数)
        todo!("解析帧")
    }

    /// 获取载荷作为字符串（仅对文本帧有效）
    pub fn payload_text(&self) -> Option<String> {
        // TODO: 将载荷转换为 UTF-8 字符串
        todo!("获取载荷文本")
    }

    /// 获取关闭帧的状态码
    pub fn close_code(&self) -> Option<u16> {
        // TODO: 解析关闭帧的状态码
        todo!("获取关闭状态码")
    }

    /// 获取关闭帧的原因
    pub fn close_reason(&self) -> Option<String> {
        // TODO: 解析关闭帧的原因
        todo!("获取关闭原因")
    }
}

/// 常见的关闭状态码
pub mod close_codes {
    /// 正常关闭
    pub const NORMAL: u16 = 1000;
    /// 终端离开
    pub const GOING_AWAY: u16 = 1001;
    /// 协议错误
    pub const PROTOCOL_ERROR: u16 = 1002;
    /// 不支持的数据类型
    pub const UNSUPPORTED: u16 = 1003;
    /// 没有收到状态码
    pub const NO_STATUS: u16 = 1005;
    /// 异常关闭
    pub const ABNORMAL: u16 = 1006;
    /// 无效的帧载荷数据
    pub const INVALID_PAYLOAD: u16 = 1007;
    /// 策略违规
    pub const POLICY_VIOLATION: u16 = 1008;
    /// 消息太大
    pub const TOO_BIG: u16 = 1009;
    /// 缺少扩展
    pub const MISSING_EXTENSION: u16 = 1010;
    /// 内部错误
    pub const INTERNAL_ERROR: u16 = 1011;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_opcode() {
        assert_eq!(WsOpcode::from_u8(0x1), Some(WsOpcode::Text));
        assert_eq!(WsOpcode::from_u8(0x2), Some(WsOpcode::Binary));
        assert_eq!(WsOpcode::from_u8(0x8), Some(WsOpcode::Close));

        assert!(WsOpcode::Close.is_control());
        assert!(WsOpcode::Ping.is_control());
        assert!(!WsOpcode::Text.is_control());
    }

    #[test]
    fn test_ws_frame_text() {
        let frame = WsFrame::text("Hello");

        assert!(frame.fin);
        assert_eq!(frame.opcode, WsOpcode::Text);
        assert_eq!(frame.payload, b"Hello");
    }

    #[test]
    fn test_ws_frame_apply_mask() {
        let mask_key = [0x37, 0xfa, 0x21, 0x3d];
        let mut data = b"Hello".to_vec();
        let original = data.clone();

        WsFrame::apply_mask(&mut data, mask_key);
        assert_ne!(data, original); // 掩码后应该不同

        WsFrame::apply_mask(&mut data, mask_key);
        assert_eq!(data, original); // 再次应用应该恢复原始
    }

    #[test]
    fn test_ws_frame_to_bytes() {
        let frame = WsFrame::text("Hi").with_mask([0x12, 0x34, 0x56, 0x78]);
        let bytes = frame.to_bytes();

        // 验证基本结构
        assert_eq!(bytes[0] & 0x80, 0x80); // FIN = 1
        assert_eq!(bytes[0] & 0x0F, 0x01); // opcode = 1 (text)
        assert_eq!(bytes[1] & 0x80, 0x80); // MASK = 1
        assert_eq!(bytes[1] & 0x7F, 2); // length = 2
    }

    #[test]
    fn test_ws_frame_parse() {
        // 一个简单的未掩码文本帧 "Hi"
        let data = [
            0x81, // FIN=1, opcode=1 (text)
            0x02, // MASK=0, len=2
            b'H', b'i',
        ];

        let (frame, len) = WsFrame::parse(&data).unwrap();

        assert!(frame.fin);
        assert_eq!(frame.opcode, WsOpcode::Text);
        assert!(!frame.masked);
        assert_eq!(frame.payload, b"Hi");
        assert_eq!(len, 4);
    }

    #[test]
    fn test_ws_frame_parse_masked() {
        // 掩码文本帧
        let mask_key = [0x12, 0x34, 0x56, 0x78];
        let mut masked_payload = b"Hi".to_vec();
        WsFrame::apply_mask(&mut masked_payload, mask_key);

        let mut data = vec![
            0x81, // FIN=1, opcode=1 (text)
            0x82, // MASK=1, len=2
        ];
        data.extend_from_slice(&mask_key);
        data.extend_from_slice(&masked_payload);

        let (frame, _) = WsFrame::parse(&data).unwrap();

        assert!(frame.masked);
        assert_eq!(frame.payload_text(), Some("Hi".to_string()));
    }

    #[test]
    fn test_ws_frame_close() {
        let frame = WsFrame::close(Some(close_codes::NORMAL), Some("Goodbye"));

        assert_eq!(frame.opcode, WsOpcode::Close);
        assert_eq!(frame.close_code(), Some(close_codes::NORMAL));
        assert_eq!(frame.close_reason(), Some("Goodbye".to_string()));
    }

    #[test]
    fn test_ws_frame_roundtrip() {
        let original = WsFrame::text("Hello, WebSocket!")
            .with_mask(WsFrame::generate_mask_key());

        let bytes = original.to_bytes();
        let (parsed, _) = WsFrame::parse(&bytes).unwrap();

        assert_eq!(parsed.fin, original.fin);
        assert_eq!(parsed.opcode, original.opcode);
        assert_eq!(parsed.payload_text(), Some("Hello, WebSocket!".to_string()));
    }
}
