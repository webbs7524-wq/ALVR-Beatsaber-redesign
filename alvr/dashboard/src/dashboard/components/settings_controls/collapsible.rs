use super::NestingInfo;
use alvr_gui_common::theme;
use alvr_packets::PathValuePair;
use eframe::egui::{self, Button, CornerRadius, RichText, Stroke, Ui};
use serde_json as json;

pub fn collapsible_button(
    ui: &mut Ui,
    nesting_info: &NestingInfo,
    session_fragment: &mut json::Value,
    request: &mut Option<PathValuePair>,
) -> bool {
    let json::Value::Bool(state_mut) = &mut session_fragment["gui_collapsed"] else {
        unreachable!()
    };

    let label = if *state_mut { "Show" } else { "Hide" };
    let response = ui.add(
        Button::new(RichText::new(label).size(12.0).strong())
            .fill(if *state_mut {
                theme::LIGHTER_BG
            } else {
                theme::ACCENT_DARK
            })
            .stroke(Stroke::new(
                1.0,
                if *state_mut {
                    theme::SEPARATOR_BG
                } else {
                    theme::ACCENT
                },
            ))
            .corner_radius(CornerRadius::same(theme::PILL_ROUNDING))
            .min_size(egui::vec2(70.0, 28.0)),
    );

    if response.clicked() {
        *state_mut = !*state_mut;
        *request = super::get_single_value(
            nesting_info,
            "gui_collapsed".into(),
            json::Value::Bool(*state_mut),
        );
    }

    *state_mut
}
