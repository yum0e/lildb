use crate::{row::Row, table::ROWS_PER_PAGE};

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Page {
    pub nb_rows: usize,
    pub rows: Vec<Row>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            nb_rows: 0,
            rows: Vec::with_capacity(ROWS_PER_PAGE),
        }
    }
}
