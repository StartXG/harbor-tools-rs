use std::fs;
use serde::{Deserialize, Serialize};
use harbor_tools_rs::types::HarborClient;
use toml;

const TEST_CONFIG_PATH: &str = "tests/test.toml";

#[derive(Serialize,Deserialize)]
struct HarborProject{
    project:String,
    repository:String,
}

#[derive(Deserialize, Clone)]
struct HarborServerConfig {
    host: String,
    port: i32,
    #[serde(rename = "use_ssl")]
    use_tls: bool,
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
struct HarborProjectConfig {
    project: String,
    repository: String,
}

#[derive(Deserialize, Clone)]
struct TestConfig {
    #[serde(rename = "HarborServer")]
    server: HarborServerConfig,
    #[serde(rename = "HarborProject")]
    project: HarborProjectConfig,
}

fn load_test_config() -> TestConfig {
    let toml_content = fs::read_to_string(TEST_CONFIG_PATH)
        .expect("Failed to read tests/test.toml");
    toml::from_str(&toml_content).expect("Invalid or incomplete tests/test.toml")
}

impl HarborProject {
    fn new(project: impl Into<String>, repo: impl Into<String>) -> Self {
        HarborProject {
            project: project.into(),
            repository: repo.into(),
        }
    }
}

fn test_client() -> HarborClient {
    let TestConfig { server, .. } = load_test_config();
    HarborClient::new(
        server.host.as_str(),
        server.port,
        server.use_tls,
        server.username.as_str(),
        server.password.as_str(),
    )
}

fn test_rp() -> HarborProject{
    // read from toml file
    let TestConfig { project, .. } = load_test_config();
    HarborProject::new(project.project, project.repository)
}

#[test]
fn to_server_builds_address_and_token() {
    let client = test_client();
    let server = client.to_server();

    println!("Server: {}", server.address);
    println!("Token: {}", server.token);
}

#[tokio::test]
async fn health_checks_server_health() {
    let client = test_client();
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
    let p = test_rp();
    let client = test_client();
    let server = client.to_server();
    let result = server.get_tags_by_project_and_repository(&p.repository,&p.project).await;
    match result {
        Ok(tags) => {

            println!("Fetched {} tags", tags.len());
        }
        Err(e) => {
            panic!("Failed to get image tags: {}", e);
        }
    }

}