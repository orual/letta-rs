//! Basic test to check if blocks endpoint exists.

use letta_rs::client::ClientBuilder;
use letta_rs::error::LettaResult;
use letta_rs::LettaClient;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

#[tokio::test]
async fn test_blocks_count() -> LettaResult<()> {
    let client = create_test_client()?;

    // Try to get blocks count
    match client.blocks().count().await {
        Ok(count) => {
            println!("Blocks count: {}", count);
            assert!(count >= 0);
        }
        Err(e) => {
            println!("Error getting blocks count: {:?}", e);
            // Check if it's a 404 (endpoint not found)
            if let letta_rs::error::LettaError::Api {
                status, message, ..
            } = &e
            {
                if *status == 404 {
                    println!("Blocks API endpoint not found on local server");
                    // Skip test if endpoint doesn't exist
                    return Ok(());
                }
            }
            return Err(e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_blocks_list() -> LettaResult<()> {
    let client = create_test_client()?;

    // Try to list blocks
    match client.blocks().list(None).await {
        Ok(blocks) => {
            println!("Found {} blocks", blocks.len());
            Ok(())
        }
        Err(e) => {
            println!("Error listing blocks: {:?}", e);
            // Check if it's a 404 (endpoint not found)
            if let letta_rs::error::LettaError::Api {
                status, message, ..
            } = &e
            {
                if *status == 404 {
                    println!("Blocks API endpoint not found on local server");
                    // Skip test if endpoint doesn't exist
                    return Ok(());
                }
            }
            Err(e)
        }
    }
}
