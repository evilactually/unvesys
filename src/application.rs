use wildflower::Pattern;
use crate::egui::Button;
use crate::logic_commands::*;
use std::thread;
use std::path::PathBuf;
use crate::outline::ProjectOutline;
use egui::Label;
use egui::Sense;
use egui::CollapsingHeader;
use std::fs::File;
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_CURRENT_USER;
use egui::RichText;
use std::sync::{Arc, Mutex};
//#[cfg(target_os = "windows")]
#[cfg(target_os = "windows")]
use winreg::*;
use std::io::prelude::*;
use std::io;
use egui::{Color32};
use ::egui::menu;
use std::cell::RefCell;
use std::time::{Duration, SystemTime};

use crate::vysis::*;
use crate::vysyslib::*;
use crate::harness_commands::*;

// ISSUE: https://github.com/bodil/smartstring/issues/7
// WORKAROUND: use format! in place of + operator to contacatenate strings


static BG_GRAPHIC: &str =
r"     








































      It appears you are trying to make a harness...


                                              ▄████▄
                                             ▐▌░░░░▐▌
                                          ▄▀▀█▀░░░░▐▌
                                          ▄░▐▄░░░░░▐▌▀▀▄
                                        ▐▀░▄▄░▀▌░▄▀▀░▀▄░▀
                                        ▐░▀██▀░▌▐░▄██▄░▌
                                         ▀▄░▄▄▀░▐░░▀▀░░▌
                                            █░░░░▀▄▄░▄▀
                                            █░█░░░░█░▐
                                            █░█░░░▐▌░█ 
                                            █░█░░░▐▌░█ 
                                            ▐▌▐▌░░░█░█
                                            ▐▌░█▄░▐▌░█
                                             █░░▀▀▀░░▐▌
                                             ▐▌░░░░░░█
                                              █▄░░░░▄█
                                               ▀████▀


";

static LOG_EXPIRATION: Duration = Duration::from_secs(5);

struct ApplicationState {
    project: Option<Project>, // VeSys Project                                                          // Opened document
    project_path: Option<PathBuf>,                                                                      // Path to project XMl for reloading
    library: Option<Library>, // VeSys Library                                                          // Loaded on start
    project_outline: Option<ProjectOutline>,   // Cached UI representation of the VeSys project         // UI representation
    output_dir: String,       // Output directory                                                       // UI storage
    log: RefCell<Vec::<(RichText, SystemTime, Option<Duration>)>>,     // Log output lines              // state of log
    //selected: bool,
    filter: String
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

    #[cfg(target_os = "windows")]
    fn load_session_data(&mut self) -> io::Result<()> {
        let hklu = RegKey::predef(HKEY_CURRENT_USER);
        let unvesys_key = hklu.open_subkey("SOFTWARE\\Unvesys")?;
        self.output_dir = unvesys_key.get_value("output_dir")?;
        println!("{}", self.output_dir);
        Ok(())
    }

    #[cfg(target_os = "windows")]
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

