//! # 练习 03: CIDR 表示法
//!
//! CIDR (Classless Inter-Domain Routing) 是现代 IP 地址分配的基础。
//!
//! ## CIDR 表示法
//!
//! ```text
//! 192.168.1.0/24
//!     │         │
//!     │         └── 前缀长度 (网络位数)
//!     └── 网络地址
//!
//! /24 = 前24位是网络地址，后8位是主机地址
//!     = 子网掩码 255.255.255.0
//!     = 256 个地址 (254 个可用主机)
//! ```
//!
//! ## 常见 CIDR 块
//!
//! | CIDR | 子网掩码 | 地址数 | 可用主机 |
//! |------|----------|--------|----------|
//! | /8 | 255.0.0.0 | 16,777,216 | 16,777,214 |
//! | /16 | 255.255.0.0 | 65,536 | 65,534 |
//! | /24 | 255.255.255.0 | 256 | 254 |
//! | /32 | 255.255.255.255 | 1 | 1 |
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test ip_cidr
//! ```

use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;

/// CIDR 网络块
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cidr {
    /// 网络地址
    pub network: Ipv4Addr,
    /// 前缀长度 (0-32)
    pub prefix_len: u8,
}

impl Cidr {
    /// 创建 CIDR
    pub fn new(network: Ipv4Addr, prefix_len: u8) -> Option<Self> {
        // TODO: 验证前缀长度并创建 CIDR
        // prefix_len 必须 <= 32
        // network 应该是有效的网络地址（主机位为0）
        todo!("创建 CIDR")
    }

    /// 从地址和前缀创建（自动计算网络地址）
    pub fn from_address(addr: Ipv4Addr, prefix_len: u8) -> Option<Self> {
        // TODO: 计算网络地址并创建 CIDR
        todo!("从地址创建 CIDR")
    }

    /// 获取子网掩码
    pub fn netmask(&self) -> Ipv4Addr {
        // TODO: 计算子网掩码
        //
        // /24 -> 255.255.255.0
        // /16 -> 255.255.0.0
        // /0  -> 0.0.0.0
        // /32 -> 255.255.255.255
        //
        // 公式: mask = !0u32 << (32 - prefix_len)
        todo!("计算子网掩码")
    }

    /// 获取通配符掩码（反掩码）
    pub fn wildcard(&self) -> Ipv4Addr {
        // TODO: 计算通配符掩码
        // wildcard = !netmask
        todo!("计算通配符掩码")
    }

    /// 获取广播地址
    pub fn broadcast(&self) -> Ipv4Addr {
        // TODO: 计算广播地址
        // broadcast = network | wildcard
        todo!("计算广播地址")
    }

    /// 获取第一个可用主机地址
    pub fn first_host(&self) -> Option<Ipv4Addr> {
        // TODO: 返回第一个可用主机地址
        // 对于 /32 和 /31，可能没有可用主机
        todo!("第一个主机")
    }

    /// 获取最后一个可用主机地址
    pub fn last_host(&self) -> Option<Ipv4Addr> {
        // TODO: 返回最后一个可用主机地址
        todo!("最后一个主机")
    }

    /// 获取网络中的地址总数
    pub fn total_addresses(&self) -> u32 {
        // TODO: 计算地址总数
        // count = 2^(32 - prefix_len)
        todo!("地址总数")
    }

    /// 获取可用主机数
    pub fn usable_hosts(&self) -> u32 {
        // TODO: 计算可用主机数
        // 通常是 total - 2（网络地址和广播地址）
        // /31 和 /32 是特殊情况
        todo!("可用主机数")
    }

    /// 检查 IP 是否在此网络中
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        // TODO: 检查 IP 是否属于此网络
        // (ip & netmask) == network
        todo!("检查包含")
    }

    /// 检查两个网络是否重叠
    pub fn overlaps(&self, other: &Cidr) -> bool {
        // TODO: 检查两个网络是否有重叠
        todo!("检查重叠")
    }

    /// 检查此网络是否是另一个网络的子网
    pub fn is_subnet_of(&self, other: &Cidr) -> bool {
        // TODO: 检查是否是子网
        todo!("检查子网")
    }

    /// 将网络划分为两个子网
    pub fn split(&self) -> Option<(Cidr, Cidr)> {
        // TODO: 将网络一分为二
        // /24 -> /25 + /25
        // /32 无法再分
        todo!("划分子网")
    }

    /// 将网络划分为多个相等大小的子网
    pub fn divide(&self, count: u32) -> Option<Vec<Cidr>> {
        // TODO: 划分为 count 个子网
        // count 必须是 2 的幂
        todo!("划分多个子网")
    }
}

