//! Integration tests for the Models API.

use letta_rs::client::{ClientConfig, LettaClient};
use letta_rs::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaClient {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    LettaClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_models() {
    let client = get_test_client();

    // List all models
    let result = client.models().list(None).await;

    match result {
        Ok(models) => {
            println!("Found {} LLM models", models.len());
            for model in models.iter().take(5) {
                println!(
                    "Model: {} - Type: {:?} - Context: {}",
                    model.model, model.model_endpoint_type, model.context_window
                );
            }
            assert!(
                !models.is_empty(),
                "Should have at least one model available"
            );
        }
        Err(e) => {
            eprintln!("Failed to list models: {:?}", e);
            panic!("Model listing should work");
        }
    }
}

#[tokio::test]
async fn test_list_models_with_filter() {
    let client = get_test_client();

    // List models with filter
    let params = ListModelsParams {
        provider_category: Some(vec![ProviderCategory::Base]),
        ..Default::default()
    };

    let result = client.models().list(Some(params)).await;

    let models = result.expect("Failed to list filtered models");
    println!(
        "Found {} models with provider_category filter",
        models.len()
    );
}

#[tokio::test]
async fn test_list_embedding_models() {
    let client = get_test_client();

    // List all embedding models
    let result = client.models().list_embedding_models(None).await;

    match result {
        Ok(models) => {
            println!("Found {} embedding models", models.len());
            for model in models.iter().take(5) {
                println!(
                    "Embedding Model: {} - Dim: {} - Chunk: {}",
                    model.embedding_model, model.embedding_dim, model.embedding_chunk_size
                );
            }
            assert!(
                !models.is_empty(),
                "Should have at least one embedding model available"
            );
        }
        Err(e) => {
            eprintln!("Failed to list embedding models: {:?}", e);
            panic!("Embedding model listing should work");
        }
    }
}
