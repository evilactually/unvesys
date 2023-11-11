use petgraph::dot::{Dot, Config};
use crate::Connection;
//use crate::WireList;
use crate::Project;
use crate::Library;
//use crate::WireEntry;
use xlsxwriter::prelude::FormatColor;
use std::collections::HashMap;
use xlsxwriter::*;
use crate::xlsxtable::*;
use xlsxwriter::format::*;
use xlsxwriter::worksheet::WorksheetRow;
use xlsxwriter::worksheet::WorksheetCol;
use xlsxwriter::worksheet::PaperType;

use crate::wirelist::*;

use crate::traverse::*;

pub struct WireListXlsxFormatter<'a> {
    table : XLSXTable,
    workbook : &'a Workbook,
    sheet : Worksheet<'a>,
    current_row: u32,
    bg_colormap: &'a HashMap<String, FormatColor>
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

    pub fn new<'a>(workbook: &'a xlsxwriter::Workbook, bg_colormap: &'a HashMap<std::string::String, xlsxwriter::format::FormatColor>) -> WireListXlsxFormatter<'a> {
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
            format.set_bg_color(*self.bg_colormap.get(&color_code_upper).unwrap_or(&FormatColor::White));
        });
        // Increment row
        self.current_row = self.current_row + 1;
    }

    pub fn bar(&mut self) {
        self.table.set_region_border_bottom(&XLSXTableRegion {
            first_row: self.current_row - 1,
            first_col: Self::LEFT,
            last_row: self.current_row - 1,
            last_col: Self::LEFT + Self::TO_PIN
        }, FormatBorder::Medium);
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

pub fn color_map() -> Box<HashMap<String, FormatColor>> {
    let mut bg_color_map: HashMap<String, FormatColor> = HashMap::new();
    bg_color_map.insert("PK".to_string(), FormatColor::Custom(0xffccff)); // 5v format
    bg_color_map.insert("RD".to_string(), FormatColor::Custom(0xff9999)); // 12v format
    bg_color_map.insert("BN".to_string(), FormatColor::Custom(0xd3a77b)); // 24v format
    bg_color_map.insert("OR".to_string(), FormatColor::Custom(0xfff2cc)); // 48v format
    bg_color_map.insert("BL".to_string(), FormatColor::Custom(0xddebf7)); // GND format
    bg_color_map.insert("YL".to_string(), FormatColor::Custom(0xffffd1)); // Analog/Bat+ format
    bg_color_map.insert("TN".to_string(), FormatColor::Custom(0xead5c0)); // 24v DO
    bg_color_map.insert("BK".to_string(), FormatColor::Custom(0xf2f2f2)); // 24v DI
    bg_color_map.insert("GN".to_string(), FormatColor::Custom(0xc6e0b4)); // Sinking output
    return Box::new(bg_color_map);
}



pub fn process_connection<'a>(connection: (&'a  Connection<'a>, &'a Option<&'a str>), library: &Library ) -> WireEndEntry {
    let mut wire_end_info : WireEndEntry = Default::default();
    match connection {
        (Connection::Connector(connector,pin), termination) => {
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
                            Connection::GroundDevice(mated_device,mated_pin) => {
                                wire_end_info.device = mated_device.get_name().into();
                                wire_end_info.pin = mated_pin.get_name().into();
                            }
                            _ => {
                                println!("Ring can't connect to device {}", connector.get_name().to_string());
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
                // wire_end_info.device = connector.get_name().into();
                // wire_end_info.pin = pin.get_name().into();
                // wire_end_info.termination = "TODO".into();

                // Same as devices
                wire_end_info.device = connector.get_name().into();
                wire_end_info.pin = pin.get_name().into();
                wire_end_info.termination = "TODO".into();
                if let Some(termination) = termination {
                    let terminal_partnumber = library.lookup_customer_partnumber(*termination);
                    wire_end_info.termination = terminal_partnumber.unwrap_or_default().into();
                    wire_end_info.termination_name = library.lookup_terminal_short_name(*termination).unwrap_or_default().into();
                }
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
        (Connection::GroundDevice(device,pin), termination) => {
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
            wire_end_info.termination = "".into();
            // TODO: Read properties of the device to find out which side of the splice wire is meant to 
        }
    }
    //wire_end_info.termination = "TODO".into();
    wire_end_info
}

pub fn output_cutlist(project: &Project, library: &Library, design_name: &str, harness: &str, filepath: &str ) -> Result<(), Box<dyn std::error::Error>> {
    let colormap = color_map();
    if let Some(design) = project.get_design(design_name) {
        if let Ok(workbook) = Workbook::new(filepath) {
            // Get harness wires            
            let wires = design.get_wires(&harness);

            // Processed wire list
            let mut wire_list: WireList = WireList::new();

            for wire in wires {
                println!("{}", wire.get_name());

                let connections = wire.get_connections();
                let connection_left = connections.get(0);

                // This is where most of VeSys non-sense is fixed regarding where wire is connected and what goes on it
                let left_wire_end = connection_left.map(|(connection_left, termination)| {
                    let mut left_wire_end = process_connection((connection_left, termination), &library);
                    left_wire_end
                });

                let connection_right = connections.get(1);
                let right_wire_end = connection_right.map(|(connection_right, termination)| {
                    let mut right_wire_end = process_connection((connection_right, termination), &library);
                    right_wire_end
                });

                wire_list.wires.insert(
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
            }

            let mut wiregroups = traverse(&wire_list);

            // for group in wiregroups {
            //     println!("{}", "BEGIN GROUP");
            //     for wireentry in group.wires {
            //         println!("  {}", wireentry.name);
            //     }
            //     println!("{}", "END GROUP")
            // }

            // let g = build_graph_from_wirelist(&wire_list);
            // let c = find_weakly_connected_components(&g);

            // println!("{:?}", c);

            // let graphs = build_graphs_from_components(&g,c);

            // for graph in graphs {
            
            //     {
            //     let dot = Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel, Config::NodeNoLabel], &move|_, edge| {
            //         //let is_mst_edge = mst_directed_graph.find_edge(edge.source(), edge.target()).is_some();
            //         if  true {
            //             format!("color=\"{}\"", "red")
            //         } else {
            //             "".to_string()
            //         }
            //     },
            //     &|_, (id,name)| {
            //         format!("label=\"{}\"", name)
            //     });

            //     // Print the DOT representation
            //     println!("{:?}", dot);
            //     }

            // }

            


            let mut xlsx_formatter = WireListXlsxFormatter::new(&workbook, &colormap);
            // Output plain wire list
            xlsx_formatter.print_header();
            // for wire in wire_list.wires.iter() {
            //     xlsx_formatter.print_entry(wire);
            //     //xlsx_formatter.bar();
            // }

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

        } else {
            // can't open path
            // return
        }
    } else {
        // design not found
        // return
    }

    Ok(())
}
