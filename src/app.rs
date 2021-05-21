use crate::checks::*;
use eframe::{egui, epi};
use wasm_stopwatch::Stopwatch;
use crate::variants::{slot2_key_options, slot4_key_options, slot6_key_options, slot8_key_options, referential_key_options, slot9iev_key_options};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    current_menu: Menu,
    answer: String,
    probabilities: Vec<f64>,
    expected_answer: (String, String),
    #[cfg_attr(feature = "persistence", serde(skip))]
    data: Data,
    key: Vec<Vec<bool>>,
    choice: Option<usize>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    stopwatch: Stopwatch,
    #[cfg_attr(feature = "persistence", serde(skip))]
    options: Vec<Vec<&'static str>>,
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
    Slot8,
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
                }
                Menu::WritingPractice => {
                    if ui.button("Slot2").clicked() {
                        self.current_menu = Menu::Slot2;
                        let out = slot2_key_options();
                        self.key = out.0;
                        self.options = out.1;
                    }
                    if ui.button("Slot4").clicked() {
                        self.current_menu = Menu::Slot4;
                        let out = slot4_key_options();
                        self.key = out.0;
                        self.options = out.1;
                    }
                    if ui.button("Slot6").clicked() {
                        self.current_menu = Menu::Slot6;
                        let out = slot6_key_options();
                        self.key = out.0;
                        self.options = out.1;
                    }/*
                    if ui.button("(Slot8)").clicked() {
                        self.current_menu = Menu::Slot8;
                        let out = slot8_key_options();
                        self.key = out.0;
                        self.options = out.1;
                    }*/
                    if ui.button("ReferentialCase").clicked() {
                        self.current_menu = Menu::ReferentialCase;
                        let out = referential_key_options();
                        self.key = out.0;
                        self.options = out.1;
                    }
                    if ui.button("Slot9IEV").clicked() {
                        self.current_menu = Menu::Slot9IEV;
                        let out = slot9iev_key_options();
                        self.key = out.0;
                        self.options = out.1;
                    }
                }
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

        if self.key.is_empty() || self.current_menu == Menu::Main {
            // if there is no parameters choosen then stop here
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.choice == None {
                let (expected_answer, choice) = match self.current_menu {
                    Menu::Slot2 => {
                        generate_slot2(&self.data, &mut self.probabilities, &mut self.key)
                    }
                    Menu::Slot4 => {
                        generate_slot4(&self.data, &mut self.probabilities, &mut self.key)
                    }
                    Menu::Slot6 => {
                        generate_slot6(&self.data, &mut self.probabilities, &mut self.key)
                    }
                    Menu::ReferentialCase => {
                        generate_referentials(&self.data, &mut self.probabilities, &mut self.key)
                    }
                    Menu::Slot8 => {
                        generate_slot8(&self.data, &mut self.probabilities, &mut self.key)
                    }
                    Menu::Slot9IEV => {
                        generate_slot9iev(&self.data, &mut self.probabilities, &mut self.key)
                    }
                    _ => panic!("should not happen"),
                };
                self.expected_answer = expected_answer;
                self.choice = Some(choice);
                self.stopwatch.reset();
            }
            ui.horizontal(|ui| {
                ui.label("Difficulty: ");
                ui.add(
                    egui::Slider::new(&mut self.difficulty, 1.0..=5.0)
                        .text("seconds(for the right answer)"),
                );
            });

            ui.heading("Task :");
            ui.add_space(10.0);
            ui.heading(self.expected_answer.0.clone());
            ui.add_space(10.0);
            ui.text_edit_singleline(&mut self.answer);
            if self.answer == self.expected_answer.1 {
                // guessed? in time or not?
                let successful = if self.stopwatch.get_time() > self.difficulty {
                    false
                } else {
                    true
                };
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
                        if ui
                            .checkbox(&mut self.key[y][x], self.options[y][x])
                            .changed()
                        {
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
