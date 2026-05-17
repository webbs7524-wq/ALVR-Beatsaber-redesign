use crate::{DisplayString, theme};
use egui::{self, Button, CornerRadius, RichText, Stroke, Ui};

// todo: use a custom widget
pub fn button_group_clicked(
    ui: &mut Ui,
    options: &[DisplayString],
    selection: &mut String,
) -> bool {
    let mut clicked = false;
    for id in options {
        let selected = selection.as_str() == id.id.as_str();
        let text = RichText::new(&id.display)
            .color(if selected { theme::FG } else { theme::MUTED_FG })
            .strong();
        let res = ui.add(
            Button::new(text)
                .selected(selected)
                .fill(if selected {
                    theme::ACCENT_DARK
                } else {
                    theme::LIGHTER_BG
                })
                .stroke(Stroke::new(
                    if selected { 1.5 } else { 1.0 },
                    if selected {
                        theme::ACCENT
                    } else {
                        theme::SEPARATOR_BG
                    },
                ))
                .corner_radius(CornerRadius::same(theme::PILL_ROUNDING))
                .min_size(egui::vec2(72.0, 32.0)),
        );
        if res.clicked() {
            selection.clone_from(&id.id);
            clicked = true;
        }

        if cfg!(debug_assertions) {
            res.on_hover_text((**id).clone());
        }
    }

    clicked
}
