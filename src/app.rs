use eframe::egui::plot::{Plot, Line, PlotPoints};
use crate::processes::{Process, TimeSeries, TimePoint}; 
use crate::rvector::{RandomVector, Sample};
use crate::rv_functions::martingale;
use rand_distr::Uniform;

pub struct TemplateApp {
    // todo: add serialization of lines?
    lines: Vec<Process<TimePoint>>
}

impl Default for TemplateApp {
    fn default() -> Self {
        let n = 20;
        let dist = Uniform::new(-10.0, 10.0);
        let mut rng = rand::thread_rng();
    
        let rv = RandomVector::new(dist, &mut rng, n);
        let p = Process::run_sim(&rv, martingale);
    
        //let m: PlotPoints = p.into();
        let lines = vec![p];
        //let line = Line::new(m);         

        Self {
            lines: lines,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { lines } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {  
            for p in lines{
                let m: PlotPoints = p.mut_into();
                let l = Line::new(m);   
                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(l));
            }        
            egui::warn_if_debug_build(ui);
        });
    }
}