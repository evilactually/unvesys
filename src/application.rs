use std::fs::File;
use winreg::enums::HKEY_CURRENT_USER;
use egui::RichText;
use std::sync::{Arc, Mutex};
use winreg::*;
use std::io::prelude::*;
use std::io;
use egui::{Color32};


use crate::vysis::*;
use crate::vysyslib::*;

struct ApplicationState {
    project: Option<Project>, // VeSys Project
    library: Option<Library>, // VeSys Library
    output_dir: String,       // Output directory
    log: Vec::<RichText>     // Log output lines
}

fn read_file(filename:&str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl ApplicationState {

    fn load_session_data(&mut self) -> io::Result<()> {
        let hklu = RegKey::predef(HKEY_CURRENT_USER);
        let unvesys_key = hklu.open_subkey("SOFTWARE\\Unvesys")?;
        self.output_dir = unvesys_key.get_value("output_dir")?;
        Ok(())
    }

    fn save_session_data(&mut self) -> io::Result<()> {
        // Save output directory to registry
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = match hkcu.create_subkey("SOFTWARE\\Unvesys") {
            Ok(ok) => ok,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };
        key.set_value("output_dir", &self.output_dir)?;
        Ok(())
    }

    fn load_library(&mut self) -> io::Result<()> {
        self.log(RichText::new("Loading library").color(Color32::YELLOW));
        let path = process_path::get_executable_path();
        match path {
            None => {}
            Some(mut path) => { 
                path.set_file_name("Library.xml");
                let library_xml = read_file(&path.display().to_string());
                match library_xml {
                    Ok(library_xml) => {
                        //println!("{}", &library_xml);
                        match Library::new(&library_xml) {
                            Ok(library) => {
                                self.library = Some(library);
                                self.log(RichText::new("Library loaded").color(Color32::GREEN));
                            }
                            _ => {
                                self.log(RichText::new("Library loading error").color(Color32::RED));
                            }
                        }
                    }
                    _ => {
                        self.log(RichText::new("Library loading error").color(Color32::RED));
                    }
                }
            },
        }
        Ok(())
    }

    fn log(&mut self, msg: RichText) {
        self.log.push(msg);
    }
}


pub struct Application {
    state : Arc<Mutex<ApplicationState>>,
}

impl<'a> eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }

    fn on_exit(&mut self, ctx: Option<&eframe::glow::Context>) {
        //self.state.save_session_data();
    }
}

// fn startup_worker(state_clone: Arc<Mutex<State>>) -> io::Result<()> {
//     read_saved_session_data(state_clone.clone());
//     load_library(state_clone.clone());
//     Ok(())
// }

impl Application {

     pub fn new(cc: &eframe::CreationContext) -> Self {
        // Any slow start-up work goes here
       
        let state = Arc::new(Mutex::new(ApplicationState {
            library: None,
            project: None,
            output_dir: String::new(),
            log: Vec::new()
        }));
        let state_clone = state.clone();
        // Start-up worker thread. Put any slow start-up work here
        std::thread::spawn(move || {
            let mut state_locked = state_clone.lock().unwrap();
            {
                state_locked.load_library(); // load Library XML
                state_locked.load_session_data(); // load registry values
            }
        });
        // Construct return value and return while thread is working
        Self {
            state : state
        }
    }


}
