//! # 练习 02: 校验和（Checksum）
//!
//! 校验和用于检测数据在传输过程中是否发生错误。
//! 网络协议（IP、TCP、UDP、ICMP等）广泛使用互联网校验和算法。
//!
//! ## 互联网校验和算法 (RFC 1071)
//!
//! 1. 将数据视为 16 位整数序列（大端序）
//! 2. 如果数据长度为奇数，末尾补一个零字节
//! 3. 将所有 16 位整数相加
//! 4. 将进位加回低 16 位（反复进行直到没有进位）
//! 5. 取反得到校验和
//!
//! ## 示例
//!
//! ```text
//! 数据: [0x45, 0x00, 0x00, 0x3c, ...]
//!
//! 第1步: 0x4500 + 0x003c + ... = 0x1ABCD (可能超过16位)
//! 第2步: 0xABCD + 0x0001 = 0xABCE (加回进位)
//! 第3步: ~0xABCE = 0x5431 (取反得到校验和)
//! ```
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test binary_checksum
//! ```

/// 计算简单的异或校验和
///
/// 这是最简单的校验和算法，将所有字节异或在一起。
///
/// # 参数
/// * `data` - 要计算校验和的数据
///
/// # 返回值
/// 所有字节的异或结果
///
/// # 示例
/// ```
/// assert_eq!(xor_checksum(&[0x12, 0x34, 0x56]), 0x12 ^ 0x34 ^ 0x56);
/// ```
pub fn xor_checksum(data: &[u8]) -> u8 {
    // TODO: 计算所有字节的异或值
    // 提示: 使用 fold 或循环
    todo!("计算异或校验和")
}

/// 计算简单的累加校验和
///
/// 将所有字节相加，忽略溢出。
///
/// # 参数
/// * `data` - 要计算校验和的数据
///
/// # 返回值
/// 所有字节相加的结果（模 256）
///
/// # 示例
/// ```
/// assert_eq!(sum_checksum(&[100, 100, 100]), 44); // (300 % 256 = 44)
/// ```
pub fn sum_checksum(data: &[u8]) -> u8 {
    // TODO: 计算所有字节的和（使用 wrapping_add 处理溢出）
    todo!("计算累加校验和")
}

/// 将进位折叠到 16 位（用于互联网校验和）
///
/// 将 32 位值的高 16 位加到低 16 位，直到结果在 16 位范围内。
///
/// # 参数
/// * `value` - 32 位累加值
///
/// # 返回值
/// 折叠后的 16 位值
///
/// # 示例
/// ```
/// assert_eq!(fold_to_u16(0x0001FFFF), 0x0000); // 0xFFFF + 0x0001 = 0x10000, 再折叠
/// assert_eq!(fold_to_u16(0x00001234), 0x1234); // 无进位
/// ```
pub fn fold_to_u16(mut value: u32) -> u16 {
    // TODO: 将高16位加到低16位，直到没有进位
    // 提示:
    // while value > 0xFFFF {
    //     value = (value & 0xFFFF) + (value >> 16);
    // }
    todo!("折叠到 16 位")
}

/// 计算互联网校验和（RFC 1071）
///
/// 这是 IP、TCP、UDP 等协议使用的标准校验和算法。
///
/// # 参数
/// * `data` - 要计算校验和的数据
///
/// # 返回值
/// 16 位校验和
///
/// # 算法
/// 1. 将数据视为 16 位字序列（大端序）
/// 2. 相加所有 16 位字
/// 3. 处理进位
/// 4. 取反
///
/// # 示例
/// ```
/// // 简单测试数据
/// let data = [0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
/// let checksum = internet_checksum(&data);
/// // 校验: 将数据与校验和一起再计算，结果应为 0xFFFF 或 0
/// ```
pub fn internet_checksum(data: &[u8]) -> u16 {
    // TODO: 实现互联网校验和算法
    //
    // 步骤:
    // 1. 初始化 sum: u32 = 0
    // 2. 每次取 2 个字节，组成大端序 u16，加到 sum
    // 3. 如果数据长度为奇数，最后一个字节左移 8 位后加入
    // 4. 调用 fold_to_u16 处理进位
    // 5. 取反 (!sum as u16)
    //
    // 提示: 使用 chunks(2) 来迭代字节对
    todo!("计算互联网校验和")
}

/// 验证数据的互联网校验和
///
/// # 参数
/// * `data` - 包含校验和的数据
///
/// # 返回值
/// 如果校验和正确返回 true
///
/// # 说明
/// 对包含正确校验和的数据再次计算校验和，结果应该是 0xFFFF 或 0
pub fn verify_internet_checksum(data: &[u8]) -> bool {
    // TODO: 验证数据的校验和
    // 提示: 对整个数据（包含校验和）计算校验和
    // 如果校验和正确，结果的折叠值应该是 0xFFFF
    todo!("验证互联网校验和")
}

