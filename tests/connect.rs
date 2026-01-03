use base64::{engine::general_purpose, Engine as _};
use harbor_rs::types::HarborClient;
use tokio;

#[test]
fn to_server_builds_address_and_token() {
    let client = HarborClient::new("", , , "", "");
    let server = client.to_server();

    assert_eq!(server.address, "");
    assert_eq!(server.token, general_purpose::STANDARD.encode(""));
}

#[tokio::test]
async fn health_checks_server_health() {
    let client = HarborClient::new("", , , "", "");
    let server = client.to_server();
    let result = server.health();

    match result.await {
        Ok(status) => {
            println!("Harbor Health Status: {}", status);
            assert_eq!(status, "healthy");
        }
        Err(e) => {
            panic!("Failed to get health status: {}", e);
        }

    }
}

#[tokio::test]
async fn get_tags_by_project_and_repository_fetches_tags() {
    let client = HarborClient::new("", , , "", "");
    let server = client.to_server();
    let result = server.get_tags_by_project_and_repository("", "").await;
    match result {
        Ok(tags) => {
            println!("Fetched {} tags", tags.len());
        }
        Err(e) => {
            panic!("Failed to get image tags: {}", e);
        }
    }

}