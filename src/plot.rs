mod csv_plot;
mod utility;

use eframe::{egui, epi};
// use std::fs::File;
use crate::utility::{Image, TexMngr};
use crate::csv_plot::{get_filename, load_csv};
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct PlotImage {
    // Example stuff:
    label: String,
    x_scale: f32,
    y_scale: f32,
    y_ind: usize,
    last_update: Instant,
    filename: String,
    #[cfg_attr(feature = "persistence", serde(skip))]
    image: Image,
    #[cfg_attr(feature = "persistence", serde(skip))]
    tex_mngr: TexMngr,
}

fn draw_point(image: &mut Image, x: f64, y: f64, color: egui::Color32) {
    let width = image.size.0;
    if x < 0.0 || x as usize >= width {
        return;
    }
    let height = image.size.1;
    if y < 0.0 || y as usize >= height {
        return;
    }
    let x = x as usize;
    let y = y as usize;
    let ind = y * width + x;
    image.pixels[ind] = color;
}

fn make_plot(mut image: &mut Image, filename: &str,  x_scale: f64, y_scale: f64) {
    let width = image.size.0;
    let height = image.size.1;
    let sc = 0.95;
    for i in 0..(width * height) {
        let r = image.pixels[i].r();
        let g = image.pixels[i].g();
        let b = image.pixels[i].b();
        image.pixels[i] = egui::Color32::from_rgb(
            (r as f64 * sc) as u8,
            (g as f64 * sc) as u8,
            (b as f64 * sc) as u8,
        );
    }

    let path = Path::new(filename);
    let csv_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(csv_file) => csv_file,
    };

    let columns = load_csv(csv_file).unwrap();

    for (col_ind, column) in columns.iter().enumerate() {
        // println!("{} {:?}", col_ind, column);
        let color = egui::Color32::from_rgb(
            (col_ind * 30) as u8,
            (255 - (col_ind * 20)) as u8,
            (50 + col_ind * 10) as u8);
        let tiles = 2;
        let x_offset = ((col_ind % tiles) * width / tiles + 10) as f64;
        let y_offset = (col_ind / tiles) as f64 * 180.0 + 120.0;

        for (i, val) in column.iter().enumerate() {
            let x = i as f64 * x_scale + 50.0 + x_offset;
            let y = val * y_scale + y_offset;
            draw_point(&mut image, x, height as f64 - y, color);
            draw_point(&mut image, x, height as f64 - y_offset, egui::Color32::GRAY);
        }
    }
}

impl Default for PlotImage {
    fn default() -> Self {
        let width: usize = 1000;
        let height: usize = 600;
        let size = (width, height);
        let mut pixels = Vec::new();
        for _ in 0..(size.0 * size.1) {
            pixels.push(egui::Color32::BLACK);
        }
        let mut image = Image {size, pixels};
        let filename = get_filename();
        make_plot(&mut image, &filename, 10.0, 50.0);

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            x_scale: 1.0,
            y_scale: 1.0,
            y_ind: 30,
            last_update: Instant::now(),
            filename,
            image,
            tex_mngr: Default::default(),
        }
    }
}

impl PlotImage {
    // TODO(lucasw) trying to copy demo code for dancing strings to get a regular timer update
    // even if window isn't active, but this isn't getting called by anything, there is special
    // demo code infrastructure involved there.
    /*
    fn ui(&mut self, ui: &mut egui::Ui) {
        println!("test");
    }
    */
}

impl epi::App for PlotImage {
    fn name(&self) -> &str {
        "Egui template"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let PlotImage {
            label,
            x_scale,
            y_scale,
            y_ind,
            last_update,
            filename,
            ref mut image,
            tex_mngr,
        } = self;

        let update_image;
        if last_update.elapsed() > Duration::from_millis(1000) {
            make_plot(image, &filename, 10.0, 50.0);
            *last_update = Instant::now();
            update_image = true;
        } else {
            update_image = false;
        }

        // This take 100% cpu
        ctx.request_repaint();

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if false {
            egui::SidePanel::left("side_panel", 400.0).show(ctx, |ui| {
                ui.heading("Side Panel");

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                    ui.text_edit_singleline(label);
                });

                if ui.button("Increment").clicked {
                    *x_scale += 1.0;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Hyperlink::new("https://github.com/emilk/egui/")
                            .text("powered by egui"),
                    );
                });
            });

            egui::TopPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    egui::menu::menu(ui, "File", |ui| {
                        if ui.button("Quit").clicked {
                            frame.quit();
                        }
                    });
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Image");
            ui.hyperlink("https://github.com/lucasw/egui_image");
            ui.add(egui::github_link_file_line!(
                "https://github.com/lucasw/egui_image/blob/main/",
                "Direct link to source code."
            ));
            egui::warn_if_debug_build(ui);

            ui.separator();

            ui.heading("Central Panel");
            ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
            ui.label("It is often a great place for big things, like drawings:");
            ui.add(egui::Slider::f32(x_scale, 0.1..=10.0).text("x scale"));
            ui.add(egui::Slider::f32(y_scale, 0.1..=10.0).text("y scale"));

            ui.add(egui::Slider::usize(y_ind, 0..=(image.size.1 - 1)).text("y ind"));

            egui::ScrollArea::auto_sized().show(ui, |ui| {
                // TODO(lucsw) this is only happening when there is a mouse motion or other change
                // over the window- as noted above the repaint needs to be triggered.
                // update the image pixels
                // image.shift(1, 0);

                if let Some(texture_id) = tex_mngr.texture(frame, update_image, &image) {
                    let size = egui::Vec2::new(
                        image.size.0 as f32 * *x_scale,
                        image.size.1 as f32 * *y_scale,
                    );
                    ui.image(texture_id, size);
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

        // TODO(lucasw) this is a little glitchy when resizing the image
        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = PlotImage::default();
    eframe::run_native(Box::new(app));
}
