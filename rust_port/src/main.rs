//! SharpHound - Active Directory Reconnaissance Tool
//!
//! Entry point for the SharpHound binary

use clap::Parser;
use sharphound::{Options, logging::BasicLogger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command-line arguments
    let options = Options::parse();

    // Initialize logger with specified verbosity
    let logger = BasicLogger::new(options.verbosity);

    logger.information("SharpHound Rust Port - Starting...");
    logger.information("This version is compatible with BloodHound 5.0.0");

    // Validate options
    if let Err(e) = options.validate() {
        logger.critical(&format!("Option validation failed: {}", e));
        std::process::exit(1);
    }

    logger.information(&format!("Collection methods: {:?}", options.collectionmethods));
    logger.information(&format!("Threads: {}", options.threads));
    logger.information(&format!("Output directory: {}", options.outputdirectory.display()));

    // TODO: Collection orchestration will be implemented in next phases

    logger.information("Initialization complete (Foundation phase only)");
    logger.warning("Full collection not yet implemented - porting in progress");

    Ok(())
}
