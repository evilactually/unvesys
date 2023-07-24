
use hard_xml::XmlError;
use std::collections::HashMap;
use crate::vysyslibxml::*;

pub struct Library<'a> {
    dom: XmlChssystem<'a>,
}

impl<'a> Library<'a> {
    pub fn new(xml:&'a str) -> Result<Library, XmlError> {
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
        let part = self.dom.terminalpart.iter().find(|part| part.partnumber == partno);
        let terminal_partno = part.and_then(|part| {
            part.customerpartnumber.get(0)
        }).and_then(|customerpartnumber| {
            Some(customerpartnumber.customerpartnumber.as_ref())
        });
        if terminal_partno.is_some() { return terminal_partno };

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

    pub fn lookup_terminal_short_name(&self, partno: &str) -> Option<&str> {
        let part = self.dom.terminalpart.iter().find(|part| part.partnumber == partno);
        part.and_then(|part| {
            let property_id = self.lookup_user_property_id("TERMINAL_NAME").unwrap();
            //println!("{}", property_id);
            part.chsuserpropertypart.iter().find(|property| property.chsuserproperty_id == property_id)
        }).and_then(|property| {
            Some(property.userpropertyvalue.as_ref())
            //None
        })
    }
}
