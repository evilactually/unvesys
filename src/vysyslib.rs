/*
 Vesys Library wrapper

 Vladislav Shcherbakov
 Copyright Firefly Automatix 2024
 9/18/2024 3:34:10 PM
*/

use crate::vysis::Component;
use hard_xml::XmlError;

use crate::vysyslibxml::*;

pub struct Library {
    dom: XmlChssystem,
}

impl Library {
    pub fn new(xml:&str) -> Result<Library, XmlError> {
        XmlChssystem::from_str(xml).map(|dom| {
            Library {
                dom : dom,
            }
        })
    }

    pub fn get_color_description(&self, colorcode: &str) -> Option<&str> {
        let colorcode_upper:String = colorcode.to_string().to_uppercase();
        let index = self.dom.librarycolor.iter().position(|color| color.colorcode == colorcode_upper);
        match index {
            Some(index) => {
                Some(&self.dom.librarycolor[index].description)
            }
            None => None
        }
    }

    pub fn lookup_customer_partnumber(&self, partno: &str) -> Option<&str> {
        // check terminals
        let part = self.dom.terminalpart.iter().find(|part| part.partnumber == partno);
        let terminal_partno = part.and_then(|part| {
            part.customerpartnumber.get(0)
        }).and_then(|customerpartnumber| {
            Some(customerpartnumber.customerpartnumber.as_ref())
        });
        if terminal_partno.is_some() { return terminal_partno };

        // check splices
        let part = self.dom.splicepart.iter().find(|part| part.partnumber == partno);
        let splice_partno = part.and_then(|part| {
            part.customerpartnumber.get(0)
        }).and_then(|customerpartnumber| {
            Some(customerpartnumber.customerpartnumber.as_ref())
        });
        if splice_partno.is_some() { return splice_partno };


        // check devices
        let part = self.dom.devicepart.iter().find(|part| part.partnumber == partno);
        let device_partno = part.and_then(|part| {
            part.customerpartnumber.get(0)
        }).and_then(|customerpartnumber| {
            Some(customerpartnumber.customerpartnumber.as_ref())
        });
        device_partno
    }

    pub fn lookup_user_property_id(&self, name: &str) -> Option<&str> {
        let prop = self.dom.chsuserproperty.iter().find(|prop| prop.userpropertyname == name);
        prop.and_then(|prop| {
            Some(prop.chsuserproperty_id.as_ref())
        })
    }

    pub fn lookup_terminal_short_name<'a>(&self, part: Option<&'a XmlTerminalPart>) -> Option<&'a str> {
        //let part = self.dom.terminalpart.iter().find(|part| part.partnumber == partno);
        part.and_then(|part| {
            let property_id = self.lookup_user_property_id("TERMINAL_NAME").unwrap_or("N/A");
            //println!("{}", property_id);
            part.chsuserpropertypart.iter().find(|property| property.chsuserproperty_id == property_id)
        }).and_then(|property| {
            Some(property.userpropertyvalue.as_ref())
            //None
        })
    }

    pub fn lookup_terminal_part(&self, partno: &str) -> Option<&XmlTerminalPart> {
        self.dom.terminalpart.iter().find(|part| part.partnumber == partno)
    }

    pub fn lookup_wire_part(&self, partno: &str) -> Option<&XmlWirePart> {
        self.dom.wirepart.iter().find(|part| part.partnumber == partno)
    }

    pub fn lookup_splice_part(&self, partno: &str) -> Option<&XmlSplicePart> {
        self.dom.splicepart.iter().find(|part| part.partnumber == partno)
    }

    pub fn lookup_device_part(&self, partno: &str) -> Option<&XmlDevicePart> {
        self.dom.devicepart.iter().find(|part| part.partnumber == partno)
    }

    pub fn lookup_connector_part(&self, partno: &str) -> Option<&XmlConnectorPart> {
        self.dom.connectorpart.iter().find(|part| part.partnumber == partno)
    }

    pub fn lookup_grounddevice_part(&self, partno: &str) -> Option<&XmlGroundDevicePart> {
        self.dom.grounddevicepart.iter().find(|part| part.partnumber == partno)
    }

    pub fn lookup_splice_short_name<'a>(&self, part: Option<&'a XmlSplicePart>) -> Option<&'a str> {
        //let part = self.dom.terminalpart.iter().find(|part| part.partnumber == partno);
        part.and_then(|part| {
            let property_id = self.lookup_user_property_id("TERMINAL_NAME").unwrap_or("N/A");
            //println!("{}", property_id);
            part.chsuserpropertypart.iter().find(|property| property.chsuserproperty_id == property_id)
        }).and_then(|property| {
            Some(property.userpropertyvalue.as_ref())
            //None
        })
    }

    pub fn lookup_wire_property(&self, partno: &str, property: &str) -> Option<&str> {
        let part = self.dom.wirepart.iter().find(|part| part.partnumber == partno);
        part.and_then(|part| {
            let property_id = self.lookup_user_property_id(property);
            property_id.and_then(|property_id| {
                part.chsuserpropertypart.iter().find(|property| property.chsuserproperty_id == property_id)
            })
            //println!("{}", property_id);
        }).and_then(|property| {
            Some(property.userpropertyvalue.as_ref())
            //None
        })
    }

    

}
