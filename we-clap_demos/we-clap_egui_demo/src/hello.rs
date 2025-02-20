use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about)]
pub struct Opts {
    /// Name to greet
    #[arg(short, long)]
    pub name: Option<String>,

    /// Value
    #[arg(short, long)]
    pub value: Option<f32>,

    /// Zoom factor
    #[arg(short, long)]
    pub zoom: Option<f32>,

    /// Start fresh, do not load stored state
    #[arg(short, long)]
    pub fresh: bool,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Hello {
    // Example stuff:
    pub greeting: String,
    pub value: f32,
}

impl Default for Hello {
    fn default() -> Self {
        Self {
            // Example stuff:
            greeting: "Hello World!".to_owned(),
            value: f32::default(),
        }
    }
}

impl Hello {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, opts: Opts) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        let storage = if opts.fresh { None } else { cc.storage };
        let mut app: Hello = match storage {
            Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
            _ => Default::default(),
        };

        // If command line arguments were given, use them.
        if let Some(ref name) = &opts.name {
            app.greeting = format!("Hello {}!", name);
        }
        if let Some(value) = opts.value {
            app.value = value;
        }
        if let Some(zoom) = opts.zoom {
            cc.egui_ctx.set_zoom_factor(zoom);
        }

        app
    }
}

impl eframe::App for Hello {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.greeting);
            });
            ui.separator();
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            ui.separator();
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to(
                    "eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
        });
    }
}
