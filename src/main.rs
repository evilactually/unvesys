
#[macro_use]
extern crate clap;

extern crate xlsxwriter;
use xlsxwriter::*;
use xlsxwriter::format::*;


use std::fs::File;
use std::io::prelude::*;

use clap::Parser;
use clap::{Arg, ArgMatches};

use std::option::Option;

use std::env::args; //command line arguments
mod vysisxml;
use crate::vysisxml::*;

mod vysis;
use crate::vysis::*;

use colored::*;

/// VeSys XML project post-processor 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// XmlProject file name
    project: String,
    /// Name of logical design to export
    #[arg(short, long)]
    design: Option<String>,
    /// Name of harness design to export. 
    /// If used together with --design argument, 
    /// exports logical design filtered by harness attribute
    #[arg(long, short = 'a')]
    harness: Option<String>,
    /// Name of label output file
    #[arg(short, long)]
    labels: Option<String>,
    /// Name of BOM output file
    #[arg(short, long)]
    bom: Option<String>,
    /// Name of wire cut list output file
    #[arg(short, long)]
    cutlist: Option<String>,
    /// Component index file name
    #[arg(short, long)]
    index: Option<String>,
}

// fn show_project_info(dom: &XmlProject) {
//     println!("{} {}", "XmlProject Name:".bright_yellow(), dom.name.yellow());
//     println!("{}", "Logical Designs:".bright_yellow());
//     for logicaldesign in &dom.designmgr.logicaldesign {
//         print!("    {} {}", "*".bright_yellow(), logicaldesign.name.yellow());
//         if let Some(cow_str) = &logicaldesign.description {
//             print!("{} {}", ":".bright_yellow(), cow_str.yellow());
//         }
//         print!("\n");
//     }
//     println!("{}", "Harness Designs:".bright_yellow());
//     for harnessdesign in &dom.designmgr.harnessdesign {
//         print!("    {} {}", "*".bright_yellow(), harnessdesign.name.yellow());
//         if let Some(cow_str) = &harnessdesign.description {
//             print!("{} {}", ":".bright_yellow(), cow_str.yellow());
//         }
//         print!("\n");
//     }
//     println!("{}", "OK".bright_green());
// }

fn show_project_info(project: &Project) {
    println!("{} {}", "XmlProject Name:".bright_yellow(), project.get_name().yellow());
    println!("{}", "Logical Designs:".bright_yellow());
    // List logical design names
    {
        let logical_designs = project.get_logical_design_names();
        for design in logical_designs {
            println!("    {} {}", "*".bright_yellow(), design.yellow());
        }
    }
    println!("{}", "Harness Designs:".bright_yellow());
    // List harness design names
    {
        let harness_designs = project.get_harness_design_names();
        for design in harness_designs {
            println!("    {} {}", "*".bright_yellow(), design.yellow());
        }
    }
    println!("{}", "OK".bright_green());
}

fn print_field_opt(fieldname:&str, field_opt: &Option<Cow<str>>) {
    field_opt.as_ref().map(|field|
        if field.len() > 0 {
            println!("\t\t{}{}{}", fieldname.bright_yellow(), ": ".bright_yellow(), field.yellow())
        });
}

fn print_field(fieldname:&str, field: Cow<str>) {
    if field.len() > 0 {
        println!("\t\t{}{}{}", fieldname.bright_yellow(), ": ".bright_yellow(), field.yellow());
    }
}

fn lookup_pinref<'a>(design_dom: &XmlLogicalDesign<'a>, pinref_uuid: Cow<str>) -> (XmlConnector<'a>, XmlPin<'a>){
    unimplemented!()
}

