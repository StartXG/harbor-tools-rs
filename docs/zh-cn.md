# Harbor Tools RS

一个用于与 [Harbor 容器镜像仓库](https://goharbor.io/) API 交互的 Rust 库。

本项目主要用于将Harbor的API 抽象出更简化的字段，方便用于DevOps平台中。

## 功能特性

- **健康检查**：检测 Harbor 实例的运行状态。
- **制品管理**：获取指定项目和仓库的镜像标签列表。

## 安装

在 `Cargo.toml` 中加入：

```toml
[dependencies]
harbor-tools-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 使用示例

下面是一个使用该库的快速示例：

```rust
use harbor_tools_rs::types::HarborClient;

#[tokio::main]
async fn main() {
	// 初始化客户端
	// 将这些值替换为你的 Harbor 实例信息
	let host = "harbor.example.com";
	let port = 443;
	let use_tls = true;
	let username = "admin";
	let password = "";

	let client = HarborClient::new(host, port, use_tls, username, password);
	let server = client.to_server();

	// 健康检查
	match server.health().await {
		Ok(status) => println!("Harbor 健康状态: {}", status),
		Err(e) => eprintln!("健康检查失败: {}", e),
	}

	// 获取指定仓库的标签
	let project = "library";
	let repository = "ubuntu";
	match server.get_tags_by_project_and_repository(repository, project).await {
		Ok(tags) => {
			println!("找到 {} 个标签:", tags.len());
			for tag in tags {
				println!("标签: {}, 推送时间: {}", tag.tag, tag.push_time);
			}
		}
		Err(e) => eprintln!("获取标签失败: {}", e),
	}
}
```

## 调试

```shell
touch tests/test.toml
```

`tests/test.toml` 的示例配置：

```toml
[HarborServer]
host = ""
port = 443
use_ssl = true
username = ""
password = ""

[HarborProject]
project = ""
repository = ""
```

```shell
cargo test --tests -- --nocapture
```

## 贡献

欢迎提交 Pull Request。若涉及重大变更，请先创建 Issue 与我们讨论。

## 许可协议

[MIT](https://choosealicense.com/licenses/mit/)
