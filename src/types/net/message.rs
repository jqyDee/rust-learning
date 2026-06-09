use std::fmt::Display;

#[derive(Clone)]
pub struct Message {
    pub sender_id: u64,
    content: String,
    is_disconnect_msg: bool,
}

impl Message {
    pub fn from(sender_id: u64, content: String) -> Self {
        Self { sender_id, content, is_disconnect_msg: false}
    }

    pub fn set_is_disconnect_msg(mut self) -> Self {
        self.is_disconnect_msg = true;
        self
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_disconnect_msg == true {
            write!(f, "sender with id: {} disconnected!", self.sender_id)
        } else {
            write!(f, "{}", self.content)
        }
    }
}
