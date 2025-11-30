//! # 练习 00: 位操作
//!
//! 网络协议中大量使用位操作来紧凑地存储信息。
//! 例如，TCP头部中的标志位（SYN, ACK, FIN等）就是用单个位表示的。
//!
//! ## 知识点
//!
//! - 位与 `&`: 两个位都为1时结果为1
//! - 位或 `|`: 任一位为1时结果为1
//! - 位异或 `^`: 两个位不同时结果为1
//! - 位非 `!`: 取反
//! - 左移 `<<`: 向左移动指定位数
//! - 右移 `>>`: 向右移动指定位数
//!
//! ## 位索引约定
//!
//! 本练习中，位的索引从右向左，从0开始：
//! ```text
//! 字节:    0b10110100
//! 位索引:    76543210
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test binary_bits
//! ```

/// 获取一个字节的第 n 位（从右往左，0-indexed）
///
/// # 参数
/// * `byte` - 要检查的字节
/// * `n` - 位索引（0-7）
///
/// # 返回值
/// 如果第 n 位是 1，返回 true；否则返回 false
///
/// # 示例
/// ```
/// // 0b10110100
/// //   76543210  <- 位索引
/// assert_eq!(get_bit(0b10110100, 2), true);  // 第2位是1
/// assert_eq!(get_bit(0b10110100, 0), false); // 第0位是0
/// ```
pub fn get_bit(byte: u8, n: u8) -> bool {
    // TODO: 使用位操作获取第 n 位
    // 提示: 先右移 n 位，然后与 1 进行位与操作
    todo!("获取字节的第 n 位")
}

/// 设置一个字节的第 n 位为 1
///
/// # 参数
/// * `byte` - 原始字节
/// * `n` - 要设置的位索引（0-7）
///
/// # 返回值
/// 第 n 位被设置为 1 后的新字节
///
/// # 示例
/// ```
/// assert_eq!(set_bit(0b00000000, 3), 0b00001000);
/// assert_eq!(set_bit(0b00001000, 3), 0b00001000); // 已经是1，不变
/// ```
pub fn set_bit(byte: u8, n: u8) -> u8 {
    // TODO: 将第 n 位设置为 1
    // 提示: 创建一个只有第 n 位为 1 的掩码，然后与原字节进行位或操作
    todo!("设置字节的第 n 位")
}

/// 清除一个字节的第 n 位（设为 0）
///
/// # 参数
/// * `byte` - 原始字节
/// * `n` - 要清除的位索引（0-7）
///
/// # 返回值
/// 第 n 位被清除后的新字节
///
/// # 示例
/// ```
/// assert_eq!(clear_bit(0b11111111, 3), 0b11110111);
/// assert_eq!(clear_bit(0b11110111, 3), 0b11110111); // 已经是0，不变
/// ```
pub fn clear_bit(byte: u8, n: u8) -> u8 {
    // TODO: 将第 n 位清除为 0
    // 提示: 创建一个只有第 n 位为 0 的掩码，然后与原字节进行位与操作
    todo!("清除字节的第 n 位")
}

/// 切换一个字节的第 n 位（0变1，1变0）
///
/// # 参数
/// * `byte` - 原始字节
/// * `n` - 要切换的位索引（0-7）
///
/// # 返回值
/// 第 n 位被切换后的新字节
///
/// # 示例
/// ```
/// assert_eq!(toggle_bit(0b00000000, 3), 0b00001000);
/// assert_eq!(toggle_bit(0b00001000, 3), 0b00000000);
/// ```
pub fn toggle_bit(byte: u8, n: u8) -> u8 {
    // TODO: 切换第 n 位
    // 提示: 使用异或操作
    todo!("切换字节的第 n 位")
}

