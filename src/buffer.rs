use std::sync::{Arc, Mutex};

// Global message buffer for storing log outputs
lazy_static::lazy_static! {
    static ref MESSAGE_BUFFER: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

/// Adds a message to the global buffer instead of printing immediately.
pub fn add_message(msg: String) {
    let mut buffer = MESSAGE_BUFFER.lock().unwrap();
    buffer.push(msg);
}

/// Reads and returns all saved messages from the buffer.
pub fn read_messages() -> Vec<String> {
    let buffer = MESSAGE_BUFFER.lock().unwrap();
    buffer.clone()
}
