// mod ui;
use eframe::egui;
fn main() {
    //create an eframe window for egui
    let native_options = eframe::NativeOptions::default();

    let _app = eframe::run_native(
        "Power Schedule",
        native_options,
        Box::new(|cc| Ok(Box::new(PowerScheduleEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct PowerScheduleEguiApp {
    clock: chrono::offset::Local::,
}

impl PowerScheduleEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for PowerScheduleEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.heading("   ");

            println!("response: {:?}", response);
            println!("Self::::::: {:?}", self.clock);
            println!("====================================================");
        });
    }
}
