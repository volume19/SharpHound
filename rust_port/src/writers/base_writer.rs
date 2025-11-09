//! Base writer trait and common functionality for output writers
//!
//! This module provides the base functionality for writing collected data to files.
//! Writers batch data in memory and flush to disk periodically to improve performance.

use anyhow::Result;
use async_trait::async_trait;

/// Batch size for writing data to disk
///
/// Writers accumulate this many items before flushing to disk.
/// This matches the C# implementation which flushes every 30 items.
pub const BATCH_SIZE: usize = 30;

/// Base writer trait for output writers
///
/// This trait defines the interface that all output writers must implement.
/// Writers are responsible for:
/// - Creating output files
/// - Batching data in memory
/// - Periodically flushing data to disk
/// - Final cleanup and metadata writing
///
/// # Type Parameters
///
/// * `T` - The type of data this writer handles (must be Send + Sync)
#[async_trait]
pub trait BaseWriter<T: Send + Sync>: Send + Sync {
    /// Accept a single item for writing
    ///
    /// Items are queued in memory and flushed to disk in batches.
    /// If the writer is in no-op mode, the item is discarded.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to write
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if writing fails
    async fn accept_object(&mut self, item: T) -> Result<()>;

    /// Write queued data to disk
    ///
    /// This method is called automatically when the batch size is reached,
    /// and must be implemented by concrete writers to handle the actual
    /// serialization and writing logic.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if writing fails
    async fn write_data(&mut self) -> Result<()>;

    /// Flush any remaining data and close the file
    ///
    /// This method should be called when all data has been written.
    /// It ensures any remaining queued items are written and performs
    /// cleanup operations like writing metadata and closing file handles.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if flushing fails
    async fn flush_writer(&mut self) -> Result<()>;

    /// Create the output file
    ///
    /// This method is called automatically when the first item is accepted.
    /// Implementations should create the file, write any headers or preamble,
    /// and initialize file handles.
    ///
    /// # Returns
    ///
    /// * `Ok(())` on success
    /// * `Err` if file creation fails
    fn create_file(&mut self) -> Result<()>;

    /// Check if the writer is in no-op mode
    ///
    /// When in no-op mode, the writer discards all data without writing.
    fn is_no_op(&self) -> bool;

    /// Get the number of items written so far
    fn count(&self) -> usize;

    /// Check if the output file has been created
    fn is_file_created(&self) -> bool;
}

/// Common state for base writers
///
/// This struct holds the common state shared by all writer implementations.
/// It manages the queue, batching logic, and file creation state.
///
/// # Type Parameters
///
/// * `T` - The type of data this writer handles
pub struct BaseWriterState<T> {
    /// The type of data being written (e.g., "users", "computers", "groups")
    pub data_type: String,

    /// Queue of items waiting to be written
    pub queue: Vec<T>,

    /// Whether the output file has been created
    pub file_created: bool,

    /// Total number of items written
    pub count: usize,

    /// If true, discard all data without writing
    pub no_op: bool,
}

impl<T> BaseWriterState<T> {
    /// Create a new base writer state
    ///
    /// # Arguments
    ///
    /// * `data_type` - The type of data being written
    ///
    /// # Examples
    ///
    /// ```
    /// use sharphound::writers::BaseWriterState;
    ///
    /// let state = BaseWriterState::<String>::new("users".to_string());
    /// assert_eq!(state.data_type, "users");
    /// assert_eq!(state.count, 0);
    /// assert!(!state.file_created);
    /// ```
    pub fn new(data_type: String) -> Self {
        BaseWriterState {
            data_type,
            queue: Vec::new(),
            file_created: false,
            count: 0,
            no_op: false,
        }
    }

    /// Check if the queue should be flushed
    ///
    /// Returns true if the queue has reached the batch size.
    pub fn should_flush(&self) -> bool {
        self.count > 0 && self.count % BATCH_SIZE == 0
    }

    /// Mark the file as created
    pub fn mark_file_created(&mut self) {
        self.file_created = true;
    }

    /// Increment the item count and add to queue
    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
        self.count += 1;
    }

    /// Clear the queue after flushing
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    /// Enable no-op mode (discard all data)
    pub fn set_no_op(&mut self, no_op: bool) {
        self.no_op = no_op;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_writer_state_new() {
        let state = BaseWriterState::<String>::new("test".to_string());
        assert_eq!(state.data_type, "test");
        assert_eq!(state.count, 0);
        assert!(!state.file_created);
        assert!(!state.no_op);
        assert_eq!(state.queue.len(), 0);
    }

    #[test]
    fn test_enqueue() {
        let mut state = BaseWriterState::new("test".to_string());
        state.enqueue("item1".to_string());
        state.enqueue("item2".to_string());

        assert_eq!(state.count, 2);
        assert_eq!(state.queue.len(), 2);
        assert_eq!(state.queue[0], "item1");
        assert_eq!(state.queue[1], "item2");
    }

    #[test]
    fn test_should_flush() {
        let mut state = BaseWriterState::new("test".to_string());

        // No flush needed initially
        assert!(!state.should_flush());

        // Add items up to batch size
        for i in 0..BATCH_SIZE {
            state.enqueue(i);
        }

        // Should flush at batch size
        assert_eq!(state.count, BATCH_SIZE);
        assert!(state.should_flush());

        // Add one more
        state.enqueue(BATCH_SIZE);
        assert!(!state.should_flush()); // Not at next batch boundary

        // Add more to reach next batch
        for i in (BATCH_SIZE + 1)..(BATCH_SIZE * 2) {
            state.enqueue(i);
        }
        assert!(state.should_flush());
    }

    #[test]
    fn test_clear_queue() {
        let mut state = BaseWriterState::new("test".to_string());
        state.enqueue("item1".to_string());
        state.enqueue("item2".to_string());

        state.clear_queue();
        assert_eq!(state.queue.len(), 0);
        assert_eq!(state.count, 2); // Count is preserved
    }

    #[test]
    fn test_file_created() {
        let mut state = BaseWriterState::<String>::new("test".to_string());
        assert!(!state.file_created);

        state.mark_file_created();
        assert!(state.file_created);
    }

    #[test]
    fn test_no_op() {
        let mut state = BaseWriterState::<String>::new("test".to_string());
        assert!(!state.no_op);

        state.set_no_op(true);
        assert!(state.no_op);

        state.set_no_op(false);
        assert!(!state.no_op);
    }

    #[test]
    fn test_batch_size_constant() {
        // Ensure batch size matches C# implementation
        assert_eq!(BATCH_SIZE, 30);
    }
}
