//! Quiz 3: Simple Chat Protocol
//!
//! Design and implement a complete chat protocol with:
//! - Message framing with length prefix
//! - Multiple message types (join, leave, message, broadcast, private)
//! - User session management
//! - Room/channel support
//! - Heartbeat/keepalive mechanism
//!
//! This quiz combines: framing, state machine, protocol design, serialization

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Chat protocol message types
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    // Connection management
    Join,           // 0x01: Join the chat server
    JoinAck,        // 0x02: Server acknowledges join
    Leave,          // 0x03: Leave the chat server
    Ping,           // 0x04: Keepalive ping
    Pong,           // 0x05: Keepalive pong

    // Chat messages
    Broadcast,      // 0x10: Message to all users
    Private,        // 0x11: Private message to specific user
    RoomMessage,    // 0x12: Message to a room

    // Room management
    RoomJoin,       // 0x20: Join a room
    RoomLeave,      // 0x21: Leave a room
    RoomList,       // 0x22: List available rooms
    RoomUsers,      // 0x23: List users in a room

    // Notifications
    UserJoined,     // 0x30: Notification that user joined
    UserLeft,       // 0x31: Notification that user left
    Error,          // 0xFF: Error message
}

impl MessageType {
    pub fn to_u8(&self) -> u8 {
        match self {
            MessageType::Join => 0x01,
            MessageType::JoinAck => 0x02,
            MessageType::Leave => 0x03,
            MessageType::Ping => 0x04,
            MessageType::Pong => 0x05,
            MessageType::Broadcast => 0x10,
            MessageType::Private => 0x11,
            MessageType::RoomMessage => 0x12,
            MessageType::RoomJoin => 0x20,
            MessageType::RoomLeave => 0x21,
            MessageType::RoomList => 0x22,
            MessageType::RoomUsers => 0x23,
            MessageType::UserJoined => 0x30,
            MessageType::UserLeft => 0x31,
            MessageType::Error => 0xFF,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        // TODO: Convert u8 to MessageType
        // Return None for invalid values
        todo!("Convert u8 to MessageType")
    }
}

/// Chat protocol message
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub msg_type: MessageType,
    pub sender: Option<String>,
    pub target: Option<String>,  // recipient user or room
    pub content: String,
    pub timestamp: u64,
}

impl ChatMessage {
    pub fn new(msg_type: MessageType, content: String) -> Self {
        Self {
            msg_type,
            sender: None,
            target: None,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_sender(mut self, sender: String) -> Self {
        self.sender = Some(sender);
        self
    }

    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Serialize message to bytes
    ///
    /// Wire format:
    /// [length: 4 bytes][type: 1 byte][timestamp: 8 bytes]
    /// [sender_len: 1 byte][sender: N bytes]
    /// [target_len: 1 byte][target: N bytes]
    /// [content: remaining bytes]
    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize the message to wire format
        // 1. Build payload: type + timestamp + sender + target + content
        // 2. Prepend 4-byte length prefix (big-endian)

        todo!("Serialize chat message")
    }

    /// Deserialize message from bytes (without length prefix)
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        // TODO: Parse the message from bytes
        // 1. Extract type (1 byte)
        // 2. Extract timestamp (8 bytes, big-endian)
        // 3. Extract sender (1 byte length + data)
        // 4. Extract target (1 byte length + data)
        // 5. Extract content (remaining bytes as UTF-8)

        todo!("Deserialize chat message")
    }
}

/// Message framer for reading/writing framed messages
pub struct MessageFramer {
    buffer: Vec<u8>,
    max_message_size: usize,
}

impl MessageFramer {
    pub fn new(max_message_size: usize) -> Self {
        Self {
            buffer: Vec::new(),
            max_message_size,
        }
    }

