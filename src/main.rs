use chrono::{Duration, Local};
use eframe::egui;

struct TimerApp {
    start_time: chrono::DateTime<Local>,
    duration_minutes: u32,
    first_frame: bool,
}

impl TimerApp {
    fn new(duration_minutes: u32) -> Self {
        Self {
            start_time: Local::now(),
            duration_minutes,
            first_frame: true,
        }
    }

    fn get_remaining_time(&self) -> Duration {
        let now = Local::now();
        let elapsed = now - self.start_time;
        let total_duration = Duration::minutes(self.duration_minutes as i64);
        total_duration - elapsed
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.first_frame {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(0.0, 0.0)));
            self.first_frame = false;
        }

        let remaining = self.get_remaining_time();
        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        let seconds = remaining.num_seconds() % 60;

        egui::CentralPanel::default().frame(
            egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(30, 30, 30, 50)),
        ).show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let remaining_minutes = remaining.num_minutes();
                    let color = if remaining_minutes < 15 {
                        egui::Color32::RED
                    } else {
                        egui::Color32::WHITE
                    };
                    ui.heading(
                        egui::RichText::new(format!(
                            "{:02} h {:02} min {:02} s",
                            hours, minutes, seconds
                        ))
                        .color(color)
                        .size(26.0)
                    );
                },
            );
        });
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let duration_str = args.get(1).cloned();

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
        Box::new(move |_cc| {
            let duration_minutes = duration_str
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            Box::new(TimerApp::new(duration_minutes))
        }),
    );
}
