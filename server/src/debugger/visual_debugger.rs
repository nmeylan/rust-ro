use std::sync::Arc;
use std::thread::spawn;
use eframe::egui::{Color32, ComboBox, emath, epaint, Frame, Pos2, Rect, Sense, Stroke, Visuals};
use eframe::egui::epaint::{RectShape};
use eframe::epi;
use epi::egui;
use egui::{Ui};
use crate::server::server::Server;
use lazy_static::lazy_static;
use crate::debugger::frame_history;
use crate::server::core::map::{WALKABLE_MASK, WARP_MASK};
use crate::util::coordinate;

pub struct VisualDebugger {
    pub name : String,
    pub server: Arc<Server>,
    pub selected_map: Option<String>,
    pub selected_tab: String,
    pub zoom: f32,
    pub zoom_center: Pos2,
    pub zoom_draw_rect: Rect,
    pub init: bool,
    frame_history: frame_history::FrameHistory,
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

        frame.set_window_size(ctx.used_size());
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
            zoom: 1.0,
            zoom_center: Pos2 { x: 0.0, y: 0.0 },
            zoom_draw_rect: Rect { min: Pos2 { x: 0.0, y: 0.0 }, max: Pos2 { x: 0.0, y: 0.0 } },
            init: false,
        };
        let native_options = eframe::NativeOptions::default();
        spawn(|| eframe::run_native(Box::new(app), native_options));
    }
    fn ui(&mut self, ui: &mut Ui) {
        self.maps_combobox(ui);
        self.map_instance_cells(ui);
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

    fn map_instance_cells(&mut self, ui: &mut Ui) {
        if self.selected_map.is_none() {
            return;
        }
        Frame::dark_canvas(ui.style()).show(ui, |ui| {
            let (_id, response) = ui.allocate_exact_size(ui.available_size_before_wrap(), Sense::click_and_drag());
            let absolute_draw_rect = response.rect;
            let relative_draw_rect = Rect {
                min: Pos2 {
                    x: 0.0,
                    y: 0.0
                },
                max: Pos2 {
                    x: absolute_draw_rect.max.x - absolute_draw_rect.min.x,
                    y: absolute_draw_rect.max.y - absolute_draw_rect.min.y,
                }
            };
            if self.zoom_draw_rect.max.x == 0.0 {
                self.zoom_draw_rect = relative_draw_rect.clone();
            }
            let mut shapes = vec![];
            let margin = 0.0;
            let mut has_zoom = false;

            if ui.input().zoom_delta() != 1.0 {
                has_zoom = true;
            }
            if ui.input().zoom_delta() > 1.0 {
                self.zoom_center = ui.input().pointer.hover_pos().unwrap();
                self.zoom *= 1.5;
            } else if ui.input().zoom_delta() < 1.0 {
                self.zoom_center = ui.input().pointer.hover_pos().unwrap();
                self.zoom /= 1.5;
                if self.zoom <= 1.0 {
                    has_zoom = false;
                    self.zoom = 1.0;
                    self.zoom_center = Pos2 { x: 0.0, y: 0.0 };
                    self.zoom_draw_rect = relative_draw_rect.clone();
                }
            }

            let map = self.server.maps.get(&*self.selected_map.as_ref().unwrap()).unwrap();
            let map_instance = read_lock!(map.map_instances);
            let map_instance = map_instance.get(0).unwrap();

            let mut shape_x_size = (relative_draw_rect.max.x - (margin * 2.0) as f32) / map_instance.x_size as f32;
            let mut shape_y_size = (relative_draw_rect.max.y - (margin * 2.0) as f32) / map_instance.y_size as f32;


            if has_zoom {
                self.zoom_draw_rect.min.x = self.zoom_center.x + (relative_draw_rect.min.x - self.zoom_center.x) / (self.zoom as f32);
                self.zoom_draw_rect.max.x = self.zoom_center.x + (relative_draw_rect.max.x - self.zoom_center.x) / (self.zoom as f32);
                self.zoom_draw_rect.min.y = self.zoom_center.y - self.zoom_center.y / (self.zoom as f32);
                self.zoom_draw_rect.max.y = self.zoom_center.y + (relative_draw_rect.max.y - self.zoom_center.y) / (self.zoom as f32);
            }

            let start_j = ((self.zoom_draw_rect.min.x) / shape_x_size) as u16;
            let mut end_j = ((self.zoom_draw_rect.max.x) / shape_x_size) as u16;
            if end_j > map_instance.x_size {
                end_j = map_instance.x_size
            }
            let mut end_i = ((relative_draw_rect.max.y - self.zoom_draw_rect.min.y ) / shape_y_size) as u16;
            let start_i = ((relative_draw_rect.max.y - self.zoom_draw_rect.max.y) / shape_y_size) as u16;
            if end_i > map_instance.y_size {
                end_i = map_instance.y_size
            }

            shape_x_size = shape_x_size * self.zoom;
            shape_y_size = shape_y_size * self.zoom;

            for i in start_i..end_i {
                for j in start_j..end_j {
                    let index = coordinate::get_cell_index_of(j, i, map_instance.x_size);
                    let cell = map_instance.cells.get(index).unwrap();
                    let mut cell_color = Color32::BLACK;
                    if cell & WARP_MASK == WARP_MASK {
                        cell_color = Color32::BLUE;
                    } else if cell & WALKABLE_MASK == WALKABLE_MASK {
                        cell_color = Color32::WHITE;
                    } else {
                        continue;
                    }
                    let option = map_instance.get_map_item_at(j, i);
                    if option.is_some() {
                        let item = option.unwrap();
                        if item.object_type() == 5 {
                            cell_color = Color32::RED;
                        } else if item.object_type() == 1 {
                            cell_color = Color32::GREEN;
                        } else if item.object_type() == 6 {
                            cell_color = Color32::BLUE;
                        }
                    }
                    shapes.push(epaint::Shape::Rect(RectShape {
                        rect: emath::Rect {
                            min: Pos2 {
                                x: absolute_draw_rect.min.x + margin + (shape_x_size * (j - start_j) as f32),
                                y: absolute_draw_rect.max.y - margin - (shape_y_size * ((i - start_i) as f32 + 1.0)),
                            },
                            max: Pos2 {
                                x: absolute_draw_rect.min.x + margin + (shape_x_size * ((j - start_j) as f32 + 1.0)),
                                y: absolute_draw_rect.max.y - margin - (shape_y_size * (i - start_i) as f32),
                            }
                        },
                        corner_radius: 0.0,
                        fill: cell_color,
                        stroke: Stroke::none()
                    }));
                }
            }
            ui.painter().extend(shapes);
            ui.ctx().request_repaint();
        });
    }
}