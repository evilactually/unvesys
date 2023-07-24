
pub use std::borrow::Cow;
pub use hard_xml::{XmlRead, XmlWrite};


/// VeSys Library root
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "chssystem")]
pub struct XmlChssystem<'a> {
    #[xml(child = "devicepart")]
    pub devicepart: Vec<XmlDevicePart<'a>>,
    #[xml(child = "terminalpart")]
    pub terminalpart: Vec<XmlTerminalPart<'a>>,
    #[xml(child = "librarycolor")]
    pub librarycolor: Vec<XmlLibraryColorCode<'a>>,
    #[xml(child = "chsuserproperty")]
    pub chsuserproperty: Vec<XmlChsUserProperty<'a>>,
}

/// Device entry in a library
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "devicepart")]
pub struct XmlDevicePart<'a> {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: Cow<'a, str>,
    #[xml(attr = "partnumber")]
    pub partnumber: Cow<'a, str>,
    #[xml(child = "customerpartnumber")]
    pub customerpartnumber: Vec<XmlCustomerPartNumber<'a>>,
}

/// Device entry in a library
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "terminalpart")]
pub struct XmlTerminalPart<'a> {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: Cow<'a, str>,
    #[xml(attr = "partnumber")]
    pub partnumber: Cow<'a, str>,
    #[xml(child = "customerpartnumber")]
    pub customerpartnumber: Vec<XmlCustomerPartNumber<'a>>,
    #[xml(child = "chsuserpropertypart")]
    pub chsuserpropertypart: Vec<XmlChsUserPropertyPart<'a>>,
}

/// Customer part number of a component
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "customerpartnumber")]
pub struct XmlCustomerPartNumber<'a> {
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: Cow<'a, str>,
}

/// Property inside a part
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "chsuserpropertypart")]
pub struct XmlChsUserPropertyPart<'a> {
    #[xml(attr = "chsuserproperty_id")]
    pub chsuserproperty_id: Cow<'a, str>,
    #[xml(attr = "userpropertyvalue")]
    pub userpropertyvalue: Cow<'a, str>,
}

/// Color code entry in a library
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "librarycolor")]
pub struct XmlLibraryColorCode<'a> {
    #[xml(attr = "librarycolor_id")]
    pub libraryobject_id: Cow<'a, str>,
    #[xml(attr = "colorcode")]
    pub colorcode: Cow<'a, str>,
    #[xml(attr = "description")]
    pub description: Cow<'a, str>,
}

/// User property in the library
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "chsuserproperty")]
pub struct XmlChsUserProperty<'a> {
    #[xml(attr = "chsuserproperty_id")]
    pub chsuserproperty_id: Cow<'a, str>,
    #[xml(attr = "userpropertyname")]
    pub userpropertyname: Cow<'a, str>,
}


