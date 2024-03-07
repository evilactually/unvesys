
pub use std::borrow::Cow;
pub use hard_xml::{XmlRead, XmlWrite};

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "project")]
pub struct XmlProject<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(child = "designmgr")]
    pub designmgr: XmlDesignMgr<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "designmgr")]
pub struct XmlDesignMgr<'a> {
    #[xml(child = "logicaldesign")]
    pub logicaldesign: Vec<XmlLogicalDesign<'a>>,
    #[xml(child = "harnessdesign")]
    pub harnessdesign: Vec<XmlHarnessDesign<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "logicaldesign")]
pub struct XmlLogicalDesign<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "description")]
    pub description: Option<Cow<'a, str>>,
    #[xml(child = "connectivity")]
    pub connectivity: XmlConnectivity<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "harnessdesign")]
pub struct XmlHarnessDesign<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "description")]
    pub description: Option<Cow<'a, str>>,
    #[xml(child = "harnessdiagram")]
    pub harnessdiagram: XmlHarnessDiagram<'a>,
    #[xml(child = "harnesscontainer")]
    pub harnesscontainer: XmlHarnessContainer<'a>,
}


#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "harnesscontainer")]
pub struct XmlHarnessContainer<'a> {
    #[xml(child = "connectivity")]
    pub connectivity: XmlConnectivity<'a>,
}



// #[derive(XmlWrite, XmlRead, PartialEq, Debug)]
// #[xml(tag = "harnessdesign")]
// pub struct XmlHarnessDesign<'a> {
//     #[xml(child = "harnessdiagram")]
//     pub harnessdiagram: XmlHarnessDiagram<'a>
// }

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "harnessdiagram")]
pub struct XmlHarnessDiagram<'a> {
    #[xml(child = "harnessdiagramcontent")]
    pub harnessdiagramcontent: XmlHarnessDiagramContent<'a>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "harnessdiagramcontent")]
pub struct XmlHarnessDiagramContent<'a> {
    #[xml(attr = "harnessdiagramid")]
    pub harnessdiagramid: Cow<'a, str>,
    #[xml(child = "tablegroup")]
    pub tablegroup: Vec<XmlTableGroup<'a>>
}

/* XmlTableGroup */

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "tablegroup")]
pub struct XmlTableGroup<'a> {
    #[xml(attr = "title")]
    pub title: Cow<'a, str>,
    #[xml(attr = "decorationname")]
    pub decorationname: Cow<'a, str>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "tabledatacache")]
pub struct XmlTableDataCache<'a> {
    #[xml(child = "colhdrnames")]
    pub colhdrnames: XmlColHdrNames<'a>,
    #[xml(child = "datavalues")]
    pub datavalues: XmlDataValues<'a>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "colhdrnames")]
pub struct XmlColHdrNames<'a> {
  #[xml(child = "row")]
  pub row: XmlRow<'a>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "row")]
pub struct XmlRow<'a> {
  #[xml(child = "cellval")]
  pub cellvals: Vec<XmlCellVal<'a>>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "datavalues")]
pub struct XmlDataValues<'a> {
    #[xml(child = "datarow")]
    pub datarow: Vec<XmlDataRow<'a>>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "datarow")]
pub struct XmlDataRow<'a> {
    #[xml(child = "cellval")]
     pub cellval: XmlCellVal<'a>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "cellval")]
pub struct XmlCellVal<'a> {
    #[xml(child = "cval")]
    pub cval: XmlCVal<'a>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "cval")]    
pub struct XmlCVal<'a> {
    #[xml(attr = "val")]
     pub val: Cow<'a, str>
}

/* XmlTableGroup */

// harnessdesign, harnessdiagram, harnessdiagramcontent, tablegroup.title

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "connectivity")]
pub struct XmlConnectivity<'a> {
    #[xml(child = "device")]
    pub device: Vec<XmlDevice<'a>>,
    #[xml(child = "connector")]
    pub connector: Vec<XmlConnector<'a>>,
    #[xml(child = "splice")]
    pub splice: Vec<XmlSplice<'a>>,
    #[xml(child = "wire")]
    pub wire: Vec<XmlWire<'a>>,
    #[xml(child = "multicore")]
    pub multicore: Vec<XmlMulticore<'a>>,
    #[xml(child = "grounddevice")]
    pub grounddevice: Vec<XmlGroundDevice<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "device")]
pub struct XmlDevice<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<Cow<'a, str>>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: Option<Cow<'a, str>>,
    #[xml(attr = "customername")]
    pub customername: Option<Cow<'a, str>>,
    #[xml(attr = "partdesc")]
    pub partdesc: Option<Cow<'a, str>>,
    #[xml(attr = "typecode")]
    pub typecode: Option<Cow<'a, str>>,
    #[xml(attr = "typecodedesc")]
    pub typecodedesc: Option<Cow<'a, str>>,
    #[xml(attr = "colorcode")]
    pub colorcode: Option<Cow<'a, str>>,
    #[xml(attr = "colordesc")]
    pub colordesc: Option<Cow<'a, str>>,
    #[xml(attr = "incbom")]
    pub incbom: Option<Cow<'a, str>>,
    #[xml(attr = "suppliername")]
    pub suppliername: Option<Cow<'a, str>>,
    #[xml(attr = "supplierpartnumber")]
    pub supplierpartnumber: Option<Cow<'a, str>>,
    #[xml(attr = "shortdescription")]
    pub shortdescription: Option<Cow<'a, str>>,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "connector")]
pub struct XmlConnector<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin<'a>>,
    #[xml(attr = "connectorusage")]
    pub connectorusage: Cow<'a, str>,
    #[xml(attr = "partnumber")]
    pub partnumber: Cow<'a, str>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "pin")]
pub struct XmlPin<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "connectedpin")]
    pub connectedpin: Option<Cow<'a, str>>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "splice")]
pub struct XmlSplice<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<Cow<'a, str>>,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "wire")]
pub struct XmlWire<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "harness")]
    pub harness: Option<Cow<'a, str>>,
    #[xml(child = "connection")]
    pub connection: Vec<XmlConnection<'a>>,
    #[xml(attr = "wirelength")]
    pub wirelength: f32,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: Option<Cow<'a, str>>,
    #[xml(attr = "wirespec")]
    pub wirespec: Option<Cow<'a, str>>,
    #[xml(attr = "wirematerial")]
    pub wirematerial: Option<Cow<'a, str>>,
    #[xml(attr = "wirecolor")]
    pub wirecolor: Option<Cow<'a, str>>, 
    #[xml(attr = "startpinref")]
    pub startpinref: Option<Cow<'a, str>>,
    #[xml(attr = "terminalpartspecend1")]
    pub terminalpartspecend1: Option<Cow<'a, str>>,
    #[xml(attr = "terminalpartspecend2")]
    pub terminalpartspecend2: Option<Cow<'a, str>>,
    #[xml(attr = "shortdescription")]
    pub shortdescription: Option<Cow<'a, str>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "multicore")]
pub struct XmlMulticore<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(attr = "sheathtype")]
    pub sheathtype: Cow<'a, str>,
    #[xml(child = "member")]
    pub member: Vec<XmlMember<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "member")]
pub struct XmlMember<'a> {
    #[xml(attr = "ref")]
    pub ref_: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "grounddevice")]
pub struct XmlGroundDevice<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin<'a>>,
}


#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "connection")]
pub struct XmlConnection<'a> {
    #[xml(attr = "pinref")]
    pub pinref: Cow<'a, str>,
}


// Library
// Divice -> Housing -> Terminals -> Single Wire Fits/Multiple Wires Fits