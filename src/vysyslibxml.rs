pub use hard_xml::{XmlRead, XmlWrite};

/// VeSys Library root
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "chssystem")]
pub struct XmlChssystem {
    #[xml(child = "devicepart")]
    pub devicepart: Vec<XmlDevicePart>,
    #[xml(child = "terminalpart")]
    pub terminalpart: Vec<XmlTerminalPart>,
    #[xml(child = "wirepart")]
    pub wirepart: Vec<XmlWirePart>,
    #[xml(child = "splicepart")]
    pub splicepart: Vec<XmlSplicePart>,
    #[xml(child = "librarycolor")]
    pub librarycolor: Vec<XmlLibraryColorCode>,
    #[xml(child = "chsuserproperty")]
    pub chsuserproperty: Vec<XmlChsUserProperty>,
}

/// Device entry in a library
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "devicepart")]
pub struct XmlDevicePart {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: String,
    #[xml(attr = "partnumber")]
    pub partnumber: String,
    #[xml(child = "customerpartnumber")]
    pub customerpartnumber: Vec<XmlCustomerPartNumber>,
}

/// Splice entry in a library
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "splicepart")]
pub struct XmlSplicePart {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: String,
    #[xml(attr = "partnumber")]
    pub partnumber: String,
    #[xml(child = "customerpartnumber")]
    pub customerpartnumber: Vec<XmlCustomerPartNumber>,
    #[xml(child = "chsuserpropertypart")]
    pub chsuserpropertypart: Vec<XmlChsUserPropertyPart>,
    #[xml(attr = "striplength")]
    pub striplength: f32,
}

/// Device entry in a library
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "terminalpart")]
pub struct XmlTerminalPart {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: String,
    #[xml(attr = "partnumber")]
    pub partnumber: String,
    #[xml(child = "customerpartnumber")]
    pub customerpartnumber: Vec<XmlCustomerPartNumber>,
    #[xml(child = "chsuserpropertypart")]
    pub chsuserpropertypart: Vec<XmlChsUserPropertyPart>,
    #[xml(attr = "striplength")]
    pub striplength: f32,
}

/// Customer part number of a component
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "customerpartnumber")]
pub struct XmlCustomerPartNumber {
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: String,
}

/// Property inside a part
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "chsuserpropertypart")]
pub struct XmlChsUserPropertyPart {
    #[xml(attr = "chsuserproperty_id")]
    pub chsuserproperty_id: String,
    #[xml(attr = "userpropertyvalue")]
    pub userpropertyvalue: String,
}

/// Color code entry in a library
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "librarycolor")]
pub struct XmlLibraryColorCode {
    #[xml(attr = "librarycolor_id")]
    pub libraryobject_id: String,
    #[xml(attr = "colorcode")]
    pub colorcode: String,
    #[xml(attr = "description")]
    pub description: String,
}

/// User property in the library
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "chsuserproperty")]
pub struct XmlChsUserProperty {
    #[xml(attr = "chsuserproperty_id")]
    pub chsuserproperty_id: String,
    #[xml(attr = "userpropertyname")]
    pub userpropertyname: String,
}

/// Wire entry in a library
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "wirepart")]
pub struct XmlWirePart {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: String,
    #[xml(attr = "partnumber")]
    pub partnumber: String,
    #[xml(child = "customerpartnumber")]
    pub customerpartnumber: Vec<XmlCustomerPartNumber>,
    #[xml(child = "chsuserpropertypart")]
    pub chsuserpropertypart: Vec<XmlChsUserPropertyPart>,
}


