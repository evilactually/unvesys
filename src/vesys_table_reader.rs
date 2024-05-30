
use crate::vysisxml::XmlTableGroup;
use std::collections::HashMap;

type ColumnIndex = usize;

#[derive(Debug)]
struct VysysTableReader<'a> {
	tablegroup: &'a XmlTableGroup,
	column_map: HashMap<String, (ColumnIndex, String)>
}

impl<'a> VysysTableReader<'a> {
	pub fn new(tablegroup: &'a XmlTableGroup) -> VysysTableReader<'a> {
		let mut column_map = HashMap::new();
		for (index, columnstyle) in tablegroup.columnstyle.iter().enumerate() {
			if columnstyle.visibility == "true" {
				column_map.insert(columnstyle.columnname.clone(), (index, columnstyle.displayname.clone()));
			}
		}

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

	pub fn get_subtable_cell(&self, subtable_index: usize, column_name: &str, row_index: usize) -> &str {
		if let Some((column_index, _)) = self.column_map.get(column_name) {
			let subtable = &self.tablegroup.tablefamily.table[subtable_index];
			if let Some(tabledatacache) = subtable.tabledatacache {
				
			}
		}
		return "test";
	}

	pub fn get_row_iter() {

	}
}