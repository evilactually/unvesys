
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui::Sense;
use egui::Label;
use egui::RichText;
use crate::egui::Button;
use crate::egui::Margin;
use crate::egui::CollapsingHeader;
use serde_json::Value;
use std::rc::Rc;
use egui::Ui;
use eframe::egui;
use egui::menu;
use ecolor::{Color32};
use rand::Rng;
use std::sync::{Arc, Mutex};

use petgraph::visit::EdgeRef;
extern crate clap;

extern crate xlsxwriter;

use xlsxwriter::worksheet::LXW_DEF_ROW_HEIGHT_PIXELS;
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

mod vysyslib;
use crate::vysyslib::*;

mod vysis;
use crate::vysis::*;

mod vysyslibxml;
use crate::vysyslibxml::*;

mod json;
use crate::json::*;

mod outline;
use crate::outline::*;

use std::cmp::max;

use colored::*;

use std::collections::{HashMap, HashSet};

use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::{Undirected};
use petgraph::data::{FromElements, Element};
use petgraph::EdgeType;

use petgraph::algo::{min_spanning_tree, MinSpanningTree};

mod bfs;
use crate::bfs::*;

use xlsxwriter::worksheet::WorksheetRow;
use xlsxwriter::worksheet::WorksheetCol;
use xlsxwriter::worksheet::PaperType;

mod xlsxtable;
use crate::xlsxtable::*;



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
    // #[arg(long)]
    // library: Option<String>,
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
            show_design_info(&project.dom, design);
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

