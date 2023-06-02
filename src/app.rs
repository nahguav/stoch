use eframe::egui::plot::{Plot, Line, PlotPoints};
use crate::processes::{Process, TimeSeries, TimePoint}; 
use crate::rvector::{RandomVector, Sample};
use crate::rv_mappings::sum;
use rand_distr::{Uniform, Normal};


pub struct TemplateApp {
    // todo: add serialization of lines?
    lines: Vec<Process<TimePoint>>
}

impl Default for TemplateApp {
    fn default() -> Self {
        let n = 2000;
        let dist = Uniform::new(-1.0, 1.0);
        let mut rng = rand::thread_rng();
    
        let rv = RandomVector::new(dist, &mut rng, n);
        let p = Process::run_sim(&rv, sum);
    
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
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });
            if ui.button("Increment").clicked() {
                let n = 2000;
                let dist = Uniform::new(-1.0, 1.0);
                let mut rng = rand::thread_rng();
            
                let rv = RandomVector::new(dist, &mut rng, n);
                let p = Process::run_sim(&rv, sum);
                lines.push(p);
                let m = lines.len();
                println!("{m:}");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {  
            {
                Plot::new("plot").view_aspect(2.0).show(ui, |plot_ui| {
                    for i in 0..lines.len(){
                        let m: PlotPoints = lines[i].mut_into();
                        let l = Line::new(m);   
                        plot_ui.line(l)}
                });
            }        
            egui::warn_if_debug_build(ui);
        });
    }
}