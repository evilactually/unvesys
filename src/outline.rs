/*
 Project outline

 Vladislav Shcherbakov
 Copyright Firefly Automatix 2024
 9/18/2024 3:34:10 PM
*/

use crate::vysis::HarnessDesign;
use crate::vysis::{Project, LogicalDesign, Connectivity};

/// This is needed mainly to decouple from Project and Design XML readers 
/// that have lifetime tied to original XML string and are really hard to pass around.

/// Project outline struct used in UI.
pub struct ProjectOutline {
	pub name: String,
	pub designs: Vec<LogicalDesignOutline>,
	pub harnessdesigns: Vec<HarnessDesignOutline>
}

impl ProjectOutline {
	pub fn new(project: &Project) -> ProjectOutline {
		let mut designs: Vec<LogicalDesignOutline> = Vec::new();
		let mut harnessdesigns: Vec<HarnessDesignOutline> = Vec::new();
		let mut name : String;

		// Accumulate logic designs
		for design in project.get_logical_design_iter() {
			designs.push(LogicalDesignOutline::new(&design));
		}

		// Accumulate harness designs
		for design in project.get_harness_design_iter() {
			harnessdesigns.push(HarnessDesignOutline::new(&design));
		}

		// Return project outline
		ProjectOutline {
			name : project.get_name().to_string(),
			designs : designs,
			harnessdesigns : harnessdesigns
		}
    }
}

/// Design outline struct used in UI.
pub struct HarnessDesignOutline {
	pub name: String,
}

impl HarnessDesignOutline {
	pub fn new(design: &HarnessDesign) -> HarnessDesignOutline {
		HarnessDesignOutline {
			name: design.get_name().to_string()
		}
	}
}

/// Design outline struct used in UI.
pub struct LogicalDesignOutline {
	pub name: String,
	pub harnesses: Vec<String>
}

impl LogicalDesignOutline {
	pub fn new(design: &LogicalDesign) -> LogicalDesignOutline {
		LogicalDesignOutline {
			name: design.get_name().to_string(),
			harnesses: design.get_harness_names().iter().map(
				|s|s.to_string()
			).collect() // convert Vec<&str> to Vec<String> 
		}
	}
}


