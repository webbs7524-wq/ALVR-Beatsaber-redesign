use egui::{
    self, Color32, Context, CornerRadius, Frame, Margin, Stroke, TextStyle, ThemePreference,
    Visuals,
};

pub const ACCENT: Color32 = Color32::from_rgb(0, 210, 255);
pub const ACCENT_DARK: Color32 = Color32::from_rgb(0, 92, 128);
pub const ACCENT_RED: Color32 = Color32::from_rgb(255, 43, 92);
pub const ACCENT_RED_DARK: Color32 = Color32::from_rgb(117, 18, 48);
pub const BG: Color32 = Color32::from_rgb(9, 10, 18);
pub const LIGHTER_BG: Color32 = Color32::from_rgb(17, 19, 34);
pub const SECTION_BG: Color32 = Color32::from_rgb(22, 24, 43);
pub const DARKER_BG: Color32 = Color32::from_rgb(7, 8, 14);
pub const ELEVATED_BG: Color32 = Color32::from_rgb(30, 33, 56);
pub const SEPARATOR_BG: Color32 = Color32::from_rgb(64, 73, 111);
pub const MUTED_FG: Color32 = Color32::from_rgb(171, 181, 210);
pub const FG: Color32 = Color32::from_rgb(246, 249, 255);
pub const SCROLLBAR_DOT_DIAMETER: f32 = 20.0;
pub const SWITCH_DOT_DIAMETER: f32 = SCROLLBAR_DOT_DIAMETER;
pub const FRAME_PADDING: f32 = 10.0;
pub const CORNER_RADIUS: u8 = 18;
pub const ROUNDING: u8 = CORNER_RADIUS;
pub const ROUNDING_SMALL: u8 = 12;
pub const PILL_ROUNDING: u8 = 28;
pub const FRAME_TEXT_SPACING: f32 = 5.0;

pub const OK_GREEN: Color32 = Color32::from_rgb(38, 237, 145);
pub const KO_RED: Color32 = ACCENT_RED;

pub mod log_colors {
    use egui::epaint::Color32;

    pub const ERROR_LIGHT: Color32 = Color32::from_rgb(255, 60, 105);
    pub const WARNING_LIGHT: Color32 = Color32::from_rgb(255, 205, 74);
    pub const INFO_LIGHT: Color32 = Color32::from_rgb(0, 210, 255);
    pub const DEBUG_LIGHT: Color32 = Color32::from_rgb(171, 181, 210);
    pub const EVENT_LIGHT: Color32 = Color32::from_rgb(136, 150, 190);
}

// Graph colors
pub mod graph_colors {
    use egui::Color32;

    pub const RENDER_EXTERNAL: Color32 = Color32::from_rgb(76, 82, 112);
    pub const RENDER_EXTERNAL_LABEL: Color32 = Color32::from_rgb(150, 158, 188);
    pub const RENDER: Color32 = super::ACCENT_RED;
    pub const IDLE: Color32 = Color32::from_rgb(255, 211, 77);
    pub const TRANSCODE: Color32 = Color32::from_rgb(38, 237, 145);
    pub const NETWORK: Color32 = super::ACCENT;

    pub const SERVER_FPS: Color32 = super::ACCENT;
    pub const CLIENT_FPS: Color32 = Color32::from_rgb(255, 211, 77);

    pub const INITIAL_CALCULATED_THROUGHPUT: Color32 = Color32::from_rgb(136, 150, 190);
    pub const ENCODER_DECODER_LATENCY_LIMITER: Color32 = TRANSCODE;
    pub const NETWORK_LATENCY_LIMITER: Color32 = NETWORK;
    pub const MIN_MAX_LATENCY_THROUGHPUT: Color32 = super::ACCENT_RED;
    pub const REQUESTED_BITRATE: Color32 = Color32::from_rgb(38, 237, 145);
    pub const RECORDED_THROUGHPUT: Color32 = Color32::from_rgb(255, 211, 77);
    pub const RECORDED_BITRATE: Color32 = super::FG;
}

pub fn section_frame() -> Frame {
    Frame::new()
        .fill(SECTION_BG)
        .inner_margin(egui::vec2(16.0, 14.0))
        .corner_radius(CornerRadius::same(ROUNDING))
        .stroke(Stroke::new(1.0, SEPARATOR_BG))
}

pub fn subsection_frame() -> Frame {
    Frame::new()
        .fill(ELEVATED_BG)
        .inner_margin(egui::vec2(14.0, 12.0))
        .corner_radius(CornerRadius::same(ROUNDING_SMALL))
        .stroke(Stroke::new(1.0, Color32::from_rgb(78, 91, 138)))
}

pub fn pill_frame(fill: Color32, stroke: Color32) -> Frame {
    Frame::new()
        .fill(fill)
        .inner_margin(Margin::symmetric(12, 5))
        .corner_radius(CornerRadius::same(PILL_ROUNDING))
        .stroke(Stroke::new(1.0, stroke))
}

pub fn set_theme(ctx: &Context) {
    ctx.set_theme(ThemePreference::Dark);

    let mut style = (*ctx.global_style()).clone();
    style.spacing.slider_width = 230_f32; // slider width can only be set globally
    style.spacing.interact_size.x = 44.0;
    style.spacing.interact_size.y = 32.0;

    style.spacing.item_spacing = egui::vec2(13.0, 13.0);
    style.spacing.button_padding = egui::vec2(14.0, 9.0);
    style.spacing.window_margin = egui::Margin::from(FRAME_PADDING);

    style.text_styles.get_mut(&TextStyle::Body).unwrap().size = 14.0;
    style.interaction.tooltip_delay = 0.0;

    ctx.set_global_style(style);

    let mut visuals = Visuals::dark();

    let corner_radius = CornerRadius::same(CORNER_RADIUS);

    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.active.corner_radius = corner_radius;

    visuals.widgets.hovered.bg_fill = ELEVATED_BG;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.hovered.expansion = 1.0;
    visuals.widgets.hovered.corner_radius = corner_radius;

    visuals.widgets.inactive.bg_fill = LIGHTER_BG;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, SEPARATOR_BG);
    visuals.widgets.inactive.corner_radius = corner_radius;

    visuals.widgets.open.bg_fill = ELEVATED_BG;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.open.corner_radius = corner_radius;

    visuals.selection.bg_fill = ACCENT;
    visuals.selection.stroke = Stroke::new(1.0, FG);

    visuals.widgets.noninteractive.bg_fill = BG;
    visuals.panel_fill = BG;
    visuals.extreme_bg_color = DARKER_BG;
    visuals.faint_bg_color = DARKER_BG;
    visuals.hyperlink_color = ACCENT;
    visuals.window_fill = SECTION_BG;
    visuals.window_stroke = Stroke::new(1.0, SEPARATOR_BG);
    visuals.window_corner_radius = CornerRadius::same(ROUNDING);
    visuals.menu_corner_radius = CornerRadius::same(ROUNDING_SMALL);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, FG);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.5, SEPARATOR_BG);
    visuals.widgets.noninteractive.corner_radius = corner_radius;

    ctx.set_visuals(visuals);
}
