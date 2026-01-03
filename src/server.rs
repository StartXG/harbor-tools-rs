use crate::tools::base64_encode;
use crate::types::{HarborServer,HarborClient,HarborHealth,ImageTags};
use reqwest::Client;


impl HarborClient {
    pub fn new(host: impl Into<String>, port: i32, use_tls: bool, username: impl Into<String>, password: impl Into<String>) -> Self {
        HarborClient {
            host: host.into(),
            port,
            use_tls,
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn to_server(&self) -> HarborServer {
        let protocol = if self.use_tls { "https" } else { "http" };
        let address = format!("{}://{}:{}", protocol, self.host, self.port);
        let credentials = format!("{}:{}", self.username, self.password);
        let token = base64_encode(&credentials);
        HarborServer {
            address,
            token,
        }
    }
}

impl HarborServer {
    pub async fn health(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/api/v2.0/health", self.address);
        let client = Client::new();
        match client.get(&url).header("Authorization", format!("Basic {}", self.token)).send().await {
            Ok(response) => {
                println!("Response Status: {}", response.status());
                let body: HarborHealth = response.json().await?;
                Ok(body.status.to_string())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_tags_by_project_and_repository(&self, repository: &str, project: &str) -> Result<Vec<ImageTags>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v2.0/projects/{}/repositories/{}/artifacts", self.address, project, repository);
        let client = Client::builder().build()?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Basic {}", self.token).parse()?);
        let request = client.request(reqwest::Method::GET, &url).headers(headers);
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(format!("Failed to fetch tags: HTTP {}", response.status()).into());
        }
        let artifacts: Vec<serde_json::Value> = response.json().await?;
        let mut tags = Vec::new();
        for artifact in artifacts {
            if let Some(tag_array) = artifact.get("tags").and_then(|t| t.as_array()) {
                for tag in tag_array {
                    if let Some(tag_str) = tag.get("name").and_then(|n| n.as_str()) {
                        let push_time = artifact.get("push_time").and_then(|p| p.as_str()).unwrap_or("").to_string();
                        tags.push(ImageTags {
                            tag: tag_str.to_string(),
                            push_time,
                        });
                    }
                }
            }
        }
        Ok(tags)
    }
}