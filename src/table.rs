use crate::row::ROW_SIZE;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    nb_rows: usize,
    pages: [Option<Box<[usize; ROWS_PER_PAGE]>>; TABLE_MAX_PAGES],
}

impl Table {
    // get a mutable reference to the row slot in the table
    pub fn row_slot(&mut self, row_num: usize) -> &mut [usize] {
        let page_num = row_num / ROWS_PER_PAGE;
        let page = &mut self.pages[page_num];

        // allocate memory for the page if it is not already allocated
        // this allocation is an array of pointers to the rows
        if page.is_none() {
            *page = Some(Box::new([0; ROWS_PER_PAGE]));
        }

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        match page.as_mut() {
            Some(page) => &mut page[byte_offset..byte_offset + ROW_SIZE],
            None => unreachable!("Unreachable: page is not allocated."),
        }
    }
}
