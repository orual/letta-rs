//! Integration tests for Composio endpoints.

use letta_rs::{LettaClient, Tool};
use serial_test::serial;

fn setup_test_client() -> LettaClient {
    LettaClient::local().expect("Failed to create local client")
}

#[tokio::test]
#[serial]
#[ignore = "ensure COMPOSIO_API_KEY is set in your server environment before running this test"]
async fn test_list_composio_apps() {
    let client = setup_test_client();

    // Try to list Composio apps
    match client.tools().list_composio_apps().await {
        Ok(apps) => {
            println!("Found {} Composio apps", apps.len());
            for app in apps.iter().take(5) {
                println!("  - {} ({}): {}", app.name, app.key, app.description);
            }
        }
        Err(e) => {
            eprintln!("Error listing Composio apps: {:?}", e);
            // This is expected if Composio is not configured
        }
    }
}

#[tokio::test]
#[serial]
#[ignore = "ensure COMPOSIO_API_KEY is set in your server environment before running this test"]
async fn test_list_composio_actions() {
    let client = setup_test_client();

    // First list apps to get a valid app name
    match client.tools().list_composio_apps().await {
        Ok(apps) => {
            if let Some(first_app) = apps.first() {
                println!("Testing actions for app: {}", first_app.name);

                // List actions for the first app
                match client.tools().list_composio_actions(&first_app.name).await {
                    Ok(actions) => {
                        println!("Found {} actions for {}", actions.len(), first_app.name);
                        for action in actions.iter().take(5) {
                            println!("  - {}: {}", action.name, action.description);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing actions: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing Composio apps: {:?}", e);
        }
    }
}

#[tokio::test]
#[serial]
#[ignore = "ensure COMPOSIO_API_KEY is set in your server environment before running this test"]
async fn test_add_composio_tool() {
    let client = setup_test_client();

    // We know this action name is valid so long as the server has a Composio key configured
    match client.tools().add_composio_tool("resend_send_email").await {
        Ok(tool) => {
            println!("Added Composio tool: {}", tool.name);

            // Clean up
            if let Some(id) = &tool.id {
                let _ = client.tools().delete(id).await;
            }
        }
        Err(e) => {
            eprintln!("Error adding Composio tool: {:?}", e);
            panic!("Configure the api key")
            // This is expected if the action doesn't exist or Composio isn't configured
        }
    }
}
