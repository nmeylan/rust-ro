use eframe::egui;

pub mod visual_debugger;
mod frame_history;
mod map_instance_view;
pub mod multi_player_simulator;

pub trait View<R> {
    fn ui(&mut self, ui: &mut egui::Ui) -> R;
}
pub trait Window<R> {
    /// Is the demo enabled for this integration?
    fn is_enabled(&self, _ctx: &eframe::egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool) -> Option<R>;
}