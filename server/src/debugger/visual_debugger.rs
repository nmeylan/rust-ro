use std::any::Any;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::spawn;
use eframe::egui::{Align, ComboBox, Layout, Pos2, Rect, Vec2, Visuals};
use eframe::epi;
use epi::egui;
use egui::{Ui};
use crate::server::server::Server;
use lazy_static::lazy_static;
use crate::debugger::frame_history;
use crate::debugger::map_instance_view::MapInstanceView;
use crate::server::core::character::Character;
use crate::server::core::map::MapItem;
use crate::server::core::mob::Mob;
use crate::server::enums::map_item::MapItemType;
use crate::util::coordinate;

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
                cursor_pos: Default::default(),
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

        let map_items_guard = read_lock!(map_instance.map_items);
        let map_items = map_items_guard.clone();
        let players = map_items.iter()
            .filter(|item| item.is_some() && item.as_ref().unwrap().object_type() == 1).map(|item| cast!(item.as_ref().unwrap(), Character))
            .collect::<Vec<&Character>>();
        egui::SidePanel::left(format!("{} info", map.name))
            .min_width(250.0)
            .resizable(true)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::top_down(Align::Min), |ui| {
                        ui.heading(format!("{}", map.name));
                        ui.separator();
                        ui.label("Characters:");
                        players.iter().for_each(|player| {
                            ui.label(format!("{} {},{}", player.name, player.x.load(Relaxed), player.y.load(Relaxed)));
                        });
                        if self.map_instance_view.cursor_pos.is_some() {
                            ui.separator();
                            let i = self.map_instance_view.cursor_pos.as_ref().unwrap().x as u16;
                            let j = self.map_instance_view.cursor_pos.as_ref().unwrap().y as u16;
                            ui.label(format!("Cursor: {}, {}", i, j));
                            let map_item = map_items.get(coordinate::get_cell_index_of(i, j, map_instance.x_size)).unwrap();
                            if map_item.is_some() {
                                let map_item = map_item.as_ref().unwrap();
                                ui.label(format!("{}: {}", MapItemType::from(map_item.object_type()), map_item.name()));

                                if map_item.as_any().downcast_ref::<Mob>().is_some() {
                                    let mob = cast!(map_item, Mob);
                                    ui.label("Items in Field of view");
                                    let mob_map_view = read_lock!(mob.map_view);
                                    mob_map_view.iter()
                                        .filter(|item| item.is_some())
                                        .map(|item| item.as_ref().unwrap())
                                        .for_each(|item| {
                                        ui.label(format!("{}: {} {},{}", MapItemType::from(item.object_type()), item.name(), item.x(), item.y()));
                                    });
                                } else if map_item.as_any().downcast_ref::<Character>().is_some() {
                                    let character = cast!(map_item, Character);
                                    ui.label("Items in Field of view");
                                    let character_map_view = read_lock!(character.map_view);
                                    character_map_view.iter()
                                        .filter(|item| item.is_some())
                                        .map(|item| item.as_ref().unwrap())
                                        .for_each(|item| {
                                            ui.label(format!("{}: {} {},{}", MapItemType::from(item.object_type()), item.name(), item.x(), item.y()));
                                        });
                                }

                            }
                        }
                    });
                })
            });
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                self.map_instance_view.draw_map_instance_view(ui, map_instance, map_items);
            });
    }
}