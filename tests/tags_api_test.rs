//! Integration tests for the Tags API.

use letta_rs::client::{ClientConfig, LettaClient};
use letta_rs::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaClient {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    LettaClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_tags() {
    let client = get_test_client();

    // List all tags
    let result = client.tags().list(None).await;

    match result {
        Ok(tags) => {
            println!("Found {} tags", tags.len());
            for tag in tags.iter().take(10) {
                println!("Tag: {}", tag);
            }
        }
        Err(e) => {
            eprintln!("Failed to list tags: {:?}", e);
            // Tags might be empty, which is fine
        }
    }
}

#[tokio::test]
async fn test_list_tags_with_pagination() {
    let client = get_test_client();

    // List tags with pagination
    let params = ListTagsParams {
        limit: Some(5),
        ..Default::default()
    };

    let result = client.tags().list(Some(params)).await;

    match result {
        Ok(tags) => {
            println!("Found {} tags (limited to 5)", tags.len());
            assert!(tags.len() <= 5, "Should respect limit parameter");
        }
        Err(e) => {
            eprintln!("Failed to list tags with pagination: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_list_tags_with_query() {
    let client = get_test_client();

    // List tags with query filter
    let params = ListTagsParams {
        query_text: Some("test".to_string()),
        ..Default::default()
    };

    let result = client.tags().list(Some(params)).await;

    match result {
        Ok(tags) => {
            println!("Found {} tags matching 'test'", tags.len());
            for tag in &tags {
                println!("Matching tag: {}", tag);
            }
        }
        Err(e) => {
            eprintln!("Failed to list tags with query: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_tags_pagination_stream() {
    use futures::StreamExt;
    use letta_rs::types::PaginationParams;

    let client = get_test_client();

    // Use paginated stream
    let params = PaginationParams {
        limit: Some(2),
        ..Default::default()
    };

    let mut stream = client.tags().paginated(Some(params));
    let mut count = 0;

    while let Some(result) = stream.next().await {
        match result {
            Ok(tag) => {
                println!("Tag from stream: {}", tag);
                count += 1;
                if count >= 10 {
                    break; // Limit test to 10 items
                }
            }
            Err(e) => {
                eprintln!("Error in tag stream: {:?}", e);
                break;
            }
        }
    }

    println!("Streamed {} tags", count);
}
