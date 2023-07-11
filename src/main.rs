


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

mod vysis;
use crate::vysis::*;

mod vysislibxml;
use crate::vysislibxml::*;

use std::cmp::max;

use colored::*;

use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::{Undirected};
use petgraph::data::{FromElements, Element};
use petgraph::EdgeType;

use petgraph::algo::{min_spanning_tree, MinSpanningTree};


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

struct WireEntry<'a> {
    name: &'a str,
    left: WireEndEntry<'a>,
    right: WireEndEntry<'a>
}

#[derive(Default)]
struct WireEndEntry<'a> {
    device : &'a str,
    pin : &'a str,
    termination : &'a str
}

fn process_connection<'a>(connection:&'a  Connection<'a>) -> WireEndEntry<'a> {
    let mut wire_end_info : WireEndEntry = Default::default();
    match connection {
        Connection::Connector(connector,pin) => {
            if connector.is_ring() {
                // If ring is connected to some device, find that device
                let ring_connection = connector.get_ring_connection();
                match ring_connection {
                    Some(ring_connection) => {
                        match ring_connection {
                            Connection::Device(mated_device,mated_pin) => {
                                wire_end_info.device = mated_device.get_name();
                                wire_end_info.pin = mated_pin.get_name();
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
                wire_end_info.termination = connector.get_customer_partno();
            } else
            {
                //println!("{}", connector.get_name());
                wire_end_info.device = connector.get_name();
                wire_end_info.pin = pin.get_name();
                wire_end_info.termination = "TODO";
            }
        }
        Connection::Device(device,pin) => {
            wire_end_info.device = device.get_name();
            wire_end_info.pin = pin.get_name();
            wire_end_info.termination = "TODO";
        }
        Connection::Splice(splice,pin) => {
            wire_end_info.device = splice.get_name();
            wire_end_info.pin = pin.get_name();
            wire_end_info.termination = "TODO";
            // TODO: Read properties of the device to find out which side of the splice wire is meant to 
        }
    }
    wire_end_info.termination = "TODO";
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

    // match read_file(&"Library.xml") {
    //     Ok(xml) => {
    //         let library = XmlChssystem::from_str(&xml);
    //         match library {
    //             Ok(library) => {
    //                 for devicepart in &library.devicepart {
    //                     println!("{}", devicepart.partnumber.bright_yellow());
    //                 }
    //             }
    //             Err(e) => {

    //             }
    //         }
    //     }
    //     Err(e) => {

    //     }
    // }

    // return;


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
                                match sheet {
                                    Ok(mut sheet) => {
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

                                        sheet.write_string(0,WIRE_NAME, "Wire Name", Some(&format_header)); 
                                        sheet.write_string(0,WIRE_FROM_DEVICE, "Device/Connector", Some(&format_header));
                                        sheet.write_string(0,WIRE_FROM_PIN, "Pin", Some(&format_header));
                                        sheet.write_string(0,WIRE_FROM_TERM, "Termination", Some(&format_header));
                                        sheet.write_string(0,WIRE_TYPE, "Wire", Some(&format_header));
                                        sheet.write_string(0,WIRE_COLOR, "Color", Some(&format_header));
                                        sheet.write_string(0,WIRE_LEN, "Length", Some(&format_header));
                                        sheet.write_string(0,WIRE_TO_TERM, "Termination", Some(&format_header));
                                        sheet.write_string(0,WIRE_TO_DEVICE, "Device/Connector", Some(&format_header));
                                        sheet.write_string(0,WIRE_TO_PIN, "Pin", Some(&format_header));
                                        sheet.set_column(0,9,20.0,None);
                                        let design = project.get_design(args.design.unwrap().as_ref()).unwrap();
                                        let wires = design.get_wires(args.harness.unwrap().as_ref());
                                        let mut row: u32 = 0;

                                        // Graph
                                        let mut graph: Graph<Box<str>, bool, Undirected> = Graph::new_undirected();

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

                                            sheet.write_string(row,WIRE_NAME,wire.get_name(), current_format);

                                            let wire_type:String = wire.get_material().to_owned() + " " + wire.get_spec();
                                            sheet.write_string(row,WIRE_TYPE,&wire_type, current_format);

                                            sheet.write_string(row,WIRE_LEN,&wire.get_length().to_string(), current_format);

                                            sheet.write_string(row,WIRE_COLOR,&wire.get_color(), current_format);

                                            let connections = wire.get_connections();
                                            let connection_left = connections.get(0);

                                            let left_wire_end = connection_left.map(|connection_left| {
                                                process_connection(connection_left)
                                            });

                                            match left_wire_end {
                                                Some(ref left_wire_end) => {
                                                    
                                                    sheet.write_string(row,WIRE_FROM_DEVICE,left_wire_end.device, current_format);
                                                    sheet.write_string(row,WIRE_FROM_PIN,left_wire_end.pin, current_format);
                                                    sheet.write_string(row,WIRE_FROM_TERM,left_wire_end.termination, current_format);
                                                }
                                                None => {
                                                    sheet.write_blank(row,WIRE_FROM_DEVICE,current_format);
                                                    sheet.write_blank(row,WIRE_FROM_PIN,current_format);
                                                    sheet.write_blank(row,WIRE_FROM_TERM,current_format);
                                                }
                                            }

                                            let connection_right = connections.get(1);
                                            let right_wire_end = connection_right.map(|connection_right| {
                                                process_connection(connection_right)
                                            });

                                            match right_wire_end {
                                                Some(ref right_wire_end) => {

                                                    sheet.write_string(row,WIRE_TO_DEVICE,right_wire_end.device, current_format);
                                                    sheet.write_string(row,WIRE_TO_PIN,right_wire_end.pin, current_format);
                                                    sheet.write_string(row,WIRE_TO_TERM,right_wire_end.termination, current_format);
                                                }
                                                None => {
                                                    sheet.write_blank(row,WIRE_TO_DEVICE,current_format);
                                                    sheet.write_blank(row,WIRE_TO_PIN,current_format);
                                                    sheet.write_blank(row,WIRE_TO_TERM,current_format);
                                                }
                                            }

                                            // Build a graph of devices and connectors
                                            // Match if both connections exist
                                            match (left_wire_end, right_wire_end) {
                                                (Some(ref left), Some(ref right)) => {
                                                    let left_node = find_node_or_add(&mut graph, left.device.into());
                                                    let right_node = find_node_or_add(&mut graph, right.device.into());
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
                                        println!("{:?}", dot);
                                        }


                                        let dot2 = Dot::with_attr_getters(&mst_directed_graph, &[Config::EdgeNoLabel, Config::NodeNoLabel], &|_, edge| {
                                            "".to_string()
                                        },
                                        &|_, (id,name)| {
                                            format!("label=\"{:?}\"", name)
                                        });

                                        println!("{:?}", dot2);

                                    }
                                    Err(e) => {

                                    }
                                }
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
