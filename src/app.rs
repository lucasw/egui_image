use eframe::{egui, epi};
use image::GenericImageView;
// use std::fs::File;

// TODO(lucasw) create a library that is just for image pixels manipulation
// this file should just be the egui ui layer

// ----------------------------------------------------------------------------
// Texture/image handling is very manual at the moment.

/// Immediate mode texture manager that supports at most one texture at the time :)
#[derive(Default)]
struct TexMngr {
    texture_id: Option<egui::TextureId>,
}

impl TexMngr {
    fn texture(
        &mut self,
        frame: &mut epi::Frame<'_>,
        // filename: &str,
        update: bool,
        image: &Image,
    ) -> Option<egui::TextureId> {
        let tex_allocator = frame.tex_allocator().as_mut()?;
        if update {
            if let Some(texture_id) = self.texture_id.take() {
                tex_allocator.free(texture_id);
            }

            self.texture_id =
                Some(tex_allocator.alloc_srgba_premultiplied(image.size, &image.pixels));
        }
        self.texture_id
    }
}

pub struct Image {
    size: (usize, usize),
    pixels: Vec<egui::Color32>,
}

// TODO(lucasw) move this into library/module, and make it generic on any
// vector with a width and height and default value supplied.
impl Image {
    fn shift(&mut self, mut shift_x: i32, mut shift_y: i32) {
        let mut shifted = vec![egui::Color32::BLUE; self.pixels.len()];

        let width = self.size.0;
        while shift_x < 0 {
            shift_x += width as i32;
        }
        let shift_x = shift_x as usize;

        let height = self.size.1;
        while shift_y < 0 {
            shift_y += height as i32;
        }
        let shift_y = shift_y as usize;

        for y in 0..height {
            for x in 0..width {
                let dst_x = (x + shift_x) % width;
                let dst_y = (y + shift_y) % height;
                let dst_ind = dst_y * width + dst_x;
                let ind = y * width + x;
                shifted[dst_ind] = self.pixels[ind];
            }
        }
        self.pixels = shifted;
    }

    /*
    if false {
        for x in 0..255 {
            if x >= image.size.0 {
                break;
            }
            let ind = *y_ind * image.size.0 + x;
            image.pixels[ind] = egui::Color32::from_rgb(*y_ind as u8, x as u8, 0);
        }
    }
    */
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ImageApp {
    // Example stuff:
    label: String,
    x_scale: f32,
    y_scale: f32,
    y_ind: usize,
    // filename: String,
    #[cfg_attr(feature = "persistence", serde(skip))]
    image: Image,
    #[cfg_attr(feature = "persistence", serde(skip))]
    tex_mngr: TexMngr,
}

impl Default for ImageApp {
    fn default() -> Self {
        // Decode the jpeg using image::GenericImageView, then paint into the screen
        // following egui url image loading example in egui/egui_demo_lib/src/app/http_app.rs
        let filename = "data/gradient_rect.jpg";
        let image = image::open(filename).unwrap();
        let image_buffer = image.to_rgba8();
        let size = (image.width() as usize, image.height() as usize);
        println!("{} {:?}", filename, size);
        let pixels = image_buffer.into_vec();
        assert_eq!(size.0 * size.1 * 4, pixels.len());
        let pixels = pixels
            .chunks(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            x_scale: 4.0,
            y_scale: 4.0,
            y_ind: 30,
            // filename: (&filename).to_string(),
            image: Image { size, pixels },
            tex_mngr: Default::default(),
        }
    }
}

impl ImageApp {
    // TODO(lucasw) trying to copy demo code for dancing strings to get a regular timer update
    // even if window isn't active, but this isn't getting called by anything, there is special
    // demo code infrastructure involved there.
    /*
    fn ui(&mut self, ui: &mut egui::Ui) {
        println!("test");
    }
    */
}

impl epi::App for ImageApp {
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
        let ImageApp {
            label,
            x_scale,
            y_scale,
            y_ind,
            // filename,
            image,
            tex_mngr,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if false {
            egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
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
                image.shift(1, 0);
                let update = true;

                if let Some(texture_id) = tex_mngr.texture(frame, update, image) {
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
        // frame.set_window_size(ctx.used_size());
    }
}

// ----------------------------------------------------------------------------

/*
/// Example code for painting on a canvas with your mouse
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct Painting {
    lines: Vec<Vec<egui::Vec2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            self.stroke.ui(ui, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked {
                self.lines.clear();
            }
        })
        .1
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());
        let rect = response.rect;

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if response.active {
            if let Some(mouse_pos) = ui.input().mouse.pos {
                let canvas_pos = mouse_pos - rect.min;
                if current_line.last() != Some(&canvas_pos) {
                    current_line.push(canvas_pos);
                }
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
        }

        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
                painter.add(egui::PaintCmd::line(points, self.stroke));
            }
        }

        response
    }
}
*/