    /// Add data to the buffer
    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Try to extract a complete message
    pub fn try_read_message(&mut self) -> Result<Option<ChatMessage>, &'static str> {
        // TODO: Try to read a complete framed message
        // 1. Check if we have at least 4 bytes for length
        // 2. Read length prefix
        // 3. Check if length exceeds max_message_size
        // 4. Check if we have enough data for the full message
        // 5. Extract and parse the message
        // 6. Remove consumed bytes from buffer

        todo!("Read framed message")
    }

    /// Frame a message for sending
    pub fn frame_message(msg: &ChatMessage) -> Vec<u8> {
        msg.to_bytes()
    }
}

/// Chat user session
#[derive(Debug)]
pub struct UserSession {
    pub username: String,
    pub rooms: HashSet<String>,
    pub last_activity: Instant,
    pub joined_at: Instant,
}

impl UserSession {
    pub fn new(username: String) -> Self {
        let now = Instant::now();
        Self {
            username,
            rooms: HashSet::new(),
            last_activity: now,
            joined_at: now,
        }
    }

    /// Check if the session has timed out
    pub fn is_timed_out(&self, timeout: Duration) -> bool {
        // TODO: Check if last_activity is older than timeout
        todo!("Check session timeout")
    }

    /// Update last activity timestamp
    pub fn touch(&mut self) {
        // TODO: Update last_activity to now
        todo!("Update activity timestamp")
    }

    /// Join a room
    pub fn join_room(&mut self, room: String) -> bool {
        // TODO: Add room to user's rooms set
        // Return true if newly joined, false if already in room
        todo!("Join room")
    }

    /// Leave a room
    pub fn leave_room(&mut self, room: &str) -> bool {
        // TODO: Remove room from user's rooms set
        // Return true if was in room, false if wasn't
        todo!("Leave room")
    }
}

/// Chat room
#[derive(Debug)]
pub struct ChatRoom {
    pub name: String,
    pub members: HashSet<String>,
    pub message_history: Vec<ChatMessage>,
    pub max_history: usize,
}

impl ChatRoom {
    pub fn new(name: String, max_history: usize) -> Self {
        Self {
            name,
            members: HashSet::new(),
            message_history: Vec::new(),
            max_history,
        }
    }

    /// Add a user to the room
    pub fn add_member(&mut self, username: String) -> bool {
        self.members.insert(username)
    }

    /// Remove a user from the room
    pub fn remove_member(&mut self, username: &str) -> bool {
        self.members.remove(username)
    }

    /// Add a message to history
    pub fn add_message(&mut self, msg: ChatMessage) {
        // TODO: Add message to history
        // Remove oldest message if history exceeds max_history
        todo!("Add message to room history")
    }

    /// Get recent messages
    pub fn recent_messages(&self, count: usize) -> &[ChatMessage] {
        // TODO: Return the last 'count' messages
        todo!("Get recent messages")
    }
}

