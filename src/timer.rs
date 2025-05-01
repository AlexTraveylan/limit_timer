use chrono::{Duration, Local};
use eframe::egui;

pub struct TimerApp {
    start_time: chrono::DateTime<Local>,
    duration_minutes: u32,
    first_frame: bool,
}

impl TimerApp {
    pub fn new(duration_minutes: u32) -> Self {
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
        let color = if remaining.num_minutes() < 15 {
            egui::Color32::RED
        } else {
            egui::Color32::WHITE
        };

        egui::CentralPanel::default().frame(
            egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(30, 30, 30, 50)),
        ).show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.heading(
                        egui::RichText::new(format!(
                            "{:02} h {:02} min {:02} s",
                            remaining.num_hours(),
                            remaining.num_minutes() % 60,
                            remaining.num_seconds() % 60
                        ))
                        .color(color)
                        .size(26.0)
                    );
                },
            );
        });
    }
}