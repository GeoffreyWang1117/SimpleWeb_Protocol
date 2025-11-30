//! # 练习 02: 长度前缀分帧
//!
//! 在消息前添加长度字段，是最灵活和高效的分帧方式。
//!
//! ## 常见格式
//!
//! ### 简单长度前缀
//! ```text
//! +--------+----------------+
//! | Length |    Payload     |
//! | (4B)   |   (N bytes)    |
//! +--------+----------------+
//! ```
//!
//! ### TLV (Type-Length-Value)
//! ```text
//! +------+--------+----------------+
//! | Type | Length |     Value      |
//! | (2B) |  (4B)  |   (N bytes)    |
//! +------+--------+----------------+
//! ```
//!
//! ## 长度字段的选择
//!
//! - 1 字节: 最大 255 字节
//! - 2 字节: 最大 64 KB
//! - 4 字节: 最大 4 GB
//! - 变长编码 (varint): 小消息更紧凑
//!
//! ## 优点
//! - 高效解析（知道确切长度）
//! - 支持二进制数据
//! - 无需转义
//!
//! ## 使用场景
//! - Protocol Buffers
//! - 大多数二进制协议
//! - 数据库协议（MySQL, PostgreSQL）
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test framing_length
//! ```

/// 长度字段的大小
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthFieldSize {
    /// 1 字节 (最大 255)
    U8,
    /// 2 字节 (最大 65535)
    U16,
    /// 4 字节 (最大 4GB)
    U32,
}

impl LengthFieldSize {
    pub fn bytes(&self) -> usize {
        match self {
            LengthFieldSize::U8 => 1,
            LengthFieldSize::U16 => 2,
            LengthFieldSize::U32 => 4,
        }
    }

    pub fn max_value(&self) -> usize {
        match self {
            LengthFieldSize::U8 => u8::MAX as usize,
            LengthFieldSize::U16 => u16::MAX as usize,
            LengthFieldSize::U32 => u32::MAX as usize,
        }
    }
}

/// 长度前缀编解码器
pub struct LengthPrefixCodec {
    /// 长度字段大小
    length_size: LengthFieldSize,
    /// 是否使用大端序
    big_endian: bool,
    /// 最大消息长度
    max_length: usize,
    /// 长度字段是否包含自身长度
    length_includes_header: bool,
}

impl LengthPrefixCodec {
    /// 创建编解码器
    pub fn new(length_size: LengthFieldSize) -> Self {
        // TODO: 创建编解码器
        // 默认: 大端序, 最大长度为长度字段能表示的最大值, 不包含头部
        todo!("创建长度前缀编解码器")
    }

    /// 设置字节序
    pub fn with_endian(mut self, big_endian: bool) -> Self {
        self.big_endian = big_endian;
        self
    }

    /// 设置最大长度
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    /// 设置长度是否包含头部
    pub fn length_includes_header(mut self, includes: bool) -> Self {
        self.length_includes_header = includes;
        self
    }

    /// 编码长度值
    fn encode_length(&self, length: usize) -> Vec<u8> {
        // TODO: 根据 length_size 和字节序编码长度
        todo!("编码长度")
    }

    /// 解码长度值
    fn decode_length(&self, data: &[u8]) -> Option<usize> {
        // TODO: 从字节解析长度值
        todo!("解码长度")
    }

    /// 编码消息
    ///
    /// # 返回值
    /// 长度前缀 + 消息内容
    pub fn encode(&self, data: &[u8]) -> Option<Vec<u8>> {
        // TODO: 编码带长度前缀的消息
        // 1. 检查长度是否超过最大值
        // 2. 计算长度值（考虑是否包含头部）
        // 3. 编码长度
        // 4. 拼接长度和数据
        todo!("编码消息")
    }

