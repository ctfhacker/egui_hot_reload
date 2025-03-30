use shared::AppState;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update(ctx: &egui::Context, state: &mut AppState) {
    // Use a central panel and get the painter to draw custom shapes.
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Count by 1").clicked() {
            state.counter += 1;
        }

        if ui.button("Count by 2").clicked() {
            state.counter += 2;
        }

        ui.label(format!("Counter: {}", state.counter));
    });
}
