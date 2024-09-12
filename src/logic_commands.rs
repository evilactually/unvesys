
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

pub fn logic_harness_bom_export(project: &Project, library: &Library, design_name: &str, harness: &str) {
    if let Some(design) = project.get_design(design_name) {
        let connectivity = design.get_connectivity();
        let wires = connectivity.get_wires(harness);

        let mut bom_rows = Vec::new();
        
        // Add wires
        for wire in wires {
            if let Some(partnumber) = &wire.dom.partnumber {
                let row : Row = Row::new([AnyValue::String(partnumber), AnyValue::Float32(wire.dom.wirelength)].to_vec());
                bom_rows.push(row);
            }
            //unimplemented!();
        }

        let df = DataFrame::from_rows(&bom_rows);
        println!("{:?}",&df);
    }
}
