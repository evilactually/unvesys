use csv::Writer;
use crate::vysisxml::XmlTableGroup;
use std::path::PathBuf;

pub fn dump_tables(table_groups: &Vec<XmlTableGroup>, basename: &str, dir: &str) -> std::io::Result<()> {
	let mut i = 0;
	let mut path : PathBuf = dir.into();
	for group in table_groups.iter() {
		println!("{:?}", group.title);
		for table in group.tablefamily.table.iter() {
			if let Some(datacache) = &table.tabledatacache {
				//println!("{:?}", datacache.colhdrnames);
				let mut path = path.clone();
				let filename = format!("basename-{}-{}.csv", group.title, i);
				path.push(filename.clone());
				println!("{:?}", path);
				i = i + 1;
				let mut wtr = Writer::from_path(path)?;
				let header = &datacache.colhdrnames.row;
				let header_names : Vec<String> = header.cellvals.iter().map(|v| {
					v.cval.val.clone()
				}).collect();
				println!("{:?}", header_names);
				wtr.write_record(&header_names)?;

				for datarow in datacache.datavalues.datarow.iter() {
					let cols : Vec<String> = datarow.cellval.iter().map(|v| {
						v.cval.val.clone()
					}).collect();

					wtr.write_record(&cols)?;
				}

			}
		}
	}

	Ok(())
}