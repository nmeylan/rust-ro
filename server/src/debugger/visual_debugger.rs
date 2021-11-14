use std::sync::Arc;
use std::thread::spawn;
use eframe::egui::{ComboBox, Pos2, Rect, Vec2, Visuals};
use eframe::epi;
use epi::egui;
use egui::{Ui};
use crate::server::server::Server;
use lazy_static::lazy_static;
use crate::debugger::frame_history;
use crate::debugger::map_instance_view::MapInstanceView;
use crate::server::core::map::MapItem;

pub struct VisualDebugger {
    pub name: String,
    pub server: Arc<Server>,
    pub selected_map: Option<String>,
    pub selected_tab: String,
    pub init: bool,
    frame_history: frame_history::FrameHistory,
    map_instance_view: MapInstanceView,
}

lazy_static! {
    pub static ref tabs: Vec<&'static str> = vec!["Map"];
}

impl epi::App for VisualDebugger {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.frame_history
            .on_new_frame(ctx.input().time, frame.info().cpu_usage);
        frame.set_window_title(&*format!("{} {}", self.name, self.frame_history.info()));
        if !self.init {
            ctx.set_visuals(Visuals::light());
            frame.set_window_size(Vec2 {
                x: 1024.0,
                y: 768.0
            });
            self.init = true;
        }
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in tabs.iter() {
                    if ui
                        .selectable_label(self.selected_tab == *tab, *tab)
                        .clicked()
                    {}
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }

    fn name(&self) -> &str {
        &*self.name
    }
}

impl VisualDebugger {
    pub fn run(server: Arc<Server>) {
        let app = VisualDebugger {
            name: "Debugger".to_string(),
            server,
            selected_map: None,
            selected_tab: "Map".to_string(),
            frame_history: Default::default(),
            init: false,
            map_instance_view: MapInstanceView {
                zoom: 1.0,
                zoom_center: Pos2 { x: 0.0, y: 0.0 },
                zoom_draw_rect: Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 0.0, y: 0.0 } },
            }
        };
        let native_options = eframe::NativeOptions::default();
        spawn(|| eframe::run_native(Box::new(app), native_options));
    }
    fn ui(&mut self, ui: &mut Ui) {
        self.maps_combobox(ui);
        self.map_instance_view(ui);
    }

    fn maps_combobox(&mut self, ui: &mut Ui) {
        let mut selected_text = "Select a map";
        if self.selected_map.is_some() {
            selected_text = &*self.selected_map.as_ref().unwrap();
        }
        ComboBox::from_id_source("Select map")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                self.server.maps
                    .iter()
                    .filter(|(map_name, map)| {
                        let instances = read_lock!(map.map_instances);
                        instances.len() > 0
                    })
                    .map(|(map_name, map)| map_name)
                    .for_each(|map_name| {
                        ui.selectable_value(&mut self.selected_map, Some(map_name.clone()), map_name);
                    })
            });
    }

    fn map_instance_view(&mut self, ui: &mut Ui) {
        if self.selected_map.is_none() {
            return;
        }
        let map = self.server.maps.get(&*self.selected_map.as_ref().unwrap()).unwrap();
        let mut map_instance;

        let map_instance_guard = read_lock!(map.map_instances);
        map_instance = map_instance_guard.get(0).unwrap();


        egui::SidePanel::left(format!("{} info", map.name))
            .min_width(250.0)
            .resizable(true)
            .show(ui.ctx(), |ui| {
                let map_items_guard = read_lock!(map_instance.map_items);
                let map_items = map_items_guard.iter()
                    .filter(|item| item.is_some())
                    .map(|item| item.as_ref().unwrap()).collect::<Vec<&Arc<dyn MapItem>>>();
                ui.vertical_centered(|ui| {
                    ui.heading(format!("{}", map.name));
                    ui.separator();
                })
            });
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                self.map_instance_view.draw_map_instance_view(ui, map_instance);
            });
    }
}