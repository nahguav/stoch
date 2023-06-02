fn main() -> eframe::Result<()> {
    // let n = 20;
    // let dist = Uniform::new(-10.0, 10.0);
    // let mut rng = rand::thread_rng();

    // let rv = RandomVector::new(dist, &mut rng, n);
    // let p = Process::run_sim(&rv, martingale);

    // let m: PlotPoints = p.into();
    
    // let line = Line::new(m);
    
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "basic plotter",
        native_options,
        Box::new(|cc| Box::new(stoch::TemplateApp::new(cc))),
    )
}