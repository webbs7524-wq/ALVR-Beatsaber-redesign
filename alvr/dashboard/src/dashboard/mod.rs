mod components;

use self::components::{
    DevicesTab, LogsTab, NotificationBar, SettingsTab, SetupWizard, SetupWizardRequest,
};
use crate::{
    DataSources,
    dashboard::components::{CloseAction, NewVersionPopup, StatisticsTab},
};
use alvr_common::{
    LogEntry,
    parking_lot::{Condvar, Mutex},
};
use alvr_events::EventType;
use alvr_gui_common::theme;
use alvr_packets::{ClientConnectionsAction, PathValuePair};
use alvr_session::SessionConfig;
use eframe::egui::{
    self, Align, Button, CentralPanel, Color32, CornerRadius, Frame, Layout, Margin, Panel,
    Response, RichText, Stroke, Ui,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerRequest {
    Log(LogEntry),
    GetSession,
    UpdateSession(Box<SessionConfig>),
    SetSessionValues(Vec<PathValuePair>),
    UpdateClientList {
        hostname: String,
        action: ClientConnectionsAction,
    },
    CaptureFrame,
    InsertIdr,
    StartRecording,
    StopRecording,
    AddFirewallRules,
    RemoveFirewallRules,
    GetDriverList,
    RegisterAlvrDriver,
    UnregisterDriver(PathBuf),
    RestartSteamvr,
    ShutdownSteamvr,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tab {
    Devices,
    Statistics,
    Settings,
    #[cfg(not(target_arch = "wasm32"))]
    Installation,
    Logs,
    Debug,
    About,
}

pub struct Dashboard {
    data_sources: DataSources,
    just_opened: bool,
    server_restarting: Arc<Mutex<bool>>,
    server_restarting_condvar: Arc<Condvar>,
    selected_tab: Tab,
    tab_labels: BTreeMap<Tab, &'static str>,
    connections_tab: DevicesTab,
    statistics_tab: StatisticsTab,
    settings_tab: SettingsTab,
    #[cfg(not(target_arch = "wasm32"))]
    installation_tab: components::InstallationTab,
    logs_tab: LogsTab,
    notification_bar: NotificationBar,
    setup_wizard: SetupWizard,
    new_version_popup: Option<components::NewVersionPopup>,
    setup_wizard_open: bool,
    session: Option<SessionConfig>,
}

impl Dashboard {
    pub fn new(creation_context: &eframe::CreationContext<'_>, data_sources: DataSources) -> Self {
        alvr_gui_common::theme::set_theme(&creation_context.egui_ctx);

        data_sources.request(ServerRequest::GetSession);

        Self {
            data_sources,
            just_opened: true,
            server_restarting: Arc::new(Mutex::new(false)),
            server_restarting_condvar: Arc::new(Condvar::new()),
            selected_tab: Tab::Devices,
            tab_labels: [
                (Tab::Devices, "🔌  Devices"),
                (Tab::Statistics, "📈  Statistics"),
                (Tab::Settings, "🔧  Settings"),
                #[cfg(not(target_arch = "wasm32"))]
                (Tab::Installation, "💾  Installation"),
                (Tab::Logs, "📝  Logs"),
                (Tab::Debug, "🐞  Debug"),
                (Tab::About, "ℹ  About"),
            ]
            .into_iter()
            .collect(),
            connections_tab: DevicesTab::new(),
            statistics_tab: StatisticsTab::new(),
            settings_tab: SettingsTab::new(),
            #[cfg(not(target_arch = "wasm32"))]
            installation_tab: components::InstallationTab::new(),
            logs_tab: LogsTab::new(),
            notification_bar: NotificationBar::new(),
            setup_wizard: SetupWizard::new(),
            setup_wizard_open: false,
            session: None,
            new_version_popup: None,
        }
    }

    // This call may block
    fn restart_steamvr(&self, requests: &mut Vec<ServerRequest>) {
        requests.push(ServerRequest::RestartSteamvr);

        let mut server_restarting_lock = self.server_restarting.lock();

        if *server_restarting_lock {
            self.server_restarting_condvar
                .wait(&mut server_restarting_lock);
        }

        *server_restarting_lock = true;

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn({
            let server_restarting = Arc::clone(&self.server_restarting);
            let condvar = Arc::clone(&self.server_restarting_condvar);
            move || {
                crate::steamvr_launcher::LAUNCHER.lock().restart_steamvr();

                *server_restarting.lock() = false;
                condvar.notify_one();
            }
        });
    }
}

fn nav_tab(ui: &mut Ui, selected: bool, label: &str) -> Response {
    let width = ui.available_width();
    let text = RichText::new(label).size(15.0).strong().color(if selected {
        theme::FG
    } else {
        theme::MUTED_FG
    });

    ui.add(
        Button::new(text)
            .selected(selected)
            .fill(if selected {
                theme::ACCENT_DARK
            } else {
                Color32::from_rgba_premultiplied(0, 0, 0, 0)
            })
            .stroke(Stroke::new(
                if selected { 1.5 } else { 1.0 },
                if selected {
                    theme::ACCENT
                } else {
                    Color32::from_rgba_premultiplied(70, 82, 126, 90)
                },
            ))
            .corner_radius(CornerRadius::same(theme::PILL_ROUNDING))
            .min_size(egui::vec2(width, 38.0)),
    )
}

fn sidebar_action(ui: &mut Ui, label: &str) -> Response {
    let width = ui.available_width();
    ui.add(
        Button::new(RichText::new(label).strong().color(theme::FG))
            .fill(theme::ACCENT_RED_DARK)
            .stroke(Stroke::new(1.0, theme::ACCENT_RED))
            .corner_radius(CornerRadius::same(theme::PILL_ROUNDING))
            .min_size(egui::vec2(width, 36.0)),
    )
}

fn steamvr_status(ui: &mut Ui, connected: bool) {
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        ui.label(RichText::new("SteamVR").size(13.0).color(theme::MUTED_FG));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let (label, fill, stroke, text_color) = if connected {
                (
                    "Connected",
                    Color32::from_rgba_premultiplied(25, 100, 73, 160),
                    theme::OK_GREEN,
                    theme::OK_GREEN,
                )
            } else {
                (
                    "Disconnected",
                    Color32::from_rgba_premultiplied(115, 20, 47, 150),
                    theme::KO_RED,
                    theme::KO_RED,
                )
            };

            theme::pill_frame(fill, stroke).show(ui, |ui| {
                ui.label(RichText::new(label).size(12.0).strong().color(text_color));
            });
        });
    });
}