/// Chat server state machine
pub struct ChatServer {
    users: HashMap<String, UserSession>,
    rooms: HashMap<String, ChatRoom>,
    session_timeout: Duration,
    ping_interval: Duration,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            rooms: HashMap::new(),
            session_timeout: Duration::from_secs(60),
            ping_interval: Duration::from_secs(15),
        }
    }

    /// Process an incoming message and generate responses
    pub fn process_message(&mut self, msg: ChatMessage) -> Vec<ChatMessage> {
        // TODO: Process the message based on its type
        // Return a list of messages to send (responses/broadcasts)
        //
        // Handle each message type:
        // - Join: Create user session, send JoinAck, broadcast UserJoined
        // - Leave: Remove session, broadcast UserLeft
        // - Ping: Respond with Pong
        // - Broadcast: Send to all users
        // - Private: Send to specific user
        // - RoomJoin: Add user to room
        // - RoomMessage: Send to room members
        // etc.

        todo!("Process incoming message")
    }

    /// Handle user join
    fn handle_join(&mut self, username: String) -> Vec<ChatMessage> {
        // TODO: Create session and generate responses
        // 1. Check if username is already taken
        // 2. Create UserSession
        // 3. Send JoinAck to the user
        // 4. Broadcast UserJoined to all other users
        todo!("Handle join")
    }

    /// Handle user leave
    fn handle_leave(&mut self, username: &str) -> Vec<ChatMessage> {
        // TODO: Remove session and notify others
        // 1. Remove from all rooms
        // 2. Remove session
        // 3. Broadcast UserLeft
        todo!("Handle leave")
    }

    /// Handle broadcast message
    fn handle_broadcast(&mut self, msg: ChatMessage) -> Vec<ChatMessage> {
        // TODO: Send message to all connected users
        todo!("Handle broadcast")
    }

    /// Handle private message
    fn handle_private(&mut self, msg: ChatMessage) -> Vec<ChatMessage> {
        // TODO: Send message to specific user
        // Return error if user doesn't exist
        todo!("Handle private message")
    }

    /// Handle room join
    fn handle_room_join(&mut self, username: &str, room_name: &str) -> Vec<ChatMessage> {
        // TODO: Add user to room
        // 1. Create room if it doesn't exist
        // 2. Add user to room members
        // 3. Update user session
        // 4. Notify room members
        todo!("Handle room join")
    }

    /// Handle room message
    fn handle_room_message(&mut self, msg: ChatMessage) -> Vec<ChatMessage> {
        // TODO: Send message to all room members
        todo!("Handle room message")
    }

    /// Check for timed out sessions
    pub fn check_timeouts(&mut self) -> Vec<ChatMessage> {
        // TODO: Find timed out sessions and generate Leave messages
        todo!("Check timeouts")
    }

    /// Get list of online users
    pub fn online_users(&self) -> Vec<&str> {
        // TODO: Return list of online usernames
        todo!("Get online users")
    }

    /// Get list of rooms
    pub fn list_rooms(&self) -> Vec<&str> {
        // TODO: Return list of room names
        todo!("List rooms")
    }
}

/// Client-side chat protocol handler
pub struct ChatClient {
    username: String,
    pending_pings: HashMap<u64, Instant>,
    last_ping_sent: Option<Instant>,
}

impl ChatClient {
    pub fn new(username: String) -> Self {
        Self {
            username,
            pending_pings: HashMap::new(),
            last_ping_sent: None,
        }
    }

    /// Create a join message
    pub fn create_join(&self) -> ChatMessage {
        // TODO: Create Join message with username
        todo!("Create join message")
    }

    /// Create a broadcast message
    pub fn create_broadcast(&self, content: String) -> ChatMessage {
        // TODO: Create Broadcast message
        todo!("Create broadcast message")
    }

    /// Create a private message
    pub fn create_private(&self, recipient: String, content: String) -> ChatMessage {
        // TODO: Create Private message
        todo!("Create private message")
    }

    /// Create a room message
    pub fn create_room_message(&self, room: String, content: String) -> ChatMessage {
        // TODO: Create RoomMessage
        todo!("Create room message")
    }

    /// Create a ping message
    pub fn create_ping(&mut self) -> ChatMessage {
        // TODO: Create Ping message and track it
        todo!("Create ping message")
    }

    /// Process a received pong, return RTT
    pub fn process_pong(&mut self, msg: &ChatMessage) -> Option<Duration> {
        // TODO: Calculate RTT from pong response
        todo!("Process pong")
    }

