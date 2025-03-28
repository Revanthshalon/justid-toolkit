use serde::Serialize;

pub struct PaginatedItems<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub total: Option<usize>,
}

impl<T> PaginatedItems<T> {
    pub fn new(items: Vec<T>, has_more: bool, total: Option<usize>) -> Self {
        Self {
            items,
            has_more,
            total,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Serialize> PaginatedItems<T> {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.items)
    }
}
