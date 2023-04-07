use prettytable::{Row, Table};

pub struct PrintableTable(pub Table);

impl PrintableTable {
    pub fn new() -> Self {
        Self(Table::new())
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        if row.is_empty() {
            return;
        }

        let cells = row
            .into_iter()
            .map(|cell| prettytable::Cell::new(&cell))
            .collect::<Vec<_>>();

        // add the row to the table
        self.0.add_row(cells.into());
    }

    pub fn add_row_cell(&mut self, row: Row) {
        if row.is_empty() {
            return;
        }

        // add the row to the table
        self.0.add_row(row.into());
    }

    pub fn print(&self) {
        self.0.printstd();
    }
}