fn show_design_info__(dom: &XmlProject, design_name: &str) {
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

fn show_design_info(dom: &XmlProject, design_name: &str) {
    let index = dom.designmgr.logicaldesign.iter().position(|design| design.name == design_name);
    match index {
        Some(index) => {
            let mut harness_set:HashSet<&str> = HashSet::new();
            let design_dom = &dom.designmgr.logicaldesign[index];
            // Collect harnesses
            for wire in &design_dom.connectivity.wire {
                if let Some(harness) = &wire.harness {
                    harness_set.insert(harness.as_ref());
                }
            }
            // Print collected harnesses
            for harness in harness_set.iter() {
                println!("        {} {}", "-".bright_yellow(), harness.yellow());
            }
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

fn show_device_info(dom: &XmlChssystem, partno: &str) {

}


// WIRE LIST POST-PROCESSING
// 1. Consolidate wire ends
// 2. Change rings from connectors to terminations and find connected device
// 3. Number groups with shared terminations(If termination is shared => Device is shared && Pin is shared ), (Pin shared <=> Termination shared)
// 4. Group by 

// GroupBy connector
// SortBy pin

// Main connectors: N, 

pub struct WireList {
    pub wires:Vec<WireEntry>,
}

pub struct WireEntry {
    pub name: Box<str>,
    pub partno: Box<str>,
    pub material: Box<str>,
    pub spec: Box<str>,
    pub color_code: Box<str>,
    pub color_description: Box<str>,
    pub length: f32,
    pub left: Option<WireEndEntry>,
    pub right: Option<WireEndEntry>
}

#[derive(Default)]
#[derive(Clone)]
pub struct WireEndEntry{
    pub device : Box<str>,
    pub pin : Box<str>,
    pub termination : Box<str>,
    pub termination_name : Box<str>
}

fn process_connection<'a>(connection: (&'a  Connection<'a>, &'a Option<&'a str>), library: &Library ) -> WireEndEntry {
    let mut wire_end_info : WireEndEntry = Default::default();
    match connection {
        (Connection::Connector(connector,pin), _) => {
            if connector.is_ring() {
                // If ring is connected to some device, find that device
                let ring_connection = connector.get_ring_connection();
                match ring_connection {
                    Some(ring_connection) => {
                        match ring_connection {
                            Connection::Device(mated_device,mated_pin) => {
                                wire_end_info.device = mated_device.get_name().into();
                                wire_end_info.pin = mated_pin.get_name().into();
                            }
                            // TODO: add grounddevice
                            _ => {
                                println!("{}{}", "Ring may only be connected to device pin: ".red(), connector.get_name().to_string().bright_red());
                            }
                        }
                    }
                    None => {}
                }
                // Ring is not connected anywhere, leave device empty
                // Show ring as termination
                wire_end_info.termination = connector.get_customer_partno().into();
                let partno = connector.get_partno();
                wire_end_info.termination_name = library.lookup_terminal_short_name(partno).unwrap_or_default().into();
            } else
            {
                //println!("{}", connector.get_name());
                wire_end_info.device = connector.get_name().into();
                wire_end_info.pin = pin.get_name().into();
                wire_end_info.termination = "TODO".into();
            }
        }
        (Connection::Device(device,pin), termination) => {
            wire_end_info.device = device.get_name().into();
            wire_end_info.pin = pin.get_name().into();
            wire_end_info.termination = "TODO".into();
            if let Some(termination) = termination {
                let terminal_partnumber = library.lookup_customer_partnumber(*termination);
                wire_end_info.termination = terminal_partnumber.unwrap_or_default().into();
                wire_end_info.termination_name = library.lookup_terminal_short_name(*termination).unwrap_or_default().into();
            }
        }
        (Connection::Splice(splice,pin), _) => {
            wire_end_info.device = splice.get_name().into();
            wire_end_info.pin = pin.get_name().into();
            wire_end_info.termination = "TODO".into();
            // TODO: Read properties of the device to find out which side of the splice wire is meant to 
        }
    }
    //wire_end_info.termination = "TODO".into();
    wire_end_info
}

//let left_node = graph.add_node(left.device.to_owned().into_boxed_str());
                                                    // let found_left_node = graph
                                                    // .node_indices()
                                                    // .find(|&node_index| graph[node_index] == left.device);


                                                    // let right_node = graph.add_node(right.device.to_owned().into_boxed_str());
                                                    // graph.add_edge(left_node, right_node, ());

fn find_node_or_add<T, S, U>(graph: &mut Graph<S, T, U>, node_weight:S) -> NodeIndex where U: EdgeType, S: std::cmp::PartialEq<S> + Clone {
    let found_node = graph
    .node_indices()
    .find(|&node_index| graph[node_index] == node_weight.clone());

    match found_node {
        Some(node) => {
            node
        }
        None => {
            graph.add_node(node_weight.clone())
        }
    }
}

/// Find node with maximum neighbor nodes
fn find_max_neighbor_node<E,N>(graph:&Graph<N,E,Undirected>) -> Option<NodeIndex> {
    // Pick the root of MST
    let mut max_neighbors = -1;
    let mut max_node = graph.node_indices().next();
    for node in graph.node_indices() {
        let neighbors = graph.neighbors(node).count() as i32;
        if neighbors >= max_neighbors {
            max_neighbors = neighbors;
            max_node = Some(node);
        }
    }
    max_node
}

// fn construct_directed_mst_graph(
//     graph:&Graph<Box<str>,bool,Undirected>, 
//     root_parent:Option<NodeIndex>, 
//     root:NodeIndex,
//     new_root:Option<NodeIndex>,
//     directed_graph:&mut Graph<Box<str>,bool>) {
//     println!("Sub-tree root is {}", graph[root]);
//     //let new_root = directed_graph.add_node(graph[root].clone());
//     let new_root = match new_root {
//         Some(new_root) => {new_root}
//         None => {
//             directed_graph.add_node(graph[root].clone())
//         }
//     }; 
//     // Every node is guaranteed to be on MST, so is the root node.
//     for child in graph.neighbors(root) {
//         println!("    Processing child node {}.{}", graph[root],graph[child]);
//         println!("        root_parent {}", root_parent.map(|root_parent| root_parent.index() as i32).unwrap_or(-1) );
//         println!("        root {}", root.index() );
//         println!("        child {}", child.index() );
//         let is_parent = root_parent.eq(&Some(child));
//         println!("        Is neighbor node parent? {}", is_parent);
//         if !is_parent && root != child {
//             println!("        {}", "following...");
//             let original_edge:Option<EdgeIndex> = graph.find_edge(root, child);
//             let is_mst_edge = *original_edge.and_then(|edge| graph.edge_weight(edge)).unwrap_or(&false);
//             println!("        is_mst_edge? {}", is_mst_edge);
            
//             // Check if node already exists
//             let new_child = find_node_or_add(directed_graph, graph[child].clone());
//             // Create child node in the new graph
//             //let new_child = directed_graph.add_node(graph[child].clone());

//             // Create edge and transfer MST edge mark
//             directed_graph.update_edge(new_root, new_child, is_mst_edge);


//             // Add other edges recursively, but only follow MST path. Avoid following back up the graph.
//             if is_mst_edge { 
//                 construct_directed_mst_graph(graph, Some(root), child, Some(new_child), directed_graph);
//             }
//             println!("        {}", "exit...");
//         } else if root == child {
//             // Add edge to self, self reference are non-mst
//             directed_graph.add_edge(new_root, new_root, false);
//         }
//     }
// }

// Depth-First Search (DFS) traversal
fn dfs_traversal<N, E: Copy>(graph: &Graph<N, E, Undirected>, directed_tree: &mut Graph<N, E>, current_node: NodeIndex, parent_edge: Option<EdgeIndex>, mst_edges: &[EdgeIndex])
{
    for edge in graph.edges(current_node) {
        let edge_index = edge.id();
        let source = edge.source();
        let target = edge.target();

        // Skip the parent edge in the DFS traversal
        if Some(edge_index) == parent_edge {
            continue;
        }

        // Assign a direction to the edge based on MST membership
        if mst_edges.contains(&edge_index) {
            directed_tree.add_edge(source, target, graph[edge_index]);
        } else {
            directed_tree.add_edge(target, source, graph[edge_index]);
        }

        // Recursively traverse the child node
        let child_node = if current_node == source { target } else { source };
        dfs_traversal(graph, directed_tree, child_node, Some(edge_index), mst_edges);
    }
}

//fn build_connectivity_graphs()


//fn print_wire_row()


// struct SheetRegion {
//     first_row: WorksheetRow, 
//     first_col: WorksheetCol, 
//     last_row: WorksheetRow, 
//     last_col: WorksheetCol,
// }

// fn outside_border<'a>(base:&Format, 
//     region : &SheetRegion,
//     row : WorksheetRow,
//     col : WorksheetCol) -> Format{
//     let mut format:Format = base.clone();
//     if row == region.first_row {
//         format.set_border_top(FormatBorder::Medium);
//     }
//     if row == region.last_row {
//         format.set_border_bottom(FormatBorder::Medium);
//     }
//     if col == region.first_col {
//         format.set_border_left(FormatBorder::Medium);
//     }
//     if col == region.last_col {
//         format.set_border_right(FormatBorder::Medium);
//     }
//     format
// }

// fn write_string_bordered(
//     sheet: &mut Worksheet,
//     row: WorksheetRow,
//     col: WorksheetCol,
//     text: &str,
//     format: &Format,
//     outside: &SheetRegion) -> Result<(), XlsxError> {
//     let format = outside_border(&format, outside, row, col);
//     sheet.write_string(row, col, text, Some(&format))
// }

// fn write_blank_bordered(
//     sheet: &mut Worksheet,
//     row: WorksheetRow,
//     col: WorksheetCol,
//     text: &str,
//     format: &Format,
//     outside: &SheetRegion) -> Result<(), XlsxError> {
//     let format = outside_border(&format, outside, row, col);
//     sheet.write_string(row, col, text, Some(&format))
// }



pub struct WireListXlsxFormatter<'a> {
    table : XLSXTable,
    workbook : &'a Workbook,
    sheet : Worksheet<'a>,
    current_row: u32,
    bg_colormap: &'a HashMap<&'a str, FormatColor>
}

impl WireListXlsxFormatter<'_> {
    // Column definitions
    // Wire
    const WIRE_ITEM: u16 = 0;
    // From
    const FROM_DEVICE: u16 = 1;
    const FROM_DASH: u16 = 2;
    const FROM_PIN: u16 = 3;
    // Terminal
    const FROM_TERM_PARTNO: u16 = 4; // Merge
    const FROM_TERM_NAME: u16 = 5;   // ^
    // Wire material
    const WIRE_PARTNO: u16 = 6; // Merge
    const WIRE_NAME: u16 = 7;   // ^
    const WIRE_COLOR: u16 = 8;
    const WIRE_LEN: u16 = 9;
    // Terminal
    const TO_TERM_PARTNO: u16 = 10; // Merge
    const TO_TERM_NAME: u16 = 11;   // ^
    // To
    const TO_DEVICE: u16 = 12;
    const TO_DASH: u16 = 13;
    const TO_PIN: u16 = 14;
    // Margins
    const LEFT:u16 = 1;
    const TOP:u32 = 1;

    pub fn new<'a>(workbook: &'a xlsxwriter::Workbook, bg_colormap: &'a HashMap<&str, xlsxwriter::format::FormatColor>) -> WireListXlsxFormatter<'a> {
        let mut table = XLSXTable::new();
        let mut format = Format::new();
        format.set_align(FormatAlignment::Center);
        table.set_default_format(format);
        WireListXlsxFormatter {
            table : table,
            workbook : workbook,
            sheet : workbook.add_worksheet(None).unwrap(),
            current_row : Self::TOP + 1,
            bg_colormap : bg_colormap
        }
    }

    pub fn print_header(&mut self) {
        let row = Self::TOP;
        // Wire
        self.table.set_cell(row, Self::LEFT + Self::WIRE_ITEM, "Wire Item");
        self.table.set_col_width_pixels(Self::LEFT + Self::WIRE_ITEM, 150);
        // From
        self.table.set_cell(row, Self::LEFT + Self::FROM_DEVICE, "Device");
        self.table.set_cell(row, Self::LEFT + Self::FROM_DASH, "-");
        self.table.set_col_width_pixels(Self::LEFT + Self::FROM_DASH, 20);
        self.table.set_cell(row, Self::LEFT + Self::FROM_PIN, "Pin");
        // Terminal
        self.table.set_cell(row, Self::LEFT + Self::FROM_TERM_PARTNO, "Termination");
        self.table.set_col_width_pixels(Self::LEFT + Self::FROM_TERM_PARTNO, 125);
        self.table.set_cell(row, Self::LEFT + Self::FROM_TERM_NAME, "");
        self.table.set_col_width_pixels(Self::LEFT + Self::FROM_TERM_NAME, 125);
        // Wire
        self.table.set_cell(row, Self::LEFT + Self::WIRE_PARTNO, "Wire");
        self.table.set_col_width_pixels(Self::LEFT + Self::WIRE_PARTNO, 125);
        self.table.set_cell(row, Self::LEFT + Self::WIRE_NAME, "");
        self.table.set_cell(row, Self::LEFT + Self::WIRE_COLOR, "Color");
        self.table.set_cell(row, Self::LEFT + Self::WIRE_LEN, "Length");
        // Terminal
        self.table.set_cell(row, Self::LEFT + Self::TO_TERM_PARTNO, "Termination");
        self.table.set_col_width_pixels(Self::LEFT + Self::TO_TERM_PARTNO, 125);
        self.table.set_cell(row, Self::LEFT + Self::TO_TERM_NAME, "");
        self.table.set_col_width_pixels(Self::LEFT + Self::TO_TERM_NAME, 125);
        // To
        self.table.set_cell(row, Self::LEFT + Self::TO_DEVICE, "Device");
        self.table.set_cell(row, Self::LEFT + Self::TO_DASH, "-");
        self.table.set_col_width_pixels(Self::LEFT + Self::TO_DASH, 20);
        self.table.set_cell(row, Self::LEFT + Self::TO_PIN, "Pin");
    }

    pub fn print_entry(&mut self, wire: &WireEntry) {
        // Wire
        self.table.set_cell(self.current_row, Self::LEFT + Self::WIRE_ITEM, &wire.name);
        // From
        let left_wire_end = wire.left.clone().unwrap_or_default();
        self.table.set_cell(self.current_row, Self::LEFT + Self::FROM_DEVICE, &left_wire_end.device);
        self.table.set_cell(self.current_row, Self::LEFT + Self::FROM_DASH, "-");
        self.table.set_cell(self.current_row, Self::LEFT + Self::FROM_PIN, &left_wire_end.pin);
        // Terminal
        self.table.set_cell(self.current_row, Self::LEFT + Self::FROM_TERM_PARTNO, &left_wire_end.termination);
        self.table.set_cell(self.current_row, Self::LEFT + Self::FROM_TERM_NAME, &left_wire_end.termination_name);
        // Wire
        self.table.set_cell(self.current_row, Self::LEFT + Self::WIRE_PARTNO, &wire.partno);
        self.table.set_cell(self.current_row, Self::LEFT + Self::WIRE_NAME, &(wire.material.to_string() + " " + &wire.spec));
        self.table.set_cell(self.current_row, Self::LEFT + Self::WIRE_COLOR, &wire.color_description);
        self.table.set_cell(self.current_row, Self::LEFT + Self::WIRE_LEN, &wire.length.to_string());
        // Terminal
        let right_wire_end = wire.right.clone().unwrap_or_default();
        self.table.set_cell(self.current_row, Self::LEFT + Self::TO_TERM_PARTNO, &right_wire_end.termination);
        self.table.set_cell(self.current_row, Self::LEFT + Self::TO_TERM_NAME, &right_wire_end.termination_name);
        // To
        self.table.set_cell(self.current_row, Self::LEFT + Self::TO_DEVICE, &right_wire_end.device);
        self.table.set_cell(self.current_row, Self::LEFT + Self::TO_DASH, "-");
        self.table.set_cell(self.current_row, Self::LEFT + Self::TO_PIN, &right_wire_end.pin);
        // Set row bg color
        self.table.modify_region_format(&XLSXTableRegion {
            first_row: self.current_row,
            first_col: Self::LEFT,
            last_row: self.current_row,
            last_col: Self::LEFT + Self::TO_PIN
        }, &|format| {
            let color_code_upper:String = wire.color_code.to_string().to_uppercase();
            format.set_bg_color(*self.bg_colormap.get(&color_code_upper.as_ref()).unwrap_or(&FormatColor::White));
        });
        // Increment row
        self.current_row = self.current_row + 1;
    }
}

impl Drop for WireListXlsxFormatter<'_> {
    fn drop(&mut self) {
        // Finalize outside border
        self.table.set_region_border(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::TO_PIN
        }, FormatBorder::Medium);
        // Header border
        let header_region = &XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT,
            last_row: Self::TOP,
            last_col: Self::LEFT + Self::TO_PIN
        };
        self.table.set_region_border(&header_region, FormatBorder::Medium);
        // Header format
        self.table.modify_region_format(&header_region, &|format| {
            format.set_bold();
        });
        // Wire item separator
        self.table.set_region_border_right(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::WIRE_ITEM
        }, FormatBorder::Dotted);
        // Left wire end separator
        self.table.set_region_border_right(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::FROM_TERM_NAME
        }, FormatBorder::Dotted);
        // Right wire end separator
        self.table.set_region_border_right(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::WIRE_LEN
        }, FormatBorder::Dotted);
        // Right align FROM device column
         self.table.modify_region_format(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT + Self::FROM_DEVICE,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::FROM_DEVICE
        }, &|format| {
            format.set_align(FormatAlignment::Right);
        });
        // Left align FROM pin column
         self.table.modify_region_format(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT + Self::FROM_PIN,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::FROM_PIN
        }, &|format| {
            format.set_align(FormatAlignment::Left);
        });
        // Right align TO device column
         self.table.modify_region_format(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT + Self::TO_DEVICE,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::TO_DEVICE
        }, &|format| {
            format.set_align(FormatAlignment::Right);
        });
        // Left align TO pin column
         self.table.modify_region_format(&XLSXTableRegion {
            first_row: Self::TOP,
            first_col: Self::LEFT + Self::TO_PIN,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::TO_PIN
        }, &|format| {
            format.set_align(FormatAlignment::Left);
        });
        // Set paper
        self.sheet.set_paper(PaperType::Tabloid);
        // Set orientation
        self.sheet.set_landscape();

        self.table.render_to_worksheet(&mut self.sheet);
    }
}

