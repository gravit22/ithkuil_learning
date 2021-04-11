use eframe::{egui, epi};
use crate::checks::*;
use stopwatch::Stopwatch;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    // Example stuff:
    current_menu: Menu,
    answer: String,
    probabilities: Vec<f64>,
    expected_answer: (String, String),
    data: Data,
    key: Vec<Vec<bool>>,
    choice: Option<usize>,
    stopwatch: Stopwatch,
    options: Vec<Vec<&'static str>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
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
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
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
                        self.options.push(vec!["(C)", "(w)", "(y)"]);
                        self.options.push(vec!["S1", "S2", "S3", "S0"]);
                        self.options.push(vec!["PRC", "CPT"]);
                    }
                    if ui.button("Slot4").clicked() {
                        self.current_menu = Menu::Slot4;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true, true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["(C)", "(w)", "(y)"]);
                        self.options.push(vec!["STA", "DYN"]);
                        self.options.push(vec!["BSC", "CTE", "CSV", "OBJ"]);
                        self.options.push(vec!["EXS", "FNC", "RPS", "AMG"]);
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
                        self.options.push(vec!["(C)", "(V)"]);
                        self.options.push(vec!["CSL", "ASO", "COA", "VAR"]);
                        self.options.push(vec!["UNI", "DUP", "MSS", "MSC", "MSF", "MDS", "MDC",
                        "MDF", "MFS", "MFC", "MFF", "DSS", "DSC", "DSF", "DDS", "DDC", "DDF",
                        "DFS", "DFC", "DFF"]);
                        self.options.push(vec!["DEL", "PRX", "ICP", "ATV", "GRA", "DPL"]);
                        self.options.push(vec!["M", "G", "N", "A"]);
                        self.options.push(vec!["NRM", "RPV"]);
                    }
                    if ui.button("ReferentialCase").clicked() {
                        self.current_menu = Menu::ReferentialCase;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true, true, true, true, true]);
                        self.key.push(vec![true, true, true, true]);
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true, true, true, true, true, true, true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["I", "you", "he/she/they", "it/these things/those things"
                                               , "animate+inanimate", "3rd party other than most recently referenced"
                                               , "whatever"]);
                        self.options.push(vec!["M", "P", "N", "A"]);
                        self.options.push(vec!["NEUTRAL", "BENEFICIAL", "DETRIMENTAL"]);
                        self.options.push(vec!["THM", "INS", "ABS", "AFF", "STM", "EFF", "ERG"
                                               , "DAT", "IND"]);
                    }
                    if ui.button("Slot9IEV").clicked() {
                        self.current_menu = Menu::Slot9IEV;
                        self.key = Vec::new();
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true]);
                        self.key.push(vec![true, true, true]);
                        self.key.push(vec![true, true, true, true, true, true, true, true]);

                        self.options = Vec::new();
                        self.options.push(vec!["(C)", "(w)", "(y)"]);
                        self.options.push(vec!["ASR", "PFM"]);
                        self.options.push(vec!["COG", "RSP", "EXE"]);
                        self.options.push(vec!["OBS", "REC", "RPR", "PUP", "IMA", "CVN", "ITU"
                                               , "INF"]);
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
                self.stopwatch.restart();
            }
            ui.heading("Task :");
            ui.add_space(10.0);
            ui.heading(self.expected_answer.0.clone());
            ui.add_space(10.0);
            ui.text_edit_singleline(&mut self.answer);
            if self.answer == self.expected_answer.1 { // guessed? in time or not?
                let difficulty = 2000; // ms on right answer
                let successful = if self.stopwatch.elapsed_ms() > difficulty {false} else {true};
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
                        if ui.checkbox(&mut self.key[y][x], self.options[y][x]).changed() {
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
            //ui.label(self.stopwatch.elapsed_ms().to_string());
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
