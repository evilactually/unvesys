use std::time::Duration;
use crate::ProjectOutline;
use egui::Label;
use egui::Sense;
use egui::CollapsingHeader;
use std::fs::File;
use winreg::enums::HKEY_CURRENT_USER;
use egui::RichText;
use std::sync::{Arc, Mutex};
use winreg::*;
use std::io::prelude::*;
use std::io;
use egui::{Color32};
use ::egui::menu;
use native_dialog::{MessageDialog, MessageType};


use crate::vysis::*;
use crate::vysyslib::*;

struct ApplicationState {
    project: Option<Project>, // VeSys Project
    library: Option<Library>, // VeSys Library
    project_outline: Option<ProjectOutline>,   // Cached UI representation of the VeSys project
    output_dir: String,       // Output directory
    log: Vec::<RichText>,     // Log output lines
}

fn read_file(filename:&str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl ApplicationState {

    fn update_project_outline(&mut self) {
        if let Some(project) = &self.project {
            self.project_outline = Some(ProjectOutline::new(project));
        } else {
            self.project_outline = None; // Clear project outline
        }
    }

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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_secs(1));

        // Draw menu
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu_ui(ui);
        });

        // Draw bottom panel first, so CentralPanel knows how much space it gets
        egui::TopBottomPanel::bottom("bottom_panel")
        .show_separator_line(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                self.output_dir_ui(ui);
                self.log_ui(ui);
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.project_view_ui(ui);
        });


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
            project_outline: None,
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


    fn menu_ui(&mut self, ui: &mut egui::Ui) {

        egui::menu::bar(ui, |ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("VeSys XML Project", &["xml"])
                                .pick_file() {
                                
                                // Clone Arc to avoid using self inside closure
                                let state_clone = self.state.clone();

                                // Wrap slow loading code in a thread
                                std::thread::spawn(move || { // state_clone and path are moved
                                    let loading_msg = format!("Loading project {:?}", path.file_name().unwrap());
                                    state_clone.lock().unwrap().log(RichText::new(loading_msg).color(Color32::YELLOW));
                                    let xmlpath = path.display().to_string();
                                    let xml = read_file(&xmlpath);
                                    match xml {
                                        Ok(xml) => {
                                            let project = Project::new(&xml);
                                            match Project::new(&xml) {
                                                Ok(project) => {
                                                    state_clone.lock().unwrap().project = Some(project);
                                                    state_clone.lock().unwrap().update_project_outline();
                                                },
                                                _ => state_clone.lock().unwrap().log(RichText::new("Failed to parse project XML!").color(Color32::RED)),
                                            }
                                        },
                                        _ => state_clone.lock().unwrap().log(RichText::new("Failed to load project XML file!").color(Color32::RED)),
                                    }
                                    let done_loading_msg = format!("Loaded project {:?}", path.file_name().unwrap());
                                    state_clone.lock().unwrap().log(RichText::new(done_loading_msg).color(Color32::GREEN));
                                });
                            }
                            ui.close_menu(); // close menu so it doesn't stay opened
                        }
                    });
                });
            });
    }

    fn project_view_ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
        .max_width(f32::INFINITY)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let state_clone = self.state.clone();
            {
                let state = state_clone.lock().unwrap();
                if let Some(project) = &state.project {
                    CollapsingHeader::new(project.get_name())
                    .default_open(true)
                    //.selectable(true) // UPGRADE
                    .show(ui, |ui| {
                        CollapsingHeader::new("Logical Designs")
                        .default_open(true)
                        .show(ui, |ui| {
                            if let Some(project_outline) = &state.project_outline {
                                for design_outline in &project_outline.designs {
                                    CollapsingHeader::new(&design_outline.name)
                                    .default_open(true)
                                    .show(ui, |ui| {
                                        for harness_name in &design_outline.harnesses {
                                            let harness_entry = ui.add(
                                                Label::new(harness_name)
                                                .selectable(false)
                                                .sense(Sense::hover()));
                                            // Highlight on hover
                                            if harness_entry.hovered() {
                                                harness_entry.highlight();
                                            }
                                        }

                                    });
                                }
                            }
                        });
                    });
                }
            }
            //}
        });
    }

    fn output_dir_ui(&mut self, ui: &mut egui::Ui) {
        let output_dir = &mut self.state.lock().unwrap().output_dir;
        ui.horizontal(|ui| {
            ui.label("Output Folder:");
            ui.add_sized(ui.available_size()-egui::vec2(75.0,0.0),egui::TextEdit::singleline(output_dir)
            .hint_text("Where do you want it?"));
            if ui.add(egui::Button::new("Browse").min_size(ui.available_size())).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() { // set_parent(frame.window_handle()) 
                    println!("{}", &path.display().to_string());
                    *output_dir = path.display().to_string();
                }
            }
            ui.end_row();
        });
    }

    fn log_ui(&mut self, ui: &mut egui::Ui) {
        // Show status
        if let Some(status) = self.state.lock().unwrap().log.last() {
            ui.label(status.clone());    
        } else {
            ui.label("");
        }
    }

}
