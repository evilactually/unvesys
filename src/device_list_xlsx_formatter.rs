use xlsxwriter::*;
use crate::xlsxtable::*;
use xlsxwriter::format::*;
use xlsxwriter::worksheet::WorksheetRow;
use xlsxwriter::worksheet::WorksheetCol;
use xlsxwriter::worksheet::PaperType;

use crate::devicelist::*;

pub struct DeviceListXlsxFormatter<'a> {
    table : XLSXTable,
    workbook : &'a Workbook,
    sheet : Worksheet<'a>,
    current_row: u32,
}

impl DeviceListXlsxFormatter<'_> {

    const DEVICE_NAME: u16 = 0;
    const SHORT_DESCR: u16 = 1;
    const LOCATION: u16 = 2;
    const PARTNO: u16 = 3;
    const PART_DESCRIPTION: u16 = 4;

    pub fn print_header(&mut self) {

    }

    pub fn print_entry(&mut self, device_entry: &DeviceEntry) {

    }
}