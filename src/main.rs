use eframe::{egui, App, Frame};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FractalType {
    Mandelbrot,
    Julia,
}

impl FractalType {
    fn as_str(&self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot Set",
            FractalType::Julia => "Julia Set",
        }
    }
}

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
    // Fractal type and Julia set parameters
    fractal_type: FractalType,
    julia_c_real: f64,
    julia_c_imag: f64,
    // UI state
    show_controls: bool,
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
            fractal_type: FractalType::Mandelbrot,
            julia_c_real: -0.7269,  // Interesting Julia set constant
            julia_c_imag: 0.1889,
            show_controls: true,
        }
    }
}

impl App for MandelbrotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Side panel for controls
        egui::SidePanel::left("controls")
            .resizable(false)
            .show_animated(ctx, self.show_controls, |ui| {
                ui.heading("Fractal Explorer");
                ui.separator();

                // Fractal type selection
                ui.label("Fractal Type:");
                let mut changed = false;
                egui::ComboBox::from_label("")
                    .selected_text(self.fractal_type.as_str())
                    .show_ui(ui, |ui| {
                        if ui.selectable_value(&mut self.fractal_type, FractalType::Mandelbrot, "Mandelbrot Set").changed() {
                            changed = true;
                        }
                        if ui.selectable_value(&mut self.fractal_type, FractalType::Julia, "Julia Set").changed() {
                            changed = true;
                        }
                    });

                if changed {
                    self.needs_redraw = true;
                    // Reset view when switching fractal types
                    if self.fractal_type == FractalType::Mandelbrot {
                        self.center_x = -0.5;
                        self.center_y = 0.0;
                        self.zoom = 1.0;
                    } else {
                        self.center_x = 0.0;
                        self.center_y = 0.0;
                        self.zoom = 1.5;
                    }
                }

                ui.separator();

                // Julia set parameters (only show for Julia set)
                if self.fractal_type == FractalType::Julia {
                    ui.label("Julia Set Parameters:");
                    
                    if ui.add(egui::Slider::new(&mut self.julia_c_real, -2.0..=2.0)
                        .text("c (real)")
                        .step_by(0.001)).changed() {
                        self.needs_redraw = true;
                    }
                    
                    if ui.add(egui::Slider::new(&mut self.julia_c_imag, -2.0..=2.0)
                        .text("c (imaginary)")
                        .step_by(0.001)).changed() {
                        self.needs_redraw = true;
                    }

                    ui.separator();
                    ui.label("Presets:");
                    
                    if ui.button("Dragon").clicked() {
                        self.julia_c_real = -0.7269;
                        self.julia_c_imag = 0.1889;
                        self.needs_redraw = true;
                    }
                    
                    if ui.button("Spiral").clicked() {
                        self.julia_c_real = -0.75;
                        self.julia_c_imag = 0.11;
                        self.needs_redraw = true;
                    }
                    
                    if ui.button("Lightning").clicked() {
                        self.julia_c_real = -0.4;
                        self.julia_c_imag = 0.6;
                        self.needs_redraw = true;
                    }
                    
                    if ui.button("Douady Rabbit").clicked() {
                        self.julia_c_real = -0.123;
                        self.julia_c_imag = 0.745;
                        self.needs_redraw = true;
                    }

                    ui.separator();
                }

                // Current view info
                ui.label("Current View:");
                ui.label(format!("Zoom: {:.2e}", self.zoom));
                ui.label(format!("Center: ({:.6}, {:.6})", self.center_x, self.center_y));

                ui.separator();
                ui.label("Controls:");
                ui.label("• Mouse wheel: Zoom");
                ui.label("• Click & drag: Pan");
                ui.label("• This panel: Toggle with Tab");

                if ui.button("Reset View").clicked() {
                    if self.fractal_type == FractalType::Mandelbrot {
                        self.center_x = -0.5;
                        self.center_y = 0.0;
                        self.zoom = 1.0;
                    } else {
                        self.center_x = 0.0;
                        self.center_y = 0.0;
                        self.zoom = 1.5;
                    }
                    self.needs_redraw = true;
                }
            });

        // Handle Tab key to toggle controls
        if ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
            self.show_controls = !self.show_controls;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the texture first
            if self.texture.is_none() || self.needs_redraw {
                let image = self.generate_fractal_image();
                self.texture = Some(ctx.load_texture(
                    "fractal_texture",
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
                        let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
                        let height_range = 3.0 / self.zoom;
                        let width_range = height_range * aspect_ratio;
                        
                        let scale_x = width_range / WIDTH as f64;
                        let scale_y = height_range / HEIGHT as f64;
                        
                        self.center_x -= delta.x as f64 * scale_x;
                        self.center_y -= delta.y as f64 * scale_y;
                        
                        self.needs_redraw = true;
                    }
                    self.last_mouse_pos = Some(mouse_pos);
                    self.dragging = true;
                }
            } else {
                self.dragging = false;
                self.last_mouse_pos = None;
            }

            // Show toggle hint if controls are hidden
            if !self.show_controls {
                ui.allocate_ui_at_rect(
                    egui::Rect::from_min_size(response.rect.left_top(), egui::vec2(150.0, 30.0)),
                    |ui| {
                        ui.group(|ui| {
                            ui.label("Press Tab for controls");
                        });
                    }
                );
            }
        });
    }
}

impl MandelbrotApp {
    fn generate_fractal_image(&self) -> egui::ColorImage {
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
                let px = left + (x as f64 / WIDTH as f64) * (right - left);
                let py = top + (y as f64 / HEIGHT as f64) * (bottom - top);

                let (mut zx, mut zy, cx, cy) = match self.fractal_type {
                    FractalType::Mandelbrot => {
                        // Mandelbrot: z starts at 0, c is the pixel coordinate
                        (0.0, 0.0, px, py)
                    },
                    FractalType::Julia => {
                        // Julia: z starts at pixel coordinate, c is fixed
                        (px, py, self.julia_c_real, self.julia_c_imag)
                    },
                };

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
                    // Color scheme varies by fractal type
                    match self.fractal_type {
                        FractalType::Mandelbrot => {
                            let intensity = (iter * 255 / max_iter) as u8;
                            egui::Color32::from_rgb(intensity, intensity / 2, 255 - intensity)
                        },
                        FractalType::Julia => {
                            let t = iter as f64 / max_iter as f64;
                            let r = (255.0 * (0.5 + 0.5 * (t * 3.0).sin())) as u8;
                            let g = (255.0 * (0.5 + 0.5 * (t * 5.0 + 2.0).sin())) as u8;
                            let b = (255.0 * (0.5 + 0.5 * (t * 7.0 + 4.0).sin())) as u8;
                            egui::Color32::from_rgb(r, g, b)
                        }
                    }
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
            .with_inner_size(egui::vec2(WIDTH as f32 + 250.0, HEIGHT as f32)), // Extra width for side panel
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Fractal Explorer",
        options,
        Box::new(|_cc| Box::new(MandelbrotApp::default())),
    );
}