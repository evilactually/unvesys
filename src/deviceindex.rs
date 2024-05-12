use crate::wirelist::WireList;


use std::hash::Hasher;
use std::hash::Hash;
use std::collections::HashSet;



#[derive(Clone)]
pub struct DeviceIndex {
    pub devices:HashSet<DeviceIndexEntry>,
}

#[derive(Clone)]
pub struct DeviceIndexEntry {
    pub name: Box<str>,
 	pub partno: Box<str>,
 	pub desc: Box<str>
}

impl PartialEq for DeviceIndexEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for DeviceIndexEntry {}

impl Hash for DeviceIndexEntry {

fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl DeviceIndex {
	pub fn build_from_wirelist(&self, wirelist: &WireList) {
        for wire_entry in wirelist.wires.iter() {

        }
    }
}