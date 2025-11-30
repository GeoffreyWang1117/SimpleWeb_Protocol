//! # 练习 03: CRC 循环冗余校验
//!
//! CRC (Cyclic Redundancy Check) 是一种更强大的错误检测算法。
//!
//! ## CRC vs 校验和
//!
//! | 特性 | 校验和 | CRC |
//! |------|--------|-----|
//! | 检错能力 | 弱 | 强 |
//! | 计算速度 | 快 | 较慢 |
//! | 应用 | IP/TCP/UDP | 以太网/USB/存储 |
//!
//! ## CRC 原理
//!
//! CRC 基于多项式除法：
//! 1. 将数据视为一个大整数
//! 2. 除以生成多项式
//! 3. 余数就是 CRC 值
//!
//! ## 常见 CRC 标准
//!
//! | 名称 | 位数 | 多项式 | 应用 |
//! |------|------|--------|------|
//! | CRC-8 | 8 | 0x07 | I²C |
//! | CRC-16 | 16 | 0x8005 | Modbus |
//! | CRC-32 | 32 | 0x04C11DB7 | Ethernet, ZIP |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test binary_crc
//! ```

/// CRC-8 计算
///
/// 多项式: x^8 + x^2 + x + 1 (0x07)
pub fn crc8(data: &[u8]) -> u8 {
    // TODO: 实现 CRC-8
    //
    // 算法:
    // 1. 初始化 crc = 0
    // 2. 对每个字节:
    //    - crc ^= byte
    //    - 对每个位 (8次):
    //      - 如果最高位为1: crc = (crc << 1) ^ 0x07
    //      - 否则: crc = crc << 1
    // 3. 返回 crc
    todo!("实现 CRC-8")
}

/// CRC-16 (Modbus) 计算
///
/// 多项式: x^16 + x^15 + x^2 + 1 (0x8005, 反转为 0xA001)
pub fn crc16_modbus(data: &[u8]) -> u16 {
    // TODO: 实现 CRC-16 Modbus
    //
    // Modbus 使用反向算法:
    // 1. 初始化 crc = 0xFFFF
    // 2. 对每个字节:
    //    - crc ^= byte
    //    - 对每个位 (8次):
    //      - 如果最低位为1: crc = (crc >> 1) ^ 0xA001
    //      - 否则: crc = crc >> 1
    // 3. 返回 crc
    todo!("实现 CRC-16 Modbus")
}

/// CRC-32 计算
///
/// 多项式: 0x04C11DB7 (反转为 0xEDB88320)
/// 用于: Ethernet, ZIP, PNG, GZIP
pub fn crc32(data: &[u8]) -> u32 {
    // TODO: 实现 CRC-32
    //
    // 1. 初始化 crc = 0xFFFFFFFF
    // 2. 对每个字节:
    //    - crc ^= byte
    //    - 对每个位 (8次):
    //      - 如果最低位为1: crc = (crc >> 1) ^ 0xEDB88320
    //      - 否则: crc = crc >> 1
    // 3. 返回 crc ^ 0xFFFFFFFF (取反)
    todo!("实现 CRC-32")
}

/// 使用查找表加速的 CRC-32
///
/// 预计算256个条目的查找表可以显著提升性能
pub struct Crc32Table {
    table: [u32; 256],
}

impl Crc32Table {
    /// 生成 CRC-32 查找表
    pub fn new() -> Self {
        // TODO: 生成查找表
        //
        // 对于 i in 0..256:
        //   计算 crc32(&[i as u8]) 作为 table[i]
        todo!("生成查找表")
    }

    /// 使用查找表计算 CRC-32
    pub fn compute(&self, data: &[u8]) -> u32 {
        // TODO: 使用查找表计算
        //
        // crc = 0xFFFFFFFF
        // for byte in data:
        //     index = (crc ^ byte) & 0xFF
        //     crc = (crc >> 8) ^ table[index]
        // return crc ^ 0xFFFFFFFF
        todo!("查找表计算")
    }
}

impl Default for Crc32Table {
    fn default() -> Self {
        Self::new()
    }
}

/// 验证 CRC
pub fn verify_crc32(data: &[u8], expected_crc: u32) -> bool {
    // TODO: 验证数据的 CRC 是否正确
    todo!("验证 CRC")
}

/// 计算带 CRC 的数据帧
///
/// 返回: 原始数据 + CRC-32 (小端序)
pub fn append_crc32(data: &[u8]) -> Vec<u8> {
    // TODO: 将 CRC 追加到数据末尾
    todo!("追加 CRC")
}

/// 验证带 CRC 的数据帧
///
/// 输入: 数据 + CRC-32 (小端序)
/// 返回: 如果 CRC 正确，返回原始数据
pub fn verify_and_strip_crc32(frame: &[u8]) -> Option<&[u8]> {
    // TODO: 验证并移除 CRC
    todo!("验证并移除 CRC")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_crc8() {
        // 测试向量
        assert_eq!(crc8(b"123456789"), 0xF4);
        assert_eq!(crc8(b""), 0x00);
        assert_eq!(crc8(&[0x00]), 0x00);
    }

    #[test]
    fn test_binary_crc16_modbus() {
        // Modbus CRC-16 测试向量
        assert_eq!(crc16_modbus(b"123456789"), 0x4B37);
        assert_eq!(crc16_modbus(&[0x01, 0x03, 0x00, 0x00, 0x00, 0x0A]), 0xC5CD);
    }

    #[test]
    fn test_binary_crc32() {
        // CRC-32 测试向量 (IEEE 802.3)
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
        assert_eq!(crc32(b""), 0x00000000);
        assert_eq!(crc32(b"The quick brown fox jumps over the lazy dog"), 0x414FA339);
    }

    #[test]
    fn test_binary_crc32_table() {
        let table = Crc32Table::new();
        assert_eq!(table.compute(b"123456789"), 0xCBF43926);
        assert_eq!(table.compute(b"Hello, World!"), crc32(b"Hello, World!"));
    }

    #[test]
    fn test_binary_crc32_frame() {
        let data = b"Hello";
        let frame = append_crc32(data);
        assert_eq!(frame.len(), data.len() + 4);

        let verified = verify_and_strip_crc32(&frame);
        assert_eq!(verified, Some(data.as_slice()));

        // 损坏的帧
        let mut corrupted = frame.clone();
        corrupted[0] ^= 0xFF;
        assert_eq!(verify_and_strip_crc32(&corrupted), None);
    }
}
