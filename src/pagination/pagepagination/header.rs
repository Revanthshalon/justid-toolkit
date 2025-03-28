use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RequestParameters {
    pub per_page: i32,
    pub page: i32,
    pub page_size: i32,
    pub page_token: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseHeaderAnnotation {
    pub link: String,
    #[serde(rename = "x-total-count")]
    pub total_count: i32,
}
