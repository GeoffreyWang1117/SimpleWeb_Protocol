//! # 练习 01: 字节序（Endianness）
//!
//! 字节序是指多字节数据在内存中的存储顺序。
//!
//! ## 知识点
//!
//! ### 大端序（Big Endian）- 网络字节序
//! 高位字节存储在低地址。人类阅读数字的自然顺序。
//! ```text
//! 数字: 0x12345678
//! 地址: [0]  [1]  [2]  [3]
//! 值:   0x12 0x34 0x56 0x78
//! ```
//!
//! ### 小端序（Little Endian）- 主机字节序（大多数PC）
//! 低位字节存储在低地址。x86/x64 处理器使用的顺序。
//! ```text
//! 数字: 0x12345678
//! 地址: [0]  [1]  [2]  [3]
//! 值:   0x78 0x56 0x34 0x12
//! ```
//!
//! ## 网络编程中的字节序
//!
//! 网络协议（如 IP、TCP、UDP）使用大端序（网络字节序）。
//! 当发送和接收数据时，需要在主机字节序和网络字节序之间转换。
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test binary_endianness
//! ```

/// 将 u16 从主机字节序转换为网络字节序（大端序）
///
/// # 参数
/// * `value` - 主机字节序的 u16 值
///
/// # 返回值
/// 网络字节序（大端序）的字节数组
///
/// # 示例
/// ```
/// let bytes = u16_to_be_bytes(0x1234);
/// assert_eq!(bytes, [0x12, 0x34]);
/// ```
pub fn u16_to_be_bytes(value: u16) -> [u8; 2] {
    // TODO: 将 u16 转换为大端序字节数组
    // 提示: 可以使用位运算，或者使用 Rust 内置的 .to_be_bytes() 方法
    todo!("将 u16 转换为大端序字节")
}

/// 将网络字节序（大端序）的字节数组转换为 u16
///
/// # 参数
/// * `bytes` - 大端序的 2 字节数组
///
/// # 返回值
/// u16 值
///
/// # 示例
/// ```
/// let value = u16_from_be_bytes([0x12, 0x34]);
/// assert_eq!(value, 0x1234);
/// ```
pub fn u16_from_be_bytes(bytes: [u8; 2]) -> u16 {
    // TODO: 将大端序字节数组转换为 u16
    // 提示: 可以使用位运算组合字节，或者使用 u16::from_be_bytes()
    todo!("从大端序字节解析 u16")
}

/// 将 u32 从主机字节序转换为网络字节序（大端序）
///
/// # 参数
/// * `value` - 主机字节序的 u32 值
///
/// # 返回值
/// 网络字节序（大端序）的字节数组
///
/// # 示例
/// ```
/// let bytes = u32_to_be_bytes(0x12345678);
/// assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78]);
/// ```
pub fn u32_to_be_bytes(value: u32) -> [u8; 4] {
    // TODO: 将 u32 转换为大端序字节数组
    todo!("将 u32 转换为大端序字节")
}

/// 将网络字节序（大端序）的字节数组转换为 u32
///
/// # 参数
/// * `bytes` - 大端序的 4 字节数组
///
/// # 返回值
/// u32 值
///
/// # 示例
/// ```
/// let value = u32_from_be_bytes([0x12, 0x34, 0x56, 0x78]);
/// assert_eq!(value, 0x12345678);
/// ```
pub fn u32_from_be_bytes(bytes: [u8; 4]) -> u32 {
    // TODO: 将大端序字节数组转换为 u32
    todo!("从大端序字节解析 u32")
}

/// 将 u16 转换为小端序字节数组
///
/// # 示例
/// ```
/// let bytes = u16_to_le_bytes(0x1234);
/// assert_eq!(bytes, [0x34, 0x12]);
/// ```
pub fn u16_to_le_bytes(value: u16) -> [u8; 2] {
    // TODO: 将 u16 转换为小端序字节数组
    todo!("将 u16 转换为小端序字节")
}

/// 从小端序字节数组解析 u16
///
/// # 示例
/// ```
/// let value = u16_from_le_bytes([0x34, 0x12]);
/// assert_eq!(value, 0x1234);
/// ```
pub fn u16_from_le_bytes(bytes: [u8; 2]) -> u16 {
    // TODO: 将小端序字节数组转换为 u16
    todo!("从小端序字节解析 u16")
}

