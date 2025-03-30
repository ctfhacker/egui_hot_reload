mod dl;
use dl::AppFuncs;

use shared::AppState;

/// Our main eframe application that uses the hot-reloaded UI.
struct MainApp {
    app_func: Option<AppFuncs>,
    state: AppState,
}

impl MainApp {
    fn new() -> Self {
        MainApp {
            app_func: Some(dl::get_app_func()),
            state: AppState::default(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Reload the library
        let new_app_func = self
            .app_func
            .take()
            .expect("AppFunc should never be null")
            .reload();

        // Update using the currently loaded library
        (new_app_func.update_fn)(ctx, &mut self.state);

        // ctx.request_repaint();

        // Update the internal app_func
        self.app_func = Some(new_app_func);
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hot Reloadable egui App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MainApp::new()))),
    );
}