    /// 解码消息
    ///
    /// # 返回值
    /// Ok(Some((message, consumed))) - 成功解码
    /// Ok(None) - 数据不足
    /// Err(msg) - 解码错误
    pub fn decode<'a>(&self, buffer: &'a [u8]) -> Result<Option<(&'a [u8], usize)>, &'static str> {
        // TODO: 解码消息
        // 1. 检查是否有足够的字节读取长度
        // 2. 读取长度
        // 3. 检查长度是否超过限制
        // 4. 检查是否有足够的数据
        // 5. 返回消息和消耗的字节数
        todo!("解码消息")
    }

    /// 获取头部大小
    pub fn header_size(&self) -> usize {
        self.length_size.bytes()
    }
}

/// TLV (Type-Length-Value) 消息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlvMessage {
    /// 消息类型
    pub msg_type: u16,
    /// 消息内容
    pub value: Vec<u8>,
}

/// TLV 编解码器
pub struct TlvCodec {
    /// 长度字段大小
    length_size: LengthFieldSize,
    /// 最大值长度
    max_value_length: usize,
}

impl TlvCodec {
    /// 创建 TLV 编解码器
    ///
    /// 格式: Type(2B) + Length(length_size) + Value
    pub fn new(length_size: LengthFieldSize, max_value_length: usize) -> Self {
        // TODO: 创建 TLV 编解码器
        todo!("创建 TLV 编解码器")
    }

    /// 编码 TLV 消息
    pub fn encode(&self, message: &TlvMessage) -> Option<Vec<u8>> {
        // TODO: 编码 TLV
        // 1. 编码 Type (2 字节, 大端)
        // 2. 编码 Length
        // 3. 追加 Value
        todo!("编码 TLV")
    }

    /// 解码 TLV 消息
    pub fn decode(&self, buffer: &[u8]) -> Result<Option<(TlvMessage, usize)>, &'static str> {
        // TODO: 解码 TLV
        todo!("解码 TLV")
    }

    /// 获取头部大小 (Type + Length)
    pub fn header_size(&self) -> usize {
        2 + self.length_size.bytes()
    }
}

/// 变长整数编码 (Varint)
///
/// 类似 Protocol Buffers 的变长编码:
/// - 每个字节的最高位表示是否还有后续字节
/// - 低 7 位是数据
///
/// 例如:
/// - 0-127: 1 字节
/// - 128-16383: 2 字节
/// - ...
pub mod varint {
    /// 编码变长整数
    ///
    /// # 示例
    /// ```
    /// assert_eq!(encode(0), vec![0x00]);
    /// assert_eq!(encode(127), vec![0x7F]);
    /// assert_eq!(encode(128), vec![0x80, 0x01]);
    /// assert_eq!(encode(300), vec![0xAC, 0x02]);
    /// ```
    pub fn encode(mut value: u64) -> Vec<u8> {
        // TODO: 编码变长整数
        // 每次取低 7 位，如果还有剩余值，设置最高位为 1
        todo!("编码 varint")
    }

    /// 解码变长整数
    ///
    /// # 返回值
    /// Some((value, consumed_bytes)) 或 None（如果数据不完整或无效）
    pub fn decode(data: &[u8]) -> Option<(u64, usize)> {
        // TODO: 解码变长整数
        todo!("解码 varint")
    }

