/*
 Logic design commands

 Vladislav Shcherbakov
 Copyright Firefly Automatix 2024
 9/18/2024 3:34:10 PM
*/

use chrono::Local;
use polars::prelude::*;
use std::fs::File;
use crate::wirelist::wirelist_dataframe_to_label_dataframe;
use polars::datatypes::AnyValue;
use polars::frame::DataFrame;
use polars::frame::row::Row;
use crate::shchleuniger::wirelist_to_schleuniger_ascii;
use crate::shchleuniger::SchleunigerASCIIConfig;
use crate::wirelist::grouped_wirelist_to_data_frame;
use std::io::Write;
use crate::wire_list_xlsx_formatter::color_map;

use xlsxwriter::Workbook;

use crate::wirelist::generate_grouped_wirelist;

use crate::wire_list_xlsx_formatter::WireListXlsxFormatter;

use crate::wirelist::sort_wirelist_by_left_device_pin;

use crate::vysis::Project;

use crate::vysyslib::Library;

pub fn export_xslx_wirelist(project: &Project,library: &Library, design_name: &str, harness: &str, filepath: &str ) -> Result<(), Box<dyn std::error::Error>> {
    let colormap = color_map();
    if let Some(design) = project.get_design(design_name) {
        let connectivity = design.get_connectivity();

        if let Ok(workbook) = Workbook::new(filepath) {

            let wiregroups = generate_grouped_wirelist(library, &connectivity, harness).unwrap();
         
            let mut xlsx_formatter = WireListXlsxFormatter::new(&workbook, &colormap);

            // Output plain wire list
            xlsx_formatter.print_header();

            for mut group in wiregroups {
                // Sort wire group
                sort_wirelist_by_left_device_pin(&mut group);
                //println!("{}", "BEGIN GROUP");
                for wireentry in group {
                    //println!("  {}", wireentry.name);
                    xlsx_formatter.print_entry(&wireentry);
                }
                xlsx_formatter.bar();
                //println!("{}", "END GROUP")
            }

            // Print title
            let current_date = Local::now().format("%m/%d/%Y").to_string();
            xlsx_formatter.print_title(&format!("{}, {}, {}", design_name, harness, current_date));
        }
        else 
        {

        }
    // outout device index
    } else {
        // can't open path
        // return
    }

    Ok(())
}

pub fn logic_harness_shchleuniger_export<W:Write>(project: &Project, library: &Library, design_name: &str, harness: &str,  writer: W) -> Result<(), Box<dyn std::error::Error>> {
    
    if let Some(design) = project.get_design(design_name) {
        let connectivity = design.get_connectivity();

        //if let Ok(workbook) = Workbook::new(filepath) {

            let wiregroups = generate_grouped_wirelist(library, &connectivity, harness).unwrap();

            let df = grouped_wirelist_to_data_frame(wiregroups);
         
        //     // let mut xlsx_formatter = WireListXlsxFormatter::new(&workbook, &colormap);
        //     // // Output plain wire list
        //     // xlsx_formatter.print_header();

            wirelist_to_schleuniger_ascii(&SchleunigerASCIIConfig::default(), &df, writer);
        // }
        // else 
        // {

        // }
    // outout device index
    } else {
        // can't open path
        // return
    }

    Ok(())
}

pub fn logic_harness_labels_csv_export(project: &Project, library: &Library, design_name: &str, harness: &str, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Export DataFrame to CSV
    let mut file = File::create(filepath)?;
    logic_harness_labels_export(project, library, design_name, harness, file)
}


pub fn logic_harness_labels_export<W:Write>(project: &Project, library: &Library, design_name: &str, harness: &str,  mut writer: W) -> Result<(), Box<dyn std::error::Error>> {
    
    if let Some(design) = project.get_design(design_name) {
        let connectivity = design.get_connectivity();

        //if let Ok(workbook) = Workbook::new(filepath) {

            let wiregroups = generate_grouped_wirelist(library, &connectivity, harness).unwrap();

            let wire_list_df = grouped_wirelist_to_data_frame(wiregroups);
         
        //     // let mut xlsx_formatter = WireListXlsxFormatter::new(&workbook, &colormap);
        //     // // Output plain wire list
        //     // xlsx_formatter.print_header();

            let mut label_df = wirelist_dataframe_to_label_dataframe(&wire_list_df);
            CsvWriter::new(&mut writer)
            .include_header(true)
            .finish(&mut label_df)?;
            println!("{}", label_df);
            //println!("{}", wire_list_df);
        // }
        // else 
        // {

        // }
    // outout device index
    } else {
        // can't open path
        // return
    }

    Ok(())
}

fn group_by_sum(df: &DataFrame) -> PolarsResult<DataFrame> {
    df
    .group_by(["mpn", "partnum", "descr"])?
    .sum()
}

pub fn logic_harness_bom_export(project: &Project, library: &Library, design_name: &str, harness: &str) {
    if let Some(design) = project.get_design(design_name) {
        let connectivity = design.get_connectivity();
        let wires = connectivity.get_wires(harness);
        let components = connectivity.get_harness_components(harness);

        let mut bom_rows = Vec::new();
        let na = "N/A".to_owned();
        
        let fields = [
            Field::new("mpn", DataType::String), 
            Field::new("partnum", DataType::String), 
            Field::new("descr", DataType::String), 
            Field::new("quantity", DataType::Float32)];
        
        let schema: Schema = Schema::from_iter(fields);

        // Add wires
        for wire in wires {
            let customer_partnumber = wire.get_customer_partno();
            let partnumber = wire.dom.partnumber.as_ref().unwrap();
            let wire_part = library.lookup_wire_part(&partnumber);
            let description = wire_part.map(|w| &w.description).unwrap_or(&na);
            let row : Row = Row::new([AnyValue::String(&partnumber), AnyValue::String(&customer_partnumber), AnyValue::String(description), AnyValue::Float32(wire.dom.wirelength)].to_vec());
            bom_rows.push(row);
        }

        for c in components.iter() {
            library.lookup_component_partno_and_descr(&c);
        }

        // Add connectors


        let df = DataFrame::from_rows_and_schema(&bom_rows, &schema);
        let df = group_by_sum(&df.unwrap());
        println!("{:?}",&df);
    }
}