fn page_header(ui: &mut Ui, label: &str) {
    ui.horizontal(|ui| {
        let (_, accent_rect) = ui.allocate_space(egui::vec2(5.0, 28.0));
        ui.painter()
            .rect_filled(accent_rect, CornerRadius::same(4), theme::ACCENT);
        ui.heading(RichText::new(label).size(26.0).strong().color(theme::FG));
    });

    let (_, line_rect) = ui.allocate_space(egui::vec2(ui.available_width(), 4.0));
    let mid = line_rect.center().x;
    ui.painter().line_segment(
        [
            egui::pos2(line_rect.left(), line_rect.center().y),
            egui::pos2(mid, line_rect.center().y),
        ],
        Stroke::new(1.5, theme::ACCENT_RED),
    );
    ui.painter().line_segment(
        [
            egui::pos2(mid, line_rect.center().y),
            egui::pos2(line_rect.right(), line_rect.center().y),
        ],
        Stroke::new(1.5, theme::ACCENT),
    );
    ui.add_space(10.0);
}

fn paint_saber_background(ui: &mut Ui) {
    let rect = ui.max_rect();
    let painter = ui.painter();
    let blue = Color32::from_rgba_premultiplied(0, 210, 255, 42);
    let red = Color32::from_rgba_premultiplied(255, 43, 92, 42);

    painter.line_segment(
        [
            egui::pos2(rect.left() - 120.0, rect.bottom() - 40.0),
            egui::pos2(rect.left() + rect.width() * 0.70, rect.top() - 30.0),
        ],
        Stroke::new(3.0, red),
    );
    painter.line_segment(
        [
            egui::pos2(rect.left() + rect.width() * 0.32, rect.bottom() + 40.0),
            egui::pos2(rect.right() + 100.0, rect.top() + 55.0),
        ],
        Stroke::new(3.0, blue),
    );
    painter.line_segment(
        [
            egui::pos2(rect.left() + 12.0, rect.top() + 24.0),
            egui::pos2(rect.right() - 12.0, rect.top() + 24.0),
        ],
        Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 117, 170, 45)),
    );
}

