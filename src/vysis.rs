
//mod vysisxml;
use std::io::Error;
use crate::vysisxml::*;

use std::fs::File;
use std::io::prelude::*;

pub use hard_xml::{XmlError};

pub struct Project<'a> {
    //xml: Box<str>,
    dom: XmlProject<'a>
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

    pub fn new(xml:&'a str) -> Result<Project, XmlError> {
        XmlProject::from_str(xml).map(|dom| {
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

    pub fn get_name(&'a self) -> &str {
        self.dom.name.as_ref()
    }

    pub fn get_design(&'a self, design_name: &str) -> Option<LogicalDesign> {
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
}

pub struct LogicalDesign<'a> {
    project: &'a Project<'a>,
    dom: &'a XmlLogicalDesign<'a>
}

impl<'a> LogicalDesign<'a> {
    pub fn get_connection_by_pinref(&self, pinref: &str) -> Option<Connection> {
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
        return None;
    }

    pub fn get_wires(&self, harness: &str) -> Vec<Wire> {
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
}

pub enum Connection<'a> {
    Device(Device<'a>, Pin<'a>),
    Connector(Connector<'a>, Pin<'a>)
}

pub struct Connector<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlConnector<'a>
}

impl<'a> Connector<'a> {
    pub fn get_name(&self) -> &str {
        self.dom.name.as_ref()
    }
}

pub struct Device<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlDevice<'a>
}

impl Device<'_> {
    pub fn get_name(&self) -> &str {
        self.dom.name.as_ref()
    }
}

pub struct Wire<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlWire<'a>
}

impl Wire<'_> {
    pub fn get_name(&self) -> &str {
        self.dom.name.as_ref()
    }

    pub fn get_length(&self) -> f32 {
        self.dom.wirelength
    }

    pub fn get_spec(&self) -> &str {
        match &self.dom.wirespec {
            Some(wirespec) => {
                wirespec.as_ref()
            }
            None => {
                ""
            }
        }
    }

    pub fn get_material(&self) -> &str {
        match &self.dom.wirematerial {
            Some(wirematerial) => {
                wirematerial.as_ref()
            }
            None => {
                ""
            }
        }
    }

    pub fn get_color(&self) -> &str {
        match &self.dom.wirecolor {
            Some(wirecolor) => {
                wirecolor.as_ref()
            }
            None => {
                ""
            }
        }
    }

    pub fn get_connections(&self) -> Vec<Connection> {
        let mut connections: Vec<Connection> = Vec::new(); 
        for connection in &self.dom.connection {
            let connection = self.design.get_connection_by_pinref(connection.pinref.as_ref());
            if connection.is_some() {
                connections.push(connection.unwrap())
            }
        }
        return connections;
    } 
}

pub struct Pin<'a> {
    design: &'a LogicalDesign<'a>,
    dom: &'a XmlPin<'a>
}

impl Pin<'_> {
    pub fn get_name(&self) -> &str {
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