fn show_design_info(dom: &XmlProject, design_name: &str) {
    println!("{} {}", "XmlProject Name:".bright_yellow(), dom.name.yellow());
    let index = dom.designmgr.logicaldesign.iter().position(|design| design.name == design_name);
    match index {
        Some(index) => {
            let design_dom = &dom.designmgr.logicaldesign[index];
            println!("{} {}", "Logical Design Name:".bright_yellow(), design_dom.name.yellow());
            println!("{}","Devices:".bright_yellow());
            for device in &design_dom.connectivity.device {
                println!("\t{}:", device.name.yellow());
                print_field_opt("MPN", &device.partnumber);
                print_field_opt("Part Number", &device.customerpartnumber);
                print_field_opt("Part Description", &device.partdesc);
                print_field_opt("Short Description", &device.shortdescription);
            }
            println!("{}","Wires:".bright_yellow());
            for wire in &design_dom.connectivity.wire {

            }
            println!("{}", "OK".bright_green());
        }
        None => {
            println!("{}{}", "Error: ".red(), "Logical design with that name was not found.".to_string().bright_red())
        }
    }
}

fn read_file(filename:&str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}



// WIRE LIST POST-PROCESSING
// 1. Consolidate wire ends
// 2. Change rings from connectors to terminations and find connected device
// 3. Number groups with shared terminations(If termination is shared => Device is shared && Pin is shared ), (Pin shared <=> Termination shared)

struct WireList {

}

struct WireListEntry<'a> {
    name: &'a str,
    left_connection: &'a Connection<'a>,
    right_connection: &'a Connection<'a>,
    
}

