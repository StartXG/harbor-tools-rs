use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HarborClient {
    pub(crate) host: String,
    pub(crate) port: i32,
    pub(crate) use_tls: bool,
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize)]
pub struct HarborServer {
    pub address: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct HarborHealth {
    pub components: Vec<HarborHealthComponent>,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct HarborHealthComponent {
    pub name: String,
    pub status: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ImageTags {
    pub tag: String,
    pub push_time: String,
}