    /// Check if we need to send a ping
    pub fn needs_ping(&self, interval: Duration) -> bool {
        // TODO: Check if it's time to send a ping
        todo!("Check needs ping")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_conversion() {
        assert_eq!(MessageType::from_u8(0x01), Some(MessageType::Join));
        assert_eq!(MessageType::from_u8(0x10), Some(MessageType::Broadcast));
        assert_eq!(MessageType::from_u8(0xFF), Some(MessageType::Error));
        assert_eq!(MessageType::from_u8(0xAB), None);
    }

    #[test]
    fn test_message_serialization() {
        let msg = ChatMessage::new(MessageType::Broadcast, "Hello!".to_string())
            .with_sender("alice".to_string());

        let bytes = msg.to_bytes();
        let parsed = ChatMessage::from_bytes(&bytes[4..]).unwrap(); // Skip length prefix

        assert_eq!(parsed.msg_type, MessageType::Broadcast);
        assert_eq!(parsed.sender, Some("alice".to_string()));
        assert_eq!(parsed.content, "Hello!");
    }

    #[test]
    fn test_message_framer() {
        let mut framer = MessageFramer::new(1024);

        let msg = ChatMessage::new(MessageType::Ping, "".to_string());
        let bytes = msg.to_bytes();

        // Feed partial data
        framer.feed(&bytes[..5]);
        assert!(framer.try_read_message().unwrap().is_none());

        // Feed rest of data
        framer.feed(&bytes[5..]);
        let parsed = framer.try_read_message().unwrap().unwrap();
        assert_eq!(parsed.msg_type, MessageType::Ping);
    }

    #[test]
    fn test_user_session() {
        let mut session = UserSession::new("alice".to_string());

        assert!(session.join_room("general".to_string()));
        assert!(!session.join_room("general".to_string())); // Already joined

        assert!(session.leave_room("general"));
        assert!(!session.leave_room("general")); // Already left
    }

    #[test]
    fn test_chat_room() {
        let mut room = ChatRoom::new("general".to_string(), 10);

        room.add_member("alice".to_string());
        room.add_member("bob".to_string());

        let msg = ChatMessage::new(MessageType::RoomMessage, "Hello room!".to_string())
            .with_sender("alice".to_string());

        room.add_message(msg);

        assert_eq!(room.members.len(), 2);
        assert_eq!(room.recent_messages(10).len(), 1);
    }

    #[test]
    fn test_server_join() {
        let mut server = ChatServer::new();

        let join_msg = ChatMessage::new(MessageType::Join, "alice".to_string());
        let responses = server.process_message(join_msg);

        // Should get JoinAck
        assert!(responses.iter().any(|m| m.msg_type == MessageType::JoinAck));

        // User should be online
        assert!(server.online_users().contains(&"alice"));
    }

    #[test]
    fn test_server_broadcast() {
        let mut server = ChatServer::new();

        // Join two users
        server.process_message(ChatMessage::new(MessageType::Join, "alice".to_string()));
        server.process_message(ChatMessage::new(MessageType::Join, "bob".to_string()));

        // Send broadcast
        let broadcast = ChatMessage::new(MessageType::Broadcast, "Hello everyone!".to_string())
            .with_sender("alice".to_string());

        let responses = server.process_message(broadcast);

        // Both users should receive the message
        assert!(responses.len() >= 2);
    }

    #[test]
    fn test_client_messages() {
        let mut client = ChatClient::new("alice".to_string());

        let join = client.create_join();
        assert_eq!(join.msg_type, MessageType::Join);

        let broadcast = client.create_broadcast("Hello!".to_string());
        assert_eq!(broadcast.msg_type, MessageType::Broadcast);
        assert_eq!(broadcast.sender, Some("alice".to_string()));

        let private = client.create_private("bob".to_string(), "Hi Bob!".to_string());
        assert_eq!(private.msg_type, MessageType::Private);
        assert_eq!(private.target, Some("bob".to_string()));
    }

    #[test]
    fn test_ping_pong() {
        let mut client = ChatClient::new("alice".to_string());

        let ping = client.create_ping();
        assert_eq!(ping.msg_type, MessageType::Ping);

        std::thread::sleep(Duration::from_millis(10));

        let pong = ChatMessage::new(MessageType::Pong, ping.content.clone());
        let rtt = client.process_pong(&pong);

        assert!(rtt.is_some());
        assert!(rtt.unwrap() >= Duration::from_millis(10));
    }
}
