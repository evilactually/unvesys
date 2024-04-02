use std::io::{Read, Seek, Write};
use crate::Project;
use hard_xml::{XmlError};

pub struct ProjectFile {
    project: String,
}

impl ProjectFile {
    pub fn from_reader<T: Read>(mut reader: T) -> std::io::Result<Self> {
        let mut file_contents = String::new();
        reader.read_to_string(&mut file_contents);
        Ok(ProjectFile {
            project : file_contents
        })
    }

    pub fn parse<'a>(&'a self) -> std::result::Result<Project, XmlError> {
        Project::new(&self.project)
    }
}