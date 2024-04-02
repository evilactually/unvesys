
//mod vysisxml;
use std::rc::Rc;
use std::sync::Arc;
use std::io::Error;
use crate::vysisxml::*;

use std::fs::File;
use std::io::prelude::*;

use std::collections::{HashSet};

pub use hard_xml::{XmlError};

pub struct Project {
    pub dom: XmlProject,
}

fn read_file(filename:&str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// #[derive(Debug)]
// enum ProjectError {
//     FileError(Error),
//     ParseError(XmlError),
//     Dud
// }

impl<'a> Project {

    pub fn new(xml: &'a str) -> Result<Project, XmlError> {
        XmlProject::from_str(&xml).map(|dom| {
            Project {
                dom : dom
            }
        })
    }

    // pub fn open(filename: &str) -> Result<Project<'static>, Error> {
    //     read_file(&filename).map(|xml| {
    //         let project = Project {
    //             xml : xml.into_boxed_str(),
    //             dom : None
    //         };
    //         project
    //     })
    // }

    // pub fn parse(&'a mut self) -> Result<(), XmlError> {
    //     XmlProject::from_str(self.xml.as_ref()).map(|dom| {
    //         self.dom = Some(dom); 
    //         ()
    //     })
    // }

    // pub fn parse_2(&'a mut self) {
        
    // }

    pub fn get_name(&'a self) -> &'a str {
        self.dom.name.as_ref()
    }

    pub fn get_design(&'a self, design_name: &str) -> Option<LogicalDesign<'a>> {
        let index = self.dom.designmgr.logicaldesign.iter().position(|design| design.name == design_name);
        match index {
            Some(index) => {
                Some(LogicalDesign {
                    project: self,
                    dom: &self.dom.designmgr.logicaldesign[index]
                })
            }
            None => None
        }
    }

    pub fn get_harness_design(&'a self, harness_design_name: &str) -> Option<HarnessDesign<'a>> {
        self.get_harness_design_iter().find(|harnessdesign| {
            harnessdesign.dom.name == harness_design_name
        })
    }

    pub fn get_logical_design_names(&'a self) -> Vec<&'a str> {
        let mut names = Vec::new();
        for logicaldesign in &self.dom.designmgr.logicaldesign {
             names.push(logicaldesign.name.as_ref());
        }
        names
    }

    pub fn get_harness_design_names(&'a self) -> Vec<&'a str> {
        let mut names = Vec::new();
        for harnessdesign in &self.dom.designmgr.harnessdesign {
             names.push(harnessdesign.name.as_ref());
        }
        names
    }

    pub fn get_logical_design_iter(&self) -> LogicalDesignIter<'_> {
        LogicalDesignIter { project: self, logicaldesign_iter: self.dom.designmgr.logicaldesign.iter() }
    }

    pub fn get_harness_design_iter(&self) -> HarnessDesignIter<'_> {
        HarnessDesignIter { project: self, harnessdesign_iter: self.dom.designmgr.harnessdesign.iter() }
    }
}

pub struct LogicalDesignIter<'a> {
    project:&'a Project,
    logicaldesign_iter: std::slice::Iter<'a, XmlLogicalDesign>
}


// Iterator that converts XmlDesign into LogicalDesign on the fly
impl<'a> Iterator for LogicalDesignIter<'a> {
    type Item = LogicalDesign<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.logicaldesign_iter.next().map(|logicaldesignxml| {
            LogicalDesign {
                project: self.project,
                dom: logicaldesignxml
            }
        })
    }
}

pub struct LogicalDesign<'a> {
    pub project: &'a Project,
    pub dom: &'a XmlLogicalDesign
}

impl<'a> LogicalDesign<'a> {

