use super::limit::Limit;

pub trait ParsePagination: Send + Sync + std::fmt::Debug {
    fn limit(&self) -> Limit;
    fn as_query_params(&self) -> String;
}
