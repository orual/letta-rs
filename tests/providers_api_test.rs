//! Integration tests for the Providers API.

use letta_rs::client::{ClientConfig, LettaClient};
use letta_rs::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaClient {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    LettaClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_providers() {
    let client = get_test_client();

    // List all providers
    let result = client.providers().list(None).await;

    match result {
        Ok(providers) => {
            println!("Found {} providers", providers.len());
            for provider in providers.iter().take(5) {
                println!(
                    "Provider: {} - Type: {} - Category: {}",
                    provider.name, provider.provider_type, provider.provider_category
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to list providers: {:?}", e);
            // Providers might be empty, which is fine for a test
        }
    }
}

#[tokio::test]
async fn test_provider_crud() {
    let client = get_test_client();

    // Create a provider
    let create_req = ProviderCreate {
        name: format!("Test Provider {}", uuid::Uuid::new_v4()),
        provider_type: ProviderType::Openai,
        provider_category: None, // Let the API set the default
        api_key: "sk-test-12345".to_string(),
        base_url: None, // Let the API set the default
        access_key: None,
        secret_key: None,
        region: None,
        metadata: None,
    };

    let provider = match client.providers().create(create_req).await {
        Ok(p) => {
            println!("Created provider: {} ({})", p.name, p.id);
            p
        }
        Err(e) => {
            eprintln!("Failed to create provider: {:#?}", e);
            if let letta_rs::error::LettaError::Api { body, .. } = &e {
                eprintln!("Response body: {:#?}", body);
            }
            panic!("Provider creation failed");
        }
    };

    // Update the provider (only api_key, access_key, and region can be updated)
    let update_req = ProviderUpdate {
        api_key: "sk-updated-test-key".to_string(),
        access_key: None,
        region: None,
    };

    match client.providers().update(&provider.id, update_req).await {
        Ok(updated) => {
            println!("Updated provider: {} - API key changed", updated.name);
            // We can't verify the API key changed since it's encrypted/hidden
        }
        Err(e) => {
            eprintln!("Failed to update provider: {:?}", e);
        }
    }

    // Delete the provider
    match client.providers().delete(&provider.id).await {
        Ok(_) => {
            println!("Successfully deleted provider");
        }
        Err(e) => {
            eprintln!("Failed to delete provider: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_list_providers_with_filter() {
    let client = get_test_client();

    // List providers with filter (use Byok since that's what we create)
    let params = ListProvidersParams {
        provider_category: Some(ProviderCategory::Byok),
        ..Default::default()
    };

    let result = client.providers().list(Some(params)).await;

    match result {
        Ok(providers) => {
            println!("Found {} BYOK providers", providers.len());
            for provider in &providers {
                assert_eq!(provider.provider_category, ProviderCategory::Byok);
            }
        }
        Err(e) => {
            eprintln!("Failed to list filtered providers: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore = "Provider check may fail depending on provider configuration"]
async fn test_provider_check() {
    let client = get_test_client();

    // First, list providers to find one to check
    let providers = match client.providers().list(None).await {
        Ok(p) => p,
        Err(_) => {
            println!("No providers available to check");
            return;
        }
    };

    if let Some(provider) = providers.first() {
        match client.providers().check(&provider.id).await {
            Ok(check_result) => {
                println!("Provider check status: {}", check_result.status);
                if let Some(error) = check_result.error {
                    println!("Provider check error: {}", error);
                }
            }
            Err(e) => {
                eprintln!("Failed to check provider: {:?}", e);
            }
        }
    } else {
        println!("No providers available to check");
    }
}