    pub fn get_name(&'a self) -> &'a str {
        self.dom.name.as_ref()
    }

    pub fn get_connectivity(&'a self) -> Connectivity {
        Connectivity { dom : &self.dom.connectivity}
    }

    // pub fn get_connection_by_pinref(&'a self, pinref: &str) -> Option<Connection<'a>> {
    //      // search connectors
    //     for connector_dom in &self.dom.connectivity.connector {
    //         let pin_dom = connector_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
    //         match pin_dom {
    //             Some(pin_dom) => {
    //                 let connector = Connector {
    //                     connectivity : self.get_connectivity(),
    //                     dom : connector_dom
    //                 };
    //                 let pin = Pin {
    //                     connectivity : self.get_connectivity(),
    //                     dom : pin_dom
    //                 };
    //                 return Some(Connection::Connector(connector, pin));    
    //             }
    //             None => {}
    //         }
    //     }
    //     // search devices
    //     for device_dom in &self.dom.connectivity.device {
    //         let pin_dom = device_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
    //         match pin_dom {
    //             Some(pin_dom) => {
    //                 let device = Device {
    //                     design : self,
    //                     dom : device_dom
    //                 };
    //                 let pin = Pin {
    //                     design : self,
    //                     dom : pin_dom
    //                 };
    //                 return Some(Connection::Device(device, pin));
    //             }
    //             None => {}
    //         }
    //     }
    //     //search splices
    //     for splice_dom in &self.dom.connectivity.splice {
    //         let pin_dom = splice_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
    //         match pin_dom {
    //             Some(pin_dom) => {
    //                 let device = Splice {
    //                     design : self,
    //                     dom : splice_dom
    //                 };
    //                 let pin = Pin {
    //                     design : self,
    //                     dom : pin_dom
    //                 };
    //                 return Some(Connection::Splice(device, pin));
    //             }
    //             None => {}
    //         }
    //     }

    //     // ground device
    //     for grounddevice_dom in &self.dom.connectivity.grounddevice {
    //         let pin_dom = grounddevice_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
    //         match pin_dom {
    //             Some(pin_dom) => {
    //                 let device = GroundDevice {
    //                     design : self,
    //                     dom : grounddevice_dom
    //                 };
    //                 let pin = Pin {
    //                     design : self,
    //                     dom : pin_dom
    //                 };
    //                 return Some(Connection::GroundDevice(device, pin));
    //             }
    //             None => {}
    //         }
    //     }

    //     // TODO: splices, ground devices, other?
    //     return None;
    // }

    // pub fn get_wires(&'a self, harness: &str) -> Vec<Wire<'a>> {
    //     self.get_wire_iter().filter(|wire| {
    //         harness.is_empty() || wire.dom.harness.as_ref().map(|cow_str| cow_str.as_ref() == harness).unwrap_or_default()
    //     }).collect()
    // }
    
    // pub fn get_connector_wires(&'a self, connector_name: &str) -> Vec<Wire<'a>> {
    //     self.get_wire_iter().filter(|wire| {
    //         wire.get_connections().iter().map(|(x,_)| x).map(|connection| {
    //             match connection {
    //                 Connection::Connector(connector,_)  => {
    //                     connector.get_name() == connector_name
    //                 }
    //                 _ => false
    //             };
    //             true
    //         } );
    //         //harness.is_empty() || wire.dom.harness.as_ref().map(|cow_str| cow_str.as_ref() == harness).unwrap_or_default()
    //         false
    //     }).collect()
    // }


    // pub fn get_wire_iter(&self) -> WireIter<'_> {
    //     WireIter { design: self, wire_iter: self.dom.connectivity.wire.iter() }
    // }

    /// Returns a list of unique harness attributes in logical design
    pub fn get_harness_names(&self) -> Vec<&'a str> {
        let dom = &self.project.dom;
        let design_name = self.get_name();
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
                
                let mut harness_vec: Vec<_> = harness_set.into_iter().collect();

                harness_vec.sort_by(|a,b| {
                    a.cmp(&b)
                });
                return harness_vec; 
            }
            None => {
                return Vec::new();
            }
        }
    }
}

pub struct Connectivity<'a> {
    pub dom: &'a XmlConnectivity
}

impl<'a> Connectivity<'a> {

