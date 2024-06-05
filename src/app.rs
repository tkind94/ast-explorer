use eframe::egui;
use eframe::egui::{Color32, Stroke};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    vx: f32,
    vy: f32,
    vs: f32,
    cx: f32,
    cy: f32,
    cs: f32,
    cc: Color32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "World!".to_owned(),
            value: 2.7,
            vx: 0.0,
            vy: 0.0,
            vs: 0.0,
            cx: 100.0,
            cy: 100.0,
            cs: 50.0,
            cc: Color32::BLUE,
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
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        // egui::Window::new("import code").show(ctx, |ui| {
        //     ui.label("This is a window");
        //     ui.label("You can put anything you want in here.");
        //     ui.label("For example, here is a button:");
        //     if ui.button("Click me").clicked() {
        //         self.label = "Button clicked".to_owned();
        //     }
        // });

        egui::Window::new("Testing 123").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Hello: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/main/",
            //     "Source code."
            // ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        self.cx += self.vx;
        self.cy += self.vy;
        self.cs += self.vs;

        if self.cx < 0.0 {
            self.cx = 0.0;
            self.vx = -self.vx;
        }
        if self.cy < 0.0 {
            self.cy = 0.0;
            self.vy = -self.vy;
        }
        if self.cx > ctx.input(|i| i.screen_rect().max.x) {
            self.cx = ctx.input(|i| i.screen_rect().max.x);
            self.vx = -self.vx;
        }
        if self.cy > ctx.input(|i| i.screen_rect().max.y) {
            self.cy = ctx.input(|i| i.screen_rect().max.y);
            self.vy = -self.vy;
        }

        self.vx *= 0.99;
        self.vy *= 0.99;
        self.vs *= 0.99;

        ctx.set_pixels_per_point(1.5);

        if ctx.input(|i| i.key_pressed(egui::Key::W)) {
            self.vy += -20.00
        }
        if ctx.input(|i| i.key_pressed(egui::Key::S)) {
            self.vy += 20.00
        }
        if ctx.input(|i| i.key_pressed(egui::Key::A)) {
            self.vx += -20.00
        }
        if ctx.input(|i| i.key_pressed(egui::Key::D)) {
            self.vx += 20.00
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            painter.circle(
                egui::Pos2 {
                    x: self.cx,
                    y: self.cy,
                },
                self.cs,
                self.cc,
                Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(255, 255, 255),
                },
            );
        });

        ctx.request_repaint();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
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
}
