pub use hard_xml::{XmlRead};

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "project")]
pub struct XmlProject {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(child = "designmgr")]
    pub designmgr: XmlDesignMgr,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "designmgr")]
pub struct XmlDesignMgr {
    #[xml(child = "logicaldesign")]
    pub logicaldesign: Vec<XmlLogicalDesign>,
    #[xml(child = "harnessdesign")]
    pub harnessdesign: Vec<XmlHarnessDesign>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "logicaldesign")]
pub struct XmlLogicalDesign {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "description")]
    pub description: Option<String>,
    #[xml(child = "connectivity")]
    pub connectivity: XmlConnectivity,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "harnessdesign")]
pub struct XmlHarnessDesign {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "description")]
    pub description: Option<String>,
    #[xml(child = "harnessdiagram")]
    pub harnessdiagram: XmlHarnessDiagram,
    #[xml(child = "harnesscontainer")]
    pub harnesscontainer: XmlHarnessContainer,
}


#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "harnesscontainer")]
pub struct XmlHarnessContainer {
    #[xml(child = "connectivity")]
    pub connectivity: XmlConnectivity,
}



// #[derive(XmlRead, PartialEq, Debug)]
// #[xml(tag = "harnessdesign")]
// pub struct XmlHarnessDesign {
//     #[xml(child = "harnessdiagram")]
//     pub harnessdiagram: XmlHarnessDiagram
// }

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "harnessdiagram")]
pub struct XmlHarnessDiagram {
    #[xml(child = "harnessdiagramcontent")]
    pub harnessdiagramcontent: XmlHarnessDiagramContent
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "harnessdiagramcontent")]
pub struct XmlHarnessDiagramContent {
    #[xml(attr = "harnessdiagramid")]
    pub harnessdiagramid: String,
    #[xml(child = "tablegroup")]
    pub tablegroup: Vec<XmlTableGroup>
}

/* XmlTableGroup */

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "tablegroup")]
pub struct XmlTableGroup {
    #[xml(attr = "title")]
    pub title: String,
    #[xml(attr = "decorationname")]
    pub decorationname: String,
    #[xml(child = "tablefamily")]
    pub tablefamily: XmlTableFamily
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "tabledatacache")]
pub struct XmlTableDataCache {
    #[xml(child = "colhdrnames")]
    pub colhdrnames: XmlColHdrNames,
    #[xml(child = "datavalues")]
    pub datavalues: XmlDataValues
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "colhdrnames")]
pub struct XmlColHdrNames {
  #[xml(child = "row")]
  pub row: XmlRow
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "row")]
pub struct XmlRow {
  #[xml(child = "cellval")]
  pub cellvals: Vec<XmlCellVal>
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "datavalues")]
pub struct XmlDataValues {
    #[xml(child = "datarow")]
    pub datarow: Vec<XmlDataRow>
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "datarow")]
pub struct XmlDataRow {
    #[xml(child = "cellval")]
     pub cellval: Vec<XmlCellVal>
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "cellval")]
pub struct XmlCellVal {
    #[xml(child = "cval")]
    pub cval: XmlCVal
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "cval")]    
pub struct XmlCVal {
    #[xml(attr = "val")]
     pub val: String
}

/* XmlTableGroup */

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "tablefamily")]    
pub struct XmlTableFamily {
    #[xml(child = "table")]
    pub table: Vec<XmlTable>
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "table")]    
pub struct XmlTable {
    #[xml(child = "tabledatacache")]
    pub tabledatacache: Option<XmlTableDataCache>
}

// harnessdesign, harnessdiagram, harnessdiagramcontent, tablegroup.title

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "connectivity")]
pub struct XmlConnectivity {
    #[xml(child = "device")]
    pub device: Vec<XmlDevice>,
    #[xml(child = "connector")]
    pub connector: Vec<XmlConnector>,
    #[xml(child = "splice")]
    pub splice: Vec<XmlSplice>,
    #[xml(child = "wire")]
    pub wire: Vec<XmlWire>,
    #[xml(child = "multicore")]
    pub multicore: Vec<XmlMulticore>,
    #[xml(child = "grounddevice")]
    pub grounddevice: Vec<XmlGroundDevice>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "device")]
pub struct XmlDevice {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<String>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: Option<String>,
    #[xml(attr = "customername")]
    pub customername: Option<String>,
    #[xml(attr = "partdesc")]
    pub partdesc: Option<String>,
    #[xml(attr = "typecode")]
    pub typecode: Option<String>,
    #[xml(attr = "typecodedesc")]
    pub typecodedesc: Option<String>,
    #[xml(attr = "colorcode")]
    pub colorcode: Option<String>,
    #[xml(attr = "colordesc")]
    pub colordesc: Option<String>,
    #[xml(attr = "incbom")]
    pub incbom: Option<String>,
    #[xml(attr = "suppliername")]
    pub suppliername: Option<String>,
    #[xml(attr = "supplierpartnumber")]
    pub supplierpartnumber: Option<String>,
    #[xml(attr = "shortdescription")]
    pub shortdescription: Option<String>,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "connector")]
pub struct XmlConnector {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin>,
    #[xml(attr = "connectorusage")]
    pub connectorusage: String,
    #[xml(attr = "partnumber")]
    pub partnumber: String,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: String,
    //TODO: add cavitydetail to get termination assignments
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "pin")]
pub struct XmlPin {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connectedpin")]
    pub connectedpin: Option<String>
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "splice")]
pub struct XmlSplice {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<String>,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "wire")]
pub struct XmlWire {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "harness")]
    pub harness: Option<String>,
    #[xml(child = "connection")]
    pub connection: Vec<XmlConnection>,
    #[xml(attr = "wirelength")] // TODO: add lengthu for harness lengths
    pub wirelength: f32,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: Option<String>,
    #[xml(attr = "wirespec")]
    pub wirespec: Option<String>,
    #[xml(attr = "wirematerial")]
    pub wirematerial: Option<String>,
    #[xml(attr = "wirecolor")]
    pub wirecolor: Option<String>, 
    #[xml(attr = "startpinref")]
    pub startpinref: Option<String>,
    #[xml(attr = "terminalpartspecend1")]
    pub terminalpartspecend1: Option<String>,
    #[xml(attr = "terminalpartspecend2")]
    pub terminalpartspecend2: Option<String>,
    #[xml(attr = "shortdescription")]
    pub shortdescription: Option<String>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "multicore")]
pub struct XmlMulticore {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "sheathtype")]
    pub sheathtype: String,
    #[xml(child = "member")]
    pub member: Vec<XmlMember>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "member")]
pub struct XmlMember {
    #[xml(attr = "ref")]
    pub ref_: String,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "grounddevice")]
pub struct XmlGroundDevice {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(child = "pin")]
    pub pin: Vec<XmlPin>,
}


#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "connection")]
pub struct XmlConnection {
    #[xml(attr = "pinref")]
    pub pinref: String,
}


// Library
// Divice -> Housing -> Terminals -> Single Wire Fits/Multiple Wires Fits