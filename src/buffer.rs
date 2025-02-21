//! Message buffering system for collecting output messages.
//!
//! This module provides message buffering using a global static buffer.
//! Use this to collect messages during processing that need to be displayed later.

use std::cell::RefCell;

// Global message buffer for storing log outputs
thread_local! {
    static MESSAGE_BUFFER: RefCell<Vec<String>> = RefCell::new(Vec::with_capacity(12));
}

/// Adds a message to the global buffer instead of printing immediately.
pub fn add_message(msg: String) {
    MESSAGE_BUFFER.with(|buffer| buffer.borrow_mut().push(msg));
}

/// Reads and returns all saved messages from the buffer.
pub fn read_messages() -> Vec<String> {
    MESSAGE_BUFFER.with(|buffer| buffer.borrow().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_read_messages() {
        // Clear any existing messages in the buffer
        MESSAGE_BUFFER.with(|buffer| buffer.borrow_mut().clear());

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
        MESSAGE_BUFFER.with(|buffer| buffer.borrow_mut().clear());

        // Read messages from an empty buffer and check if it's empty
        let messages = read_messages();
        assert!(messages.is_empty());
    }
}
