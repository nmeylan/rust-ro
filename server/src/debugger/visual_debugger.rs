use std::fmt::format;
use std::sync::Arc;
use std::thread;
use eframe::{CreationContext, egui, HardwareAcceleration, Theme};
use eframe::egui::{Grid, ScrollArea, ViewportCommand};
use egui::{Align, ComboBox, Layout, Pos2, Rect, Ui, Visuals};
use crate::server::Server;
use lazy_static::lazy_static;
use crate::debugger::frame_history;
use crate::debugger::map_instance_view::MapInstanceView;
use crate::server::model::events::map_event::MapEvent;
use crate::server::state::character::Character;
use crate::server::model::map_item::MapItem;
use crate::server::model::map_item::MapItemType;

#[cfg(target_os = "linux")]
use winit::platform::x11::EventLoopBuilderExtX11;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::raw_window_handle::HasWindowHandle;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::util::debug::{WearAmmoForDisplay, WearGearForDisplay, WearWeaponForDisplay};

pub struct VisualDebugger {
    pub name: String,
    pub server: Arc<Server>,
    pub selected_tab: String,
    pub selected_char: Option<(u32, String)>,
    pub selected_map: Option<String>,
    pub selected_map_item: Option<MapItem>,
    pub init: bool,
    frame_history: frame_history::FrameHistory,
    map_instance_view: MapInstanceView,
}

lazy_static! {
    pub static ref TABS: Vec<&'static str> = vec!["Map", "Character"];
}

