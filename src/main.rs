use eframe::{egui, App, Frame};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FractalType {
    Mandelbrot,
    Julia,
    Koch,
}

impl FractalType {
    fn as_str(&self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot Set",
            FractalType::Julia => "Julia Set",
            FractalType::Koch => "Koch Curve",
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
    // Zoom rectangle selection
    zoom_rect_start: Option<egui::Pos2>,
    zoom_rect_end: Option<egui::Pos2>,
    selecting_zoom_rect: bool,
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
            zoom_rect_start: None,
            zoom_rect_end: None,
            selecting_zoom_rect: false,
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
                        if ui.selectable_value(&mut self.fractal_type, FractalType::Koch, "Koch Curve").changed() {
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
                    } else if self.fractal_type == FractalType::Julia {
                        self.center_x = 0.0;
                        self.center_y = 0.0;
                        self.zoom = 1.5;
                    } else {
                        // Koch curve
                        self.center_x = 0.0;
                        self.center_y = -0.2;
                        self.zoom = 0.8;
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
                ui.label("• Shift + drag: Zoom to rectangle");
                ui.label("• Tab: Toggle this panel");

                if ui.button("Reset View").clicked() {
                    if self.fractal_type == FractalType::Mandelbrot {
                        self.center_x = -0.5;
                        self.center_y = 0.0;
                        self.zoom = 1.0;
                    } else if self.fractal_type == FractalType::Julia {
                        self.center_x = 0.0;
                        self.center_y = 0.0;
                        self.zoom = 1.5;
                    } else {
                        // Koch curve
                        self.center_x = 0.0;
                        self.center_y = -0.2;
                        self.zoom = 0.8;
                    }
                    self.needs_redraw = true;
                }
            });

