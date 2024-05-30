use std::thread;
use std::path::PathBuf;
use crate::outline::ProjectOutline;
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
use std::cell::RefCell;
use std::time::{Duration, SystemTime};

use crate::vysis::*;
use crate::vysyslib::*;
use crate::table_dump::*;

use crate::wire_list_xlsx_formatter::output_cutlist;

static BG_GRAPHIC: &str =
r"     








                                                               
                ▒▒░░░░░░                                        
              ░░░░░░░░░░░░                                      
            ░░░░░░░░░░░░░░░░                                    
            ░░░░░░░░    ░░░░░░                                  
            ░░░░██░░    ░░░░░░                                  
              ████      ░░░░░░                                  
              ░░        ░░░░░░        ░░░░░░░░░░░░              
              ░░      ░░░░░░      ░░░░░░░░░░░░░░                
                    ░░░░░░      ░░░░░░░░░░░░░░                  
                ░░░░░░░░      ░░░░░░░░░░░░░░                    
              ░░░░░░░░      ░░░░░░░░░░░░░░░░  ░░░░░░░░          
            ░░░░░░░░        ░░░░░░░░░░░░░░░░░░░░░░░░            
          ░░░░░░░░░░░░    ░░░░░░░░░░░░░░▒▒░░░░░░░░              
          ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                
          ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                  
          ░░▒▒▒▒░░░░░░░░▒▒░░░░░░░░▒▒░░░░░░░░░░                  
          ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                  
            ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                  
              ▒▒░░░░░░▒▒░░░░░░░░░░░░░░░░░░░░                    
                ░░░░░░░░░░░░░░░░░░░░░░░░░░                      
                  ░░░░░░░░░░░░░░░░░░░░░░              

                        CHANGELOG
                    - Made it better








    
  

";

static LOG_EXPIRATION: Duration = Duration::from_secs(5);

struct ApplicationState {
    project: Option<Project>, // VeSys Project
    library: Option<Library>, // VeSys Library
    project_outline: Option<ProjectOutline>,   // Cached UI representation of the VeSys project
    output_dir: String,       // Output directory
    log: RefCell<Vec::<(RichText, SystemTime, Option<Duration>)>>,     // Log output lines
}

fn read_file(filename:&str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn ui_hover_label_with_menu(ui: &mut egui::Ui, name: &str, context_menu_ui: impl FnOnce(&mut egui::Ui)) {
   let label = ui.add(Label::new(name)
          .selectable(false)
          .sense(Sense::hover()));
    label.context_menu(context_menu_ui);
    // Highlight on hover
    if label.hovered() {
        label.highlight();
    }
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

    fn load_library(&mut self) -> Result<(), String> {
        //self.log(RichText::new("Loading library").color(Color32::YELLOW), None);
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
                            }
                            _ => {
                                return Err("Failed to parse Library.xml".to_string());
                            }
                        }
                    }
                    _ => {
                        return Err("Failed to load Library.xml".to_string());
                    }
                }
            },
        }
        Ok(())
    }

    fn log(&self, msg: RichText, expire: Option<Duration>) { // note, RefCell allows this function to take immutable &self
        self.log.borrow_mut().push((msg, SystemTime::now(), expire));
    }

    fn with_library_and_project<T>(&self, action: impl FnOnce(&Library, &Project) -> T) -> Result<T, String>  {
        if let Some(project) = &self.project {
            if let Some(library) = &self.library {
                Ok(action(&library, &project))
            } else {
                let msg = "Library not loaded!";
                self.log(RichText::new(msg).color(Color32::RED), Some(LOG_EXPIRATION));
                Err(msg.to_string())
            }
        } else {
            let msg = "Project not loaded!";
            self.log(RichText::new(msg).color(Color32::RED), Some(LOG_EXPIRATION));
            Err(msg.to_string())
        }
    }
}

pub struct Application {
    state : Arc<Mutex<ApplicationState>>,
}

impl<'a> eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_secs(1)); // refresh the UI occasionally

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
        self.state.lock().unwrap().save_session_data();
    }
}

impl Application {