/// 计算带伪头部的 TCP/UDP 校验和
///
/// TCP 和 UDP 的校验和不仅包括头部和数据，还包括一个"伪头部"。
/// 伪头部包含: 源IP、目标IP、协议号、段长度。
///
/// # 参数
/// * `src_ip` - 源 IP 地址（4字节）
/// * `dst_ip` - 目标 IP 地址（4字节）
/// * `protocol` - 协议号（TCP=6, UDP=17）
/// * `data` - TCP/UDP 头部 + 数据
///
/// # 返回值
/// 16 位校验和
pub fn tcp_udp_checksum(src_ip: [u8; 4], dst_ip: [u8; 4], protocol: u8, data: &[u8]) -> u16 {
    // TODO: 计算带伪头部的校验和
    //
    // 伪头部格式（12字节）:
    // - 源 IP 地址: 4 字节
    // - 目标 IP 地址: 4 字节
    // - 零: 1 字节
    // - 协议: 1 字节
    // - 数据长度: 2 字节（大端序）
    //
    // 步骤:
    // 1. 构造伪头部
    // 2. 将伪头部和数据连接起来
    // 3. 计算互联网校验和
    todo!("计算 TCP/UDP 校验和")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_checksum_xor() {
        assert_eq!(xor_checksum(&[]), 0);
        assert_eq!(xor_checksum(&[0xFF]), 0xFF);
        assert_eq!(xor_checksum(&[0xFF, 0xFF]), 0);
        assert_eq!(xor_checksum(&[0x12, 0x34, 0x56]), 0x12 ^ 0x34 ^ 0x56);
    }

    #[test]
    fn test_binary_checksum_sum() {
        assert_eq!(sum_checksum(&[]), 0);
        assert_eq!(sum_checksum(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_checksum(&[100, 100, 100]), 44); // 300 % 256 = 44
        assert_eq!(sum_checksum(&[255, 1]), 0); // 溢出
    }

    #[test]
    fn test_binary_checksum_fold() {
        assert_eq!(fold_to_u16(0x00001234), 0x1234);
        assert_eq!(fold_to_u16(0x00010000), 0x0001);
        assert_eq!(fold_to_u16(0x0001FFFF), 0x0001); // 0xFFFF + 1 + 1 = 0x10001 -> 2
        assert_eq!(fold_to_u16(0xFFFFFFFF), 0xFFFF + 1); // 多次折叠
    }

    #[test]
    fn test_binary_checksum_internet() {
        // 测试用例来自 RFC 1071
        let data = [0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
        let checksum = internet_checksum(&data);

        // 验证: 原数据 + 校验和 再计算应该得到 0xFFFF
        let mut verify_data = data.to_vec();
        verify_data.push((checksum >> 8) as u8);
        verify_data.push((checksum & 0xFF) as u8);

        // 重新计算（不取反）
        let mut sum: u32 = 0;
        for chunk in verify_data.chunks(2) {
            let word = if chunk.len() == 2 {
                ((chunk[0] as u32) << 8) | (chunk[1] as u32)
            } else {
                (chunk[0] as u32) << 8
            };
            sum += word;
        }
        let folded = fold_to_u16(sum);
        assert_eq!(folded, 0xFFFF);
    }

    #[test]
    fn test_binary_checksum_internet_odd_length() {
        // 奇数长度数据
        let data = [0x45, 0x00, 0x00];
        let _checksum = internet_checksum(&data);
        // 只要不 panic 就通过
    }

    #[test]
    fn test_binary_checksum_verify() {
        // 创建带有效校验和的数据
        let original = [0x45, 0x00, 0x00, 0x3c];
        let checksum = internet_checksum(&original);

        let mut data_with_checksum = original.to_vec();
        data_with_checksum.push((checksum >> 8) as u8);
        data_with_checksum.push((checksum & 0xFF) as u8);

        assert!(verify_internet_checksum(&data_with_checksum));
    }

    #[test]
    fn test_binary_checksum_tcp_udp() {
        // 简单测试 TCP/UDP 校验和计算不会 panic
        let src_ip = [192, 168, 1, 1];
        let dst_ip = [192, 168, 1, 2];
        let data = [0x00, 0x50, 0x00, 0x51, 0x00, 0x08, 0x00, 0x00]; // 简单UDP头

        let _checksum = tcp_udp_checksum(src_ip, dst_ip, 17, &data);
        // 只要计算完成就通过
    }
}
