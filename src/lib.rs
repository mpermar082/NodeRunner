// src/lib.rs
/*
 * Core library for NodeRunner
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Custom result type with error handling
/// # Type alias for a Result type with a boxed error
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents the result of processing
/// # Struct containing the outcome of processing
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    /// Whether the processing was successful
    pub success: bool,
    /// Message describing the outcome
    pub message: String,
    /// Optional data returned from processing
    pub data: Option<serde_json::Value>,
}

/// NodeRunner processor
/// # Struct representing the processor with verbosity and processed count
#[derive(Debug)]
pub struct NodeRunnerProcessor {
    /// Whether to print debug information
    pub verbose: bool,
    /// Number of items processed
    pub processed_count: usize,
}

impl NodeRunnerProcessor {
    /// Creates a new processor with the specified verbosity
    /// # Creates a new instance of the processor with verbosity
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Processes the given data
    /// # Processes the data and returns the result
    /// # Arguments:
    /// * `data`: The data to process as a string
    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        // Log processing information if verbosity is enabled
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    /// Returns statistics about the processor
    /// # Returns statistics as a JSON value
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}