use alvr_gui_common::theme::{self, log_colors};
use eframe::{
    egui::{self, CornerRadius, Frame, Label, RichText, Stroke, Ui},
    epaint::Color32,
};

pub fn notice(ui: &mut Ui, text: &str) {
    Frame::new()
        .fill(log_colors::WARNING_LIGHT)
        .inner_margin(egui::vec2(12.0, 8.0))
        .corner_radius(CornerRadius::same(theme::ROUNDING_SMALL))
        .stroke(Stroke::new(1.0, Color32::from_rgb(255, 235, 150)))
        .show(ui, |ui| {
            ui.add(Label::new(RichText::new(text).size(11.0).color(Color32::BLACK)).wrap());
        });
}
