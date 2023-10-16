
use std::hash::Hasher;
use std::hash::Hash;
use crate::HashSet;

#[derive(Clone)]
pub struct WireList {
    pub wires:HashSet<WireEntry>,
}

#[derive(Clone)]
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