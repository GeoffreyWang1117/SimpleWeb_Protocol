//! # 练习 03: HTTP 分块传输编码
//!
//! Chunked Transfer Encoding 允许发送未知长度的响应。
//!
//! ## 为什么需要分块传输？
//!
//! - 服务器不知道响应总长度（动态生成的内容）
//! - 允许流式传输，减少内存占用
//! - 支持服务器推送
//!
//! ## 格式
//!
//! ```text
//! HTTP/1.1 200 OK
//! Transfer-Encoding: chunked
//!
//! 7\r\n
//! Mozilla\r\n
//! 9\r\n
//! Developer\r\n
//! 7\r\n
//! Network\r\n
//! 0\r\n
//! \r\n
//! ```
//!
//! 每个 chunk: `大小(十六进制)\r\n数据\r\n`
//! 结束标记: `0\r\n\r\n`
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test http_chunked
//! ```

/// 单个数据块
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    /// 数据
    pub data: Vec<u8>,
    /// 扩展（可选，如 chunk extensions）
    pub extensions: Option<String>,
}

impl Chunk {
    /// 创建数据块
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            extensions: None,
        }
    }

    /// 创建带扩展的数据块
    pub fn with_extensions(data: Vec<u8>, extensions: String) -> Self {
        Self {
            data,
            extensions: Some(extensions),
        }
    }

    /// 编码为字节
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码单个 chunk
        //
        // 格式: 大小(十六进制)[;扩展]\r\n数据\r\n
        //
        // 例如:
        // - Chunk { data: b"Hello" } -> "5\r\nHello\r\n"
        // - Chunk { data: b"Hi", extensions: "name=value" } -> "2;name=value\r\nHi\r\n"
        todo!("编码 chunk")
    }

    /// 是否是最后一个块（长度为0）
    pub fn is_last(&self) -> bool {
        self.data.is_empty()
    }
}

/// 分块编码器
pub struct ChunkedEncoder {
    /// 每个 chunk 的最大大小
    pub chunk_size: usize,
}

impl ChunkedEncoder {
    /// 创建编码器
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// 默认大小（8KB）
    pub fn default_size() -> Self {
        Self::new(8192)
    }

    /// 将数据编码为分块格式
    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        // TODO: 将数据编码为分块传输格式
        //
        // 1. 将数据分成 chunk_size 大小的块
        // 2. 每块编码为: 大小\r\n数据\r\n
        // 3. 添加结束块: 0\r\n\r\n
        todo!("编码数据")
    }

    /// 创建迭代器，支持流式编码
    pub fn encode_iter<'a>(&self, data: &'a [u8]) -> impl Iterator<Item = Vec<u8>> + 'a {
        let chunk_size = self.chunk_size;
        let mut offset = 0;
        let mut finished = false;

        std::iter::from_fn(move || {
            if finished {
                return None;
            }

            if offset >= data.len() {
                finished = true;
                // 返回结束块
                return Some(b"0\r\n\r\n".to_vec());
            }

            let end = (offset + chunk_size).min(data.len());
            let chunk = &data[offset..end];
            offset = end;

            // TODO: 编码这个块
            todo!("编码块")
        })
    }
}

impl Default for ChunkedEncoder {
    fn default() -> Self {
        Self::default_size()
    }
}

/// 分块解码器
pub struct ChunkedDecoder {
    /// 已解码的数据
    buffer: Vec<u8>,
    /// 解码状态
    state: DecoderState,
    /// 当前块的剩余字节数
    remaining: usize,
}

/// 解码器状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecoderState {
    /// 等待块大小
    WaitingSize,
    /// 读取块数据
    ReadingData,
    /// 等待块结束的 \r\n
    WaitingCrlf,
    /// 等待 trailer
    WaitingTrailer,
    /// 解码完成
    Complete,
    /// 错误状态
    Error,
}

