use std::collections::HashMap;

use http::{HeaderMap, HeaderName, HeaderValue};

pub struct PaginationHeaders {
    values: HashMap<String, String>,
}

impl Default for PaginationHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl PaginationHeaders {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn with_total(mut self, total: usize) -> Self {
        self.values
            .insert("X-Total-Count".to_string(), total.to_string());
        self
    }

    pub fn with_next_link(mut self, next_link: Option<String>) -> Self {
        if let Some(link) = next_link {
            self.values.insert("X-Next-Page".to_string(), link);
        }
        self
    }
    pub fn with_prev_link(mut self, prev_link: Option<String>) -> Self {
        if let Some(link) = prev_link {
            self.values.insert("X-Prev-Page".to_string(), link);
        }
        self
    }

    pub fn add_to_headers(self, headers: &mut HeaderMap) {
        for (key, value) in self.values {
            let header_name = key.parse::<HeaderName>().expect("valid header name");
            let header_value = value.parse::<HeaderValue>().expect("valid header value");
            headers.insert(header_name, header_value);
        }
    }
}