    pub fn get_device_by_name(&'a self, name: &str) -> Option<Device<'a>> {
        // search devices
        if let Some(device_dom) = self.dom.device.iter().find(|device_dom| device_dom.name == name) {
            Some(Device {
               connectivity: self,
               dom: device_dom
           })
        } else {
            None
        }
    }

    pub fn get_connector_by_name(&'a self, name: &str) -> Option<Connector<'a>> {
        // search devices
        if let Some(connector_dom) = self.dom.connector.iter().find(|connector_dom| connector_dom.name == name) {
            Some(Connector {
               connectivity: self,
               dom: connector_dom
           })
        } else {
            None
        }
    }

    pub fn get_splice_by_name(&'a self, name: &str) -> Option<Splice<'a>> {
        // search devices
        if let Some(splice_dom) = self.dom.splice.iter().find(|splice_dom| splice_dom.name == name) {
            Some(Splice {
               connectivity: self,
               dom: splice_dom
           })
        } else {
            None
        }
    }

    pub fn get_grounddevice_by_name(&'a self, name: &str) -> Option<GroundDevice<'a>> {
        // search devices
        if let Some(gnd_dom) = self.dom.grounddevice.iter().find(|gnd_dom| gnd_dom.name == name) {
            Some(GroundDevice {
               connectivity: self,
               dom: gnd_dom
           })
        } else {
            None
        }
    }

    pub fn get_connection_by_pinref(&'a self, pinref: &str) -> Option<Connection<'a>> {
         // search connectors
        for connector_dom in &self.dom.connector {
            let pin_dom = connector_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let connector = Connector {
                        connectivity : self,
                        dom : connector_dom
                    };
                    let pin = Pin {
                        connectivity : self,
                        dom : pin_dom
                    };
                    return Some(Connection::Connector(connector, pin));    
                }
                None => {}
            }
        }
        // search devices
        for device_dom in &self.dom.device {
            let pin_dom = device_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let device = Device {
                        connectivity : self,
                        dom : device_dom
                    };
                    let pin = Pin {
                        connectivity : self,
                        dom : pin_dom
                    };
                    return Some(Connection::Device(device, pin));
                }
                None => {}
            }
        }
        //search splices
        for splice_dom in &self.dom.splice {
            let pin_dom = splice_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let device = Splice {
                        connectivity : self,
                        dom : splice_dom
                    };
                    let pin = Pin {
                        connectivity : self,
                        dom : pin_dom
                    };
                    return Some(Connection::Splice(device, pin));
                }
                None => {}
            }
        }

        // ground device
        for grounddevice_dom in &self.dom.grounddevice {
            let pin_dom = grounddevice_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let device = GroundDevice {
                        connectivity : self,
                        dom : grounddevice_dom
                    };
                    let pin = Pin {
                        connectivity : self,
                        dom : pin_dom
                    };
                    return Some(Connection::GroundDevice(device, pin));
                }
                None => {}
            }
        }

        // TODO: splices, ground devices, other?
        return None;
    }

    pub fn get_wires(&'a self, harness: &str) -> Vec<Wire<'a>> {
        self.get_wire_iter().filter(|wire| {
            harness.is_empty() || wire.dom.harness.as_ref().map(|cow_str| cow_str == harness).unwrap_or_default()
        }).collect()
    }
    
    pub fn get_connector_wires(&'a self, connector_name: &str) -> Vec<Wire<'a>> {
        self.get_wire_iter().filter(|wire| {
            wire.get_connections().iter().map(|(x,_)| x).map(|connection| {
                match connection {
                    Connection::Connector(connector,_)  => {
                        connector.get_name() == connector_name
                    }
                    _ => false
                };
                true
            } );
            //harness.is_empty() || wire.dom.harness.as_ref().map(|cow_str| cow_str.as_ref() == harness).unwrap_or_default()
            false
        }).collect()
    }


    pub fn get_wire_iter(&self) -> WireIter<'_> {
        WireIter { connectivity: self, wire_iter: self.dom.wire.iter() }
    }
}

pub struct WireIter<'a> {
    connectivity:&'a Connectivity<'a>,
    wire_iter: std::slice::Iter<'a, XmlWire>
}

// Iterator that converts XmlDesign into LogicalDesign on the fly
impl<'a> Iterator for WireIter<'a> {
    type Item = Wire<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.wire_iter.next().map(|wirexml| {
            Wire {
                connectivity: self.connectivity,
                dom: wirexml
            }
        })
    }
}

pub enum Connection<'a> {
    Device(Device<'a>, Pin<'a>),
    GroundDevice(GroundDevice<'a>, Pin<'a>),
    Connector(Connector<'a>, Pin<'a>),
    Splice(Splice<'a>, Pin<'a>)
}

pub struct Connector<'a> {
    connectivity: &'a Connectivity<'a>,
    pub dom: &'a XmlConnector
}

pub struct Splice<'a> {
    connectivity: &'a Connectivity<'a>,
    pub dom: &'a XmlSplice
}

