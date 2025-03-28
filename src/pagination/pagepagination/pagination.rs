pub struct PagePaginator {
    pub max_items: i32,
    pub default_items: i32,
}

impl Default for PagePaginator {
    fn default() -> Self {
        Self {
            max_items: 1000,
            default_items: 250,
        }
    }
}