    #[cfg(not(target_os = "windows"))]
    fn load_session_data(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    fn save_session_data(&mut self) -> io::Result<()> {
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

            //Draw bottom panel first, so CentralPanel knows how much space it gets
            egui::TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.output_dir_ui(ui);
                    self.log_ui(ui); // BUG: Layout issue, this takes an extra frame to pop up for some reason
                })
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.filter_ui(ui);
                    self.project_view_ui(ui);
                })
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
            project_path: None,
            project_outline: None,
            output_dir: String::new(),
            log: Vec::new().into(),
            //selected: false
            filter: "*".to_owned()
        }));

        // Construct return value and return while thread is working
        let application = Self {
            state : state
        };

        application.load_library();
        application.state.lock().unwrap().load_session_data();
        application
    }

    fn load_project(&self, path: PathBuf) {
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

    fn load_library(&self) {
        // Clone Arc to avoid using self inside closure
        let state_clone = self.state.clone();

        // Wrap slow loading code in a thread
        std::thread::spawn(move || { // state_clone and path are moved
            state_clone.lock().unwrap().log(RichText::new("Loading library").color(Color32::YELLOW), None);
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
                                    state_clone.lock().unwrap().library = Some(library);
                                }
                                _ => {
                                    state_clone.lock().unwrap().log(RichText::new("Failed to parse Library.xml").color(Color32::RED), Some(LOG_EXPIRATION))
                                }
                            }
                        }
                        _ => {
                            state_clone.lock().unwrap().log(RichText::new("Failed to load Library.xml").color(Color32::RED), Some(LOG_EXPIRATION))
                        }
                    }
                },
            }
            state_clone.lock().unwrap().log(RichText::new("Library loaded").color(Color32::GREEN), Some(LOG_EXPIRATION));
        });
    }

    fn menu_ui(&mut self, ui: &mut egui::Ui) {

        egui::menu::bar(ui, |ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        // OPEN
                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("VeSys XML Project", &["xml"])
                                .pick_file() {
                                self.state.clone().lock().unwrap().project_path = Some(path.clone());
                                self.load_project(path); // spawns a thread
                            }
                            ui.close_menu(); // close menu so it doesn't stay opened
                        }

                        // RELOAD
                        let project_path_is_some = self.state.clone().lock().unwrap().project_path.is_some();
                        if ui.add_enabled(project_path_is_some, Button::new("Reload")).clicked() {
                            self.load_project(self.state.clone().lock().unwrap().project_path.clone().unwrap_or_default());
                            ui.close_menu(); // close menu so it doesn't stay opened
                        }
                    });
                });
            });
    }

    fn filter_ui(&mut self, ui: &mut egui::Ui) { 
        //let state = state_clone.lock().unwrap();
        let state_clone = self.state.clone();
        let state_locked = state_clone.lock();
        if let Ok(mut state) = state_locked { // make mut for selectable
            if let Some(project) = &state.project {
                //let mut filter = String::new();
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    ui.add(egui::TextEdit::singleline(&mut state.filter)
                        .desired_width(f32::INFINITY)
                        .hint_text("WILDCARD SYNTAX: * (any) \\ (escape) ? (single) Ex.: *J5*"));
                });
                ui.add_space(5.0);

            }
        }
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
                if let Ok(mut state) = state_locked { // make mut for selectable
                    if let Some(project) = &state.project {
                        let pattern = Pattern::new(state.filter.clone());

                        // let id = ui.make_persistent_id("my_collapsing_header");
                        // //let mut selected = true
                        // egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                        // .show_header(ui, |ui| {
                        //     ui.toggle_value(&mut state.selected, "Click to select/unselect");
                        //     //ui.radio_value(&mut self.radio_value, false, "");
                        //     //ui.radio_value(&mut self.radio_value, true, "");
                        // })
                        // .body(|ui| {
                        //     ui.label("The body is always custom");
                        // });

                        CollapsingHeader::new(project.get_name())
                        .default_open(true)
                        //.selectable(true) // UPGRADE
                        .show(ui, |ui| {
                            let c = CollapsingHeader::new("Logical Designs")
                            .default_open(true)
                            .show(ui, |ui| {
                                if let Some(project_outline) = &state.project_outline {
                                    for design_outline in &project_outline.designs {
                                        CollapsingHeader::new(&design_outline.name)

                                        .default_open(true)
                                        .show(ui, |ui| {
                                            let filtered_harnesses = design_outline.harnesses.iter().filter(|x| pattern.matches(x) );
                                            for harness_name in filtered_harnesses {
                                                ui_hover_label_with_menu(ui, &harness_name, |ui| {
                                                    // Right click on logic harness
                                                    self.logic_design_context_menu(ui, &state, &design_outline.name, &harness_name)
                                                });
                                            }
                                        });
                                    }
                                }
                            });
                            //c.selectable = true;
                            CollapsingHeader::new("Harness Designs")
                            .default_open(true)
                            .show(ui, |ui| {
                                if let Some(project_outline) = &state.project_outline {
                                    let filtered_harnesses = project_outline.harnessdesigns.iter().filter(|x| pattern.matches(&x.name) );
                                    for harness_design in filtered_harnesses {
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
        if ui.button("Export Excell wire list").clicked() {
            println!("Generating wire list for {}, {}", current_design_name, current_harness);
            let _ = state.with_library_and_project(|library, project| {
                let mut filepath = PathBuf::from(state.output_dir.clone());
                let filename = current_harness.to_owned() + ".xlsx";
                state.log(RichText::new(format!("Generating wire list {}", &filename)).color(Color32::YELLOW), None);
                filepath.push(current_harness.to_owned() + ".xlsx");
                export_xslx_wirelist(&project, &library, &current_design_name, &current_harness, &filepath.display().to_string());
                state.log(RichText::new(format!("Finished wire list {}", &filename)).color(Color32::GREEN), Some(LOG_EXPIRATION));
            });
            ui.close_menu();
        }
        if ui.button("Export CSV label list").clicked() {
            println!("Generating CSV label list for {}, {}", current_design_name, current_harness);
            let _ = state.with_library_and_project(|library, project| {
                let mut filepath = PathBuf::from(state.output_dir.clone());
                let filename = current_harness.to_owned() + ".csv";
                state.log(RichText::new(format!("Generating CSV labellist {}", &filename)).color(Color32::YELLOW), None);
                filepath.push(current_harness.to_owned() + ".csv");
                if let Err(e) = logic_harness_labels_csv_export(&project, &library, &current_design_name, &current_harness, &filepath.display().to_string()) {
                    println!{"{}", e};
                } else {
                    state.log(RichText::new(format!("Finished CSV label list {}", &filename)).color(Color32::GREEN), Some(LOG_EXPIRATION));
                }
            });
            ui.close_menu();
        }
        if ui.button("Export Schleuniger ASCII").clicked() {

            state.log(RichText::new(format!("{}{}","Exporting Schleuniger ASCII file to ", &state.output_dir)).color(Color32::YELLOW), None);
            let _ = state.with_library_and_project(|library, project| {
                let mut filepath = PathBuf::from(state.output_dir.clone());
                let filename = current_harness.to_owned() + ".txt";
                state.log(RichText::new(format!("Generating wire list {}", &filename)).color(Color32::YELLOW), None);
                filepath.push(current_harness.to_owned() + ".txt");
                export_xslx_wirelist(&project, &library, &current_design_name, &current_harness, &filepath.display().to_string());
                if let Ok(mut file) = File::create(filepath) {
                    logic_harness_shchleuniger_export(&project, &library, current_design_name, current_harness,  &mut file);
                    state.log(RichText::new(format!("Exported Schleuniger ASCII file to {}", &filename)).color(Color32::GREEN), Some(LOG_EXPIRATION));
                } else {
                    state.log(RichText::new(format!("Failed to create {}", &filename)).color(Color32::RED), Some(LOG_EXPIRATION));
                }
                state.log(RichText::new(format!("Finished wire list {}", &filename)).color(Color32::GREEN), Some(LOG_EXPIRATION));
            });


            ui.close_menu();
        }
        if ui.button("Export Excell BOM").clicked() {
            let _ = state.with_library_and_project(|library, project| {
                let mut filepath = PathBuf::from(state.output_dir.clone());
                let filename = current_harness.to_owned() + ".txt";
                logic_harness_bom_export(project, library, &current_design_name, &current_harness);
            });
        }
    }

    fn harness_design_context_menu(&mut self, ui: &mut egui::Ui, state: &ApplicationState, current_design_name: &str) {
        if ui.button("Dump tables to CSV").clicked() {
            let _ = state.with_library_and_project(|_, project| {
                state.log(RichText::new(format!("{}{}", "Dumping tables to ", &state.output_dir)).color(Color32::YELLOW), None);
                if let Some(harness_design) = project.get_harness_design(&current_design_name) {
                    let table_groups = harness_design.get_table_groups();
                    dump_tables(table_groups, &current_design_name, &state.output_dir);
                    state.log(RichText::new(format!("Dumped {} tables from \"{}\" to CSV", &table_groups.len().to_string(), &current_design_name)).color(Color32::GREEN), Some(LOG_EXPIRATION));
                }
            });
            ui.close_menu();
        }

        if ui.button("Export Schleuniger ASCII").clicked() {
            let _ = state.with_library_and_project(|library, project| {
                state.log(RichText::new(format!("{}{}","Exporting Schleuniger ASCII file to ", &state.output_dir)).color(Color32::YELLOW), None);
                if let Some(harness_design) = project.get_harness_design(&current_design_name) {
                    let mut path : PathBuf = state.output_dir.clone().into();
                    let filename = current_design_name.to_owned() + ".txt";
                    path.push(String::from(&filename));
                    if let Ok(mut file) = File::create(path) {
                        harness_schleuniger_ascii_export(&library, &harness_design, &mut file);
                        state.log(RichText::new(format!("Exported Schleuniger ASCII file to {}", &filename)).color(Color32::GREEN), Some(LOG_EXPIRATION));
                    } else {
                        state.log(RichText::new(format!("Failed to create {}", &filename)).color(Color32::RED), Some(LOG_EXPIRATION));
                    }
                }
                ui.close_menu();
            });
        }

        if ui.button("Export CSV label list").clicked() {
            let _ = state.with_library_and_project(|library, project| {
                state.log(RichText::new(format!("{}{}","Exporting CSV label list file to ", &state.output_dir)).color(Color32::YELLOW), None);
                if let Some(harness_design) = project.get_harness_design(&current_design_name) {
                    let mut path : PathBuf = state.output_dir.clone().into();
                    let filename = current_design_name.to_owned() + ".csv";
                    path.push(String::from(&filename));
                    if let Ok(mut file) = File::create(path) {
                        harness_labels_export(&library, &harness_design, &mut file);
                        state.log(RichText::new(format!("Exported CSV label list to {}", &filename)).color(Color32::GREEN), Some(LOG_EXPIRATION));
                    } else {
                        state.log(RichText::new(format!("Failed to create {}", &filename)).color(Color32::RED), Some(LOG_EXPIRATION));
                    }
                }
                ui.close_menu();
            });
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