fn show_project_info_gui(project: &Project, ui : &mut Ui) {
    println!("{} {}", "XmlProject Name:".bright_yellow(), project.get_name().yellow());
    println!("{}", "Logical Designs:".bright_yellow());
    // List logical design names
    {
        //let logical_designs = project.get_logical_design_names();
        for design in project.get_logical_design_iter() {
            ui.label(design.get_name());
            //println!("    {} {}", "*".bright_yellow(), design.yellow());
            //show_design_info(&project.dom, design);
        }
    }
    // println!("{}", "Harness Designs:".bright_yellow());
    // // List harness design names
    // {
    //     let harness_designs = project.get_harness_design_names();
    //     for design in harness_designs {
    //         println!("    {} {}", "*".bright_yellow(), design.yellow());
    //     }
    // }
    // println!("{}", "OK".bright_green());
}



fn startup_worker(state_clone: Arc<Mutex<State>>) {
    // Any slow start-up work goes here
}

struct State {
    duration: u64,
    ctx: Option<egui::Context>,
    project_outline: Option<ProjectOutline>,

}

impl State {
    pub fn new() -> Self {
        Self {
            duration: 0,
            ctx: None,
            project_outline: None
        }
    }
}

pub struct App {
    state: Arc<Mutex<State>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let state = Arc::new(Mutex::new(State::new()));
        state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        let state_clone = state.clone();
        // Any slow start-up work goes here
        std::thread::spawn(move || {
            startup_worker(state_clone);
        });
        Self {
            state
        }
    }
}

