use eframe::{egui, epi};
use crate::checks::*;
use wasm_stopwatch::Stopwatch;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    current_menu: Menu,
    answer: String,
    probabilities: Vec<f64>,
    expected_answer: (String, String),
    data: Data,
    key: Vec<Vec<bool>>,
    choice: Option<usize>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    stopwatch: Stopwatch,
    options: Vec<Vec<String>>,
    difficulty: f64,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
enum Menu {
    Main,
    WritingPractice,
    Slot2,
    Slot4,
    Slot6,
    ReferentialCase,
    Slot9IEV,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            current_menu: Menu::Main,
            answer: "".to_owned(),
            probabilities: Vec::new(),
            expected_answer: (String::new(), String::new()),
            data: Data::new(),
            key: Vec::new(),
            choice: None,
            stopwatch: Stopwatch::new(),
            options: Vec::new(),
            difficulty: 2.0,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "ithkuil learning"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // Side panel where you chose practice
        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Practice: ");
            match self.current_menu {
                Menu::Main => {
                    if ui.button("Writing practice").clicked() {
                        self.current_menu = Menu::WritingPractice;
                    } else if ui.button("Reading practice").clicked() {

                    }
                },
                Menu::WritingPractice => {
                    if ui.button("Slot2").clicked() {
                        self.current_menu = Menu::Slot2;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["(C)".to_owned(), "(w)".to_owned(), "(y)".to_owned()]);
                        self.options.push(vec!["S1".to_owned(), "S2".to_owned(), "S3".to_owned()
                                               , "S0".to_owned()]);
                        self.options.push(vec!["PRC".to_owned(), "CPT".to_owned()]);
                    }
                    if ui.button("Slot4").clicked() {
                        self.current_menu = Menu::Slot4;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true, true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["(C)".to_owned(), "(w)".to_owned(), "(y)".to_owned()]);
                        self.options.push(vec!["STA".to_owned(), "DYN".to_owned()]);
                        self.options.push(vec!["BSC".to_owned(), "CTE".to_owned(), "CSV".to_owned()
                                               , "OBJ".to_owned()]);
                        self.options.push(vec!["EXS".to_owned(), "FNC".to_owned(), "RPS".to_owned()
                                               , "AMG".to_owned()]);
                    }
                    if ui.button("Slot6").clicked() {
                        self.current_menu = Menu::Slot6;
                        self.key = Vec::new();
                        self.key.push(vec![true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true, true, true, true, true, true, true, true, true
                                           , true, true, true, true, true, true, true, true, true, true]);
                        self.key.push(vec![true, true, true, true, true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["(C)".to_owned(), "(V)".to_owned()]);
                        self.options.push(vec!["CSL".to_owned(), "ASO".to_owned(), "COA".to_owned()
                                               , "VAR".to_owned()]);
                        self.options.push(vec!["UNI".to_owned(), "DUP".to_owned(), "MSS".to_owned()
                                               , "MSC".to_owned(), "MSF".to_owned(), "MDS".to_owned()
                                               , "MDC".to_owned(),
                        "MDF".to_owned(), "MFS".to_owned(), "MFC".to_owned(), "MFF".to_owned()
                                               , "DSS".to_owned(), "DSC".to_owned(), "DSF".to_owned()
                                               , "DDS".to_owned(), "DDC".to_owned(), "DDF".to_owned(),
                        "DFS".to_owned(), "DFC".to_owned(), "DFF".to_owned()]);
                        self.options.push(vec!["DEL".to_owned(), "PRX".to_owned(), "ICP".to_owned()
                                               , "ATV".to_owned(), "GRA".to_owned(), "DPL".to_owned()]);
                        self.options.push(vec!["M".to_owned(), "G".to_owned(), "N".to_owned(), "A".to_owned()]);
                        self.options.push(vec!["NRM".to_owned(), "RPV".to_owned()]);
                    }
                    if ui.button("ReferentialCase").clicked() {
                        self.current_menu = Menu::ReferentialCase;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true, true, true, true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true, true, true, true, true, true, true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["I".to_owned(), "you".to_owned(), "he/she/they".to_owned()
                                               , "it/these things/those things".to_owned()
                                               , "animate+inanimate".to_owned(), "3rd party other than most recently referenced".to_owned()
                                               , "whatever".to_owned()]);
                        self.options.push(vec!["M".to_owned(), "P".to_owned(), "N".to_owned()
                                               , "A".to_owned()]);
                        self.options.push(vec!["NEUTRAL".to_owned(), "BENEFICIAL".to_owned(), "DETRIMENTAL".to_owned()]);
                        self.options.push(vec!["THM".to_owned(), "INS".to_owned(), "ABS".to_owned()
                                               , "AFF".to_owned(), "STM".to_owned(), "EFF".to_owned()
                                               , "ERG".to_owned()
                                               , "DAT".to_owned(), "IND".to_owned()]);
                    }
                    if ui.button("Slot9IEV").clicked() {
                        self.current_menu = Menu::Slot9IEV;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true]);
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true, true, true, true, true, true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["(C)".to_owned(), "(w)".to_owned(), "(y)".to_owned()]);
                        self.options.push(vec!["ASR".to_owned(), "PFM".to_owned()]);
                        self.options.push(vec!["COG".to_owned(), "RSP".to_owned(), "EXE".to_owned()]);
                        self.options.push(vec!["OBS".to_owned(), "REC".to_owned(), "RPR".to_owned()
                                               , "PUP".to_owned(), "IMA".to_owned(), "CVN".to_owned()
                                               , "ITU".to_owned()
                                               , "INF".to_owned()]);
                    }
                },
                _ => (),
            }
        });

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "Menu", |ui| {
                    if ui.button("Main").clicked() {
                        self.current_menu = Menu::Main;
                        self.choice = None;
                        self.key.clear();
                        self.probabilities.clear();
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        if self.key.is_empty() || self.current_menu == Menu::Main { // if there is no parameters choosen then stop here
            return
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.choice == None {
                let (expected_answer, choice) = match self.current_menu {
                    Menu::Slot2 => generate_slot2(&self.data, &mut self.probabilities, &mut self.key),
                    Menu::Slot4 => generate_slot4(&self.data, &mut self.probabilities, &mut self.key),
                    Menu::Slot6 => generate_slot6(&self.data, &mut self.probabilities, &mut self.key),
                    Menu::ReferentialCase => generate_referentials(&self.data, &mut self.probabilities, &mut self.key),
                    Menu::Slot9IEV => generate_slot9iev(&self.data, &mut self.probabilities, &mut self.key),
                    _ => panic!("should not happen"),
                    };
                self.expected_answer = expected_answer;
                self.choice = Some(choice);
                self.stopwatch.reset();
            }
            ui.horizontal(|ui| {
                ui.label("Difficulty: ");
                ui.add(egui::Slider::new(&mut self.difficulty, 1.0..=5.0).text("seconds"));
            });
            ui.heading("Task :");
            ui.add_space(10.0);
            ui.heading(self.expected_answer.0.clone());
            ui.add_space(10.0);
            ui.text_edit_singleline(&mut self.answer);
            if self.answer == self.expected_answer.1 { // guessed? in time or not?
                let successful = if self.stopwatch.get_time() > self.difficulty {false} else {true};
                modify_probability(&mut self.probabilities, successful, self.choice.unwrap());
                self.choice = None;
                self.answer.clear();
            }
            if is_win(&self.probabilities) {
                self.current_menu = Menu::Main;
                self.choice = None;
                self.key.clear();
                self.probabilities.clear();
            }

            // write buttons to chose parts of categories to include
            for y in 0..self.key.len() {
                ui.horizontal(|ui| {
                    for x in 0..self.key[y].len() {
                        if ui.checkbox(&mut self.key[y][x], &self.options[y][x]).changed() {
                            self.choice = None;
                            self.probabilities.clear();
                        };
                    }
                });
            }

            ui.add_space(100.0);
            let mut debug_info = "Debug: ".to_string();
            for i in &self.probabilities {
                debug_info.push_str(&format!("{:.2}", i));
                debug_info.push_str(", ");
            }
            ui.label(debug_info);
            ui.label(self.stopwatch.get_time().to_string());
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