impl eframe::App for VisualDebugger {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        let handle = frame.window_handle().unwrap();
        ctx.send_viewport_cmd(ViewportCommand::Title(format!("{} {}", self.name, self.frame_history.info())));
        if !self.init {
            ctx.set_visuals(Visuals::light());
            self.init = true;
        }
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in TABS.iter() {
                    if ui
                        .selectable_label(self.selected_tab == *tab, *tab)
                        .clicked()
                    {
                        self.selected_tab = tab.to_string();
                    }
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
            selected_tab: "Map".to_string(),
            selected_char: None,
            selected_map: None,
            selected_map_item: None,
            frame_history: Default::default(),
            init: false,
            map_instance_view: MapInstanceView {
                cursor_pos: Default::default(),
                clicked: false,
                zoom: 1.0,
                zoom_center: Pos2 { x: 0.0, y: 0.0 },
                zoom_draw_rect: Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 0.0, y: 0.0 } },
                server: server.clone(),
            },
        };

        thread::spawn(|| {
            let native_options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
                vsync: true,
                multisampling: 0,
                depth_buffer: 0,
                stencil_buffer: 0,
                hardware_acceleration: HardwareAcceleration::Preferred,
                renderer: Default::default(),
                follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
                default_theme: Theme::Dark,
                run_and_return: true,
                event_loop_builder: Some(Box::new(move |builder| {
                    builder.with_any_thread(true);
                })),
                window_builder: None,
                shader_version: None,
                centered: false,
                persist_window: false,
                persistence_path: None,
            };
            eframe::run_native("Debugger", native_options, Box::new(|_cc: &CreationContext| Ok(Box::new(app)))).unwrap();
        });
    }
    fn ui(&mut self, ui: &mut Ui) {
        match self.selected_tab.as_str() {
            "Map" => {
                self.maps_combobox(ui);
                self.map_instance_view(ui);
            }
            "Character" => {
                self.character_combobox(ui);
                self.character_view(ui);
            }
            _ => {}
        }
    }

    fn maps_combobox(&mut self, ui: &mut Ui) {
        let mut selected_text = "Select a map";
        if self.selected_map.is_some() {
            selected_text = self.selected_map.as_ref().unwrap();
        }
        ComboBox::from_id_source("Select map")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                self.server.state().map_instances()
                    .iter()
                    .map(|(map_name, _map)| map_name)
                    .for_each(|map_name| {
                        ui.selectable_value(&mut self.selected_map, Some(map_name.clone()), map_name);
                    })
            });
    }

    fn character_combobox(&mut self, ui: &mut Ui) {
        let mut selected_text = "Select a character";
        if let Some((selected_char)) = &self.selected_char {
            selected_text = selected_char.1.as_str();
        }
        ComboBox::from_id_source("Select char")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                self.server.state().characters()
                    .iter()
                    .map(|(id, char)| char)
                    .for_each(|char| {
                        ui.selectable_value(&mut self.selected_char, Some((char.char_id, char.name.clone())), char.name.clone());
                    })
            });
    }

    fn map_instance_view(&mut self, ui: &mut Ui) {
        if self.selected_map.is_none() {
            return;
        }
        let instances = self.server.state().map_instances();
        let map_instances = instances.get(self.selected_map.as_ref().unwrap()).unwrap();
        let map_instance = map_instances.first().unwrap();
        let map_name = map_instance.name().to_string();
        let map_instance_id = map_instance.id();
        let map_items_clone = map_instance.state().map_items().clone();
        let server_characters = self.server.state().characters();
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
                        ui.heading(map_name.to_string());
                        ui.separator();
                        if ui.button("Toggle mob movement").clicked() {
                            map_instance.add_to_next_tick(MapEvent::AdminTogglePauseMobMovement);
                        }
                        if ui.button("Killall mob").clicked() {
                            map_instance.add_to_next_tick(MapEvent::AdminKillAllMobs(150000));
                        }
                        ui.separator();
                        ui.label("Characters:");
                        characters.iter().for_each(|character| {
                            ui.label(format!("{} {},{}", character.name, character.x(), character.y()));
                        });

                        if let Some(map_item) = self.selected_map_item.as_ref() {
                            ui.separator();
                            let state = map_instance.state();
                            if let Some(mob_ref) = state.get_mob(map_item.id()) {
                                ui.label(format!("Selected map item: {}: {} ({})", map_item.object_type(), mob_ref.name_english, map_item.id()));
                                if *map_item.object_type() == MapItemType::Mob {
                                    ui.label(format!("{},{}", mob_ref.x(), mob_ref.y()));
                                }
                            }
                        }
                        if self.map_instance_view.cursor_pos.is_some() {
                            ui.separator();
                            let i = self.map_instance_view.cursor_pos.as_ref().unwrap().x as u16;
                            let j = self.map_instance_view.cursor_pos.as_ref().unwrap().y as u16;
                            ui.label(format!("Cursor: {}, {}", i, j));
                            let map_item = map_items_clone.iter().find(|(_, map_item)| {
                                let position = self.server.state().map_item_x_y(map_item, &map_name, map_instance_id).unwrap();
                                position.x() == i && position.y() == j
                            }).map(|(_, map_item)| map_item);
                            if self.map_instance_view.clicked {
                                self.selected_map_item = map_item.cloned();
                            }

                            if let Some(map_item) = map_item {
                                // map_items_mut_clone.remove(&*map_item.clone());
                                let item_name = self.server.state().map_item_name(map_item, &map_name, map_instance_id).unwrap();
                                ui.label(format!("{}: {}", map_item.object_type(), item_name));
                                if *map_item.object_type() == MapItemType::Mob {
                                    let state = map_instance.state();
                                    let mob_ref = state.get_mob(map_item.id()).unwrap();
                                    let mob = mob_ref;
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
                                    character_map_view.sort_by_key(|a| a.id());
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
                self.map_instance_view.draw_map_instance_view(ui, map_instance, map_items_clone, &self.selected_map_item);
            });
    }

    fn character_view(&mut self, ui: &mut Ui) {
        if let Some((char_id, _name)) = &self.selected_char {
            if let Some(character) = self.server.state().get_character(*char_id) {
                let status = StatusService::instance().to_snapshot(&character.status);
                ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Status");
                    let grid = Grid::new("status grid").num_columns(2);
                    ui.group(|ui| {
                        ui.set_max_width(600.0);
                        ui.set_width(600.0);
                        ui.columns(3, |columns| {
                            columns[0].vertical(|ui| {
                                ui.label(format!("str: {}+{}", status.base_str(), status.bonus_str()));
                                ui.label(format!("agi: {}+{}", status.base_agi(), status.bonus_agi()));
                                ui.label(format!("vit: {}+{}", status.base_vit(), status.bonus_vit()));
                                ui.label(format!("int: {}+{}", status.base_int(), status.bonus_int()));
                                ui.label(format!("dex: {}+{}", status.base_dex(), status.bonus_dex()));
                                ui.label(format!("luk: {}+{}", status.base_luk(), status.bonus_luk()));
                            });
                            columns[1].vertical(|ui| {
                                ui.label(format!("atk: {}+{}", status.atk_left_side(), status.atk_right_side()));
                                ui.label(format!("matk: {}+{}", status.matk_min(), status.matk_min()));
                                ui.label(format!("hit: {}", status.hit()));
                                ui.label(format!("crit: {}", status.crit()));
                                ui.label(format!("def: {}+{}", status.def(), StatusService::instance().character_vit_def(&status)));
                                ui.label(format!("mdef: {}", status.mdef()));
                            });
                            columns[2].vertical(|ui| {
                                ui.label(format!("flee: {}+{}", status.flee(), status.flee()));
                                ui.label(format!("aspd: {}", status.aspd()));
                                ui.label(format!("movement speed: {}", status.speed()));
                            });
                        });
                    });


                    ui.heading("Equipment");
                    ui.group(|ui| {
                        ui.set_max_width(600.0);
                        ui.set_width(600.0);
                        character.status.equipped_weapons().iter()
                            .for_each(|item| { ui.label(format!("{}", WearWeaponForDisplay::new(&item, GlobalConfigService::instance()))); });
                        character.status.equipped_gears().iter()
                            .for_each(|item| { ui.label(format!("{}", WearGearForDisplay::new(&item, GlobalConfigService::instance()))); });
                        character.status.equipped_ammo().map(|item| { ui.label(format!("{}", WearAmmoForDisplay::new(&item, GlobalConfigService::instance()))); });
                    });
                    ui.heading("Status bonuses");
                    ui.group(|ui| {
                        ui.set_max_width(600.0);
                        ui.set_width(600.0);
                        status.bonuses().iter()
                            .for_each(|item| { ui.label(format!("{}", item)); });
                    });
                    ui.heading("Temporary bonuses");
                    ui.group(|ui| {
                        ui.set_max_width(600.0);
                        ui.set_width(600.0);
                        character.status.temporary_bonuses().iter()
                            .for_each(|item| { ui.label(format!("{}", item)); });
                    });
                });
            } else {
                self.selected_char = None;
            }
        }
    }
}