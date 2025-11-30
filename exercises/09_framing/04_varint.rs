//! # 练习 04: 变长整数编码
//!
//! 变长整数（Varint）是一种压缩整数的编码方式，用于节省空间。
//!
//! ## 为什么需要变长整数？
//!
//! - 小数字很常见，用固定32/64位浪费空间
//! - 网络协议需要紧凑的编码
//! - Protocol Buffers, QUIC, SQLite 都使用 varint
//!
//! ## 常见的 Varint 编码
//!
//! ### LEB128 (Little Endian Base 128)
//! - 每字节7位数据 + 1位继续标志
//! - Protocol Buffers 使用此格式
//!
//! ### QUIC Varint
//! - 前2位表示长度: 00=1字节, 01=2字节, 10=4字节, 11=8字节
//! - RFC 9000 定义
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test framing_varint
//! ```

/// LEB128 无符号变长整数编码
pub mod leb128 {
    /// 编码无符号整数
    ///
    /// 每字节:
    /// - bit 7: 继续标志 (1=还有更多字节, 0=最后一字节)
    /// - bit 0-6: 7位数据
    ///
    /// 例如:
    /// - 0 -> [0x00]
    /// - 127 -> [0x7F]
    /// - 128 -> [0x80, 0x01]
    /// - 300 -> [0xAC, 0x02]
    pub fn encode_unsigned(mut value: u64) -> Vec<u8> {
        // TODO: 编码无符号整数
        //
        // while value >= 0x80:
        //     output.push((value & 0x7F) | 0x80)
        //     value >>= 7
        // output.push(value & 0x7F)
        todo!("编码 LEB128 无符号")
    }

    /// 解码无符号整数
    ///
    /// 返回 (值, 消耗的字节数)
    pub fn decode_unsigned(data: &[u8]) -> Option<(u64, usize)> {
        // TODO: 解码无符号整数
        //
        // result = 0, shift = 0
        // for byte in data:
        //     result |= (byte & 0x7F) << shift
        //     if byte & 0x80 == 0:
        //         return (result, bytes_read)
        //     shift += 7
        todo!("解码 LEB128 无符号")
    }

    /// 编码有符号整数 (使用 ZigZag 编码)
    ///
    /// ZigZag 编码将有符号数映射到无符号数:
    /// 0 -> 0, -1 -> 1, 1 -> 2, -2 -> 3, 2 -> 4, ...
    ///
    /// 公式: (n << 1) ^ (n >> 63)
    pub fn encode_signed(value: i64) -> Vec<u8> {
        // TODO: 使用 ZigZag + LEB128 编码有符号整数
        todo!("编码 LEB128 有符号")
    }

    /// 解码有符号整数
    pub fn decode_signed(data: &[u8]) -> Option<(i64, usize)> {
        // TODO: 解码有符号整数
        // 先解码为无符号，再反 ZigZag
        // (n >> 1) ^ -(n & 1)
        todo!("解码 LEB128 有符号")
    }

    /// 计算编码所需字节数
    pub fn encoded_size(value: u64) -> usize {
        // TODO: 计算编码后的长度
        todo!("计算编码长度")
    }
}

/// QUIC 风格变长整数 (RFC 9000)
pub mod quic_varint {
    /// 编码 QUIC varint
    ///
    /// 前2位表示长度:
    /// - 00: 1字节, 6位值 (0-63)
    /// - 01: 2字节, 14位值 (0-16383)
    /// - 10: 4字节, 30位值 (0-1073741823)
    /// - 11: 8字节, 62位值
    pub fn encode(value: u64) -> Vec<u8> {
        // TODO: 编码 QUIC varint
        //
        // if value <= 63:
        //     return [value as u8]
        // elif value <= 16383:
        //     return [(value >> 8) | 0x40, value & 0xFF]
        // ...
        todo!("编码 QUIC varint")
    }

    /// 解码 QUIC varint
    pub fn decode(data: &[u8]) -> Option<(u64, usize)> {
        // TODO: 解码 QUIC varint
        //
        // 读取第一字节，根据前2位确定长度
        // 清除前2位，读取剩余字节
        todo!("解码 QUIC varint")
    }

    /// 计算编码所需字节数
    pub fn encoded_size(value: u64) -> usize {
        // TODO: 计算编码长度
        if value <= 63 {
            1
        } else if value <= 16383 {
            2
        } else if value <= 1073741823 {
            4
        } else {
            8
        }
    }

    /// 最大可编码值
    pub const MAX_VALUE: u64 = (1 << 62) - 1;
}

/// SQLite 风格变长整数
pub mod sqlite_varint {
    /// 编码 SQLite varint (1-9 字节)
    ///
    /// - 0-240: 1字节
    /// - 241-2287: 2字节, 第一字节=241+(值-240)/256
    /// - ...
    /// - 大数字: 第一字节=255, 后面8字节大端序
    pub fn encode(value: u64) -> Vec<u8> {
        // TODO: 编码 SQLite varint
        todo!("编码 SQLite varint")
    }