/// 从字节切片中读取大端序 u16（从指定偏移开始）
///
/// # 参数
/// * `data` - 字节切片
/// * `offset` - 起始偏移量
///
/// # 返回值
/// 如果有足够的字节，返回 Some(u16)；否则返回 None
///
/// # 示例
/// ```
/// let data = &[0x00, 0x12, 0x34, 0x56];
/// assert_eq!(read_u16_be(data, 1), Some(0x1234));
/// assert_eq!(read_u16_be(data, 3), None); // 不够字节
/// ```
pub fn read_u16_be(data: &[u8], offset: usize) -> Option<u16> {
    // TODO: 从字节切片中安全地读取大端序 u16
    // 提示:
    // 1. 检查是否有足够的字节
    // 2. 提取两个字节
    // 3. 组合成 u16
    todo!("从字节切片读取大端序 u16")
}

/// 从字节切片中读取大端序 u32（从指定偏移开始）
///
/// # 参数
/// * `data` - 字节切片
/// * `offset` - 起始偏移量
///
/// # 返回值
/// 如果有足够的字节，返回 Some(u32)；否则返回 None
pub fn read_u32_be(data: &[u8], offset: usize) -> Option<u32> {
    // TODO: 从字节切片中安全地读取大端序 u32
    todo!("从字节切片读取大端序 u32")
}

/// 检测当前系统的字节序
///
/// # 返回值
/// 如果是小端序返回 true，大端序返回 false
pub fn is_little_endian() -> bool {
    // TODO: 检测当前系统的字节序
    // 提示: 将一个多字节整数转换为字节数组，检查第一个字节
    // 例如: 0x0001 在小端序系统中，第一个字节是 0x01
    todo!("检测系统字节序")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_endianness_u16_be() {
        assert_eq!(u16_to_be_bytes(0x0000), [0x00, 0x00]);
        assert_eq!(u16_to_be_bytes(0x1234), [0x12, 0x34]);
        assert_eq!(u16_to_be_bytes(0xFFFF), [0xFF, 0xFF]);
        assert_eq!(u16_to_be_bytes(0x0001), [0x00, 0x01]);

        assert_eq!(u16_from_be_bytes([0x00, 0x00]), 0x0000);
        assert_eq!(u16_from_be_bytes([0x12, 0x34]), 0x1234);
        assert_eq!(u16_from_be_bytes([0xFF, 0xFF]), 0xFFFF);
    }

    #[test]
    fn test_binary_endianness_u32_be() {
        assert_eq!(u32_to_be_bytes(0x12345678), [0x12, 0x34, 0x56, 0x78]);
        assert_eq!(u32_to_be_bytes(0x00000001), [0x00, 0x00, 0x00, 0x01]);

        assert_eq!(u32_from_be_bytes([0x12, 0x34, 0x56, 0x78]), 0x12345678);
        assert_eq!(u32_from_be_bytes([0x00, 0x00, 0x00, 0x01]), 0x00000001);
    }

    #[test]
    fn test_binary_endianness_u16_le() {
        assert_eq!(u16_to_le_bytes(0x1234), [0x34, 0x12]);
        assert_eq!(u16_from_le_bytes([0x34, 0x12]), 0x1234);
    }

    #[test]
    fn test_binary_endianness_read_u16() {
        let data = &[0xAA, 0x12, 0x34, 0xBB];
        assert_eq!(read_u16_be(data, 0), Some(0xAA12));
        assert_eq!(read_u16_be(data, 1), Some(0x1234));
        assert_eq!(read_u16_be(data, 2), Some(0x34BB));
        assert_eq!(read_u16_be(data, 3), None);
        assert_eq!(read_u16_be(data, 10), None);
    }

    #[test]
    fn test_binary_endianness_read_u32() {
        let data = &[0x12, 0x34, 0x56, 0x78, 0x9A];
        assert_eq!(read_u32_be(data, 0), Some(0x12345678));
        assert_eq!(read_u32_be(data, 1), Some(0x3456789A));
        assert_eq!(read_u32_be(data, 2), None);
    }

    #[test]
    fn test_binary_endianness_system() {
        // 大多数现代PC是小端序
        let result = is_little_endian();
        // 验证检测逻辑正确
        let test_value: u16 = 0x0001;
        let bytes = test_value.to_ne_bytes();
        let expected = bytes[0] == 0x01;
        assert_eq!(result, expected);
    }
}
