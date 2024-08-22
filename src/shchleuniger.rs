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


pub struct SchleunigerASCIIConfig {
    left_position: f32,
    right_position: f32,
    min_double_label_length: f32 // length of wire before switching to single centered label
}

impl Default for SchleunigerASCIIConfig {
    fn default() -> Self {
        SchleunigerASCIIConfig { 
            left_position:3.25, 
            right_position: 2.0, 
            min_double_label_length: 8.0
        } 
    }
}

fn anyvalue_to_str(anyvalue: &AnyValue) -> std::string::String {
    match anyvalue {
        AnyValue::String(s) => s.to_string(),
        AnyValue::Float32(f) => f.to_string(),
        AnyValue::Float64(f) => f.to_string(),
        AnyValue::Int8(i) => i.to_string(),
        AnyValue::Int16(i) => i.to_string(),
        AnyValue::Int32(i) => i.to_string(),
        AnyValue::Int64(i) => i.to_string(),
        AnyValue::UInt8(i) => i.to_string(),
        AnyValue::UInt16(i) => i.to_string(),
        AnyValue::UInt32(i) => i.to_string(),
        AnyValue::Null => "N/A".to_string(),
        _ => anyvalue.to_string()
    }
}

fn center_label(config: &SchleunigerASCIIConfig, record: Vec<String>) -> Vec<String> {
    // Center label
    let length_f32 = f32::from_str(&record[2]);
    if let Ok(length_f32) = length_f32 {
        if length_f32 < config.min_double_label_length {
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

pub fn wirelist_to_schleuniger_ascii<W: Write>(config: &SchleunigerASCIIConfig, wire_list: &DataFrame, writer: W)  {
    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .flexible(true) // allow number of fields to change
        .terminator(Terminator::CRLF)
        .from_writer(writer); 

    wtr.write_record(vec![
        String::from("Import"), String::from("ASCII"),
    ]);

    wtr.write_record(vec![
        String::from("Units"), String::from("inch"),
    ]);

    wtr.write_record(vec![
        String::from("Area"), String::from("TT"), // Thermal Transfer
    ]);

    wtr.write_record(vec![
       String::from("Name"), 
        String::from("Part"), 
        String::from("Length"), 
        String::from("Style"), 
        String::from("Stripping type"),
        String::from("Right strip"),
        String::from("Left strip"),
        String::from("Partial strip %"),
        String::from("Marker left text"),
        String::from("Marker left position"),
        String::from("Marker right text"),
        String::from("Marker right position"),
        String::from("Autorotation"),
    ]);

    let mut wire_list = wire_list.clone(); 
    wire_list.as_single_chunk_par(); // need to run this before getting columns
    println!("{:?}", wire_list);
    let mut iters = wire_list.columns(["WIRE_NAME",
                                       "WIRE_FROM_PINLIST", 
                                       "WIRE_FROM_CAVITY", 
                                       "WIRE_TERMINAL_STRIP_LEN1", 
                                       "WIRE_TO_PINLIST", 
                                       "WIRE_TO_CAVITY", 
                                       "WIRE_TERMINAL_STRIP_LEN2", 
                                       "MODIFIED_LENGTH",
                                       "PROCESSING"
                                       ]).unwrap().iter().map(|s| s.iter()).collect::<Vec<_>>();

    for row in 0..wire_list.height() {
        let wire_name = anyvalue_to_str(&iters[0].next().unwrap_or_default());
        let wire_from_pinlist = anyvalue_to_str(&iters[1].next().unwrap_or_default());
        let wire_from_cavity = anyvalue_to_str(&iters[2].next().unwrap_or_default());
        let wire_terminal_strip_len1 = anyvalue_to_str(&iters[3].next().unwrap_or_default());
        let wire_to_pinlist = anyvalue_to_str(&iters[4].next().unwrap_or_default());
        let wire_to_cavity = anyvalue_to_str(&iters[5].next().unwrap_or_default());
        let wire_terminal_strip_len2 = anyvalue_to_str(&iters[6].next().unwrap_or_default());
        let modified_length = anyvalue_to_str(&iters[7].next().unwrap_or_default());
        let processing = anyvalue_to_str(&iters[8].next().unwrap_or_default());

        let from = format!("{}-{}", wire_from_pinlist, wire_from_cavity);
        let to = format!("{}-{}", wire_to_pinlist, wire_to_cavity);
        let article_name = format!("{}/{}",from, &to);
        let part = (row + 1).to_string(); // count rows and use it as "part" which is just a number
        let length = modified_length;

        let style = processing;
        let stripping_type = "9".to_owned();
        let right_strip = wire_terminal_strip_len1;
        let left_strip = wire_terminal_strip_len2;
        let partial_strip = "50%".to_owned();
        let marker_text = "\\#C@7\\&n\\&@7".to_owned();
        let marker_left_position = config.left_position;
        let marker_right_position = config.right_position;
        let autorotation = "X".to_owned();

        println!("{}", wire_name);

        wtr.write_record(center_label(config, vec![
            article_name, // 0
            part, // 1
            length, // 2
            style, // 3
            stripping_type, // 4
            right_strip, // 5
            left_strip, // 6
            partial_strip, // 7
            marker_text.clone(), // 8
            marker_left_position.to_string(), // 9
            marker_text, // 10
            marker_right_position.to_string(), // 11
            autorotation, // 12
        ]));
    
    }
}