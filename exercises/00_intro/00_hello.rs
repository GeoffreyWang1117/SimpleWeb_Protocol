//! # 练习 00: Hello, Network!
//!
//! 欢迎来到 Rust 网络协议学习项目！
//!
//! 这是你的第一个练习，目的是让你熟悉项目的结构和工作流程。
//!
//! ## 任务
//!
//! 修改下面的函数，使其返回正确的字符串。
//!
//! ## 如何运行测试
//!
//! ```bash
//! cargo test intro_hello
//! ```

/// 返回一个问候语
///
/// # 示例
/// ```
/// let greeting = hello_network();
/// assert!(greeting.contains("Network"));
/// ```
pub fn hello_network() -> String {
    // TODO: 修改返回值，使其包含 "Hello" 和 "Network" 两个单词
    // 例如: "Hello, Network!"
    todo!("返回一个包含 Hello 和 Network 的问候语")
}

/// 返回你的名字
///
/// 这个函数用于个性化你的学习体验
pub fn your_name() -> String {
    // TODO: 返回你的名字（或昵称）
    todo!("返回你的名字")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro_hello_greeting() {
        let greeting = hello_network();
        assert!(
            greeting.contains("Hello") || greeting.contains("hello"),
            "问候语应该包含 'Hello'"
        );
        assert!(
            greeting.contains("Network") || greeting.contains("network"),
            "问候语应该包含 'Network'"
        );
    }

    #[test]
    fn test_intro_hello_name() {
        let name = your_name();
        assert!(!name.is_empty(), "名字不能为空");
        assert!(name.len() >= 2, "名字至少应该有2个字符");
    }
}