impl eframe::App for Dashboard {
    fn ui(&mut self, ui: &mut Ui, _: &mut eframe::Frame) {
        let mut requests = vec![];

        let connected_to_server = self.data_sources.server_connected();

        while let Some(event) = self.data_sources.poll_event() {
            self.logs_tab.push_event(event.inner.clone());

            match event.inner.event_type {
                EventType::Log(log_event) => {
                    self.notification_bar
                        .push_notification(log_event, event.from_dashboard);
                }
                EventType::GraphStatistics(graph_statistics) => self
                    .statistics_tab
                    .update_graph_statistics(graph_statistics),
                EventType::StatisticsSummary(statistics) => {
                    self.statistics_tab.update_statistics(statistics)
                }
                EventType::Session(session) => {
                    let settings = session.to_settings();

                    self.connections_tab.update_client_list(&session);
                    self.settings_tab.update_session(&session.session_settings);
                    self.logs_tab.update_settings(&settings);
                    self.notification_bar.update_settings(&settings);
                    if self.just_opened {
                        if settings.extra.open_setup_wizard {
                            self.setup_wizard_open = true;
                        }

                        self.just_opened = false;
                    }

                    self.session = Some(*session);
                }
                EventType::ServerRequestsSelfRestart => self.restart_steamvr(&mut requests),
                #[cfg(not(target_arch = "wasm32"))]
                EventType::DriversList(list) => self.installation_tab.update_drivers(list),
                EventType::Adb(adb_event) => self
                    .connections_tab
                    .update_adb_download_progress(adb_event.download_progress),
                EventType::NewVersionFound { version, message } => {
                    self.new_version_popup = Some(NewVersionPopup::new(version, message));
                }
                EventType::DebugGroup { .. }
                | EventType::Tracking(_)
                | EventType::Buttons(_)
                | EventType::Haptics(_) => (),
            }
        }

        if *self.server_restarting.lock() {
            CentralPanel::default()
                .frame(Frame::new().fill(theme::BG))
                .show_inside(ui, |ui| {
                    paint_saber_background(ui);
                    // todo: find a way to center both vertically and horizontally
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.heading(
                            RichText::new("SteamVR is restarting")
                                .size(30.0)
                                .color(theme::FG),
                        );
                    });
                });

