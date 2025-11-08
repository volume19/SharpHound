//! SharpHound - Active Directory Reconnaissance Tool
//!
//! Entry point for the SharpHound binary

use sharphound::logging::BasicLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logger = BasicLogger::new(2); // Information level
    logger.information("SharpHound Rust Port - Starting...");
    logger.information("This version is compatible with BloodHound 5.0.0");

    // TODO: CLI parsing and collection orchestration will be implemented here

    logger.information("Initialization complete (basic modules only)");
    Ok(())
}
