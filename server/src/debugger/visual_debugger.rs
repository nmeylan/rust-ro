use std::borrow::Borrow;
use std::sync::Arc;
use std::thread;
use eframe::{CreationContext, egui, HardwareAcceleration, Theme};
use egui::{Align, ComboBox, Layout, Pos2, Rect, Ui, Vec2, Visuals};
use crate::server::Server;
use lazy_static::lazy_static;
use crate::debugger::frame_history;
use crate::debugger::map_instance_view::MapInstanceView;
use crate::server::state::character::Character;
use crate::server::model::map_item::MapItem;
use crate::server::model::map_item::MapItemType;

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
    pub static ref TABS: Vec<&'static str> = vec!["Map"];
}

impl eframe::App for VisualDebugger {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input().time, frame.info().cpu_usage);
        frame.set_window_title(&*format!("{} {}", self.name, self.frame_history.info()));
        if !self.init {
            ctx.set_visuals(Visuals::light());
            frame.set_window_size(Vec2 {
                x: 1024.0,
                y: 768.0,
            });
            self.init = true;
        }
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in TABS.iter() {
                    if ui
                        .selectable_label(self.selected_tab == *tab, *tab)
                        .clicked()
                    {}
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

impl VisualDebugger {
    pub fn run(server: Arc<Server>) {
        let app = VisualDebugger {
            name: "Debugger".to_string(),
            server: server.clone(),
            selected_map: None,
            selected_tab: "Map".to_string(),
            frame_history: Default::default(),
            init: false,
            map_instance_view: MapInstanceView {
                cursor_pos: Default::default(),
                zoom: 1.0,
                zoom_center: Pos2 { x: 0.0, y: 0.0 },
                zoom_draw_rect: Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 0.0, y: 0.0 } },
                server: server.clone(),
            },
        };

        thread::spawn(|| {
            let native_options = eframe::NativeOptions {
                always_on_top: false,
                maximized: false,
                decorated: true,
                fullscreen: false,
                drag_and_drop_support: true,
                icon_data: None,
                initial_window_pos: None,
                initial_window_size: None,
                min_window_size: None,
                max_window_size: None,
                resizable: true,
                transparent: false,
                mouse_passthrough: false,
                vsync: true,
                multisampling: 0,
                depth_buffer: 0,
                stencil_buffer: 0,
                hardware_acceleration: HardwareAcceleration::Preferred,
                renderer: Default::default(),
                follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
                default_theme: Theme::Dark,
                run_and_return: true,
                event_loop_builder: None,
                shader_version: None,
                centered: false,
            };
            eframe::run_native("Debugger", native_options, Box::new(|_cc: &CreationContext| Box::new(app))).unwrap();
        });
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
                self.server.state().map_instances().borrow()
                    .iter()
                    .map(|(map_name, _map)| map_name)
                    .for_each(|map_name| {
                        ui.selectable_value(&mut self.selected_map, Some(map_name.clone()), map_name);
                    })
            });
    }

    fn map_instance_view(&mut self, ui: &mut Ui) {
        if self.selected_map.is_none() {
            return;
        }
        let instances = self.server.state().map_instances().borrow();
        let map_instances = instances.get(&*self.selected_map.as_ref().unwrap()).unwrap();
        let map_instance = map_instances.get(0).unwrap();
        let map_name = map_instance.name().to_string();
        let map_instance_id = map_instance.id();
        let map_items_clone = map_instance.state().map_items().clone();
        let server_characters = self.server.state().characters().borrow();
        let characters = map_items_clone.iter()
            .filter(|(_, item)| *item.object_type() == MapItemType::Character)
            .map(|(_, item)| { server_characters.get(&item.id()).unwrap() })
            .collect::<Vec<&Character>>();
        egui::SidePanel::left(format!("{} info", map_name))
            .min_width(250.0)
            .resizable(true)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::top_down(Align::Min), |ui| {
                        ui.heading(format!("{}", map_name));
                        ui.separator();
                        ui.label("Characters:");
                        characters.iter().for_each(|character| {
                            ui.label(format!("{} {},{}", character.name, character.x(), character.y()));
                        });
                        if self.map_instance_view.cursor_pos.is_some() {
                            ui.separator();
                            let i = self.map_instance_view.cursor_pos.as_ref().unwrap().x as u16;
                            let j = self.map_instance_view.cursor_pos.as_ref().unwrap().y as u16;
                            ui.label(format!("Cursor: {}, {}", i, j));
                            let map_item = map_items_clone.iter().find(|(_, map_item)| {
                                let position = self.server.state().map_item_x_y(map_item, &map_name, map_instance_id).unwrap();
                                position.x() == i && position.y() == j
                            });
                            if map_item.is_some() {
                                let (_, map_item) = map_item.unwrap().clone();
                                // map_items_mut_clone.remove(&*map_item.clone());
                                let item_name = self.server.state().map_item_name(map_item, &map_name, map_instance_id).unwrap();
                                ui.label(format!("{}: {}", map_item.object_type(), item_name));
                                if *map_item.object_type() == MapItemType::Mob {
                                    let state = map_instance.state();
                                    let mob_ref = state.get_mob(map_item.id()).unwrap();
                                    let mob = mob_ref.borrow();
                                    ui.label("Items in Field of view");
                                    mob.map_view.iter()
                                        .for_each(|item| {
                                            let item_name = self.server.state().map_item_name(item, &map_name, map_instance_id).unwrap();
                                            let position = self.server.state().map_item_x_y(item, &map_name, map_instance_id).unwrap();
                                            ui.label(format!("{}: {} {},{}", item.object_type(), item_name, position.x(), position.y()));
                                        });
                                } else if *map_item.object_type() == MapItemType::Character {
                                    let character = self.server.state().map_item_character(map_item).unwrap();
                                    ui.label("Items in Field of view");
                                    let mut character_map_view: Vec<MapItem> = character.map_view.clone().into_iter().collect::<Vec<MapItem>>();
                                    character_map_view.sort_by(|a, b| a.id().cmp(&b.id()));
                                    character_map_view.iter()
                                        .for_each(|item| {
                                            let item_name = self.server.state().map_item_name(item, &map_name, map_instance_id).unwrap();
                                            let position = self.server.state().map_item_x_y(item, &map_name, map_instance_id).unwrap();
                                            ui.label(format!("{}: {} {},{}", item.object_type(), item_name, position.x(), position.y()));
                                        });
                                }
                            }
                        }
                    });
                })
            });
        egui::CentralPanel::default()
            .show(ui.ctx(), |ui| {
                self.map_instance_view.draw_map_instance_view(ui, map_instance, map_items_clone);
            });
    }
}