impl ChunkedDecoder {
    /// 创建解码器
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            state: DecoderState::WaitingSize,
            remaining: 0,
        }
    }

    /// 输入数据进行解码
    pub fn decode(&mut self, input: &[u8]) -> Result<Option<Vec<u8>>, ChunkedError> {
        // TODO: 实现流式解码
        //
        // 状态机:
        // 1. WaitingSize: 读取十六进制大小，遇到 \r\n 后转到 ReadingData 或 Complete
        // 2. ReadingData: 读取 remaining 字节的数据
        // 3. WaitingCrlf: 跳过块结尾的 \r\n
        // 4. WaitingTrailer: 处理 trailer headers（如果有）
        // 5. Complete: 解码完成，返回数据
        todo!("流式解码")
    }

    /// 一次性解码完整的分块数据
    pub fn decode_all(data: &[u8]) -> Result<Vec<u8>, ChunkedError> {
        // TODO: 解码完整的分块数据
        //
        // 解析格式:
        // 大小\r\n数据\r\n大小\r\n数据\r\n...0\r\n\r\n
        todo!("完整解码")
    }

    /// 是否解码完成
    pub fn is_complete(&self) -> bool {
        self.state == DecoderState::Complete
    }

    /// 是否出错
    pub fn is_error(&self) -> bool {
        self.state == DecoderState::Error
    }
}

impl Default for ChunkedDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// 分块解码错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkedError {
    /// 无效的块大小
    InvalidChunkSize,
    /// 意外的数据结尾
    UnexpectedEof,
    /// 无效的格式
    InvalidFormat,
    /// 块太大
    ChunkTooLarge(usize),
}

/// 解析十六进制块大小
pub fn parse_chunk_size(line: &str) -> Result<usize, ChunkedError> {
    // TODO: 解析块大小
    //
    // 输入: "7" 或 "1a" 或 "10;name=value"
    // 输出: 7, 26, 16
    //
    // 注意处理 chunk extensions（分号后的部分）
    todo!("解析块大小")
}

/// Trailer headers（在结束块之后的 headers）
#[derive(Debug, Clone, Default)]
pub struct Trailers {
    pub headers: Vec<(String, String)>,
}

impl Trailers {
    /// 解析 trailers
    pub fn parse(data: &[u8]) -> Option<Self> {
        // TODO: 解析 trailer headers
        // 格式与普通 headers 相同
        todo!("解析 trailers")
    }

    /// 编码 trailers
    pub fn encode(&self) -> Vec<u8> {
        // TODO: 编码 trailers
        todo!("编码 trailers")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_chunked_encode_chunk() {
        let chunk = Chunk::new(b"Hello".to_vec());
        assert_eq!(chunk.encode(), b"5\r\nHello\r\n");

        let last = Chunk::new(vec![]);
        assert_eq!(last.encode(), b"0\r\n");
        assert!(last.is_last());
    }

    #[test]
    fn test_http_chunked_encode_full() {
        let encoder = ChunkedEncoder::new(5);
        let data = b"HelloWorld";
        let encoded = encoder.encode(data);

        // 应该产生: "5\r\nHello\r\n5\r\nWorld\r\n0\r\n\r\n"
        assert!(encoded.starts_with(b"5\r\nHello\r\n"));
        assert!(encoded.ends_with(b"0\r\n\r\n"));
    }

    #[test]
    fn test_http_chunked_decode() {
        let encoded = b"5\r\nHello\r\n5\r\nWorld\r\n0\r\n\r\n";
        let decoded = ChunkedDecoder::decode_all(encoded).unwrap();
        assert_eq!(decoded, b"HelloWorld");
    }

    #[test]
    fn test_http_chunked_decode_hex() {
        // 使用十六进制大小
        let encoded = b"a\r\n0123456789\r\n0\r\n\r\n";
        let decoded = ChunkedDecoder::decode_all(encoded).unwrap();
        assert_eq!(decoded.len(), 10);
    }

    #[test]
    fn test_http_chunked_parse_size() {
        assert_eq!(parse_chunk_size("5").unwrap(), 5);
        assert_eq!(parse_chunk_size("1a").unwrap(), 26);
        assert_eq!(parse_chunk_size("10").unwrap(), 16);
        assert_eq!(parse_chunk_size("ff").unwrap(), 255);
        assert_eq!(parse_chunk_size("10;name=value").unwrap(), 16);
    }

    #[test]
    fn test_http_chunked_roundtrip() {
        let original = b"The quick brown fox jumps over the lazy dog";
        let encoder = ChunkedEncoder::new(10);
        let encoded = encoder.encode(original);
        let decoded = ChunkedDecoder::decode_all(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
