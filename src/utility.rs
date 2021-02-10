use eframe::{egui, epi};
// use std::fs::File;

// TODO(lucasw) create a library that is just for image pixels manipulation
// this file should just be the egui ui layer

// ----------------------------------------------------------------------------
// Texture/image handling is very manual at the moment.

/// Immediate mode texture manager that supports at most one texture at the time :)
#[derive(Default)]
pub struct TexMngr {
    texture_id: Option<egui::TextureId>,
}

impl TexMngr {
    pub fn texture(
        &mut self,
        frame: &mut epi::Frame<'_>,
        // filename: &str,
        update: bool,
        image: &Image,
    ) -> Option<egui::TextureId> {
        if update {
            let tex_allocator = frame.tex_allocator().as_mut()?;
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
    pub size: (usize, usize),
    pub pixels: Vec<egui::Color32>,
}

// TODO(lucasw) move this into library/module, and make it generic on any
// vector with a width and height and default value supplied.
impl Image {
    pub fn shift(&mut self, mut shift_x: i32, mut shift_y: i32) {
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
