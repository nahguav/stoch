use stochastic_graphing::app;
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "basic plotter",
        native_options,
        Box::new(|cc| Box::new(crate::app::GraphApp::new(cc))),
    )
}