use std::str::FromStr;
use crate::vysyslib::Library;
use std::io::Write;
use csv::Terminator;
use crate::vesys_table_reader::VysysTableReader;
use crate::vysis::HarnessDesign;
use csv::{Writer, WriterBuilder};
use crate::vysisxml::XmlTableGroup;
use std::path::PathBuf;
use std::error::Error;
use polars::prelude::*;
use crate::shchleuniger::*;

pub fn dump_tables(table_groups: &Vec<XmlTableGroup>, basename: &str, dir: &str) -> std::io::Result<()> {
    let mut i = 0;
    let mut path : PathBuf = dir.into();
    for group in table_groups.iter() {
        println!("{:?}", group.title);
        for table in group.tablefamily.table.iter() {
            if let Some(datacache) = &table.tabledatacache {
                //println!("{:?}", datacache.colhdrnames);
                let mut path = path.clone();
                let filename = format!("{}-{}-{}.csv", basename, group.title, i);
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

fn lookup_wire_processing<'a>(library: &'a Library, harness_design: &'a HarnessDesign<'a>, wire_name: &'a str) -> Option<&'a str> {
    harness_design.get_connectivity().get_wire_by_name(wire_name).and_then(|wire| {
        wire.dom.partnumber.as_ref().and_then(|part_number| {
            library.lookup_wire_property(&part_number, "PROCESSING")
        })
    })
}

pub fn center_label(record: Vec<String>) -> Vec<String> {
    // Center label
    let length_f32 = f32::from_str(&record[2]);
    if let Ok(length_f32) = length_f32 {
        if length_f32 < 8.0 {
            let pos = length_f32/2.0 + 0.5;
            let mut record = record.clone();
            record[9] = pos.to_string();
            record[10] = "".to_string();
            record[11] = "".to_string();
            return record;
        } else {
            return record;
        }
    } else {
        return record;
    }
}



pub fn harness_schleuniger_ascii_export<W: Write>(library: &Library, harness_design: &HarnessDesign, writer: W)  {
    // let mut wtr = WriterBuilder::new()
    //     .delimiter(b'\t')
    //     .flexible(true) // allow number of fields to change
    //     .terminator(Terminator::CRLF)
    //     .from_writer(writer); 

    // wtr.write_record(vec![
    //     String::from("Import"), String::from("ASCII"),
    // ]);

    // wtr.write_record(vec![
    //     String::from("Units"), String::from("inch"),
    // ]);

    // wtr.write_record(vec![
    //     String::from("Area"), String::from("TT"), // Thermal Transfer
    // ]);

    // wtr.write_record(vec![
    //    String::from("Name"), 
    //     String::from("Part"), 
    //     String::from("Length"), 
    //     String::from("Style"), 
    //     String::from("Stripping type"),
    //     String::from("Right strip"),
    //     String::from("Left strip"),
    //     String::from("Partial strip %"),
    //     String::from("Marker left text"),
    //     String::from("Marker left position"),
    //     String::from("Marker right text"),
    //     String::from("Marker right position"),
    //     String::from("Autorotation"),
    // ]);

    let table_groups = harness_design.get_table_groups();

    let harness_wire_table = table_groups.into_iter().find(|x| x.decorationname == "HarnessWireTable");

    if let Some(harness_wire_table) = harness_wire_table {

        println!("{}", &harness_wire_table.title);
        let table_reader = VysysTableReader::new(&harness_wire_table);

        let mut wirelist_df : DataFrame = table_reader.clone().into();
        //println!("{}", wirelist_df);
        wirelist_df.as_single_chunk_par();

        wirelist_to_schleuniger_ascii(&SchleunigerASCIIConfig::default(), &wirelist_df, writer);

        


        // let row_iter = table_reader.get_row_iter();
        // for (index, row) in row_iter.enumerate() {
        //     let from = row.get_column("WIRE_FROM_PINLIST").unwrap_or("N/A").to_owned() + "-" + row.get_column("WIRE_FROM_CAVITY").unwrap_or("N/A");
        //     let to = row.get_column("WIRE_TO_PINLIST").unwrap_or("N/A").to_owned() + "-" + row.get_column("WIRE_TO_CAVITY").unwrap_or("N/A");
        //     let article_name = format!("{}/{}", from, &to);
        //     let part = (index + 1).to_string();
        //     let length = row.get_column("MODIFIED_LENGTH").unwrap_or("N/A").to_owned();
            
        //     let style = row.get_column("WIRE_NAME").and_then(|wire_name| {
        //         lookup_wire_processing(library, harness_design, wire_name).ok_or("Property not found".to_string())
        //     });
        //     let stripping_type = "9".to_owned();
        //     let right_strip = row.get_column("WIRE_TERMINAL_STRIP_LEN1").unwrap_or("N/A").to_owned();
        //     let left_strip = row.get_column("WIRE_TERMINAL_STRIP_LEN2").unwrap_or("N/A").to_owned();
        //     let partial_strip = "50%".to_owned();
        //     let marker_text = "\\#C@7\\&n\\&@7".to_owned();
        //     let marker_left_position = "3.25".to_owned();
        //     let marker_right_position = "2.0".to_owned();
        //     let autorotation = "X".to_owned();

        //     wtr.write_record(center_label(vec![
        //         article_name, // 0
        //         part, // 1
        //         length, // 2
        //         style.unwrap_or("N/A").to_string(), // 3
        //         stripping_type, // 4
        //         right_strip, // 5
        //         left_strip, // 6
        //         partial_strip, // 7
        //         marker_text.clone(), // 8
        //         marker_left_position, // 9
        //         marker_text, // 10
        //         marker_right_position, // 11
        //         autorotation, // 12
        //     ]));
        // }
    }
}