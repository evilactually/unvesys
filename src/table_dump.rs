use crate::vysis::HarnessDesign;
use csv::{Writer, WriterBuilder};
use crate::vysisxml::XmlTableGroup;
use std::path::PathBuf;
use std::error::Error;

pub fn dump_tables(table_groups: &Vec<XmlTableGroup>, basename: &str, dir: &str) -> std::io::Result<()> {
    let mut i = 0;
    let mut path : PathBuf = dir.into();
    for group in table_groups.iter() {
        println!("{:?}", group.title);
        for table in group.tablefamily.table.iter() {
            if let Some(datacache) = &table.tabledatacache {
                //println!("{:?}", datacache.colhdrnames);
                let mut path = path.clone();
                let filename = format!("{}-{}-{}.csv", basename, group.title, i);
                path.push(filename.clone());
                println!("{:?}", path);
                i = i + 1;
                let mut wtr = Writer::from_path(path)?;
                let header = &datacache.colhdrnames.row;
                let header_names : Vec<String> = header.cellvals.iter().map(|v| {
                    v.cval.val.clone()
                }).collect();
                println!("{:?}", header_names);
                wtr.write_record(&header_names)?;

                for datarow in datacache.datavalues.datarow.iter() {
                    let cols : Vec<String> = datarow.cellval.iter().map(|v| {
                        v.cval.val.clone()
                    }).collect();

                    wtr.write_record(&cols)?;
                }

            }
        }
    }

    Ok(())
}

pub fn schleuniger_ascii_export(harness_design: &HarnessDesign, basename: &str, dir: &str) ->  std::io::Result<()> {
    let mut path : PathBuf = dir.into();
    path.push(String::from("test").clone());
    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(path).unwrap(); 

    wtr.write_record(vec![
        String::from("Import"), String::from("ASCII"),
    ])?;

    wtr.write_record(vec![
        String::from("Units"), String::from("inch"),
    ])?;

    wtr.write_record(vec![
        String::from("Area"), String::from("TT"),
    ])?;
    
    wtr.write_record(vec![
        String::from("Name"), 
        String::from("Part"), 
        String::from("Length"), 
        String::from("Style"), 
        String::from("Stripping type"), 
        String::from("Right strip"),
        String::from("Left strip"),
        String::from("Partial strip %"),
        String::from("Marker left text"),
        String::from("Marker left position"),
        String::from("Marker right text"),
        String::from("Marker right position"),
        String::from("Autorotation"),
    ])?;
    



    // wtr?.write_record(vec![
    //  String::from("Import"), String::from("ASCII"),
    // ])?;
    // wtr.write_record(vec![
    //  String::from("Import"), String::from("ASCII"),
    // ])?;
    // wtr.write_record(vec![
    //  String::from("Units "), String::from("inch"),
    // ])?;
    // wtr.write_record(vec![
    //  String::from("Area"), String::from("TT"),
    // ])?;


    // let mut i = 0;
    // let mut path : PathBuf = dir.into();
    // for group in table_groups.iter() {
    //  println!("{:?}", group.title);
    //  for table in group.tablefamily.table.iter() {
    //      if let Some(datacache) = &table.tabledatacache {
    //          //println!("{:?}", datacache.colhdrnames);
    //          let mut path = path.clone();
    //          let filename = format!("{}-{}-{}.csv", basename, group.title, i);
    //          path.push(filename.clone());
    //          println!("{:?}", path);
    //          i = i + 1;
    //          let mut wtr = Writer::from_path(path)?;
    //          let header = &datacache.colhdrnames.row;
    //          let header_names : Vec<String> = header.cellvals.iter().map(|v| {
    //              v.cval.val.clone()
    //          }).collect();
    //          println!("{:?}", header_names);
    //          wtr.write_record(&header_names)?;

    //          for datarow in datacache.datavalues.datarow.iter() {
    //              let cols : Vec<String> = datarow.cellval.iter().map(|v| {
    //                  v.cval.val.clone()
    //              }).collect();

    //              wtr.write_record(&cols)?;
    //          }

    //      }
    //  }
    // }

    Ok(())
}