impl<'a> eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    
                                    // Clone Arc to avoid using self inside closure
                                    let state_clone = self.state.clone();

                                    // Wrap slow loading code in a thread
                                    std::thread::spawn(move || { // state_clone and path are moved
                                        let xmlpath = path.display().to_string();
                                        let xml = read_file(&xmlpath);
                                        if let Ok(xml) = xml {
                                            let project = Project::new(&xml);
                                            if let Ok(project) = project {
                                                state_clone.lock().unwrap().project_outline = Some(ProjectOutline::new(&project));
                                            }
                                        }
                                    });
                                }
                                ui.close_menu(); // close menu so it doesn't stay opened
                            }
                        });
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
            .max_width(f32::INFINITY)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                if let Some(outline) = &self.state.lock().unwrap().project_outline {
                    CollapsingHeader::new(&outline.name)
                    .default_open(true)
                    .selectable(true)
                    .show(ui, |ui| {
                        CollapsingHeader::new("Logical Designs")
                        .show(ui, |ui| {
                            for design in outline.designs.iter() {
                                CollapsingHeader::new(&design.name)
                                .default_open(true)
                                .show(ui, |ui| {
                                    for harness in design.harnesses.iter() {
                                        let harness_entry = ui.add(Label::new(harness).sense(Sense::hover())) // Enable hover event
                                        .context_menu(|ui| {
                                            if ui.button("Generate wire list").clicked() {
                                                println!("Generating wire list for {}, {}", &design.name, harness);
                                                ui.close_menu();
                                            }
                                            ui.button("Generate label data");
                                        });
                                        // Highlight on hover
                                        if harness_entry.hovered() {
                                            harness_entry.highlight();
                                        }
                                    }
                                })
                                .header_response.context_menu(|ui| {
                                    if ui.button("Generate wire list").clicked() {
                                        println!("Generating wire list for {}", &design.name);
                                        ui.close_menu();
                                    }
                                    ui.button("Generate label data");
                                });
                            }
                        });
                    });
                } // else state locked by worker thread
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

