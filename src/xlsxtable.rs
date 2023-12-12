use std::collections::HashMap;
use xlsxwriter::worksheet::{WorksheetRow, WorksheetCol};
use xlsxwriter::Format;
use xlsxwriter::format::FormatBorder;
use xlsxwriter::worksheet::Worksheet;

/// Generic table representation
pub struct XLSXTable {
    cells: HashMap<(WorksheetRow, WorksheetCol), XLSXTableCell>,
    default_format: Format,
    column_widths: HashMap<WorksheetCol, u32>
}

struct XLSXTableCell {
    value: Option<Box<str>>,
    format: Option<Format>
}

pub struct XLSXTableRegion {
    pub first_row: WorksheetRow, 
    pub first_col: WorksheetCol,
    pub last_row: WorksheetRow, 
    pub last_col: WorksheetCol,
}

// TODO: implement cell merge
impl XLSXTable {

    pub fn new() -> XLSXTable {
        XLSXTable {
            cells:  HashMap::new(),
            default_format: Format::new(),
            column_widths: HashMap::new()
        }
    }

    /// Set contents of table cell or add new cell with default format
    pub fn set_cell(&mut self, row: WorksheetRow, col: WorksheetCol, value: &str) {
        let k = (row, col);
        if let Some(cell) = self.cells.get_mut(&k) {
            cell.value = Some(value.to_owned().into_boxed_str());
        } else {
            self.cells.insert(k, XLSXTableCell {
                value: Some(value.to_owned().into_boxed_str()),
                format: Some(self.default_format.clone())
            });
        }
    }

    /// Default format for cells
    pub fn set_default_format(&mut self, format: Format) {
        self.default_format = format;
    }

    /// Insert cell if it does not exist
    fn insert_if_dne(&mut self, key:(WorksheetRow, WorksheetCol)) {
        if !self.cells.contains_key(&key) {
            self.cells.insert(key, XLSXTableCell {
                value: None,
                format: Some(self.default_format.clone())
            });
        }
    }

    /// Modify cell format. Creates blank cell if cell value was not defined and assigns default format if cell format is None
    /// before calling the closure.
    pub fn modify_cell_format(&mut self, row: WorksheetRow, col: WorksheetCol, f: &dyn Fn (&mut Format)  ) {
        let k = (row, col);
        self.insert_if_dne(k);
        if let Some(cell) = self.cells.get_mut(&k) {
            if cell.format.is_none() {
                cell.format = Some(self.default_format.clone());
            }
            f(&mut cell.format.as_mut().unwrap()); // guaranteed to have format set at this point
        }
    }

    pub fn modify_region_format(&mut self, region: &XLSXTableRegion, f: &dyn Fn (&mut Format)  ) {
        for row in region.first_row ..=region.last_row {
            for col in region.first_col ..=region.last_col {
                //println!("({}, {})", row, col);
                self.modify_cell_format(row, col, f);
            }
        }
    }

    pub fn set_region_border(&mut self, region: &XLSXTableRegion, border: FormatBorder) {
        self.set_region_border_top(region, border);
        self.set_region_border_right(region, border);
        self.set_region_border_bottom(region, border);
        self.set_region_border_left(region, border);
    }

    pub fn set_region_border_top(&mut self, region: &XLSXTableRegion, border: FormatBorder) {
        for col in region.first_col ..=region.last_col {
            self.set_cell_border_top(region.first_row, col, border);
        }

    }

    pub fn set_region_border_bottom(&mut self, region: &XLSXTableRegion,border: FormatBorder) {
        for col in region.first_col ..=region.last_col {
            self.set_cell_border_bottom(region.last_row, col, border);
        }
    }

    pub fn set_region_border_left(&mut self, region: &XLSXTableRegion,border: FormatBorder) {
        for row in region.first_row ..=region.last_row {
            self.set_cell_border_left(row, region.first_col, border);
        }
    }

    pub fn set_region_border_right(&mut self, region: &XLSXTableRegion,border: FormatBorder) {
        for row in region.first_row ..=region.last_row {
            self.set_cell_border_right(row, region.last_col, border);
        }
    }

    pub fn set_cell_border_top(&mut self,row: WorksheetRow, col: WorksheetCol, border: FormatBorder) {
        self.modify_cell_format(row, col, &|mut format| {
            format.set_border_top(border);
        });
    }

    pub fn set_cell_border_bottom(&mut self,row: WorksheetRow, col: WorksheetCol, border: FormatBorder) {
        self.modify_cell_format(row, col, &|mut format| {
            format.set_border_bottom(border);
        });
    }

    pub fn set_cell_border_left(&mut self,row: WorksheetRow, col: WorksheetCol, border: FormatBorder) {
        self.modify_cell_format(row, col, &|mut format| {
            format.set_border_left(border);
        });
    }

    pub fn set_cell_border_right(&mut self, row: WorksheetRow, col: WorksheetCol, border: FormatBorder) {
        self.modify_cell_format(row, col, &|mut format| {
            format.set_border_right(border);
        });
    }

    pub fn render_to_worksheet(&self, sheet: &mut Worksheet) {
        for ((row,col), cell) in self.cells.iter() {
            match &cell.value {
                Some(value) => {
                    sheet.write_string(*row, *col, value, cell.format.as_ref());
                }
                None => {
                    sheet.write_blank(*row, *col, cell.format.as_ref());
                }
            }
        }
        // Update column width
        for (col, width) in self.column_widths.iter() {
            sheet.set_column_pixels(*col, *col, *width, None);
        }
    }

    pub fn set_col_width_pixels(&mut self, col: WorksheetCol, width: u32) {
        self.column_widths.insert(col, width);
    }
}
