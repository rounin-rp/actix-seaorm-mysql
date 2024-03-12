use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PaginatedModel<M> {
    data: M,
    total_pages: u64,
    current_page: u64,
    page_size: u64,
    has_next_page: bool,
}

impl<M: Clone + PartialEq + Eq + Serialize> PaginatedModel<M> {
    pub fn new(data: M, total_pages: u64, current_page: u64, page_size: u64) -> Self {
        let has_next_page = current_page < total_pages;
        Self {
            data,
            total_pages,
            current_page,
            page_size,
            has_next_page,
        }
    }
}