     pub fn new(cc: &eframe::CreationContext) -> Self {
        // Any slow start-up work goes here
       
        let state = Arc::new(Mutex::new(ApplicationState {
            library: None,
            project: None,
            project_outline: None,
            output_dir: String::new(),
            log: Vec::new().into()
        }));
        let state_clone = state.clone();
        // Start-up worker thread. Put any slow start-up work here
        std::thread::spawn(move || {

                state_clone.clone().lock().unwrap().log(RichText::new("Loading library...").color(Color32::YELLOW), None);
                {
                    thread::sleep(Duration::from_secs(1)); // let UI have the first lock
                    let mut state = state_clone.lock().unwrap();
                    match state.load_library() { // load Library XML {
                        Ok(_) => state.log(RichText::new("Library Loaded").color(Color32::GREEN), Some(LOG_EXPIRATION)),
                        Err(msg) => state.log(RichText::new(msg).color(Color32::RED), None)
                    }
                }

                state_clone.clone().lock().unwrap().load_session_data(); // load registry values
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
                                    state_clone.lock().unwrap().log(RichText::new(loading_msg).color(Color32::YELLOW), None);
                                    let xmlpath = path.display().to_string();
                                    let xml = read_file(&xmlpath);
                                    match xml {
                                        Ok(xml) => {
                                            let project = Project::new(&xml);
                                            match Project::new(&xml) {
                                                Ok(project) => {
                                                    state_clone.lock().unwrap().project = Some(project);
                                                    state_clone.lock().unwrap().update_project_outline();
                                                    let done_loading_msg = format!("Loaded project {:?}", path.file_name().unwrap());
                                                    state_clone.lock().unwrap().log(RichText::new(done_loading_msg).color(Color32::GREEN), Some(LOG_EXPIRATION));
                                                },
                                                _ => state_clone.lock().unwrap().log(RichText::new("Failed to parse project XML!").color(Color32::RED), Some(LOG_EXPIRATION)),
                                            }
                                        },
                                        _ => state_clone.lock().unwrap().log(RichText::new("Failed to load project XML file!").color(Color32::RED), Some(LOG_EXPIRATION)),
                                    }
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
                //let state = state_clone.lock().unwrap();
                let state_locked = state_clone.lock();
                if let Ok(state) = state_locked {
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
                                                ui_hover_label_with_menu(ui, &harness_name, |ui| {
                                                    // Right click on logic harness
                                                    self.logic_design_context_menu(ui, &state, &design_outline.name, &harness_name)
                                                });
                                            }
                                        });
                                    }
                                }
                            });
                            CollapsingHeader::new("Harness Designs")
                            .default_open(true)
                            .show(ui, |ui| {
                                if let Some(project_outline) = &state.project_outline {
                                    for harness_design in project_outline.harnessdesigns.iter() {
                                        ui_hover_label_with_menu(ui, &harness_design.name, |ui| {
                                            // Right click on logic harness
                                            self.harness_design_context_menu(ui, &state, &harness_design.name)
                                        });
                                    }
                                }
                            });
                        });
                    } else {
                        ui.horizontal_centered(|ui| {
                            ui.monospace(BG_GRAPHIC);
                        });
                    }
                }
            }
            //}
        });
    }
    
    fn logic_design_context_menu(&mut self, ui: &mut egui::Ui, state: &ApplicationState, current_design_name: &str, current_harness: &str) {
        if ui.button("Generate wire list").clicked() {
            println!("Generating wire list for {}, {}", current_design_name, current_harness);
            let _ = state.with_library_and_project(|library, project| {
                let mut filepath = PathBuf::from(state.output_dir.clone());
                let filename = current_harness.to_owned() + ".xlsx";
                state.log(RichText::new("Generating wire list ".to_owned() + &filename).color(Color32::YELLOW), None);
                filepath.push(current_harness.to_owned() + ".xlsx");
                output_cutlist(&project, &library, &current_design_name, &current_harness, &filepath.display().to_string());
                state.log(RichText::new("Finished wire list ".to_owned() + &filename).color(Color32::GREEN), Some(LOG_EXPIRATION));
            });
            ui.close_menu();   
        }
    }

    fn harness_design_context_menu(&mut self, ui: &mut egui::Ui, state: &ApplicationState, current_design_name: &str) {
        if ui.button("Dump tables to CSV").clicked() {
            let _ = state.with_library_and_project(|_, project| {
                state.log(RichText::new("Dumping tables to ".to_owned() + &state.output_dir).color(Color32::YELLOW), None);
                if let Some(harness_design) = project.get_harness_design(&current_design_name) {
                    let table_groups = harness_design.get_table_groups();
                    dump_tables(table_groups, &current_design_name, &state.output_dir);
                    state.log(RichText::new("Dumped ".to_owned() + &table_groups.len().to_string() + " tables from \"" + &current_design_name + "\" to CSV").color(Color32::GREEN), Some(LOG_EXPIRATION));
                }
            });
            ui.close_menu();
        }

        if ui.button("Export Schleuniger ASCII").clicked() {
            schleuniger_ascii_export("test",  &state.output_dir);
            ui.close_menu();
        }

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
        if let Some((status,timestamp, expire)) = self.state.try_lock().unwrap().log.borrow().last() {
            let duration = SystemTime::now().duration_since(*timestamp).unwrap_or(Duration::ZERO);

            if duration < expire.unwrap_or(Duration::MAX) {
                ui.label(status.clone());    
            } else {
                ui.label("");
            }
        } else {
            ui.label("");
        }
    }
}
