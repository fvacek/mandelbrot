use eframe::{egui, App, Frame};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

struct MandelbrotApp {
    texture: Option<egui::TextureHandle>,
    // Viewport parameters for zoom and pan
    center_x: f64,
    center_y: f64,
    zoom: f64,
    // Mouse interaction state
    dragging: bool,
    last_mouse_pos: Option<egui::Pos2>,
    needs_redraw: bool,
}

impl Default for MandelbrotApp {
    fn default() -> Self {
        Self {
            texture: None,
            center_x: -0.5,  // Center on the main body of the Mandelbrot set
            center_y: 0.0,
            zoom: 1.0,
            dragging: false,
            last_mouse_pos: None,
            needs_redraw: true,
        }
    }
}

impl App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the texture first
            if self.texture.is_none() || self.needs_redraw {
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
                self.needs_redraw = false;
            }

            // Display the image and get response for interaction
            let response = if let Some(texture) = &self.texture {
                ui.image(texture)
            } else {
                ui.allocate_response(
                    egui::vec2(WIDTH as f32, HEIGHT as f32), 
                    egui::Sense::click_and_drag()
                )
            };

            // Handle zoom with scroll wheel
            if response.hovered() {
                let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
                    self.zoom *= zoom_factor;
                    self.needs_redraw = true;
                }
            }

            // Handle pan with mouse drag
            if response.dragged() {
                if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                    if let Some(last_pos) = self.last_mouse_pos {
                        let delta = mouse_pos - last_pos;

                        // Convert pixel delta to complex plane delta
                        let scale = 3.5 / self.zoom / WIDTH as f64;
                        self.center_x -= delta.x as f64 * scale;
                        self.center_y -= delta.y as f64 * scale;

                        self.needs_redraw = true;
                    }
                    self.last_mouse_pos = Some(mouse_pos);
                    self.dragging = true;
                }
            } else {
                self.dragging = false;
                self.last_mouse_pos = None;
            }



            // Show zoom level and coordinates in a small overlay
            ui.allocate_ui_at_rect(
                egui::Rect::from_min_size(response.rect.left_top(), egui::vec2(200.0, 60.0)),
                |ui| {
                    ui.group(|ui| {
                        ui.label(format!("Zoom: {:.2e}", self.zoom));
                        ui.label(format!("Center: ({:.6}, {:.6})", self.center_x, self.center_y));
                    });
                }
            );
        });
    }
}

impl MandelbrotApp {
    fn generate_mandelbrot_image(&self) -> egui::ColorImage {
        let mut image = egui::ColorImage::new([WIDTH as usize, HEIGHT as usize], egui::Color32::BLACK);

        // Calculate the bounds of the current view
        let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
        let height_range = 3.0 / self.zoom;
        let width_range = height_range * aspect_ratio;

        let left = self.center_x - width_range / 2.0;
        let right = self.center_x + width_range / 2.0;
        let top = self.center_y - height_range / 2.0;
        let bottom = self.center_y + height_range / 2.0;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // Map pixel coordinates to complex plane based on current view
                let cx = left + (x as f64 / WIDTH as f64) * (right - left);
                let cy = top + (y as f64 / HEIGHT as f64) * (bottom - top);

                let mut zx = 0.0;
                let mut zy = 0.0;

                // Increase max iterations for higher zoom levels to maintain detail
                let max_iter = (255.0 + (self.zoom.log10() * 100.0).max(0.0)) as i32;
                let max_iter = max_iter.min(1000); // Cap at 1000 for performance

                let mut iter = 0;
                while zx * zx + zy * zy < 4.0 && iter < max_iter {
                    let xtemp = zx * zx - zy * zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = xtemp;
                    iter += 1;
                }

                let color = if iter == max_iter {
                    egui::Color32::BLACK
                } else {
                    // Simple gradient coloring for clear visibility
                    let intensity = (iter * 255 / max_iter) as u8;
                    egui::Color32::from_rgb(intensity, intensity / 2, 255 - intensity)
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
        "Mandelbrot Explorer",
        options,
        Box::new(|_cc| Box::new(MandelbrotApp::default())),
    );
}