        // Handle Tab key to toggle controls
        if ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
            self.show_controls = !self.show_controls;
        }

        // Handle keyboard navigation for zooming
        if ctx.input(|i| i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals)) {
            // Zoom in with + or = key
            self.zoom *= 1.5;
            self.needs_redraw = true;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Minus)) {
            // Zoom out with - key
            self.zoom *= 0.67;
            self.needs_redraw = true;
        }

        // Handle arrow keys for panning
        let pan_distance = 0.1 / self.zoom; // Pan distance scales with zoom level
        let mut pan_changed = false;
        
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.center_x -= pan_distance;
            pan_changed = true;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.center_x += pan_distance;
            pan_changed = true;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.center_y -= pan_distance;
            pan_changed = true;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.center_y += pan_distance;
            pan_changed = true;
        }
        
        if pan_changed {
            self.needs_redraw = true;
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
            let (response, image_rect) = if let Some(texture) = &self.texture {
                let img_response = ui.add(egui::Image::from_texture(texture).sense(egui::Sense::click_and_drag()));
                let rect = img_response.rect;
                (img_response, rect)
            } else {
                let resp = ui.allocate_response(
                    egui::vec2(WIDTH as f32, HEIGHT as f32),
                    egui::Sense::click_and_drag()
                );
                let rect = resp.rect;
                (resp, rect)
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

            // Handle zoom rectangle selection (Shift + drag) or pan (normal drag)
            let shift_held = ctx.input(|i| i.modifiers.shift);

            if response.drag_started() {
                if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                    // Only process if mouse is within the image
                    if image_rect.contains(mouse_pos) {
                        if shift_held {
                            // Start zoom rectangle selection
                            self.zoom_rect_start = Some(mouse_pos);
                            self.selecting_zoom_rect = true;
                        } else {
                            // Start panning
                            self.last_mouse_pos = Some(mouse_pos);
                            self.dragging = true;
                        }
                    }
                }
            }

            if response.dragged() {
                if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                    if self.selecting_zoom_rect && shift_held {
                        // Update zoom rectangle end point
                        self.zoom_rect_end = Some(mouse_pos);
                    } else if self.dragging && !shift_held {
                        // Handle panning
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
                    }
                }
            }

            if response.drag_stopped() {
                if self.selecting_zoom_rect {
                    // Complete zoom rectangle selection
                    if let (Some(start), Some(end)) = (self.zoom_rect_start, self.zoom_rect_end) {

                        self.zoom_to_rectangle(start, end, image_rect);
                    }
                    self.zoom_rect_start = None;
                    self.zoom_rect_end = None;
                    self.selecting_zoom_rect = false;
                } else {
                    self.dragging = false;
                    self.last_mouse_pos = None;
                }
            }

            // Draw zoom rectangle if selecting
            if let (Some(start), Some(end)) = (self.zoom_rect_start, self.zoom_rect_end) {
                let rect = egui::Rect::from_two_pos(start, end);
                let rect_width = (end.x - start.x).abs();
                let rect_height = (end.y - start.y).abs();

                // Change color based on rectangle validity
                let (stroke_color, fill_color) = if rect_width >= 10.0 && rect_height >= 10.0 {
                    (egui::Color32::LIGHT_GREEN, egui::Color32::from_rgba_unmultiplied(0, 255, 0, 30))
                } else {
                    (egui::Color32::LIGHT_RED, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 30))
                };

                // Draw filled rectangle background
                ui.painter().rect_filled(rect, 0.0, fill_color);

                // Draw rectangle outline
                ui.painter().rect_stroke(
                    rect,
                    0.0,
                    egui::Stroke::new(2.0, stroke_color)
                );

                // Draw corner indicators
                let corner_size = 4.0;
                ui.painter().circle_filled(rect.left_top(), corner_size, stroke_color);
                ui.painter().circle_filled(rect.right_top(), corner_size, stroke_color);
                ui.painter().circle_filled(rect.left_bottom(), corner_size, stroke_color);
                ui.painter().circle_filled(rect.right_bottom(), corner_size, stroke_color);

                // Show size hint
                let size_text = format!("{}x{} px", rect_width as i32, rect_height as i32);
                let text_pos = rect.center() + egui::vec2(0.0, -15.0);
                ui.painter().text(
                    text_pos,
                    egui::Align2::CENTER_CENTER,
                    &size_text,
                    egui::FontId::monospace(12.0),
                    stroke_color,
                );
            }

            // Show toggle hint if controls are hidden
            if !self.show_controls {
                ui.allocate_ui_at_rect(
                    egui::Rect::from_min_size(response.rect.left_top(), egui::vec2(200.0, 50.0)),
                    |ui| {
                        ui.group(|ui| {
                            ui.label("Press Tab for controls");
                            ui.label("Shift+drag to zoom to area");
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

        match self.fractal_type {
            FractalType::Koch => {
                self.generate_koch_curve(&mut image);
            },
            _ => {
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
                            _ => unreachable!(),
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
                            // High-contrast color schemes for better visibility
                            match self.fractal_type {
                                FractalType::Mandelbrot => {
                                    // Hot color palette with green: black -> red -> yellow -> green -> cyan -> white
                                    let t = iter as f64 / max_iter as f64;
                                    let t = t.powf(0.5); // Apply gamma correction for better distribution

                                    if t < 0.2 {
                                        // Black to red
                                        let intensity = (t * 5.0 * 255.0) as u8;
                                        egui::Color32::from_rgb(intensity, 0, 0)
                                    } else if t < 0.4 {
                                        // Red to yellow
                                        let intensity = ((t - 0.2) * 5.0 * 255.0) as u8;
                                        egui::Color32::from_rgb(255, intensity, 0)
                                    } else if t < 0.6 {
                                        // Yellow to green
                                        let intensity = ((t - 0.4) * 5.0 * 255.0) as u8;
                                        egui::Color32::from_rgb(255 - intensity, 255, 0)
                                    } else if t < 0.8 {
                                        // Green to cyan
                                        let intensity = ((t - 0.6) * 5.0 * 255.0) as u8;
                                        egui::Color32::from_rgb(0, 255, intensity)
                                    } else {
                                        // Cyan to white
                                        let intensity = ((t - 0.8) * 5.0 * 255.0) as u8;
                                        egui::Color32::from_rgb(intensity, 255, 255)
                                    }
                                },
                                FractalType::Julia => {
                                    // Rainbow palette with high contrast
                                    let t = iter as f64 / max_iter as f64;
                                    let t = t.powf(0.7); // Gamma correction
                                    let hue = t * 6.0; // 6 color segments

                                    match hue as i32 {
                                        0 => {
                                            // Red to Orange
                                            let f = hue.fract();
                                            egui::Color32::from_rgb(255, (f * 165.0) as u8, 0)
                                        },
                                        1 => {
                                            // Orange to Yellow
                                            let f = hue.fract();
                                            egui::Color32::from_rgb(255, (165.0 + f * 90.0) as u8, 0)
                                        },
                                        2 => {
                                            // Yellow to Green
                                            let f = hue.fract();
                                            egui::Color32::from_rgb((255.0 * (1.0 - f)) as u8, 255, 0)
                                        },
                                        3 => {
                                            // Green to Cyan
                                            let f = hue.fract();
                                            egui::Color32::from_rgb(0, 255, (f * 255.0) as u8)
                                        },
                                        4 => {
                                            // Cyan to Blue
                                            let f = hue.fract();
                                            egui::Color32::from_rgb(0, (255.0 * (1.0 - f)) as u8, 255)
                                        },
                                        _ => {
                                            // Blue to Magenta
                                            let f = hue.fract();
                                            egui::Color32::from_rgb((f * 255.0) as u8, 0, 255)
                                        }
                                    }
                                },
                                _ => unreachable!(),
                            }
                        };
                        let index = y as usize * image.width() + x as usize;
                        image.pixels[index] = color;
                    }
                }
            }
        }
        image
    }

    fn generate_koch_curve(&self, image: &mut egui::ColorImage) {
        // Generate Koch snowflake with iteration depth based on zoom level
        let iterations = ((self.zoom.log2() + 1.0).max(0.0) as usize).min(5);

        // Create initial horizontal line segment centered at current view
        let size = 2.0 / self.zoom;

        let p1 = (self.center_x - size / 2.0, self.center_y);
        let p2 = (self.center_x + size / 2.0, self.center_y);

        // Generate Koch curve for a single line
        let mut segments = Vec::new();
        self.generate_koch_segments(p1, p2, iterations, &mut segments);

        // Draw the segments
        for (start, end) in segments {
            self.draw_line(image, start, end);
        }
    }

    fn generate_koch_segments(&self, start: (f64, f64), end: (f64, f64), depth: usize, segments: &mut Vec<((f64, f64), (f64, f64))>) {
        if depth == 0 {
            segments.push((start, end));
            return;
        }

        // Calculate the four points for Koch curve iteration
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        // Four key points along the line
        let p1 = start;
        let p2 = (start.0 + dx / 3.0, start.1 + dy / 3.0);
        let p4 = (start.0 + 2.0 * dx / 3.0, start.1 + 2.0 * dy / 3.0);
        let p5 = end;

        // Calculate the peak point (equilateral triangle bump)
        let mid_x = (p2.0 + p4.0) / 2.0;
        let mid_y = (p2.1 + p4.1) / 2.0;
        let segment_length = (dx * dx + dy * dy).sqrt() / 3.0;
        let height = segment_length * (3.0_f64.sqrt() / 2.0);

        // Create the bump perpendicular to the line
        let normal_x = -dy / (dx * dx + dy * dy).sqrt();
        let normal_y = dx / (dx * dx + dy * dy).sqrt();
        let p3 = (mid_x + normal_x * height, mid_y + normal_y * height);

        // Recursively generate segments
        self.generate_koch_segments(p1, p2, depth - 1, segments);
        self.generate_koch_segments(p2, p3, depth - 1, segments);
        self.generate_koch_segments(p3, p4, depth - 1, segments);
        self.generate_koch_segments(p4, p5, depth - 1, segments);
    }

    fn draw_line(&self, image: &mut egui::ColorImage, start: (f64, f64), end: (f64, f64)) {
        // Convert world coordinates to screen coordinates
        let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
        let height_range = 3.0 / self.zoom;
        let width_range = height_range * aspect_ratio;

        let left = self.center_x - width_range / 2.0;
        let right = self.center_x + width_range / 2.0;
        let top = self.center_y - height_range / 2.0;
        let bottom = self.center_y + height_range / 2.0;

        let sx = ((start.0 - left) / (right - left) * WIDTH as f64) as i32;
        let sy = ((start.1 - top) / (bottom - top) * HEIGHT as f64) as i32;
        let ex = ((end.0 - left) / (right - left) * WIDTH as f64) as i32;
        let ey = ((end.1 - top) / (bottom - top) * HEIGHT as f64) as i32;

        // Simple line drawing with thick lines for better visibility
        let dx = (ex - sx).abs();
        let dy = (ey - sy).abs();
        let steps = dx.max(dy).max(1);

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = (sx as f64 + t * (ex - sx) as f64) as i32;
            let y = (sy as f64 + t * (ey - sy) as f64) as i32;

            // Draw thick line (3x3 pixels)
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = x + dx;
                    let py = y + dy;
                    if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                        let index = py as usize * image.width() + px as usize;
                        // Use bright green color for Koch curve
                        image.pixels[index] = egui::Color32::from_rgb(0, 255, 0);
                    }
                }
            }
        }
    }

    fn zoom_to_rectangle(&mut self, start: egui::Pos2, end: egui::Pos2, image_rect: egui::Rect) {
        // Ensure we have a valid rectangle
        let rect_width = (end.x - start.x).abs();
        let rect_height = (end.y - start.y).abs();

        // Ignore tiny rectangles (likely accidental clicks)
        if rect_width < 10.0 || rect_height < 10.0 {
            return;
        }

        // Ensure start is top-left and end is bottom-right
        let rect_start = egui::Pos2::new(start.x.min(end.x), start.y.min(end.y));
        let rect_end = egui::Pos2::new(start.x.max(end.x), start.y.max(end.y));

        // Convert rectangle to relative coordinates within the image
        let rel_start_x = (rect_start.x - image_rect.left()) / image_rect.width();
        let rel_start_y = (rect_start.y - image_rect.top()) / image_rect.height();
        let rel_end_x = (rect_end.x - image_rect.left()) / image_rect.width();
        let rel_end_y = (rect_end.y - image_rect.top()) / image_rect.height();

        // Clamp to valid range
        let rel_start_x = rel_start_x.clamp(0.0, 1.0);
        let rel_start_y = rel_start_y.clamp(0.0, 1.0);
        let rel_end_x = rel_end_x.clamp(0.0, 1.0);
        let rel_end_y = rel_end_y.clamp(0.0, 1.0);

        // Calculate current view bounds in complex plane
        let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
        let height_range = 3.0 / self.zoom;
        let width_range = height_range * aspect_ratio;

        let current_left = self.center_x - width_range / 2.0;
        let current_right = self.center_x + width_range / 2.0;
        let current_top = self.center_y - height_range / 2.0;
        let current_bottom = self.center_y + height_range / 2.0;

        // Map relative coordinates to complex plane coordinates
        let selected_left = current_left + rel_start_x as f64 * (current_right - current_left);
        let selected_right = current_left + rel_end_x as f64 * (current_right - current_left);
        let selected_top = current_top + rel_start_y as f64 * (current_bottom - current_top);
        let selected_bottom = current_top + rel_end_y as f64 * (current_bottom - current_top);

        // Calculate new center
        let new_center_x = (selected_left + selected_right) / 2.0;
        let new_center_y = (selected_top + selected_bottom) / 2.0;

        // Calculate how much we need to zoom to fit the selected rectangle
        let selected_width = selected_right - selected_left;
        let selected_height = selected_bottom - selected_top;

        // Calculate zoom factor to fit the selection in the viewport
        let zoom_factor_x = width_range / selected_width;
        let zoom_factor_y = height_range / selected_height;
        let zoom_factor = zoom_factor_x.min(zoom_factor_y);

        // Apply the zoom (only if it would zoom in)

        if zoom_factor > 1.0 {
            self.center_x = new_center_x;
            self.center_y = new_center_y;
            self.zoom *= zoom_factor;
            self.needs_redraw = true;
        } else {
            // Show visual feedback for invalid rectangle selection
        }
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
