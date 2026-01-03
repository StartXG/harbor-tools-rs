# Harbor Tools RS

A Rust library for interacting with the [Harbor Container Registry](https://goharbor.io/) API.

## Features

- **Health Check**: Check the health status of a Harbor instance.
- **Artifact Management**: Fetch image tags for a specific project and repository.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
harbor-tools-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Usage

Here's a quick example of how to use the library:

```rust
use harbor_tools_rs::types::HarborClient;

#[tokio::main]
async fn main() {
    // Initialize the client
    // Replace these with your actual Harbor instance details
    let host = "harbor.example.com";
    let port = 443;
    let use_tls = true;
    let username = "admin";
    let password = "Harbor12345";

    let client = HarborClient::new(host, port, use_tls, username, password);
    let server = client.to_server();

    // Check health
    match server.health().await {
        Ok(status) => println!("Harbor Health Status: {}", status),
        Err(e) => eprintln!("Error checking health: {}", e),
    }

    // Get tags for a repository
    let project = "library";
    let repository = "ubuntu";
    match server.get_tags_by_project_and_repository(repository, project).await {
        Ok(tags) => {
            println!("Found {} tags:", tags.len());
            for tag in tags {
                println!("Tag: {}, Pushed: {}", tag.tag, tag.push_time);
            }
        }
        Err(e) => eprintln!("Error fetching tags: {}", e),
    }
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
