// mod ui;
use eframe::egui;
fn main() {
    let native_options = eframe::NativeOptions::default();

    let _app = eframe::run_native(
        "Power Schedule",
        native_options,
        Box::new(|cc| Ok(Box::new(PowerScheduleEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct PowerScheduleEguiApp {}

impl PowerScheduleEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for PowerScheduleEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello Power Schedule!");
        });
    }
}
