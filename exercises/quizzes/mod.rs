//! Quizzes - Mini Project Exercises
//!
//! These are comprehensive exercises that combine multiple concepts
//! into small project-level applications.

#[path = "quiz1_ping.rs"]
pub mod quiz1_ping;

#[path = "quiz2_http_downloader.rs"]
pub mod quiz2_http_downloader;

#[path = "quiz3_chat_protocol.rs"]
pub mod quiz3_chat_protocol;

#[path = "quiz4_dns_client.rs"]
pub mod quiz4_dns_client;

#[path = "quiz5_reliable_udp.rs"]
pub mod quiz5_reliable_udp;

pub use quiz1_ping::*;
pub use quiz2_http_downloader::*;
pub use quiz3_chat_protocol::*;
pub use quiz4_dns_client::*;
pub use quiz5_reliable_udp::*;
