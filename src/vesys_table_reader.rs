
use crate::vysisxml::XmlTableGroup;
use std::collections::HashMap;

type ColumnIndex = usize;

/// Convience parser of Vesys tables.
#[derive(Debug)]
pub struct VysysTableReader<'a> {
	tablegroup: &'a XmlTableGroup,
	column_map: HashMap<String, (ColumnIndex, String)>
}

impl<'a> VysysTableReader<'a> {
	pub fn new(tablegroup: &'a XmlTableGroup) -> VysysTableReader<'a> {
		let mut column_map = HashMap::new();
		let mut index = 0;
		for (_, columnstyle) in tablegroup.columnstyle.iter().enumerate() {
			if columnstyle.visibility == "true" {
				column_map.insert(columnstyle.columnname.clone(), (index, columnstyle.displayname.clone()));
				index = index + 1;
			}
		}

		//println!("{:?}", index, );
		VysysTableReader {
			tablegroup: tablegroup,
			column_map : column_map
		}
	}

	pub fn get_subtable_count(&self) -> usize {
		self.tablegroup.tablefamily.table.len()
	}

	pub fn get_subtable_row_count(&self, subtable_index: usize) -> usize {
		if let Some(tabledatacache) = &self.tablegroup.tablefamily.table[subtable_index].tabledatacache {
			tabledatacache.datavalues.datarow.len()
		} else {
			0
		}
	}

	pub fn get_subtable_cell(&self, subtable_index: usize, column_name: &str, row_index: usize) -> Result<&str, String> {
		if let Some((column_index, _)) = self.column_map.get(column_name) {
			let subtable = &self.tablegroup.tablefamily.table[subtable_index];
			if let Some(tabledatacache) = &subtable.tabledatacache {
				if row_index < tabledatacache.datavalues.datarow.len() {
					if *column_index < tabledatacache.datavalues.datarow[row_index].cellval.len() {
						Ok(&tabledatacache.datavalues.datarow[row_index].cellval[*column_index].cval.val)
					} else {
						println!("{:?}", "Column index out of bounds! Update your design.");
						Err("Column index out of bounds!".to_owned())
					}
				} else {
					println!("{:?}", "Row index out of bounds! Update your design.");
					Err("Row index out of bounds!".to_owned())
				} 
			} else {
				Err("Table data cache missing!".to_owned())
			}
		} else {
			Err("Invalid column name!".to_owned())
		}
	}

	pub fn get_row_iter(&self) -> VysysTableRowIter {
		VysysTableRowIter {
			table_reader: &self,
			subtable_index: 0,
			row_index: 0
		}
	}
}

pub struct VysysTableRow<'a> {
	table_reader: &'a VysysTableReader<'a>,
	subtable_index: usize,
	row_index : usize
}

impl<'a> VysysTableRow<'a> {
	pub fn get_column(&self, column_name: &str) -> Result<&str, String> { 
		self.table_reader.get_subtable_cell(self.subtable_index, column_name, self.row_index)
	}
}

pub struct VysysTableRowIter<'a> {
    table_reader:&'a VysysTableReader<'a>,
    subtable_index: usize,
	row_index : usize
}

impl<'a> Iterator for VysysTableRowIter<'a> {
    type Item = VysysTableRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
    	let row_count = self.table_reader.get_subtable_row_count(self.subtable_index);
    	
    	self.row_index = self.row_index + 1;
    	if self.row_index >= row_count {
    		self.row_index = 0;
    		self.subtable_index = self.subtable_index + 1;
    	}

    	// No more subtables
    	if self.subtable_index >= self.table_reader.get_subtable_count() {
    		return None;
    	}

    	Some(VysysTableRow {
    			table_reader : self.table_reader,
    			subtable_index : self.subtable_index,
    			row_index : self.row_index
    		}
    	)
    }
}