impl fmt::Display for Cidr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.network, self.prefix_len)
    }
}

impl FromStr for Cidr {
    type Err = CidrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: 解析 CIDR 字符串
        // 格式: "192.168.1.0/24"
        todo!("解析 CIDR")
    }
}

/// CIDR 解析错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CidrParseError {
    /// 格式无效
    InvalidFormat,
    /// IP 地址无效
    InvalidAddress,
    /// 前缀长度无效
    InvalidPrefix,
}

/// 私有地址范围
pub struct PrivateNetworks;

impl PrivateNetworks {
    /// RFC 1918 私有地址范围
    pub fn rfc1918() -> Vec<Cidr> {
        // TODO: 返回 RFC 1918 私有地址范围
        // 10.0.0.0/8
        // 172.16.0.0/12
        // 192.168.0.0/16
        todo!("RFC 1918")
    }

    /// 检查 IP 是否是私有地址
    pub fn is_private(ip: Ipv4Addr) -> bool {
        // TODO: 检查是否是私有地址
        todo!("检查私有地址")
    }

    /// 环回地址范围
    pub fn loopback() -> Cidr {
        // TODO: 返回 127.0.0.0/8
        todo!("环回地址")
    }

    /// 链路本地地址范围
    pub fn link_local() -> Cidr {
        // TODO: 返回 169.254.0.0/16
        todo!("链路本地")
    }
}

/// IP 地址聚合（超网）
pub fn aggregate(cidrs: &[Cidr]) -> Vec<Cidr> {
    // TODO: 将多个连续的 CIDR 聚合为更大的块
    // 例如: [192.168.0.0/24, 192.168.1.0/24] -> [192.168.0.0/23]
    todo!("CIDR 聚合")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_cidr_basic() {
        let cidr = Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap();
        assert_eq!(cidr.network, Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(cidr.prefix_len, 24);
    }

    #[test]
    fn test_ip_cidr_netmask() {
        let cidr = Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap();
        assert_eq!(cidr.netmask(), Ipv4Addr::new(255, 255, 255, 0));

        let cidr16 = Cidr::new(Ipv4Addr::new(172, 16, 0, 0), 16).unwrap();
        assert_eq!(cidr16.netmask(), Ipv4Addr::new(255, 255, 0, 0));

        let cidr32 = Cidr::new(Ipv4Addr::new(10, 0, 0, 1), 32).unwrap();
        assert_eq!(cidr32.netmask(), Ipv4Addr::new(255, 255, 255, 255));
    }

    #[test]
    fn test_ip_cidr_broadcast() {
        let cidr = Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap();
        assert_eq!(cidr.broadcast(), Ipv4Addr::new(192, 168, 1, 255));

        let cidr23 = Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 23).unwrap();
        assert_eq!(cidr23.broadcast(), Ipv4Addr::new(192, 168, 1, 255));
    }

    #[test]
    fn test_ip_cidr_hosts() {
        let cidr = Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap();
        assert_eq!(cidr.first_host(), Some(Ipv4Addr::new(192, 168, 1, 1)));
        assert_eq!(cidr.last_host(), Some(Ipv4Addr::new(192, 168, 1, 254)));
        assert_eq!(cidr.total_addresses(), 256);
        assert_eq!(cidr.usable_hosts(), 254);
    }

    #[test]
    fn test_ip_cidr_contains() {
        let cidr = Cidr::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap();
        assert!(cidr.contains(Ipv4Addr::new(192, 168, 1, 100)));
        assert!(!cidr.contains(Ipv4Addr::new(192, 168, 2, 1)));
    }

    #[test]
    fn test_ip_cidr_parse() {
        let cidr: Cidr = "192.168.1.0/24".parse().unwrap();
        assert_eq!(cidr.network, Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(cidr.prefix_len, 24);
    }

    #[test]
    fn test_ip_cidr_split() {
        let cidr = Cidr::new(Ipv4Addr::new(192, 168, 0, 0), 24).unwrap();
        let (a, b) = cidr.split().unwrap();
        assert_eq!(a.network, Ipv4Addr::new(192, 168, 0, 0));
        assert_eq!(a.prefix_len, 25);
        assert_eq!(b.network, Ipv4Addr::new(192, 168, 0, 128));
        assert_eq!(b.prefix_len, 25);
    }

    #[test]
    fn test_ip_cidr_private() {
        assert!(PrivateNetworks::is_private(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(PrivateNetworks::is_private(Ipv4Addr::new(192, 168, 1, 1)));
        assert!(!PrivateNetworks::is_private(Ipv4Addr::new(8, 8, 8, 8)));
    }
}
