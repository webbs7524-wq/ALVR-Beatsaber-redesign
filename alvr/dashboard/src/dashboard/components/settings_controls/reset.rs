use alvr_gui_common::theme;
use eframe::{
    egui::{Button, CornerRadius, Layout, Response, Stroke, Ui},
    emath::Align,
};

pub fn reset_button(ui: &mut Ui, enabled: bool, default_str: &str) -> Response {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_space(5.0);

        ui.add_enabled(
            enabled,
            Button::new("⟲")
                .fill(theme::LIGHTER_BG)
                .stroke(Stroke::new(1.0, theme::SEPARATOR_BG))
                .corner_radius(CornerRadius::same(theme::PILL_ROUNDING)),
        )
        .on_hover_text(format!("Reset to {default_str}"))
    })
    .inner
}
