use base64::{Engine as _, engine::general_purpose};

pub fn base64_encode(s: &str) -> String {
    general_purpose::STANDARD.encode(s)
}