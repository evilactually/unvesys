
// mod vysis;
// use crate::vysis::*;

// mod vysyslibxml;
// use crate::vysyslibxml::*;

use std::collections::HashSet;
use serde_json::*;
use crate::vysis::{Project, LogicalDesign};

pub fn project_outline_json(project: &Project) -> Value {
	//let mut json_designs = Value::Array(Vec::new());
	let mut designs_json: Vec<Value> = Vec::new();

    // List logical design names and harnesses
    {
        let logical_designs = project.get_logical_design_names();
        for design in project.get_logical_design_iter() {
        	let mut design_detail_json : Map<String, Value> = Map::new();
            design_detail_json.insert("name".to_string(), design.get_name().into());
            design_detail_json.insert("harnesses".to_string(), design_outline_json(&design));
        	designs_json.push(serde_json::Value::Object(design_detail_json));
            //println!("    {} {}", "*".bright_yellow(), design.yellow());
            //show_design_info(&project.dom, design);
        }
    }
    //println!("{}", "Harness Designs:".bright_yellow());
    // List harness design names
    {
        let harness_designs = project.get_harness_design_names();
        for design in harness_designs {
            //println!("    {} {}", "*".bright_yellow(), design.yellow());
        }
    }
    Value::Array(designs_json)
}

pub fn design_outline_json(design: &LogicalDesign) -> Value {
    let mut harness_set:HashSet<String> = HashSet::new();
    let design_dom = &design.dom;
    // Collect harnesses
    for wire in &design_dom.connectivity.wire {
        if let Some(harness) = &wire.harness {
            harness_set.insert(harness.as_ref().to_string());
        }
    }
    Value::Array(harness_set.into_iter().map(Value::String).collect())
}