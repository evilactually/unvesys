use polars::prelude::NamedFrom;
use polars::series::Series;
use polars::frame::DataFrame;
use polars::datatypes::Field;
use polars::datatypes::DataType;
use polars::prelude::Schema;
use crate::traverse::traverse;
use crate::vysis::Connectivity;
use crate::vysyslib::Library;
use crate::vysis::Connection;
use std::hash::Hasher;
use std::hash::Hash;
use std::collections::HashSet;
use std::cmp::Ordering::*;

#[derive(Clone)]
pub struct WireList {
    pub wires:HashSet<WireEntry>,
}

#[derive(Clone)]
pub struct WireEntry {
    pub name: Box<str>,
    pub descr: Box<str>,
    pub partno: Box<str>,
    pub material: Box<str>,
    pub spec: Box<str>,
    pub color_code: Box<str>,
    pub color_description: Box<str>,
    pub length: f32,
    pub left: Option<WireEndEntry>,
    pub right: Option<WireEndEntry>,
    pub twisted_with: Option<Box<str>>
}

impl PartialEq for WireEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for WireEntry {}

impl Hash for WireEntry {

fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }

}

impl WireEntry {
    pub fn swap(&mut self) {
        let left_old = self.left.clone();
        self.left = self.right.clone();
        self.right = left_old;
    }
}

#[derive(Default)]
#[derive(Clone)]
pub struct WireEndEntry {
    pub device : Box<str>,
    pub pin : Box<str>,
    pub termination : Box<str>,
    pub termination_name : Box<str>,
    pub strip: f32
}




impl WireList {
    pub fn new() -> WireList {
        WireList {
            wires: HashSet::new()
        }
    }


    // pub fn get_wires_between_devices(&self, from: &str, _to: &str) {
    //     for wire in self.wires.iter() {
    //         match (wire.left, wire.right) {
    //             (Some(left_end), Some(right_end)) => {

    //             }
    //         }
    //     }
    // }

    //remove
    //add

    pub fn get_wires_between_devices(&self, from: &str, to: &str) -> Vec<WireEntry> {
        let mut result = Vec::new();

        for wire_entry in self.wires.iter() {
            match (&wire_entry.left, &wire_entry.right)  {
                (Some(left_end), Some(right_end))  => {
                    // If device ends match, add wire name to the list
                    if ((left_end.device.as_ref() == from) && (right_end.device.as_ref() == to)) ||
                       ((right_end.device.as_ref() == from) && (left_end.device.as_ref() == to)) {
                        result.push(wire_entry.clone());
                    }
                }
                _ => {} // skip if either end is missing
            }
        };
        return result;
    }

}

pub fn sort_wirelist_by_left_device_pin(wirelist: &mut Vec<WireEntry>) {
    wirelist.sort_by(|a,b| {
        match (&a.left, &b.left) {
            (Some(left_end_a), Some(left_end_b)) => {
                let device_cmp = left_end_a.device.cmp(&left_end_b.device);
                let pin_number_a_opt = left_end_a.pin.parse::<i32>();
                let pin_number_b_opt = left_end_b.pin.parse::<i32>();
                // If same device name, compare by pin
                if (device_cmp == Equal) {
                    match (pin_number_a_opt, pin_number_b_opt) {
                        (Ok(pin_num_a), Ok(pin_num_b)) => {
                            pin_num_a.cmp(&pin_num_b)
                        }
                        _ => {
                            // compare alphanumerically
                            //print!("{}{}", left_end_a.pin, )
                            let pin_cmp = left_end_a.pin.cmp(&left_end_b.pin);
                            if (pin_cmp == Equal) {
                                device_cmp // if pins are equal defer to device comparison  
                            } else {
                                pin_cmp
                            }
                        }
                    }
                } else { // one of the pins is not a number
                    device_cmp
                }
            }
            _ => Equal
        }

    });
}

fn process_connection<'a>(connection: (&'a  Connection<'a>, &'a Option<&'a str>), library: &Library ) -> WireEndEntry {
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
                    //println!("termination {}", termination);
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
                if termination.trim() == "^" { // two wire going to same terminal of device
                    wire_end_info.termination = "^".into(); // leave ^ alone for now
                    wire_end_info.termination_name = "".into();
                } else {
                    let terminal_partnumber = library.lookup_customer_partnumber(*termination);
                    wire_end_info.termination = terminal_partnumber.unwrap_or_default().into();
                    wire_end_info.termination_name = library.lookup_terminal_short_name(*termination).unwrap_or_default().into();
                }
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
            // For splices, use splice part number and short name instead of termination
            if let Some(library_partnumber) = splice.get_partno() {
                let customer_partnumber = library.lookup_customer_partnumber(library_partnumber);
                wire_end_info.termination = customer_partnumber.unwrap_or(library_partnumber).into();
                wire_end_info.termination_name = library.lookup_terminal_short_name(library_partnumber).unwrap_or_default().into();
            }
            // TODO: Read properties of the device to find out which side of the splice wire is meant to 
        }
    }
    //wire_end_info.termination = "TODO".into();
    wire_end_info
}