fn main_____________() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    let mut projectPath: Option<String> = None;
    let mut projectXml: Option<std::io::Result<String>> = None;

    let mut c = 0;
    
    let mut closure = || {
        // This will not compile because `outside_variable` is immutable inside the closure.
        c = c + 1;
        println!("in the closure, c={}", c);
    };
    
    closure();
    closure();  
    closure();  
    closure();     

    println!("Outside closure: {}", c);

    // Main GUI loop
    eframe::run_simple_native("UnVeSys", options,  move |ctx, _frame| {
        //println!("{}", "1")

    let mut project: Option<Result<Project, XmlError>> = None;
    let xml = "123".to_owned();
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                //projectPath = Some(path.display().to_string());
                                
                                ui.close_menu(); // close menu so it doesn't stay opened
                            }
                            // projectPath.as_ref().map(|path| {
                            //     projectXml = Some(read_file(path));
                            //     // projectXml.as_ref().map(|xml| {
                            //     //     xml.as_ref().map(|xml| {
                            //     //         project = Some(Project::new(xml)); 
                            //     //     });
                            //     // });
                            //     // if let Some(Ok(xml)) = &projectXml {
                            //     //     project = Some(Project::new(&xml)); 
                            //     // }
                            // });

                            //project = Some(Project::new(&xml));
                            // projectXml.as_ref().map(|xml| {
                            //     xml.as_ref().map(|xml| {
                            //         project = Some(Project::new(&xml));
                            //         //project = Some(Project::new(xml)); 
                            //     });
                            // });
                        }
                    });
                });
            });
        });
        // End Menu bar

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                if let Some(path) = &projectPath {
                    ui.label(path);
                }
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    //self.picked_path = Some(path.display().to_string());
                }
            }

            // if let Some(projectPath) = projectPath {

            // }

            
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}


