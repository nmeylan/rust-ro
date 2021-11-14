use std::sync::Arc;
use eframe::egui::{Color32, emath, epaint, Frame, Pos2, Rect, Sense, Stroke, Ui};
use eframe::egui::epaint::RectShape;
use crate::server::core::map::{WALKABLE_MASK, WARP_MASK};
use crate::server::core::map_instance::MapInstance;
use crate::util::coordinate;

pub struct MapInstanceView {
    pub zoom: f32,
    pub zoom_center: Pos2,
    pub zoom_draw_rect: Rect,
}

impl MapInstanceView {
    pub fn draw_map_instance_view(&mut self,  ui: &mut Ui, map_instance: &Arc<MapInstance>) {
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