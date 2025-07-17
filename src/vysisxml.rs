/*
 Vesys XML

 Vladislav Shcherbakov
 Copyright Firefly Automatix 2024
 9/18/2024 3:34:10 PM
*/

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
    pub connectivity: XmlConnectivity, // Logic connectivity of design
    #[xml(child = "property")]
    pub properties: Vec<XmlProperty>,
    #[xml(child = "diagram")]
    pub diagram: Vec<XmlDiagram> // All digrams in design
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "property")]
pub struct XmlProperty {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "val")]
    pub val: String,
    #[xml(attr = "type")]
    pub r#type: String,
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
    #[xml(child = "columnstyle")]
    pub columnstyle: Vec<XmlColumnStyle>,
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
#[xml(tag = "columnstyle")]    
pub struct XmlColumnStyle {
    #[xml(attr = "columnname")]
    pub columnname: String,
    #[xml(attr = "displayname")]
    pub displayname: String,
    #[xml(attr = "visibility")]
    pub visibility: String,
    #[xml(attr = "hideempty")]
    pub hideempty: String
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
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<String>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: String,
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
    #[xml(attr = "harness")]
    pub harness: Option<String>,
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
    pub partnumber: Option<String>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: String,
    #[xml(attr = "harness")]
    pub harness: Option<String>,
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
    #[xml(attr = "harness")]
    pub harness: Option<String>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: String,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "wire")]
pub struct XmlWire {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<String>,
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
    #[xml(attr = "harness")]
    pub harness: Option<String>,
    #[xml(attr = "customerpartnumber")]
    pub customerpartnumber: String,
    #[xml(attr = "partnumber")]
    pub partnumber: Option<String>,
}


#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "connection")]
pub struct XmlConnection {
    #[xml(attr = "pinref")]
    pub pinref: String,
}

//
// Diagram XML
//

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "diagram")]
pub struct XmlDiagram {
    #[xml(attr = "name")]
    pub name: String,
    #[xml(child = "diagramcontent")]
    pub diagramcontent : XmlDiagramContent
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "diagramcontent")]
pub struct XmlDiagramContent {
    #[xml(child = "schemconnector")]
    pub schemconnector: Vec<XmlSchemConnector>,
    #[xml(child = "schemdevice")]
    pub schemdevice: Vec<XmlSchemDevice>,
    #[xml(child = "schemsplice")]
    pub schemsplice: Vec<XmlSchemSplice>,
    #[xml(child = "schemgrounddevice")]
    pub schemgrounddevice: Vec<XmlSchemGroundDevice>,
    #[xml(child = "schemwire")]
    pub schemwire: Vec<XmlSchemWire>,
    #[xml(child = "schemshield")]
    pub schemshield: Vec<XmlSchemShield>,
}

// Schematic representation of connector
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "schemconnector")]
pub struct XmlSchemConnector {
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connref")]
    pub connref: String, // reference to object in XmlConnectivity
}

// Schematic representation of device
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "schemdevice")]
pub struct XmlSchemDevice {
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connref")]
    pub connref: String, // reference to object in XmlConnectivity
}

// Schematic representation of splice
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "schemsplice")]
pub struct XmlSchemSplice {
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connref")]
    pub connref: String, // reference to object in XmlConnectivity
}

// Schematic representation of ground device
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "schemgrounddevice")]
pub struct XmlSchemGroundDevice {
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connref")]
    pub connref: String, // reference to object in XmlConnectivity
}

// Schematic representation of wire
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "schemwire")]
pub struct XmlSchemWire {
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connref")]
    pub connref: String, // reference to object in XmlConnectivity
}

// Schematic representation of wire
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "schemshield")]
pub struct XmlSchemShield {
    #[xml(attr = "id")]
    pub id: String,
    #[xml(attr = "connref")]
    pub connref: String, // reference to object in XmlConnectivity
}



// Library
// Divice -> Housing -> Terminals -> Single Wire Fits/Multiple Wires Fits