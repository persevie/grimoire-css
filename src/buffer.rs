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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_read_messages() {
        // Clear any existing messages in the buffer
        {
            let mut buffer = MESSAGE_BUFFER.lock().unwrap();
            buffer.clear();
        }

        // Add messages to the buffer
        add_message("Message 1".to_string());
        add_message("Message 2".to_string());

        // Read messages and verify content
        let messages = read_messages();
        assert_eq!(
            messages,
            vec!["Message 1".to_string(), "Message 2".to_string()]
        );
    }

    #[test]
    fn test_empty_buffer() {
        // Clear any existing messages in the buffer
        {
            let mut buffer = MESSAGE_BUFFER.lock().unwrap();
            buffer.clear();
        }

        // Read messages from an empty buffer and check if it's empty
        let messages = read_messages();
        assert!(messages.is_empty());
    }
}