fn main____________() {
    // Map of row background colors for each wire color
    let mut bg_color_map: HashMap<&str, FormatColor> = HashMap::new();
    bg_color_map.insert("PK", FormatColor::Custom(0xffccff)); // 5v format
    bg_color_map.insert("RD", FormatColor::Custom(0xff9999)); // 12v format
    bg_color_map.insert("BN", FormatColor::Custom(0xd3a77b)); // 24v format
    bg_color_map.insert("OR", FormatColor::Custom(0xfff2cc)); // 48v format
    bg_color_map.insert("BL", FormatColor::Custom(0xddebf7)); // GND format
    bg_color_map.insert("YL", FormatColor::Custom(0xffffd1)); // Analog/Bat+ format
    bg_color_map.insert("TN", FormatColor::Custom(0xead5c0)); // 24v DO
    bg_color_map.insert("BK", FormatColor::Custom(0xf2f2f2)); // 24v DI
    bg_color_map.insert("GN", FormatColor::Custom(0xc6e0b4)); // Sinking output

    // let mut format_5v = base_format.clone();
    // format_5v.set_bg_color(FormatColor::Custom(0xffccff));
    // let mut format_12v = base_format.clone();
    // format_12v.set_bg_color(FormatColor::Custom(0xff9999));
    // let mut format_24v = base_format.clone();
    // format_24v.set_bg_color(FormatColor::Custom(0xd3a77b));
    // let mut format_48v = base_format.clone();
    // format_48v.set_bg_color(FormatColor::Custom(0xfff2cc));
    // let mut format_rtn = base_format.clone();
    // format_rtn.set_bg_color(FormatColor::Custom(0xddebf7));
    // let mut format_analog = base_format.clone();
    // format_analog.set_bg_color(FormatColor::Custom(0xffffd1));
    // let mut format_24V_sourcing = base_format.clone();
    // format_24V_sourcing.set_bg_color(FormatColor::Custom(0xead5c0));
    // let mut format_input = base_format.clone();
    // format_input.set_bg_color(FormatColor::Custom(0xf2f2f2));
    // let mut format_sinking_output = base_format.clone();
    // format_sinking_output.set_bg_color(FormatColor::Custom(0xc6e0b4));
   
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

   


    match (read_file(&args.project), read_file("Library.xml")) {
        // Project xml file and Library files were read successfully
        (Ok(xml_project), Ok(xml_library)) => {
            let project = Project::new(&xml_project);
            let library = Library::new(&xml_library);
            match (project, library) {
                // Project and Library parsed successfuly 
                (Ok(project), Ok(library)) => {
                    let json = project_outline_json(&project);
                    println!("{}", json.to_string());
                    let no_outputs_specified = args.labels.is_none()
                                               && args.bom.is_none()
                                               && args.cutlist.is_none()
                                               && args.index.is_none();
                    // No design specified, show project info
                    if args.design.is_none() || args.harness.is_none() {
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
                                //let sheet = workbook.add_worksheet(args.harness.as_deref());
                                let mut format_header = Format::new();
                                format_header.set_align(FormatAlignment::Center)
                                .set_bold();

                                let mut base_format : Format = Format::new();
                                base_format.set_align(FormatAlignment::Center);

                                
                                let mut format_5v = base_format.clone();
                                format_5v.set_bg_color(FormatColor::Custom(0xffccff));
                                let mut format_12v = base_format.clone();
                                format_12v.set_bg_color(FormatColor::Custom(0xff9999));
                                let mut format_24v = base_format.clone();
                                format_24v.set_bg_color(FormatColor::Custom(0xd3a77b));
                                let mut format_48v = base_format.clone();
                                format_48v.set_bg_color(FormatColor::Custom(0xfff2cc));
                                let mut format_rtn = base_format.clone();
                                format_rtn.set_bg_color(FormatColor::Custom(0xddebf7));
                                let mut format_analog = base_format.clone();
                                format_analog.set_bg_color(FormatColor::Custom(0xffffd1));
                                let mut format_24V_sourcing = base_format.clone();
                                format_24V_sourcing.set_bg_color(FormatColor::Custom(0xead5c0));
                                let mut format_input = base_format.clone();
                                format_input.set_bg_color(FormatColor::Custom(0xf2f2f2));
                                let mut format_sinking_output = base_format.clone();
                                format_sinking_output.set_bg_color(FormatColor::Custom(0xc6e0b4));
                                //match sheet {
                                    //Ok(mut sheet) => {

                                        let table: XLSXTable = XLSXTable::new();

                                        let WIRE_NAME = 0;
                                        let WIRE_FROM_DEVICE = 1;
                                        let WIRE_FROM_PIN = 2;
                                        let WIRE_FROM_TERM = 3;
                                        let WIRE_TYPE = 4;
                                        let WIRE_COLOR = 5;
                                        let WIRE_LEN = 6;
                                        let WIRE_TO_TERM = 7;
                                        let WIRE_TO_DEVICE = 8;
                                        let WIRE_TO_PIN = 9;

                                        // sheet.write_string(0,WIRE_NAME, "Wire Name", Some(&format_header)); 
                                        // sheet.write_string(0,WIRE_FROM_DEVICE, "Device/Connector", Some(&format_header));
                                        // sheet.write_string(0,WIRE_FROM_PIN, "Pin", Some(&format_header));
                                        // sheet.write_string(0,WIRE_FROM_TERM, "Termination", Some(&format_header));
                                        // sheet.write_string(0,WIRE_TYPE, "Wire", Some(&format_header));
                                        // sheet.write_string(0,WIRE_COLOR, "Color", Some(&format_header));
                                        // sheet.write_string(0,WIRE_LEN, "Length", Some(&format_header));
                                        // sheet.write_string(0,WIRE_TO_TERM, "Termination", Some(&format_header));
                                        // sheet.write_string(0,WIRE_TO_DEVICE, "Device/Connector", Some(&format_header));
                                        // sheet.write_string(0,WIRE_TO_PIN, "Pin", Some(&format_header));
                                        // sheet.set_column(0,9,20.0,None);
                                        let harness = args.harness.unwrap();
                                        let design = project.get_design(args.design.unwrap().as_ref()).unwrap();
                                        let wires = design.get_wires(&harness);
                                        let mut row: u32 = 0;

                                        // Graph
                                        let mut graph: Graph<Box<str>, bool, Undirected> = Graph::new_undirected();
                                        // Wire list
                                        let mut wire_list: WireList = WireList {
                                            wires : Vec::new()
                                        };

                                        for wire in wires {
                                            //println!("{}", wire.get_name());
                                            row += 1;
                                            let mut current_format = None;
                                            if wire.get_color().to_uppercase() == "PK" {
                                                current_format = Some(&format_5v);
                                            }
                                            if wire.get_color().to_uppercase() == "BL" {
                                                current_format = Some(&format_rtn);
                                            }
                                            if wire.get_color().to_uppercase() == "OR" {
                                                current_format = Some(&format_48v);
                                            }
                                            if wire.get_color().to_uppercase() == "BN" {
                                                current_format = Some(&format_24v);
                                            }
                                            if wire.get_color().to_uppercase() == "RD" {
                                                current_format = Some(&format_12v);
                                            }
                                            if wire.get_color().to_uppercase() == "YL" {
                                                current_format = Some(&format_analog);
                                            }
                                            if wire.get_color().to_uppercase() == "BK" {
                                                current_format = Some(&format_input);
                                            }

                                            // sheet.write_string(row,WIRE_NAME,wire.get_name(), current_format);

                                            // let wire_type:String = wire.get_material().to_owned() + " " + wire.get_spec();
                                            // sheet.write_string(row,WIRE_TYPE,&wire_type, current_format);

                                            // sheet.write_string(row,WIRE_LEN,&wire.get_length().to_string(), current_format);

                                            // sheet.write_string(row,WIRE_COLOR,&wire.get_color(), current_format);

                                            let connections = wire.get_connections();
                                            let connection_left = connections.get(0);

                                            let left_wire_end = connection_left.map(|(connection_left, termination)| {
                                                let mut left_wire_end = process_connection((connection_left, termination), &library);
                                                //if let Some(termination) = termination {
                                                //    let terminal_partnumber = library.lookup_customer_partnumber(*termination);
                                                //    left_wire_end.termination = terminal_partnumber.unwrap_or_default().into();
                                                    //println!("{}{}", *termination, left_wire_end.termination_name);
                                                //}
                                                //left_wire_end.termination_name = library.lookup_terminal_short_name(*termination).unwrap_or_default().into();
                                                left_wire_end
                                            });

                                            // match left_wire_end {
                                            //     Some(ref left_wire_end) => {
                                                    
                                            //         sheet.write_string(row,WIRE_FROM_DEVICE,&left_wire_end.device, current_format);
                                            //         sheet.write_string(row,WIRE_FROM_PIN,&left_wire_end.pin, current_format);
                                            //         sheet.write_string(row,WIRE_FROM_TERM,&left_wire_end.termination, current_format);
                                            //     }
                                            //     None => {
                                            //         sheet.write_blank(row,WIRE_FROM_DEVICE,current_format);
                                            //         sheet.write_blank(row,WIRE_FROM_PIN,current_format);
                                            //         sheet.write_blank(row,WIRE_FROM_TERM,current_format);
                                            //     }
                                            // }

                                            let connection_right = connections.get(1);
                                            let right_wire_end = connection_right.map(|(connection_right, termination)| {
                                                let mut right_wire_end = process_connection((connection_right, termination), &library);
                                                //if let Some(termination) = termination {
                                                //    let terminal_partnumber = library.lookup_customer_partnumber(*termination);
                                                //    right_wire_end.termination = terminal_partnumber.unwrap_or_default().into();
                                                    //right_wire_end.termination_name = library.lookup_terminal_short_name(*termination).unwrap_or_default().into();
                                                //}
                                                right_wire_end
                                            });

                                            // match right_wire_end {
                                            //     Some(ref right_wire_end) => {

                                            //         sheet.write_string(row,WIRE_TO_DEVICE,&right_wire_end.device, current_format);
                                            //         sheet.write_string(row,WIRE_TO_PIN,&right_wire_end.pin, current_format);
                                            //         sheet.write_string(row,WIRE_TO_TERM,&right_wire_end.termination, current_format);
                                            //     }
                                            //     None => {
                                            //         sheet.write_blank(row,WIRE_TO_DEVICE,current_format);
                                            //         sheet.write_blank(row,WIRE_TO_PIN,current_format);
                                            //         sheet.write_blank(row,WIRE_TO_TERM,current_format);
                                            //     }
                                            // }

                                            wire_list.wires.push(
                                                WireEntry {
                                                    name : wire.get_name().into(),
                                                    partno : wire.get_customer_partno().into(),
                                                    material : wire.get_material().into(),
                                                    spec : wire.get_spec().into(),
                                                    color_code : wire.get_color().into(),
                                                    color_description: library.get_color_description(wire.get_color()).unwrap_or_default().into(),
                                                    length : wire.get_length(),
                                                    left : left_wire_end.clone(),
                                                    right : right_wire_end.clone()
                                                }
                                            );

                                            // Build a graph of devices and connectors
                                            // Match if both connections exist
                                            match (left_wire_end, right_wire_end) {
                                                (Some(ref left), Some(ref right)) => {
                                                    let left_node = find_node_or_add(&mut graph, left.device.clone().into());
                                                    let right_node = find_node_or_add(&mut graph, right.device.clone().into());
                                                    // Chech if edge exists
                                                    match graph.find_edge(left_node, right_node) {
                                                        Some(_) => {}
                                                        None => {
                                                            // Add edges only once
                                                            graph.add_edge(left_node, right_node, false);
                                                        }
                                                    }
                                                }
                                                _ => {

                                                }
                                            }
                                        } // wire loop


                                        let mut xlsx_formatter = WireListXlsxFormatter::new(&workbook, &bg_color_map);
                                        // Output plane wire list
                                        xlsx_formatter.print_header();
                                        for wire in wire_list.wires.iter() {
                                            xlsx_formatter.print_entry(wire);
                                        }

                                        // Build a Minimum Spaning Tree from connectivity graph. Each node is a reference to original graph.
                                        let mut mst_edges:Vec<EdgeIndex> = Vec::new();
                                        let mut mst_unidirected_graph: Graph<NodeIndex, (), Undirected> = Graph::new_undirected();
                                        {
                                            let mut mst = min_spanning_tree(&graph);
                                            for i in 0..mst.clone().count() {
                                                let element = mst.next().unwrap();
                                                match element {
                                                    Element::Node{weight} => {
                                                        mst_unidirected_graph.add_node(NodeIndex::new(i));
                                                    }
                                                    Element::Edge{source, target, ..} => {
                                                        let out_source = find_node_or_add(&mut mst_unidirected_graph, NodeIndex::new(source));
                                                        let out_target = find_node_or_add(&mut mst_unidirected_graph, NodeIndex::new(target));
                                                        let edge = mst_unidirected_graph.add_edge(out_source, out_target, ());
                                                        mst_edges.push(edge);
                                                    }
                                                }
                                            }
                                        }

                                        let mut mst_directed_graph: Graph<NodeIndex, ()> = Graph::new();
                                        // Add nodes to the directed tree
                                        for node in mst_unidirected_graph.node_indices() {
                                            mst_directed_graph.add_node(node);
                                        }
                                        // Pick the root of MST
                                        let root_node = find_max_neighbor_node(&graph);
                                        dfs_traversal(&mst_unidirected_graph, &mut mst_directed_graph, root_node.unwrap(), None, mst_edges.as_slice());

                                        // Perform BST traversal of mst_directed_graph
                                        let mut bfs = Bfs::new(&mst_unidirected_graph, root_node.unwrap());
                                        let mut current_root = root_node;
                                        while let (Some(node), level_end) = bfs.next(&mst_unidirected_graph) {
                                            let parent = mst_directed_graph.neighbors_directed(node, petgraph::Direction::Incoming).next();
                                            match parent {
                                                Some(parent) => {
                                                    //println!("{}", graph[parent]);
                                                }
                                                None => {}
                                            }
                                            //println!("    {}", graph[node]);

                                            if level_end {
                                                //println!("{}", "----------");
                                                //current_node = node
                                            }
                                        }


                                        {
                                        let dot = Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel, Config::NodeNoLabel], &move|_, edge| {
                                            //let is_mst_edge = mst_directed_graph.find_edge(edge.source(), edge.target()).is_some();
                                            if  true {
                                                format!("color=\"{}\"", "red")
                                            } else {
                                                "".to_string()
                                            }
                                        },
                                        &|_, (id,name)| {
                                            format!("label=\"{}\"", name)
                                        });

                                        // Print the DOT representation
                                        //println!("{:?}", dot);
                                        }


                                        let dot2 = Dot::with_attr_getters(&mst_directed_graph, &[Config::EdgeNoLabel, Config::NodeNoLabel], &|_, edge| {
                                            "".to_string()
                                        },
                                        &|_, (id,name)| {
                                            format!("label=\"{:?}\"", name)
                                        });

                                        //println!("{:?}", dot2);

                                    //}
                                    //Err(e) => {

                                    //}
                                //}
                                        println!("{} {} {} {} {}", "Harness".yellow(), &harness.yellow(), "contains".yellow(), wire_list.wires.iter().len().to_string().yellow(), "wires.".yellow());
                                        println!("{}", "OK".bright_green());
                            }
                            Err(e) => {
                                // TODO: xmlwrite is panicing when it can't create thh file, how do I catch it?
                                println!("{}{}", "Could not create workbook: ".red(), e.to_string().bright_red())
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
                (Err(e), _) => {
                    println!("{}{}", "Xml error: ".red(), e.to_string().bright_red())
                }
                (_,Err(e)) => {
                    println!("{}{}", "Xml error: ".red(), e.to_string().bright_red())
                }
            }
        }
        (Err(e), _) => {
            println!("{}{}", "File read error: ".red(), e.to_string().bright_red())
        }
        (_,Err(e)) => {
            println!("{}{}", "File read error: ".red(), e.to_string().bright_red())
        }
    }

}

