use crate::dashboard::ServerRequest;
use alvr_gui_common::theme;
use eframe::egui::Ui;

pub fn debug_tab_ui(ui: &mut Ui) -> Option<ServerRequest> {
    let mut request = None;

    theme::section_frame().show(ui, |ui| {
        ui.label(
            "Recording from ALVR using the buttons below is not suitable for capturing gameplay.
For that, use other means of recording, for example through headset or desktop VR output.",
        );

        ui.add_space(10.0);

        ui.columns(4, |ui| {
            if ui[0].button("Capture frame").clicked() {
                request = Some(ServerRequest::CaptureFrame);
            }

            if ui[1].button("Insert IDR").clicked() {
                request = Some(ServerRequest::InsertIdr);
            }

            if ui[2].button("Start recording").clicked() {
                request = Some(ServerRequest::StartRecording);
            }

            if ui[3].button("Stop recording").clicked() {
                request = Some(ServerRequest::StopRecording);
            }
        });
    });

    request
}
