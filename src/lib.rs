// src/lib.rs
/*
 * Core library for NodeRunner
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Custom result type with error handling
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents the result of processing
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
#[derive(Debug)]
pub struct NodeRunnerProcessor {
    /// Whether to print debug information
    pub verbose: bool,
    /// Number of items processed
    pub processed_count: usize,
}

impl NodeRunnerProcessor {
    /// Creates a new processor with the specified verbosity
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Processes the given data
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
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Initialize logging based on verbosity
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    // Log start of processing
    info!("Starting NodeRunner processing");
    
    // Create a new processor
    let mut processor = NodeRunnerProcessor::new(verbose);
    
    // Read input from the provided path
    let input_data = match input {
        Some(path) => {
            info!("Reading input from: {}", path);
            fs::read_to_string(path)?
        }
        None => {
            info!("No input provided");
            "".to_string()
        }
    };

    // Process the input data
    let result = processor.process(&input_data)?;

    // Optionally write the result to a file
    if let Some(output_path) = output {
        info!("Writing result to: {}", output_path);
        fs::write(output_path, serde_json::to_string_pretty(&result)?)?;
    }

    Ok(())
}