    /// 解码 SQLite varint
    pub fn decode(data: &[u8]) -> Option<(u64, usize)> {
        // TODO: 解码 SQLite varint
        todo!("解码 SQLite varint")
    }
}

/// 通用变长整数读写器
pub struct VarintReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> VarintReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// 读取 LEB128 无符号整数
    pub fn read_leb128_unsigned(&mut self) -> Option<u64> {
        // TODO: 读取并前进位置
        todo!("读取 LEB128")
    }

    /// 读取 QUIC varint
    pub fn read_quic_varint(&mut self) -> Option<u64> {
        // TODO: 读取并前进位置
        todo!("读取 QUIC varint")
    }

    /// 获取当前位置
    pub fn position(&self) -> usize {
        self.pos
    }

    /// 剩余字节数
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

/// 通用变长整数写入器
pub struct VarintWriter {
    buffer: Vec<u8>,
}

impl VarintWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// 写入 LEB128 无符号整数
    pub fn write_leb128_unsigned(&mut self, value: u64) {
        // TODO: 写入 LEB128
        todo!("写入 LEB128")
    }

    /// 写入 QUIC varint
    pub fn write_quic_varint(&mut self, value: u64) {
        // TODO: 写入 QUIC varint
        todo!("写入 QUIC varint")
    }

    /// 获取缓冲区
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    /// 获取引用
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for VarintWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framing_varint_leb128_small() {
        // 小数字
        assert_eq!(leb128::encode_unsigned(0), vec![0x00]);
        assert_eq!(leb128::encode_unsigned(1), vec![0x01]);
        assert_eq!(leb128::encode_unsigned(127), vec![0x7F]);
    }

    #[test]
    fn test_framing_varint_leb128_large() {
        // 需要多字节
        assert_eq!(leb128::encode_unsigned(128), vec![0x80, 0x01]);
        assert_eq!(leb128::encode_unsigned(300), vec![0xAC, 0x02]);
        assert_eq!(leb128::encode_unsigned(16384), vec![0x80, 0x80, 0x01]);
    }

    #[test]
    fn test_framing_varint_leb128_decode() {
        assert_eq!(leb128::decode_unsigned(&[0x00]), Some((0, 1)));
        assert_eq!(leb128::decode_unsigned(&[0x7F]), Some((127, 1)));
        assert_eq!(leb128::decode_unsigned(&[0x80, 0x01]), Some((128, 2)));
        assert_eq!(leb128::decode_unsigned(&[0xAC, 0x02]), Some((300, 2)));
    }

    #[test]
    fn test_framing_varint_leb128_signed() {
        // ZigZag 编码
        assert_eq!(leb128::encode_signed(0), vec![0x00]);
        assert_eq!(leb128::encode_signed(-1), vec![0x01]);
        assert_eq!(leb128::encode_signed(1), vec![0x02]);
        assert_eq!(leb128::encode_signed(-2), vec![0x03]);

        assert_eq!(leb128::decode_signed(&[0x00]), Some((0, 1)));
        assert_eq!(leb128::decode_signed(&[0x01]), Some((-1, 1)));
        assert_eq!(leb128::decode_signed(&[0x02]), Some((1, 1)));
    }

    #[test]
    fn test_framing_varint_quic() {
        // 1字节
        assert_eq!(quic_varint::encode(0), vec![0x00]);
        assert_eq!(quic_varint::encode(37), vec![0x25]);
        assert_eq!(quic_varint::encode(63), vec![0x3F]);

        // 2字节
        assert_eq!(quic_varint::encode(15293), vec![0x7B, 0xBD]);

        // 解码
        assert_eq!(quic_varint::decode(&[0x25]), Some((37, 1)));
        assert_eq!(quic_varint::decode(&[0x7B, 0xBD]), Some((15293, 2)));
    }

    #[test]
    fn test_framing_varint_roundtrip() {
        for value in [0u64, 1, 127, 128, 255, 256, 16383, 16384, 1000000] {
            // LEB128
            let encoded = leb128::encode_unsigned(value);
            let (decoded, _) = leb128::decode_unsigned(&encoded).unwrap();
            assert_eq!(decoded, value);

            // QUIC
            if value <= quic_varint::MAX_VALUE {
                let encoded = quic_varint::encode(value);
                let (decoded, _) = quic_varint::decode(&encoded).unwrap();
                assert_eq!(decoded, value);
            }
        }
    }

    #[test]
    fn test_framing_varint_size() {
        assert_eq!(leb128::encoded_size(0), 1);
        assert_eq!(leb128::encoded_size(127), 1);
        assert_eq!(leb128::encoded_size(128), 2);
        assert_eq!(leb128::encoded_size(16383), 2);
        assert_eq!(leb128::encoded_size(16384), 3);
    }
}
