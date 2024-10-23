pub mod aggregates;

#[derive(Clone, Debug)]
pub struct Pagination {
    pub page: u64,
    pub items_per_page: u64,
}
