//! # 练习 01: 字节基础
//!
//! 在网络编程中，数据以字节（bytes）的形式在网络上传输。
//! 理解字节操作是学习网络协议的基础。
//!
//! ## 知识点
//!
//! - `u8`: 无符号8位整数，表示一个字节（0-255）
//! - `&[u8]`: 字节切片，表示一段连续的字节
//! - `Vec<u8>`: 字节向量，可动态增长的字节数组
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test intro_bytes
//! ```

/// 计算字节切片的长度
///
/// # 参数
/// * `data` - 字节切片
///
/// # 返回值
/// 字节切片中的字节数量
///
/// # 示例
/// ```
/// let data = &[0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello" in ASCII
/// assert_eq!(byte_length(data), 5);
/// ```
pub fn byte_length(data: &[u8]) -> usize {
    // TODO: 返回字节切片的长度
    // 提示: 使用 .len() 方法
    todo!("返回字节切片的长度")
}

/// 获取字节切片中指定索引的字节
///
/// # 参数
/// * `data` - 字节切片
/// * `index` - 要获取的字节的索引
///
/// # 返回值
/// 如果索引有效，返回 Some(byte)；否则返回 None
///
/// # 示例
/// ```
/// let data = &[0xAB, 0xCD, 0xEF];
/// assert_eq!(get_byte(data, 1), Some(0xCD));
/// assert_eq!(get_byte(data, 5), None);
/// ```
pub fn get_byte(data: &[u8], index: usize) -> Option<u8> {
    // TODO: 安全地获取指定索引的字节
    // 提示: 使用 .get() 方法并解引用
    todo!("获取指定索引的字节")
}

/// 将两个字节切片连接成一个新的 Vec<u8>
///
/// # 参数
/// * `first` - 第一个字节切片
/// * `second` - 第二个字节切片
///
/// # 返回值
/// 包含两个切片所有字节的新向量
///
/// # 示例
/// ```
/// let a = &[1, 2, 3];
/// let b = &[4, 5, 6];
/// assert_eq!(concat_bytes(a, b), vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn concat_bytes(first: &[u8], second: &[u8]) -> Vec<u8> {
    // TODO: 将两个字节切片连接起来
    // 提示: 创建一个新的 Vec，然后使用 extend_from_slice
    todo!("连接两个字节切片")
}

/// 将字符串转换为字节向量
///
/// # 参数
/// * `s` - 要转换的字符串
///
/// # 返回值
/// 字符串的 UTF-8 字节表示
///
/// # 示例
/// ```
/// let bytes = string_to_bytes("Hi");
/// assert_eq!(bytes, vec![0x48, 0x69]); // 'H' = 0x48, 'i' = 0x69
/// ```
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    // TODO: 将字符串转换为字节向量
    // 提示: 使用 .as_bytes() 方法，然后转换为 Vec
    todo!("将字符串转换为字节")
}

/// 将字节切片转换为十六进制字符串
///
/// # 参数
/// * `data` - 字节切片
///
/// # 返回值
/// 十六进制表示的字符串（小写，无分隔符）
///
/// # 示例
/// ```
/// let data = &[0xDE, 0xAD, 0xBE, 0xEF];
/// assert_eq!(bytes_to_hex(data), "deadbeef");
/// ```
pub fn bytes_to_hex(data: &[u8]) -> String {
    // TODO: 将字节切片转换为十六进制字符串
    // 提示: 遍历每个字节，使用 format!("{:02x}", byte) 转换
    todo!("将字节转换为十六进制字符串")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro_bytes_length() {
        assert_eq!(byte_length(&[]), 0);
        assert_eq!(byte_length(&[1, 2, 3]), 3);
        assert_eq!(byte_length(&[0; 100]), 100);
    }

    #[test]
    fn test_intro_bytes_get_byte() {
        let data = &[0xAA, 0xBB, 0xCC];
        assert_eq!(get_byte(data, 0), Some(0xAA));
        assert_eq!(get_byte(data, 2), Some(0xCC));
        assert_eq!(get_byte(data, 3), None);
        assert_eq!(get_byte(&[], 0), None);
    }

    #[test]
    fn test_intro_bytes_concat() {
        assert_eq!(concat_bytes(&[], &[]), vec![]);
        assert_eq!(concat_bytes(&[1, 2], &[]), vec![1, 2]);
        assert_eq!(concat_bytes(&[], &[3, 4]), vec![3, 4]);
        assert_eq!(concat_bytes(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_intro_bytes_string_to_bytes() {
        assert_eq!(string_to_bytes(""), vec![]);
        assert_eq!(string_to_bytes("A"), vec![0x41]);
        assert_eq!(string_to_bytes("Hello"), vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
    }

    #[test]
    fn test_intro_bytes_to_hex() {
        assert_eq!(bytes_to_hex(&[]), "");
        assert_eq!(bytes_to_hex(&[0x00]), "00");
        assert_eq!(bytes_to_hex(&[0xFF]), "ff");
        assert_eq!(bytes_to_hex(&[0xDE, 0xAD, 0xBE, 0xEF]), "deadbeef");
    }
}