impl<'a> Splice<'a> {
    pub fn get_name(&self) -> &'a str { // Lifetime of returned string must match dom struct, but not &self reference
        self.dom.name.as_ref()
    }

    pub fn get_partno(&'a self) -> Option<&'a str> {
        self.dom.partnumber.as_ref().map(|x| x.as_ref())
    }

}

impl<'a> Connector<'a> {
    pub fn get_name(&self) -> &'a str { // Lifetime of returned string must match dom struct, but not &self reference
        self.dom.name.as_ref()
    }

    // pub fn get_design_name(&self) -> &'a str { // Lifetime of returned string must match dom struct, but not &self reference
    //     self.design.dom.name.as_ref()
    // }

    // pub fn get_ring_connection2(&self) -> &XmlConnector {
    //     let c = &self.design.dom.connectivity.connector[0];
    //     //let p = &c.pin[0];
    //     //Connection::Connector(c,p)
    //     c
    // }

    // pub fn get_ring_connection3(&'a self) -> Connector<'a> {
    //     let c = &self.design.dom.connectivity.connector[0];
    //     let p = &c.pin[0];
    //     let conn = Connector {
    //         design : &self.design,
    //         dom : &c
    //     };
    //     //Connection::Connector(c,p)
    //     conn
    // }


    /// Trace where ring is connected
    pub fn get_ring_connection(&'a self) -> Option<Connection<'a>> {
        self.dom.pin.get(0).as_ref()
        .and_then(|pin| {
            pin.connectedpin.as_ref()
        }).and_then(|connectedpin| {
            self.connectivity.get_connection_by_pinref(connectedpin.as_ref())
        })
    }

    /// Trace where ring is connected
    // pub fn get_ring_connection(&'a self) -> Option<Connection<'a>> {
    //     let pin = &self.dom.pin[0];
    //     self.design.get_connection_by_pinref(pin.connectedpin.clone().unwrap().as_ref())
    // }

    pub fn is_ring(&self) -> bool {
        self.dom.connectorusage == "RingTerminal"
    }


    pub fn get_partno(&'a self) -> &'a str {
        self.dom.partnumber.as_ref()
    }

    pub fn get_customer_partno(&'a self) -> &'a str {
        self.dom.customerpartnumber.as_ref()
    }
}

pub struct Device<'a> {
    connectivity: &'a Connectivity<'a>,
    dom: &'a XmlDevice
}

impl<'a> Device<'a> {
    pub fn get_name(&self) -> &'a str { // Note that function takes &self NOT &'a self! In &'a str we are declaring the lifetime of returned reference to be different 
        self.dom.name.as_ref()          // from the lifetime of containing struct Device to allow returned reference to outlive the struct.
    }                                   // This is important for passing those references around later on.
}

pub struct GroundDevice<'a> {
    connectivity: &'a Connectivity<'a>,
    dom: &'a XmlGroundDevice
}

impl<'a> GroundDevice<'a> {
    pub fn get_name(&self) -> &'a str { // Note that function takes &self NOT &'a self! In &'a str we are declaring the lifetime of returned reference to be different 
        self.dom.name.as_ref()          // from the lifetime of containing struct Device to allow returned reference to outlive the struct.
    }                                   // This is important for passing those references around later on.
}

pub struct Wire<'a> {
    connectivity: &'a Connectivity<'a>,
    dom: &'a XmlWire
}

