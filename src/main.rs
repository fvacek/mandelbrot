use eframe::{egui, App, Frame};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

struct MandelbrotApp {
    texture: Option<egui::TextureHandle>,
}

impl Default for MandelbrotApp {
    fn default() -> Self {
        Self {
            texture: None,
        }
    }
}

impl App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.texture.is_none() {
                let image = self.generate_mandelbrot_image();
                self.texture = Some(ctx.load_texture(
                    "mandelbrot_texture",
                    image,
                    egui::TextureOptions {
                        minification: egui::TextureFilter::Linear,
                        magnification: egui::TextureFilter::Linear,
                        ..Default::default()
                    },
                ));
            }

            if let Some(texture) = &self.texture {
                ui.image(texture);
            }
        });
    }


}

impl MandelbrotApp {
    fn generate_mandelbrot_image(&self) -> egui::ColorImage {
        let mut image = egui::ColorImage::new([WIDTH as usize, HEIGHT as usize], egui::Color32::BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cx = (x as f64 / WIDTH as f64) * 3.5 - 2.5;
                let cy = (y as f64 / HEIGHT as f64) * 2.0 - 1.0;

                let mut zx = 0.0;
                let mut zy = 0.0;

                let mut iter = 0;
                while zx * zx + zy * zy < 4.0 && iter < 255 {
                    let xtemp = zx * zx - zy * zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = xtemp;
                    iter += 1;
                }

                let color = if iter == 255 {
                    egui::Color32::BLACK
                } else {
                    let r = (iter * 5) as u8;
                    let g = (iter * 7) as u8;
                    let b = (iter * 13) as u8;
                    egui::Color32::from_rgb(r, g, b)
                };
                let index = y as usize * image.width() + x as usize;
                image.pixels[index] = color;
            }
        }
        image
    }
}

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Mandelbrot",
        options,
        Box::new(|_cc| Box::new(MandelbrotApp::default())),
    );
}