pub fn generate_grouped_wirelist(library: &Library, connectivity: &Connectivity, harness: &str) -> Result<Vec<Vec<WireEntry>>, Box<dyn std::error::Error>> {
    // Get harness wires            
    let wires = connectivity.get_wires(&harness);

    // Processed wire list
    let mut wire_list: WireList = WireList::new();

    for wire in wires {
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
                descr : wire.get_short_descr().into(),
                partno : wire.get_customer_partno().into(),
                material : wire.get_material().into(),
                spec : wire.get_spec().into(),
                color_code : wire.get_color().into(),
                color_description : library.get_color_description(wire.get_color()).unwrap_or_default().into(),
                length : wire.get_length(),
                left : left_wire_end.clone(),
                right : right_wire_end.clone(),
                twisted_with : wire.get_twisted_with().map(|x| x.into()) // check if wire is in twist with any other
            }
        );
    }

    let mut wiregroups : Vec<Vec<WireEntry>> = traverse(&wire_list);

    Ok(wiregroups)
}

// columns(["WIRE_NAME",
//                                        "WIRE_FROM_PINLIST", 
//                                        "WIRE_FROM_CAVITY", 
//                                        "WIRE_TERMINAL_STRIP_LEN1", 
//                                        "WIRE_TO_PINLIST", 
//                                        "WIRE_TO_CAVITY", 
//                                        "WIRE_TERMINAL_STRIP_LEN2", 
//                                        "MODIFIED_LENGTH",
//                                        "PROCESSING"

pub fn grouped_wirelist_to_data_frame(grouped_wirelist: Vec<Vec<WireEntry>>) -> DataFrame {

    let column_names = [
     "WIRE_NAME",
     "SHORT_DESCRIPTION", // TODO: VERIFY THIS!
     "CUSTOMER_PART_NUMBER", // TODO: VERIFY THIS!
     "MATERIAL", // TODO: VERIFY THIS!
     "SPEC", // TODO: VERIFY THIS!
     "COLOR", //TODO: VERIFY THIS!
     "COLOR_DESCRIPTION", // TODO: VERIFY THIS!
     "WIRE_FROM_PINLIST", 
     "WIRE_FROM_CAVITY", 
     "WIRE_TERMINAL_STRIP_LEN1", 
     "WIRE_TO_PINLIST", 
     "WIRE_TO_CAVITY", 
     "WIRE_TERMINAL_STRIP_LEN2", 
     "MODIFIED_LENGTH",
     "TWIST_WIDTH",
     "PROCESSING"]; // CUSTOM FIELD

    let fields = column_names.map(|k| Field::new(k, DataType::String));

    let sc: Schema = Schema::from_iter(fields);
    let mut df = DataFrame::empty_with_schema(&sc);

    for group in  grouped_wirelist.iter() {
        for wire in group.iter() {
            let wire = wire.clone();
            let left_end = wire.left.unwrap_or_default();
            let right_end = wire.right.unwrap_or_default();
            let column_values = [wire.name.as_ref(), 
            wire.descr.as_ref(),
            wire.partno.as_ref(), 
            wire.material.as_ref(), 
            wire.spec.as_ref(), 
            wire.color_code.as_ref(), 
            wire.color_description.as_ref(), 
            left_end.device.as_ref(), 
            left_end.pin.as_ref(), 
            &left_end.strip.to_string(), 
            right_end.device.as_ref(), 
            right_end.pin.as_ref(), 
            &right_end.strip.to_string(), 
            &wire.length.to_string(),
            &wire.twisted_with.unwrap_or_default(),
            ""];
            //println!("{:?}", column_values);
            let series = column_names.iter().zip(column_values).map(|(name, value)| Series::new(name, &[value]) ).collect::<Vec<_>>();
            println!("{:?}", &(DataFrame::new(series.clone()).unwrap()));
            let _ = df.vstack_mut(&(DataFrame::new(series).unwrap()));
        }
        // let series: Vec<_> = table_reader.column_map.keys().map(|k| Series::new(k, &[row.get_column(k).unwrap_or("N/A")])).collect();
    }
    println!("{:?}", df);
    df
}
/*
pub struct WireEntry {
    pub name: Box<str>,
    pub descr: Box<str>,
    pub partno: Box<str>,
    pub material: Box<str>,
    pub spec: Box<str>,
    pub color_code: Box<str>,
    pub color_description: Box<str>,
    pub length: f32,
    pub left: Option<WireEndEntry>,
    pub right: Option<WireEndEntry>,
    pub twisted_with: Option<Box<str>>
}


impl From<VysysTableReader<'_>> for DataFrame {
    fn from(table_reader: VysysTableReader<'_>) -> Self {
        let fields = table_reader.column_map.keys().map(|k| Field::new(k, DataType::String));

        let sc: Schema = Schema::from_iter(fields);
        let mut df = DataFrame::empty_with_schema(&sc);

        let row_iter = table_reader.get_row_iter();
        for row in row_iter {
            let series: Vec<_> = table_reader.column_map.keys().map(|k| Series::new(k, &[row.get_column(k).unwrap_or("N/A")])).collect();
            df.vstack_mut(&(DataFrame::new(series).unwrap()));
        }
        df
    }
}
*/