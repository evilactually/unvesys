
//mod vysisxml;
use std::rc::Rc;
use std::sync::Arc;
use std::io::Error;
use crate::vysisxml::*;

use std::fs::File;
use std::io::prelude::*;

use std::collections::{HashSet};

pub use hard_xml::{XmlError};

pub struct Project<'a> {
    pub dom: XmlProject<'a>,
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

impl<'a> Project<'a> {

    pub fn new(xml: &'a str) -> Result<Project<'a>, XmlError> {
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
}

pub struct LogicalDesignIter<'a> {
    project:&'a Project<'a>,
    logicaldesign_iter: std::slice::Iter<'a, XmlLogicalDesign<'a>>
}

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
    pub project: &'a Project<'a>,
    pub dom: &'a XmlLogicalDesign<'a>
}

impl<'a> LogicalDesign<'a> {

    pub fn get_name(&'a self) -> &'a str {
        self.dom.name.as_ref()
    }

    pub fn get_connection_by_pinref(&'a self, pinref: &str) -> Option<Connection<'a>> {
         // search connectors
        for connector_dom in &self.dom.connectivity.connector {
            let pin_dom = connector_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let connector = Connector {
                        design : self,
                        dom : connector_dom
                    };
                    let pin = Pin {
                        design : self,
                        dom : pin_dom
                    };
                    return Some(Connection::Connector(connector, pin));    
                }
                None => {}
            }
        }
        // search devices
        for device_dom in &self.dom.connectivity.device {
            let pin_dom = device_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let device = Device {
                        design : self,
                        dom : device_dom
                    };
                    let pin = Pin {
                        design : self,
                        dom : pin_dom
                    };
                    return Some(Connection::Device(device, pin));
                }
                None => {}
            }
        }
        //search splices
        for splice_dom in &self.dom.connectivity.splice {
            let pin_dom = splice_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let device = Splice {
                        design : self,
                        dom : splice_dom
                    };
                    let pin = Pin {
                        design : self,
                        dom : pin_dom
                    };
                    return Some(Connection::Splice(device, pin));
                }
                None => {}
            }
        }

        // ground device
        for grounddevice_dom in &self.dom.connectivity.grounddevice {
            let pin_dom = grounddevice_dom.pin.iter().find(|pin_dom| pin_dom.id == pinref);
            match pin_dom {
                Some(pin_dom) => {
                    let device = GroundDevice {
                        design : self,
                        dom : grounddevice_dom
                    };
                    let pin = Pin {
                        design : self,
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
        let mut wires:Vec<Wire> = Vec::new();
        for wire in &self.dom.connectivity.wire {
            if harness.is_empty() || wire.harness.as_ref().map(|cow_str| cow_str.as_ref() == harness).unwrap_or_default() {
                wires.push(Wire {
                    design : self,
                    dom : wire
                });
            }
        }
        wires
    }

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

pub enum Connection<'a> {
    Device(Device<'a>, Pin<'a>),
    GroundDevice(GroundDevice<'a>, Pin<'a>),
    Connector(Connector<'a>, Pin<'a>),
    Splice(Splice<'a>, Pin<'a>)
}

pub struct Connector<'a> {
    design: &'a LogicalDesign<'a>,
    pub dom: &'a XmlConnector<'a>
}

pub struct Splice<'a> {
    design: &'a LogicalDesign<'a>,
    pub dom: &'a XmlSplice<'a>
}

impl<'a> Splice<'a> {
    pub fn get_name(&self) -> &'a str { // Lifetime of returned string must match dom struct, but not &self reference
        self.dom.name.as_ref()
    }
}

pub enum Connection2<'a> {
    Thing(&'a str)
}

impl<'a> Connector<'a> {
    pub fn get_name(&self) -> &'a str { // Lifetime of returned string must match dom struct, but not &self reference
        self.dom.name.as_ref()
    }

    pub fn get_design_name(&self) -> &'a str { // Lifetime of returned string must match dom struct, but not &self reference
        self.design.dom.name.as_ref()
    }

    pub fn get_ring_connection2(&self) -> &XmlConnector {
        let c = &self.design.dom.connectivity.connector[0];
        //let p = &c.pin[0];
        //Connection::Connector(c,p)
        c
    }

    pub fn get_ring_connection3(&'a self) -> Connector<'a> {
        let c = &self.design.dom.connectivity.connector[0];
        let p = &c.pin[0];
        let conn = Connector {
            design : &self.design,
            dom : &c
        };
        //Connection::Connector(c,p)
        conn
    }


    /// Trace where ring is connected
    pub fn get_ring_connection(&'a self) -> Option<Connection<'a>> {
        self.dom.pin.get(0).as_ref()
        .and_then(|pin| {
            pin.connectedpin.as_ref()
        }).and_then(|connectedpin| {
            self.design.get_connection_by_pinref(connectedpin.as_ref())
        })
    }

    /// Trace where ring is connected
    // pub fn get_ring_connection(&'a self) -> Option<Connection<'a>> {
    //     let pin = &self.dom.pin[0];
    //     self.design.get_connection_by_pinref(pin.connectedpin.clone().unwrap().as_ref())
    // }

    pub fn is_ring(&self) -> bool {
        self.dom.connectorusage.as_ref() == "RingTerminal"
    }


    pub fn get_partno(&'a self) -> &'a str {
        self.dom.partnumber.as_ref()
    }

    pub fn get_customer_partno(&'a self) -> &'a str {
        self.dom.customerpartnumber.as_ref()
    }
}

pub struct Device<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlDevice<'a>
}

impl<'a> Device<'a> {
    pub fn get_name(&self) -> &'a str { // Note that function takes &self NOT &'a self! In &'a str we are declaring the lifetime of returned reference to be different 
        self.dom.name.as_ref()          // from the lifetime of containing struct Device to allow returned reference to outlive the struct.
    }                                   // This is important for passing those references around later on.
}

pub struct GroundDevice<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlGroundDevice<'a>
}

impl<'a> GroundDevice<'a> {
    pub fn get_name(&self) -> &'a str { // Note that function takes &self NOT &'a self! In &'a str we are declaring the lifetime of returned reference to be different 
        self.dom.name.as_ref()          // from the lifetime of containing struct Device to allow returned reference to outlive the struct.
    }                                   // This is important for passing those references around later on.
}

pub struct Wire<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlWire<'a>
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
                    self.dom.terminalpartspecend1.as_ref().map(Cow::as_ref)
                } else {
                    // Option<Cow<'a, str>> -> Option<&Cow<'a, str>> -> Option<&str>
                    self.dom.terminalpartspecend2.as_ref().map(Cow::as_ref)
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
            let connection = self.design.get_connection_by_pinref(connection_dom.pinref.as_ref());
            let wire_end = self.get_wire_end_by_pinref(connection_dom.pinref.as_ref());
            if connection.is_some() {
                connections.push((connection.unwrap(), wire_end))
            }
        }
        return connections;
    }
}

pub struct Pin<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlPin<'a>
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