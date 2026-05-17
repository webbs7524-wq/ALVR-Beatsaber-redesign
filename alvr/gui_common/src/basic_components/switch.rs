use crate::theme;
use egui::{self, Color32, Response, Sense, Stroke, StrokeKind, Ui, WidgetInfo, WidgetType};

pub fn switch(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = egui::vec2(58.0, 30.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, true, *on, ""));

    let how_on = ui.ctx().animate_bool(response.id, *on);
    let visuals = ui.style().interact(&response);
    let rect = rect.expand(visuals.expansion);
    let radius = 0.5 * rect.height();
    let track_fill = if *on {
        theme::ACCENT_DARK
    } else {
        theme::DARKER_BG
    };
    let track_stroke = if *on {
        theme::ACCENT
    } else if response.hovered() {
        theme::ACCENT_RED
    } else {
        theme::SEPARATOR_BG
    };

    ui.painter().rect(
        rect,
        radius,
        track_fill,
        Stroke::new(1.5, track_stroke),
        StrokeKind::Middle,
    );

    if response.hovered() || *on {
        ui.painter().rect_stroke(
            rect.expand(2.0),
            radius + 2.0,
            Stroke::new(
                1.0,
                if *on {
                    theme::ACCENT
                } else {
                    theme::ACCENT_RED
                },
            ),
            StrokeKind::Middle,
        );
    }

    let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
    let center = egui::pos2(circle_x, rect.center().y);
    let thumb_fill = if *on { theme::FG } else { theme::MUTED_FG };
    let thumb_stroke = if *on {
        Stroke::new(1.5, theme::ACCENT)
    } else {
        Stroke::new(1.0, Color32::from_rgb(95, 103, 136))
    };
    ui.painter()
        .circle(center, 0.72 * radius, thumb_fill, thumb_stroke);

    response
}