impl<'a> Wire<'a> {
    pub fn get_name(&self) -> &'a str {
        self.dom.name.as_ref()
    }

    pub fn get_short_descr(&self) -> &'a str {
        match &self.dom.shortdescription {
            Some(shortdescription) => {
                shortdescription.as_ref()
            }
            None => {
                ""
            }
        }
    }


    pub fn get_length(&self) -> f32 {
        self.dom.wirelength
    }

    pub fn get_spec(&self) -> &'a str {
        match &self.dom.wirespec {
            Some(wirespec) => {
                wirespec.as_ref()
            }
            None => {
                ""
            }
        }
    }

    pub fn get_customer_partno(&self) -> &'a str {
        match &self.dom.customerpartnumber {
            Some(customerpartnumber) => {
                customerpartnumber.as_ref()
            }
            None => {
                ""
            }
        }
    }

    pub fn get_material(&self) -> &'a str {
        match &self.dom.wirematerial {
            Some(wirematerial) => {
                wirematerial.as_ref()
            }
            None => {
                ""
            }
        }
    }

    pub fn get_color(&self) -> &'a str {
        match &self.dom.wirecolor {
            Some(wirecolor) => {
                wirecolor.as_ref()
            }
            None => {
                ""
            }
        }
    }

    /// Look-up wire terminal end associated with wire connection pinref
    pub fn get_wire_end_by_pinref(&self, pinref: &str) -> Option<&str> {
        // If wire has terminal ends it must have startpinref marking first connection
        match &self.dom.startpinref {
            Some(startpinref) => {
                // Return first wire end
                if pinref == startpinref {
                    // Option<Cow<'a, str>> -> Option<&Cow<'a, str>> -> Option<&str>
                    self.dom.terminalpartspecend1.as_deref()
                } else {
                    // Option<Cow<'a, str>> -> Option<&Cow<'a, str>> -> Option<&str>
                    self.dom.terminalpartspecend2.as_deref()
                }
            }
            // If there's no startpinref than there's nothing I can do to help you determine wire end
            None => {None}
        }
    } 

    /// Get wire connections and associated wire terminal ends
    pub fn get_connections(&self) -> Vec<(Connection, Option<&str>)> {
        let mut connections: Vec<(Connection, Option<&str>)> = Vec::new(); 
        for connection_dom in &self.dom.connection {
            let connection = self.connectivity.get_connection_by_pinref(connection_dom.pinref.as_ref());
            let wire_end = self.get_wire_end_by_pinref(connection_dom.pinref.as_ref());
            if connection.is_some() {
                connections.push((connection.unwrap(), wire_end))
            }
        }
        return connections;
    }

    pub fn get_twisted_with(&self) -> Option<&'a str> {
        let twisted_pair = self.connectivity.dom.multicore.iter().find(|x| {
            if x.sheathtype == "Twisted" {
                // Find multicore that contains this wire id
                x.member.iter().find(|y| y.ref_ == self.dom.id).is_some()
            } else { false }
        });

        if let Some(twisted_pair) = twisted_pair {
            // for member in twisted_pair.member.iter() {
            //     println!("{}", member.ref_);
            // }
            // Filter out current wire, leave other(s?)
            let member_dom = twisted_pair.member.iter().filter(|x| x.ref_ != self.dom.id).next();
            if let Some(member_dom) = member_dom {
                let wire_dom = self.connectivity.dom.wire.iter().find(|x| x.id == member_dom.ref_);
                if let Some(wire_dom) = wire_dom {
                    return Some(wire_dom.name.as_ref());
                }
            }
        }

        None
    }
}

pub struct Pin<'a> {
    connectivity: &'a Connectivity<'a>,
    dom: &'a XmlPin
}

impl<'a> Pin<'a> {
    pub fn get_name(&self) -> &'a str {
        self.dom.name.as_ref()
    }
}

struct Test<'a> {
    box_str:Box<str>,
    test:Test2<'a>
}


struct Test2<'a> {
    test:&'a str
}

fn mkTest2(s:&str) -> Test2 {
    Test2 {
        test:s
    }
}

struct Test3 {
    a:u32
}


// fn test() -> io::Result<Test<'static>> {
//     let file_contents = fs::read_to_string("path/to/file.txt")?;
//     let t2 = mkTest2(&file_contents);
//     Ok(Test { test: t2 })
// }

pub struct HarnessDesignIter<'a> {
    project:&'a Project,
    harnessdesign_iter: std::slice::Iter<'a, XmlHarnessDesign>
}

impl<'a> Iterator for HarnessDesignIter<'a> {
    type Item = HarnessDesign<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.harnessdesign_iter.next().map(|harnessdesignxml| {
            HarnessDesign {
                project: self.project,
                dom: harnessdesignxml
            }
        })
    }
}

pub struct HarnessDesign<'a> {
    pub project: &'a Project,
    pub dom: &'a XmlHarnessDesign
}

impl<'a> HarnessDesign<'a> {
    pub fn get_name(&'a self) -> &'a str {
        self.dom.name.as_ref()
    }

    pub fn get_connectivity(&'a self) -> Connectivity<'a>{
        Connectivity { dom : &self.dom.harnesscontainer.connectivity}
    }

    pub fn get_bom_table(&'a self) ->  Option<&'a XmlTableGroup> {
        return self.dom.harnessdiagram.harnessdiagramcontent.tablegroup.iter().find(|tablegroup| {
            tablegroup.decorationname == "BOM Table"
        });

    }
}


