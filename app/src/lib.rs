use shared::AppState;

use egui::{Color32, Painter, Pos2, Rect, Stroke, StrokeKind, Vec2};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update(ctx: &egui::Context, state: &mut AppState) {
    // Use a central panel and get the painter to draw custom shapes.
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Hi!").clicked() {
            state.counter += 1;
        }

        ui.label(format!("Counter: {}", state.counter));
    });
}
