use eframe::egui;
use timer::TimerApp;

mod timer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let duration_minutes = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([240.0, 80.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Timer",
        options,
        Box::new(move |_cc| Box::new(TimerApp::new(duration_minutes))),
    );
}