            return;
        }

        self.notification_bar.ui(ui);

        if self.setup_wizard_open {
            CentralPanel::default()
                .frame(Frame::new().inner_margin(Margin::same(22)).fill(theme::BG))
                .show_inside(ui, |ui| {
                    paint_saber_background(ui);
                    if let Some(request) = self.setup_wizard.ui(ui) {
                        match request {
                            SetupWizardRequest::ServerRequest(request) => {
                                requests.push(request);
                            }
                            SetupWizardRequest::Close { finished } => {
                                if finished {
                                    requests.push(ServerRequest::SetSessionValues(vec![
                                        PathValuePair {
                                            path: alvr_packets::parse_path(
                                                "session_settings.extra.open_setup_wizard",
                                            ),
                                            value: serde_json::Value::Bool(false),
                                        },
                                    ]))
                                }

                                self.setup_wizard_open = false;
                            }
                        }
                    }
                });
        } else {
            Panel::left("side_panel")
                .resizable(false)
                .frame(
                    Frame::new()
                        .fill(theme::LIGHTER_BG)
                        .inner_margin(Margin::same(10))
                        .stroke(Stroke::new(1.0, theme::SEPARATOR_BG)),
                )
                .exact_size(190.0)
                .show_inside(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.heading(RichText::new("ALVR").size(29.0).strong().color(theme::FG));
                        ui.label(
                            RichText::new("SABER LINK")
                                .size(11.0)
                                .strong()
                                .color(theme::ACCENT),
                        );
                        egui::warn_if_debug_build(ui);
                    });

                    ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                        ui.add_space(12.0);
                        for (tab, label) in &self.tab_labels {
                            if nav_tab(ui, self.selected_tab == *tab, label).clicked() {
                                self.selected_tab = *tab;
                            }
                        }
                    });

                    #[cfg(not(target_arch = "wasm32"))]
                    ui.with_layout(
                        Layout::bottom_up(Align::Center).with_cross_justify(true),
                        |ui| {
                            ui.add_space(5.0);

                            if connected_to_server {
                                if sidebar_action(ui, "Restart SteamVR").clicked() {
                                    self.restart_steamvr(&mut requests);
                                }
                            } else if sidebar_action(ui, "Launch SteamVR").clicked() {
                                crate::steamvr_launcher::LAUNCHER.lock().launch_steamvr();
                            }

                            steamvr_status(ui, connected_to_server);
                        },
                    )
                });

            CentralPanel::default()
                .frame(Frame::new().inner_margin(Margin::same(22)).fill(theme::BG))
                .show_inside(ui, |ui| {
                    paint_saber_background(ui);
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        page_header(ui, self.tab_labels[&self.selected_tab]);
                        match self.selected_tab {
                            Tab::Devices => {
                                requests.extend(self.connections_tab.ui(ui, connected_to_server));
                            }
                            Tab::Statistics => {
                                if let Some(request) = self.statistics_tab.ui(ui) {
                                    requests.push(request);
                                }
                            }
                            Tab::Settings => {
                                requests.extend(self.settings_tab.ui(ui));
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            Tab::Installation => {
                                for request in self.installation_tab.ui(ui) {
                                    match request {
                                        components::InstallationTabRequest::OpenSetupWizard => {
                                            self.setup_wizard_open = true
                                        }
                                        components::InstallationTabRequest::ServerRequest(
                                            request,
                                        ) => {
                                            requests.push(request);
                                        }
                                    }
                                }
                            }
                            Tab::Logs => self.logs_tab.ui(ui),
                            Tab::Debug => {
                                if let Some(request) = components::debug_tab_ui(ui) {
                                    requests.push(request);
                                }
                            }
                            Tab::About => components::about_tab_ui(ui),
                        }
                    })
                });
        }

        let shutdown_alvr = || {
            self.data_sources.request(ServerRequest::ShutdownSteamvr);

            crate::steamvr_launcher::LAUNCHER
                .lock()
                .ensure_steamvr_shutdown();
        };

        if let Some(popup) = &self.new_version_popup
            && let Some(action) = popup.ui(ui, shutdown_alvr)
        {
            if let CloseAction::CloseWithRequest(request) = action {
                requests.push(request);
            }

            self.new_version_popup = None;
        }

        for request in requests {
            self.data_sources.request(request);
        }

        if ui.input(|state| state.viewport().close_requested())
            && self.session.as_ref().is_some_and(|s| {
                s.to_settings()
                    .extra
                    .steamvr_launcher
                    .open_close_steamvr_with_dashboard
            })
        {
            shutdown_alvr();
        }
    }
}