/// 提取字节中从 start 到 end（包含）的位
///
/// # 参数
/// * `byte` - 原始字节
/// * `start` - 起始位索引（包含）
/// * `end` - 结束位索引（包含）
///
/// # 返回值
/// 提取的位组成的值（右对齐）
///
/// # 示例
/// ```
/// // 从 0b11010110 中提取第 2-5 位
/// // 字节:  0b11010110
/// // 位:      76543210
/// // 提取:     ^^^^    (第2-5位是 0101)
/// assert_eq!(extract_bits(0b11010110, 2, 5), 0b0101);
/// ```
pub fn extract_bits(byte: u8, start: u8, end: u8) -> u8 {
    // TODO: 提取从 start 到 end 的位
    // 提示:
    // 1. 创建一个掩码，只有 start 到 end 位为 1
    // 2. 与原字节进行位与操作
    // 3. 右移 start 位得到结果
    todo!("提取字节中指定范围的位")
}

/// 统计字节中 1 的个数（汉明权重）
///
/// # 参数
/// * `byte` - 要统计的字节
///
/// # 返回值
/// 字节中 1 的个数
///
/// # 示例
/// ```
/// assert_eq!(count_ones(0b00000000), 0);
/// assert_eq!(count_ones(0b11111111), 8);
/// assert_eq!(count_ones(0b10101010), 4);
/// ```
pub fn count_ones(byte: u8) -> u32 {
    // TODO: 统计字节中 1 的个数
    // 提示: 可以使用循环检查每一位，或者使用内置方法
    todo!("统计字节中 1 的个数")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_bits_get() {
        // 0b10110100 = 180
        assert_eq!(get_bit(0b10110100, 0), false);
        assert_eq!(get_bit(0b10110100, 2), true);
        assert_eq!(get_bit(0b10110100, 4), true);
        assert_eq!(get_bit(0b10110100, 7), true);

        // 边界测试
        assert_eq!(get_bit(0xFF, 0), true);
        assert_eq!(get_bit(0xFF, 7), true);
        assert_eq!(get_bit(0x00, 0), false);
    }

    #[test]
    fn test_binary_bits_set() {
        assert_eq!(set_bit(0b00000000, 0), 0b00000001);
        assert_eq!(set_bit(0b00000000, 7), 0b10000000);
        assert_eq!(set_bit(0b11110000, 0), 0b11110001);
        assert_eq!(set_bit(0b11111111, 3), 0b11111111); // 已经是1
    }

    #[test]
    fn test_binary_bits_clear() {
        assert_eq!(clear_bit(0b11111111, 0), 0b11111110);
        assert_eq!(clear_bit(0b11111111, 7), 0b01111111);
        assert_eq!(clear_bit(0b00001111, 0), 0b00001110);
        assert_eq!(clear_bit(0b00000000, 3), 0b00000000); // 已经是0
    }

    #[test]
    fn test_binary_bits_toggle() {
        assert_eq!(toggle_bit(0b00000000, 0), 0b00000001);
        assert_eq!(toggle_bit(0b00000001, 0), 0b00000000);
        assert_eq!(toggle_bit(0b10101010, 0), 0b10101011);
        assert_eq!(toggle_bit(0b10101010, 1), 0b10101000);
    }

    #[test]
    fn test_binary_bits_extract() {
        // 提取单个位
        assert_eq!(extract_bits(0b11010110, 0, 0), 0);
        assert_eq!(extract_bits(0b11010110, 1, 1), 1);

        // 提取多个位
        assert_eq!(extract_bits(0b11010110, 0, 3), 0b0110);
        assert_eq!(extract_bits(0b11010110, 4, 7), 0b1101);
        assert_eq!(extract_bits(0b11010110, 2, 5), 0b0101);
    }

    #[test]
    fn test_binary_bits_count_ones() {
        assert_eq!(count_ones(0b00000000), 0);
        assert_eq!(count_ones(0b00000001), 1);
        assert_eq!(count_ones(0b11111111), 8);
        assert_eq!(count_ones(0b10101010), 4);
        assert_eq!(count_ones(0b01010101), 4);
    }
}
