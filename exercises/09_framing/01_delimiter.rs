//! # 练习 01: 分隔符分帧
//!
//! 使用特殊字符或字符序列作为消息边界。
//!
//! ## 常见分隔符
//!
//! - `\n` (LF): Unix 风格行结束
//! - `\r\n` (CRLF): Windows/网络协议（HTTP, SMTP, Redis）
//! - `\0` (NUL): C 风格字符串
//! - 自定义序列: 如 `|||` 或 `END`
//!
//! ## 优点
//! - 简单直观
//! - 适合文本协议
//! - 人类可读（方便调试）
//!
//! ## 缺点
//! - 需要转义消息中的分隔符
//! - 解析效率略低（需要扫描）
//! - 不适合二进制数据
//!
//! ## 使用场景
//! - Redis 协议（RESP）
//! - 行式日志协议
//! - 简单的文本命令协议
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test framing_delimiter
//! ```

/// 分隔符类型
#[derive(Debug, Clone)]
pub enum Delimiter {
    /// 单字节分隔符
    Byte(u8),
    /// 多字节分隔符
    Bytes(Vec<u8>),
}

impl Delimiter {
    /// 换行符 (\n)
    pub fn lf() -> Self {
        Delimiter::Byte(b'\n')
    }

    /// 回车换行 (\r\n)
    pub fn crlf() -> Self {
        Delimiter::Bytes(vec![b'\r', b'\n'])
    }

    /// 空字符 (\0)
    pub fn nul() -> Self {
        Delimiter::Byte(0)
    }

    /// 获取分隔符长度
    pub fn len(&self) -> usize {
        match self {
            Delimiter::Byte(_) => 1,
            Delimiter::Bytes(b) => b.len(),
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 转换为字节切片
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Delimiter::Byte(b) => std::slice::from_ref(b),
            Delimiter::Bytes(b) => b,
        }
    }
}

/// 分隔符分帧编解码器
pub struct DelimiterCodec {
    delimiter: Delimiter,
    max_length: usize,
}

impl DelimiterCodec {
    /// 创建分隔符编解码器
    ///
    /// # 参数
    /// * `delimiter` - 分隔符
    /// * `max_length` - 单条消息的最大长度（防止内存耗尽）
    pub fn new(delimiter: Delimiter, max_length: usize) -> Self {
        // TODO: 创建编解码器
        todo!("创建分隔符编解码器")
    }

    /// 创建行分隔编解码器（使用 \n）
    pub fn lines(max_length: usize) -> Self {
        Self::new(Delimiter::lf(), max_length)
    }

    /// 创建 CRLF 分隔编解码器（使用 \r\n）
    pub fn crlf_lines(max_length: usize) -> Self {
        Self::new(Delimiter::crlf(), max_length)
    }

    /// 编码消息（添加分隔符）
    ///
    /// # 返回值
    /// 消息 + 分隔符
    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        // TODO: 在消息末尾添加分隔符
        todo!("编码带分隔符的消息")
    }

    /// 在缓冲区中查找分隔符位置
    ///
    /// # 返回值
    /// 如果找到分隔符，返回 Some(分隔符起始位置)
    pub fn find_delimiter(&self, buffer: &[u8]) -> Option<usize> {
        // TODO: 在缓冲区中搜索分隔符
        //
        // 对于单字节分隔符：使用 memchr 或简单遍历
        // 对于多字节分隔符：使用滑动窗口匹配
        todo!("查找分隔符")
    }

    /// 从缓冲区提取下一条消息
    ///
    /// # 返回值
    /// Ok(Some((message, consumed))) - 成功提取消息
    /// Ok(None) - 缓冲区中没有完整消息
    /// Err(msg) - 消息太长
    pub fn decode<'a>(&self, buffer: &'a [u8]) -> Result<Option<(&'a [u8], usize)>, &'static str> {
        // TODO: 从缓冲区提取消息
        //
        // 1. 查找分隔符
        // 2. 如果找到，检查长度是否超过 max_length
        // 3. 返回消息（不包含分隔符）和消耗的总字节数
        todo!("解码带分隔符的消息")
    }
}

/// 分隔符消息读取器
pub struct DelimiterReader {
    codec: DelimiterCodec,
    buffer: Vec<u8>,
}

impl DelimiterReader {
    pub fn new(delimiter: Delimiter, max_length: usize) -> Self {
        // TODO: 创建读取器
        todo!("创建分隔符读取器")
    }

