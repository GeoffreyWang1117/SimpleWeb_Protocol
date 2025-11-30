//! # 练习 03: 编解码器抽象
//!
//! 创建通用的编解码器接口，支持异步流处理。
//!
//! ## 编解码器模式
//!
//! ```text
//! +----------+    encode    +----------+
//! | Message  | -----------> |  Bytes   |
//! +----------+              +----------+
//!      ^                         |
//!      |        decode           |
//!      +-------------------------+
//! ```
//!
//! ## 流式处理
//!
//! ```text
//! TCP Stream --> [Buffer] --> [Decoder] --> Messages
//!                   ^
//!                   |
//!              feed(bytes)
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test framing_codec
//! ```

use std::io::{self, Read, Write};

/// 编码器 trait
pub trait Encoder {
    /// 消息类型
    type Item;
    /// 编码错误类型
    type Error;

    /// 将消息编码为字节
    fn encode(&mut self, item: Self::Item, dst: &mut Vec<u8>) -> Result<(), Self::Error>;
}

/// 解码器 trait
pub trait Decoder {
    /// 消息类型
    type Item;
    /// 解码错误类型
    type Error;

    /// 尝试从缓冲区解码消息
    ///
    /// # 返回值
    /// - Ok(Some((item, consumed))) - 成功解码
    /// - Ok(None) - 数据不足，需要更多输入
    /// - Err(e) - 解码错误
    fn decode(&mut self, src: &[u8]) -> Result<Option<(Self::Item, usize)>, Self::Error>;
}

/// 编解码器 trait（结合编码和解码）
pub trait Codec: Encoder + Decoder {}

impl<T: Encoder + Decoder> Codec for T {}

/// 简单的字符串行编解码器
///
/// 用于演示编解码器接口的实现
#[derive(Debug, Default)]
pub struct LinesCodec {
    max_length: usize,
}

impl LinesCodec {
    pub fn new(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl Encoder for LinesCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut Vec<u8>) -> Result<(), Self::Error> {
        // TODO: 编码字符串，添加换行符
        todo!("编码行")
    }
}

impl Decoder for LinesCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &[u8]) -> Result<Option<(Self::Item, usize)>, Self::Error> {
        // TODO: 解码一行
        // 1. 查找 \n
        // 2. 提取行（不包含 \n）
        // 3. 转换为 String
        todo!("解码行")
    }
}

/// 带缓冲的流读取器
///
/// 将字节流转换为消息流
pub struct FramedReader<R, D> {
    reader: R,
    decoder: D,
    buffer: Vec<u8>,
    read_buffer: Vec<u8>,
}

impl<R: Read, D: Decoder> FramedReader<R, D> {
    /// 创建带缓冲的读取器
    pub fn new(reader: R, decoder: D) -> Self {
        Self::with_capacity(reader, decoder, 4096)
    }

    /// 创建带指定缓冲区大小的读取器
    pub fn with_capacity(reader: R, decoder: D, capacity: usize) -> Self {
        // TODO: 创建读取器
        todo!("创建带缓冲的读取器")
    }

    /// 读取下一条消息
    ///
    /// 这个方法会阻塞直到读取到完整的消息或发生错误
    pub fn read_message(&mut self) -> Result<Option<D::Item>, D::Error>
    where
        D::Error: From<io::Error>,
    {
        // TODO: 读取消息
        //
        // 循环:
        // 1. 尝试从缓冲区解码消息
        // 2. 如果成功，移除已消耗的数据并返回消息
        // 3. 如果数据不足，从 reader 读取更多数据
        // 4. 如果 reader 返回 0（EOF），返回 None
        todo!("读取消息")
    }

    /// 获取底层读取器的引用
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// 获取底层读取器的可变引用
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }
}

/// 带缓冲的流写入器
pub struct FramedWriter<W, E> {
    writer: W,
    encoder: E,
    buffer: Vec<u8>,
}

impl<W: Write, E: Encoder> FramedWriter<W, E> {
    pub fn new(writer: W, encoder: E) -> Self {
        // TODO: 创建写入器
        todo!("创建带缓冲的写入器")
    }

    /// 发送消息
    pub fn send(&mut self, item: E::Item) -> Result<(), E::Error>
    where
        E::Error: From<io::Error>,
    {
        // TODO: 发送消息
        // 1. 编码消息到缓冲区
        // 2. 写入到 writer
        // 3. flush
        todo!("发送消息")
    }

    /// 刷新缓冲区
    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// 双向带缓冲的流
pub struct Framed<T, C> {
    inner: T,
    codec: C,
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
}

impl<T: Read + Write, C: Codec> Framed<T, C> {
    pub fn new(inner: T, codec: C) -> Self {
        // TODO: 创建双向流
        todo!("创建 Framed")
    }

    /// 读取下一条消息
    pub fn read(&mut self) -> Result<Option<C::Item>, C::Error>
    where
        C::Error: From<io::Error>,
    {
        // TODO: 读取消息
        todo!("读取")
    }

    /// 发送消息
    pub fn write(&mut self, item: <C as Encoder>::Item) -> Result<(), C::Error>
    where
        C::Error: From<io::Error>,
    {
        // TODO: 发送消息
        todo!("写入")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_framing_codec_lines_encode() {
        let mut codec = LinesCodec::new(1024);
        let mut dst = Vec::new();

        codec.encode("Hello".to_string(), &mut dst).unwrap();
        assert_eq!(dst, b"Hello\n");

        codec.encode("World".to_string(), &mut dst).unwrap();
        assert_eq!(dst, b"Hello\nWorld\n");
    }

    #[test]
    fn test_framing_codec_lines_decode() {
        let mut codec = LinesCodec::new(1024);

        // 完整行
        let (line, consumed) = codec.decode(b"Hello\nWorld\n").unwrap().unwrap();
        assert_eq!(line, "Hello");
        assert_eq!(consumed, 6);

        // 不完整
        assert!(codec.decode(b"No newline").unwrap().is_none());
    }

    #[test]
    fn test_framing_codec_framed_reader() {
        let data = b"Line1\nLine2\nLine3\n";
        let cursor = Cursor::new(data.to_vec());
        let codec = LinesCodec::new(1024);

        let mut reader = FramedReader::new(cursor, codec);

        assert_eq!(reader.read_message().unwrap(), Some("Line1".to_string()));
        assert_eq!(reader.read_message().unwrap(), Some("Line2".to_string()));
        assert_eq!(reader.read_message().unwrap(), Some("Line3".to_string()));
        assert_eq!(reader.read_message().unwrap(), None); // EOF
    }

    #[test]
    fn test_framing_codec_framed_writer() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let codec = LinesCodec::new(1024);

        let mut writer = FramedWriter::new(cursor, codec);

        writer.send("Hello".to_string()).unwrap();
        writer.send("World".to_string()).unwrap();

        let result = writer.writer.into_inner();
        assert_eq!(result, b"Hello\nWorld\n");
    }
}
