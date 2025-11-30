//! # 练习 00: 固定长度分帧
//!
//! 最简单的分帧方式：每条消息都是固定长度。
//!
//! ## 优点
//! - 实现简单
//! - 解析高效
//! - 无需特殊的边界标记
//!
//! ## 缺点
//! - 浪费空间（短消息需要填充）
//! - 不灵活（长消息需要分割）
//!
//! ## 使用场景
//! - 金融交易协议（FIX）
//! - 游戏协议中的固定格式命令
//! - 硬件通信协议
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test framing_fixed
//! ```

/// 固定长度消息编码器/解码器
pub struct FixedLengthCodec {
    /// 每条消息的固定长度
    message_size: usize,
    /// 填充字节
    padding_byte: u8,
}

impl FixedLengthCodec {
    /// 创建固定长度编解码器
    ///
    /// # 参数
    /// * `message_size` - 每条消息的固定长度
    /// * `padding_byte` - 用于填充短消息的字节（通常是 0x00 或空格）
    pub fn new(message_size: usize, padding_byte: u8) -> Self {
        // TODO: 创建编解码器
        todo!("创建固定长度编解码器")
    }

    /// 编码消息
    ///
    /// 如果消息太短，用 padding_byte 填充
    /// 如果消息太长，返回 None
    ///
    /// # 示例
    /// ```
    /// let codec = FixedLengthCodec::new(8, 0x00);
    /// let encoded = codec.encode(b"Hello").unwrap();
    /// assert_eq!(encoded, vec![b'H', b'e', b'l', b'l', b'o', 0, 0, 0]);
    /// ```
    pub fn encode(&self, data: &[u8]) -> Option<Vec<u8>> {
        // TODO: 编码消息
        // 1. 检查长度是否超过 message_size
        // 2. 创建 message_size 长度的向量
        // 3. 复制数据
        // 4. 填充剩余空间
        todo!("编码固定长度消息")
    }

    /// 解码消息（移除填充）
    ///
    /// # 参数
    /// * `data` - 固定长度的原始数据
    ///
    /// # 返回值
    /// 移除尾部填充后的数据
    pub fn decode(&self, data: &[u8]) -> Option<Vec<u8>> {
        // TODO: 解码消息
        // 1. 检查长度是否正确
        // 2. 找到最后一个非填充字节
        // 3. 返回有效数据
        todo!("解码固定长度消息")
    }

    /// 从字节流中提取下一条消息
    ///
    /// # 参数
    /// * `buffer` - 输入缓冲区
    ///
    /// # 返回值
    /// 如果缓冲区有足够数据，返回 (消息, 剩余缓冲区)
    pub fn extract<'a>(&self, buffer: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
        // TODO: 从缓冲区提取一条消息
        // 如果缓冲区长度 >= message_size，返回 (前 message_size 字节, 剩余字节)
        todo!("提取固定长度消息")
    }
}

/// 固定长度消息读取器
///
/// 用于从流中读取固定长度的消息
pub struct FixedLengthReader {
    codec: FixedLengthCodec,
    buffer: Vec<u8>,
}

impl FixedLengthReader {
    pub fn new(message_size: usize) -> Self {
        // TODO: 创建读取器
        todo!("创建固定长度读取器")
    }

    /// 向缓冲区添加数据
    pub fn feed(&mut self, data: &[u8]) {
        // TODO: 将数据添加到缓冲区
        todo!("添加数据到缓冲区")
    }

    /// 尝试读取下一条完整消息
    ///
    /// # 返回值
    /// 如果有完整消息返回 Some(message)，否则返回 None
    pub fn next_message(&mut self) -> Option<Vec<u8>> {
        // TODO: 尝试从缓冲区提取消息
        // 如果成功，从缓冲区移除已处理的数据
        todo!("读取下一条消息")
    }

    /// 检查缓冲区中是否有完整消息
    pub fn has_message(&self) -> bool {
        self.buffer.len() >= self.codec.message_size
    }

    /// 获取缓冲区中待处理的字节数
    pub fn pending_bytes(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framing_fixed_codec_new() {
        let codec = FixedLengthCodec::new(16, 0x00);
        assert_eq!(codec.message_size, 16);
        assert_eq!(codec.padding_byte, 0x00);
    }

    #[test]
    fn test_framing_fixed_encode() {
        let codec = FixedLengthCodec::new(8, 0x00);

        // 正好 8 字节
        let encoded = codec.encode(b"12345678").unwrap();
        assert_eq!(encoded.len(), 8);
        assert_eq!(encoded, b"12345678");

        // 短消息需要填充
        let encoded = codec.encode(b"Hi").unwrap();
        assert_eq!(encoded, vec![b'H', b'i', 0, 0, 0, 0, 0, 0]);

        // 空消息
        let encoded = codec.encode(b"").unwrap();
        assert_eq!(encoded, vec![0; 8]);

        // 太长的消息
        assert!(codec.encode(b"123456789").is_none());
    }

    #[test]
    fn test_framing_fixed_decode() {
        let codec = FixedLengthCodec::new(8, 0x00);

        // 解码带填充的消息
        let decoded = codec.decode(&[b'H', b'i', 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(decoded, b"Hi");

        // 解码无填充的消息
        let decoded = codec.decode(b"12345678").unwrap();
        assert_eq!(decoded, b"12345678");

        // 长度不对
        assert!(codec.decode(b"short").is_none());
    }

    #[test]
    fn test_framing_fixed_extract() {
        let codec = FixedLengthCodec::new(4, 0x00);

        // 正好一条消息
        let (msg, rest) = codec.extract(b"ABCD").unwrap();
        assert_eq!(msg, b"ABCD");
        assert_eq!(rest, b"");

        // 多条消息
        let (msg, rest) = codec.extract(b"ABCDEFGH").unwrap();
        assert_eq!(msg, b"ABCD");
        assert_eq!(rest, b"EFGH");

        // 不够一条消息
        assert!(codec.extract(b"AB").is_none());
    }

    #[test]
    fn test_framing_fixed_reader() {
        let mut reader = FixedLengthReader::new(4);

        // 分多次接收数据
        reader.feed(b"AB");
        assert!(!reader.has_message());
        assert!(reader.next_message().is_none());

        reader.feed(b"CD");
        assert!(reader.has_message());

        let msg = reader.next_message().unwrap();
        assert_eq!(msg, b"ABCD");
        assert!(!reader.has_message());
    }

    #[test]
    fn test_framing_fixed_reader_multiple() {
        let mut reader = FixedLengthReader::new(4);

        // 一次接收多条消息
        reader.feed(b"ABCDEFGHIJ");

        let msg1 = reader.next_message().unwrap();
        assert_eq!(msg1, b"ABCD");

        let msg2 = reader.next_message().unwrap();
        assert_eq!(msg2, b"EFGH");

        assert!(reader.next_message().is_none());
        assert_eq!(reader.pending_bytes(), 2); // "IJ" 还在缓冲区
    }
}
