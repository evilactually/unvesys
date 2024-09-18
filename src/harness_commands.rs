/*
 Harness design commands

 Vladislav Shcherbakov
 Copyright Firefly Automatix 2024
 9/18/2024 3:34:10 PM
*/

use std::fs::File;
use crate::wirelist::wirelist_dataframe_to_label_dataframe;
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
use polars::lazy::dsl::col;
use polars::df;
use crate::shchleuniger::*;

/// Dump all harness tables into CSV
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

/// Get SHCHLEUNIGER wire processing property of the wire from the library
fn lookup_wire_processing<'a>(library: &'a Library, harness_design: &'a HarnessDesign<'a>, wire_name: &'a str) -> Option<&'a str> {
    harness_design.get_connectivity().get_wire_by_name(wire_name).and_then(|wire| {
        wire.dom.partnumber.as_ref().and_then(|part_number| {
            library.lookup_wire_property(&part_number, "PROCESSING")
        })
    })
}

/// Check if wire is in multicore
fn is_in_multicore(harness_design: &HarnessDesign, wire_name: &Series) -> bool {
    let wire_name = wire_name.str().unwrap().get(0).unwrap();
    harness_design.get_connectivity().get_wire_by_name(wire_name).map(|wire| {
        wire.is_in_multicore()
    }).unwrap_or(false)
}

pub fn harness_labels_csv_export(library: &Library, harness_design: &HarnessDesign, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Export DataFrame to CSV
    let mut file = File::create(filepath)?;
    harness_labels_export(library, harness_design, file)
}

/// Export harness design HarnessWireTable into CSV label file
pub fn harness_labels_export<W: Write>(library: &Library, harness_design: &HarnessDesign, mut writer: W) -> std::result::Result<(), Box<dyn std::error::Error>>  {
    let table_groups = harness_design.get_table_groups();

    let harness_wire_table = table_groups.into_iter().find(|x| x.decorationname == "HarnessWireTable");

    if let Some(harness_wire_table) = harness_wire_table { // if harness wire table is present

        println!("{}", &harness_wire_table.title);
        let table_reader = VysysTableReader::new(&harness_wire_table);

        let mut wirelist_df : DataFrame = table_reader.into();
        wirelist_df.as_single_chunk_par(); // need to run this before getting columns

        let mut label_df = wirelist_dataframe_to_label_dataframe(&wirelist_df);
        CsvWriter::new(&mut writer)
        .include_header(true)
        .finish(&mut label_df)?;
        println!("{}", label_df);

    } else {
        return Err("HarnessWireTable not found".into())
    }
    Ok(())
}



/// Export harness design HarnessWireTable into SHCHLEUNIGER ASCII file for the wire cutting machine
pub fn harness_schleuniger_ascii_export<W: Write>(library: &Library, harness_design: &HarnessDesign, writer: W) -> std::result::Result<(), String>  {

    let table_groups = harness_design.get_table_groups();

    let harness_wire_table = table_groups.into_iter().find(|x| x.decorationname == "HarnessWireTable");

    if let Some(harness_wire_table) = harness_wire_table { // if harness wire table is present

        println!("{}", &harness_wire_table.title);
        let table_reader = VysysTableReader::new(&harness_wire_table);

        let mut wirelist_df : DataFrame = table_reader.into();
        wirelist_df.as_single_chunk_par(); // need to run this before getting columns
        
        // Add generated PROCESSING column to the DataField
        let processing = wirelist_df.column("WIRE_NAME")
        .unwrap() // may not have the column
        .str() // assume string type
        .unwrap() // may not be a string type
        .into_iter() // iterate
        .map(|wire_name| { // replace wire name with its processing value
            wire_name.map(|wire_name| {
                lookup_wire_processing(library, harness_design, wire_name).unwrap_or("N/A")
            })
        }).collect::<Vec<_>>(); // place in vector
        let processing_col = Series::new("PROCESSING", &processing); // make a Series from Vec

        let wirelist_df = wirelist_df.hstack(&[processing_col]).unwrap();

        //wirelist_df.lazy().filter(col("WIRE_NAME").apply(|w| Ok(Some(Series(&[false]))), GetOutput::from_type(DataType::Boolean) ));
        // let filtered_df: DataFrame = wirelist_df.clone()
        // .lazy()
        // .filter(col("WIRE_NAME").lt(2))
        // .collect().unwrap();


        wirelist_to_schleuniger_ascii(&SchleunigerASCIIConfig::default(), &wirelist_df, writer);

        return Ok(());
    } else {
        return Err("No wire table!".to_string());
    }
}