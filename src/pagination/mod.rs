use error::PaginationResult;
use parse::ParsePagination;

pub mod error;
pub mod header;
pub mod items;
pub mod limit;
pub mod pagepagination;
pub mod parse;

pub trait PaginationStrategy: Send + Sync + 'static {
    fn parse_request(&self, req: &http::Request<()>) -> PaginationResult<Box<dyn ParsePagination>>;
    fn add_headers(&self, headers: &mut http::HeaderMap, pagination: &dyn ParsePagination);
}
