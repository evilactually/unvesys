
pub use std::borrow::Cow;
pub use hard_xml::{XmlRead, XmlWrite};


/// VeSys Library root
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "chssystem")]
pub struct XmlChssystem<'a> {
    #[xml(child = "devicepart")]
    pub devicepart: Vec<XmlDevicePart<'a>>,
}

/// Device entry in a library
#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "devicepart")]
pub struct XmlDevicePart<'a> {
    #[xml(attr = "libraryobject_id")]
    pub libraryobject_id: Cow<'a, str>,
    #[xml(attr = "partnumber")]
    pub partnumber: Cow<'a, str>,
}