    /// 计算编码后的长度
    pub fn encoded_len(value: u64) -> usize {
        // TODO: 计算需要多少字节
        todo!("计算 varint 长度")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framing_length_prefix_encode() {
        let codec = LengthPrefixCodec::new(LengthFieldSize::U32);

        let encoded = codec.encode(b"Hello").unwrap();
        assert_eq!(encoded.len(), 4 + 5); // 4字节长度 + 5字节数据
        assert_eq!(&encoded[0..4], &[0, 0, 0, 5]); // 大端序长度
        assert_eq!(&encoded[4..], b"Hello");
    }

    #[test]
    fn test_framing_length_prefix_encode_u16() {
        let codec = LengthPrefixCodec::new(LengthFieldSize::U16);

        let encoded = codec.encode(b"Hi").unwrap();
        assert_eq!(encoded.len(), 2 + 2);
        assert_eq!(&encoded[0..2], &[0, 2]);
    }

    #[test]
    fn test_framing_length_prefix_decode() {
        let codec = LengthPrefixCodec::new(LengthFieldSize::U32);

        // 完整消息
        let buffer = [0, 0, 0, 5, b'H', b'e', b'l', b'l', b'o', b'X', b'Y'];
        let (msg, consumed) = codec.decode(&buffer).unwrap().unwrap();
        assert_eq!(msg, b"Hello");
        assert_eq!(consumed, 9);

        // 数据不足
        let buffer = [0, 0, 0, 5, b'H', b'e'];
        assert!(codec.decode(&buffer).unwrap().is_none());

        // 头部不完整
        let buffer = [0, 0, 0];
        assert!(codec.decode(&buffer).unwrap().is_none());
    }

    #[test]
    fn test_framing_length_prefix_max_length() {
        let codec = LengthPrefixCodec::new(LengthFieldSize::U32).with_max_length(10);

        // 太长的消息
        let buffer = [0, 0, 0, 100, /* ... */];
        assert!(codec.decode(&buffer).is_err());
    }

    #[test]
    fn test_framing_length_prefix_little_endian() {
        let codec = LengthPrefixCodec::new(LengthFieldSize::U32).with_endian(false);

        let encoded = codec.encode(b"Hi").unwrap();
        assert_eq!(&encoded[0..4], &[2, 0, 0, 0]); // 小端序
    }

    #[test]
    fn test_framing_tlv_encode() {
        let codec = TlvCodec::new(LengthFieldSize::U16, 1024);

        let msg = TlvMessage {
            msg_type: 1,
            value: b"Hello".to_vec(),
        };

        let encoded = codec.encode(&msg).unwrap();
        assert_eq!(encoded.len(), 2 + 2 + 5); // Type + Length + Value
        assert_eq!(&encoded[0..2], &[0, 1]); // Type = 1
        assert_eq!(&encoded[2..4], &[0, 5]); // Length = 5
        assert_eq!(&encoded[4..], b"Hello");
    }

    #[test]
    fn test_framing_tlv_decode() {
        let codec = TlvCodec::new(LengthFieldSize::U16, 1024);

        let buffer = [0, 1, 0, 5, b'H', b'e', b'l', b'l', b'o'];
        let (msg, consumed) = codec.decode(&buffer).unwrap().unwrap();

        assert_eq!(msg.msg_type, 1);
        assert_eq!(msg.value, b"Hello");
        assert_eq!(consumed, 9);
    }

    #[test]
    fn test_framing_varint_encode() {
        assert_eq!(varint::encode(0), vec![0x00]);
        assert_eq!(varint::encode(1), vec![0x01]);
        assert_eq!(varint::encode(127), vec![0x7F]);
        assert_eq!(varint::encode(128), vec![0x80, 0x01]);
        assert_eq!(varint::encode(300), vec![0xAC, 0x02]); // 300 = 0x12C
    }

    #[test]
    fn test_framing_varint_decode() {
        assert_eq!(varint::decode(&[0x00]), Some((0, 1)));
        assert_eq!(varint::decode(&[0x7F]), Some((127, 1)));
        assert_eq!(varint::decode(&[0x80, 0x01]), Some((128, 2)));
        assert_eq!(varint::decode(&[0xAC, 0x02]), Some((300, 2)));

        // 不完整
        assert_eq!(varint::decode(&[0x80]), None);
    }

    #[test]
    fn test_framing_varint_roundtrip() {
        for value in [0, 1, 127, 128, 255, 256, 16383, 16384, 1000000] {
            let encoded = varint::encode(value);
            let (decoded, _) = varint::decode(&encoded).unwrap();
            assert_eq!(decoded, value);
        }
    }
}