    /// 创建行读取器
    pub fn lines(max_length: usize) -> Self {
        Self::new(Delimiter::lf(), max_length)
    }

    /// 向缓冲区添加数据
    pub fn feed(&mut self, data: &[u8]) {
        // TODO: 添加数据到缓冲区
        todo!("添加数据")
    }

    /// 尝试读取下一条完整消息
    pub fn next_message(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        // TODO: 提取下一条消息
        todo!("读取下一条消息")
    }

    /// 清空缓冲区
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// 获取缓冲区内容（用于调试）
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

/// 转义消息中的分隔符
///
/// # 参数
/// * `data` - 原始数据
/// * `delimiter` - 需要转义的分隔符
/// * `escape` - 转义字符
///
/// # 示例
/// ```
/// // 将消息中的 \n 转义为 \\n
/// let escaped = escape_delimiter(b"hello\nworld", b'\n', b'\\');
/// assert_eq!(escaped, b"hello\\nworld");
/// ```
pub fn escape_delimiter(data: &[u8], delimiter: u8, escape: u8) -> Vec<u8> {
    // TODO: 转义分隔符
    // 同时需要转义转义字符本身
    todo!("转义分隔符")
}

/// 反转义消息
pub fn unescape_delimiter(data: &[u8], escape: u8) -> Vec<u8> {
    // TODO: 反转义
    todo!("反转义分隔符")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framing_delimiter_encode() {
        let codec = DelimiterCodec::lines(1024);

        let encoded = codec.encode(b"Hello");
        assert_eq!(encoded, b"Hello\n");
    }

    #[test]
    fn test_framing_delimiter_encode_crlf() {
        let codec = DelimiterCodec::crlf_lines(1024);

        let encoded = codec.encode(b"Hello");
        assert_eq!(encoded, b"Hello\r\n");
    }

    #[test]
    fn test_framing_delimiter_find() {
        let codec = DelimiterCodec::lines(1024);

        assert_eq!(codec.find_delimiter(b"Hello\nWorld"), Some(5));
        assert_eq!(codec.find_delimiter(b"No newline"), None);
        assert_eq!(codec.find_delimiter(b"\n"), Some(0));
    }

    #[test]
    fn test_framing_delimiter_find_crlf() {
        let codec = DelimiterCodec::crlf_lines(1024);

        assert_eq!(codec.find_delimiter(b"Hello\r\nWorld"), Some(5));
        assert_eq!(codec.find_delimiter(b"Hello\nWorld"), None); // 只有 \n 不行
        assert_eq!(codec.find_delimiter(b"Hello\rWorld"), None); // 只有 \r 不行
    }

    #[test]
    fn test_framing_delimiter_decode() {
        let codec = DelimiterCodec::lines(1024);

        // 完整消息
        let result = codec.decode(b"Hello\nWorld\n").unwrap();
        let (msg, consumed) = result.unwrap();
        assert_eq!(msg, b"Hello");
        assert_eq!(consumed, 6); // "Hello" + "\n"

        // 无完整消息
        let result = codec.decode(b"No newline").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_framing_delimiter_decode_too_long() {
        let codec = DelimiterCodec::lines(5);

        // 消息太长
        let result = codec.decode(b"TooLongMessage\n");
        assert!(result.is_err());
    }

    #[test]
    fn test_framing_delimiter_reader() {
        let mut reader = DelimiterReader::lines(1024);

        // 分次接收
        reader.feed(b"Hel");
        assert!(reader.next_message().unwrap().is_none());

        reader.feed(b"lo\nWor");
        let msg = reader.next_message().unwrap().unwrap();
        assert_eq!(msg, b"Hello");

        reader.feed(b"ld\n");
        let msg = reader.next_message().unwrap().unwrap();
        assert_eq!(msg, b"World");
    }

    #[test]
    fn test_framing_delimiter_escape() {
        let escaped = escape_delimiter(b"hello\nworld", b'\n', b'\\');
        assert_eq!(escaped, b"hello\\nworld");

        // 转义字符本身也要转义
        let escaped = escape_delimiter(b"back\\slash", b'\n', b'\\');
        assert_eq!(escaped, b"back\\\\slash");
    }

    #[test]
    fn test_framing_delimiter_unescape() {
        let unescaped = unescape_delimiter(b"hello\\nworld", b'\\');
        assert_eq!(unescaped, b"hello\nworld");

        let unescaped = unescape_delimiter(b"back\\\\slash", b'\\');
        assert_eq!(unescaped, b"back\\slash");
    }
}
