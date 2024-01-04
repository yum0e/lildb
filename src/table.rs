use crate::{
    page::{Page, PAGE_SIZE},
    row::{Row, ROW_SIZE},
    Statement, StatementCommandResult,
};

pub const TABLE_MAX_PAGES: usize = 100;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

#[derive(Debug)]
pub struct Table {
    pub pages: Vec<Page>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            pages: Vec::with_capacity(TABLE_MAX_PAGES),
        }
    }

    // Insert a row into the table
    fn insert_row(&mut self, row_data: [u8; ROW_SIZE]) {
        if let Some(page) = self.pages.last_mut() {
            // Check if the current page has space for more rows
            if page.rows.len() < ROWS_PER_PAGE {
                page.rows.push(Row { data: row_data });
            } else {
                // If the current page is full, create a new page
                let mut new_page = Page::new();
                new_page.rows.push(Row { data: row_data });
                self.pages.push(new_page);
            }
        } else {
            // If the table is empty, create the first page and insert the row
            let mut new_page = Page::new();
            new_page.rows.push(Row { data: row_data });
            self.pages.push(new_page);
        }
    }

    pub fn execute(&mut self, statement: Statement) -> StatementCommandResult {
        match statement {
            Statement::Insert(row) => {
                if self.pages.len() >= TABLE_MAX_PAGES {
                    return StatementCommandResult::SyntaxError("Error: Table full.".to_string());
                }

                self.insert_row(row.data);
                StatementCommandResult::Success(Statement::Insert(row))
            }
            Statement::Select => {
                Row::print_header();
                for page in &self.pages {
                    for row in &page.rows {
                        row.print();
                    }
                }
                StatementCommandResult::Success(Statement::Select)
            }
        }
    }
}
