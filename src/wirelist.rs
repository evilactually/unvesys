
use std::hash::Hasher;
use std::hash::Hash;
use crate::HashSet;
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
    pub termination_name : Box<str>
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
                            let pin_cmp = left_end_a.pin.cmp(&left_end_a.pin);
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