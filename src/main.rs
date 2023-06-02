use stoch::processes::{Process, TimeSeries}; 
use stoch::rvector::{RandomVector, Sample};
use stoch::rv_functions::martingale;
use rand_distr::Uniform;
use eframe::egui::plot::{Line, Plot, PlotPoints};

fn main() -> eframe::Result<()> {
    let n = 20;
    let dist = Uniform::new(-10.0, 10.0);
    let mut rng = rand::thread_rng();

    let rv = RandomVector::new(dist, &mut rng, n);
    let p = Process::run_sim(&rv, martingale);

    let m: PlotPoints = p.into();
    
    let line = Line::new(m);
    
    //Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "basic plotter",
        native_options,
        Box::new(|cc| Box::new(stoch::TemplateApp::new(cc))),
    )
}