fn main() {
    colored::control::set_virtual_terminal(true).expect("Failed to set terminal");
    let args = Args::parse();

    let msg = format!("Reading {}...", args.project );
    println!("{}", msg.bright_yellow());

    // match read_file(&args.project) {
    //     Ok(xml) => {
    //         let dom = XmlProject::from_str(&xml).unwrap();

    //         let no_outputs_specified = args.labels.is_none()
    //                                    && args.bom.is_none()
    //                                    && args.cutlist.is_none()
    //                                    && args.index.is_none();

    //         // No design specified, show project info
    //         if args.design.is_none() && args.harness.is_none() {
    //             println!("{}", "Showing project info...".bright_yellow());
    //             show_project_info(&dom);
    //         // Logical design specified, but no outputs. Show design info 
    //         } else if args.design.is_some() && no_outputs_specified {
    //             println!("{}", "Showing logical design info...".bright_yellow());
    //             show_design_info(&dom, &args.design.unwrap());
    //         }
    //     },
    //     Err(e) => println!("{}{}", "Error: ".red(), e.to_string().bright_red()),
    // }

    match read_file(&args.project) {
        Ok(xml) => {
            let project = Project::new(&xml);
            match project {
                // Project parsed successfuly 
                Ok(project) => {
                    let no_outputs_specified = args.labels.is_none()
                                               && args.bom.is_none()
                                               && args.cutlist.is_none()
                                               && args.index.is_none();
                    // No design specified, show project info
                    if args.design.is_none() && args.harness.is_none() {
                        println!("{}", "Showing project info...".bright_yellow());
                        show_project_info(&project);
                    }
                    if args.design.is_some() 
                       && args.harness.is_some() 
                       && args.cutlist.is_some() {
                        let workbook = Workbook::new(args.cutlist.unwrap().as_ref());
                        match workbook {
                            Ok(workbook) => {
                                println!("{}", "Generating cut list...".bright_yellow());
                                // connector pin wire color length connector pin
                                let sheet = workbook.add_worksheet(args.harness.as_deref());
                                let mut format_header = Format::new();
                                format_header.set_bold();
                                let mut format_5v = Format::new();
                                format_5v.set_bg_color(FormatColor::Custom(0xff7df4));
                                let mut format_12v = Format::new();
                                format_12v.set_bg_color(FormatColor::Custom(0xf74343));
                                let mut format_24v = Format::new();
                                format_24v.set_bg_color(FormatColor::Custom(0xb07541));
                                let mut format_48v = Format::new();
                                format_48v.set_bg_color(FormatColor::Custom(0xeb750e));
                                let mut format_rtn = Format::new();
                                format_rtn.set_bg_color(FormatColor::Custom(0x4b51c9));
                                match sheet {
                                    Ok(mut sheet) => {
                                        sheet.write_string(0,0, "Wire Name", Some(&format_header)); 
                                        sheet.write_string(0,1, "Device/Connector", Some(&format_header));
                                        sheet.set_column(0,1,20.0,None);
                                        sheet.write_string(0,2, "Pin", Some(&format_header));
                                        sheet.write_string(0,3, "Wire", Some(&format_header));
                                        sheet.write_string(0,4, "Color", Some(&format_header));
                                        sheet.write_string(0,5, "Length", Some(&format_header));
                                        sheet.write_string(0,6, "Device/Connector", Some(&format_header));
                                        sheet.set_column(0,6,20.0,None);
                                        sheet.write_string(0,7, "Pin", Some(&format_header));
                                        let design = project.get_design(args.design.unwrap().as_ref()).unwrap();
                                        let wires = design.get_wires(args.harness.unwrap().as_ref());
                                        let mut row: u32 = 0;
                                        for wire in wires {
                                            //println!("{}", wire.get_name());
                                            row += 1;
                                            let mut current_format = None;
                                            if (wire.get_color() == "PK") {
                                                current_format = Some(&format_5v);
                                            }
                                            if (wire.get_color() == "BL") {
                                                current_format = Some(&format_rtn);
                                            }
                                            if (wire.get_color() == "OR") {
                                                current_format = Some(&format_48v);
                                            }
                                            if (wire.get_color() == "BN") {
                                                current_format = Some(&format_24v);
                                            }
                                            if (wire.get_color() == "RD") {
                                                current_format = Some(&format_12v);
                                            }

                                            sheet.write_string(row,0,wire.get_name(), current_format);
                                            let connections = wire.get_connections();
                                            let connection_left = connections.get(0);

                                            let wire_type:String = wire.get_material().to_owned() + " " + wire.get_spec();
                                            sheet.write_string(row,3,&wire_type, current_format);
                                            sheet.write_string(row,3,&wire_type, current_format);

                                            sheet.write_string(row,5,&wire.get_length().to_string(), current_format);

                                            sheet.write_string(row,4,&wire.get_color(), current_format);

                                            

                                            connection_left.map(|connection| {
                                                match connection {
                                                    Connection::Connector(connector, pin) => {
                                                        sheet.write_string(row,1,connector.get_name(), current_format);
                                                        sheet.write_string(row,2,pin.get_name(), current_format);
                                                    }
                                                    Connection::Device(device, pin) => {
                                                        sheet.write_string(row,1,device.get_name(), current_format);
                                                        sheet.write_string(row,2,pin.get_name(), current_format);
                                                    }
                                                }
                                            });
                                            let connection_right = connections.get(1);
                                            connection_right.map(|connection| {
                                                match connection {
                                                    Connection::Connector(connector, pin) => {
                                                        sheet.write_string(row,6,connector.get_name(), current_format);
                                                        sheet.write_string(row,7,pin.get_name(), current_format);
                                                    }
                                                    Connection::Device(device, pin) => {
                                                        sheet.write_string(row,6,device.get_name(), current_format);
                                                        sheet.write_string(row,7,pin.get_name(), current_format);
                                                    }
                                                }
                                            });

                                            // for connection in wire.get_connections() {
                                            //     match connection {
                                            //         Connection::Connector(connector, pin) => {
                                                        
                                            //             println!("{} {} {} {}", wire.get_name(), connector.get_name(), pin.get_name(), wire.get_length());
                                            //         }
                                            //         Connection::Device(device, pin) => {
                                            //             println!("{} {} {}", wire.get_name(), device.get_name(), pin.get_name());
                                            //         }
                                            //     }
                                            // }
                                            // println!("");
                                        }
                                    }
                                    Err(e) => {

                                    }
                                }
                            }
                            Err(e) => {

                            }
                        }
                        //design.unwrap().get_wires("");
                        //design.unwrap().get_wires("");
                        // get wires in harness
                        // end points
                        // connectors
                        //show_project_info(&project);
                    }
                }
                Err(e) => {
                    println!("{}{}", "Xml error: ".red(), e.to_string().bright_red())
                }
            }
        }
        Err(e) => {
            println!("{}{}", "File read error: ".red(), e.to_string().bright_red())
        }
    }
}
