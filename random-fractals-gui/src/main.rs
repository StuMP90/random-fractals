use eframe::egui;
use rand::Rng;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Random Fractals",
        native_options,
        Box::new(|_cc| Box::new(RandomFractalsApp::default())),
    )
}

#[derive(Default)]
struct RandomFractalsApp {
    running: bool,
    show_select: bool,
    did_set_initial_size: bool,
    settings: Settings,
    texture: Option<egui::TextureHandle>,
    last_render_ms: Option<u128>,
    next_render_at: Option<std::time::Instant>,
}

impl eframe::App for RandomFractalsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.did_set_initial_size {
            let monitor_size = ctx.input(|i| i.viewport().monitor_size);
            let desired = monitor_size
                .map(|s| egui::vec2(s.x * 0.5, s.y * 0.5))
                .unwrap_or_else(|| egui::vec2(640.0, 480.0));

            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(desired));
            self.did_set_initial_size = true;
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Select").clicked() {
                    self.show_select = true;
                }

                if ui.button("Start").clicked() {
                    self.running = true;
                    self.next_render_at = Some(std::time::Instant::now());
                }
                if ui.button("Stop").clicked() {
                    self.running = false;
                    self.next_render_at = None;
                }
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            let now = std::time::Instant::now();
            let should_render = self.running
                && self
                    .next_render_at
                    .is_some_and(|t| now >= t);

            if should_render {
                self.settings.randomize_params();
                self.render(ctx, rect.size());
                self.next_render_at = Some(now + self.settings.refresh_time);
            }

            if self.running {
                if let Some(next) = self.next_render_at {
                    let wait = next.saturating_duration_since(now);
                    ctx.request_repaint_after(wait);
                } else {
                    ctx.request_repaint();
                }
            }

            if let Some(texture) = &self.texture {
                ui.painter().image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            } else {
                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Random Fractals");
                        ui.separator();
                        ui.label("Press Start to generate a random fractal.");
                    });
                });
            }

            let status_text = if self.running { "Running" } else { "Stopped" };
            let render_text = self
                .last_render_ms
                .map(|ms| format!("Last render: {ms} ms"))
                .unwrap_or_else(|| "Last render: -".to_string());

            egui::Area::new("overlay_status".into())
                .fixed_pos(rect.min + egui::vec2(10.0, 10.0))
                .show(ctx, |ui| {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        ui.label(format!("Status: {status_text}"));
                        ui.label(render_text);
                    });
                });
        });

        let mut open_select = self.show_select;
        if open_select {
            let mut close_clicked = false;
            egui::Window::new("Select")
                .collapsible(false)
                .resizable(false)
                .open(&mut open_select)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        egui::ComboBox::from_id_source("fractal_type")
                            .selected_text(self.settings.fractal_type.label())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.settings.fractal_type,
                                    FractalType::Mandelbrot,
                                    FractalType::Mandelbrot.label(),
                                );
                                ui.selectable_value(
                                    &mut self.settings.fractal_type,
                                    FractalType::Julia,
                                    FractalType::Julia.label(),
                                );
                            });
                    });

                    ui.separator();
                    ui.label("Performance");
                    ui.add(egui::Slider::new(&mut self.settings.max_iter, 16..=4096).text("Max iterations"));

                    ui.separator();
                    ui.label("Refresh Time");
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.settings.refresh_time_ms)
                                .speed(50)
                                .clamp_range(0..=600_000),
                        );
                        ui.label("ms");
                    });
                    self.settings.refresh_time = std::time::Duration::from_millis(self.settings.refresh_time_ms);

                    if ui.button("Close").clicked() {
                        close_clicked = true;
                    }
                });

            if close_clicked {
                open_select = false;
            }
        }
        self.show_select = open_select;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FractalType {
    Mandelbrot,
    Julia,
}

impl FractalType {
    fn label(self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot",
            FractalType::Julia => "Julia",
        }
    }
}

struct Settings {
    fractal_type: FractalType,
    max_iter: u32,
    params: FractalParams,
    refresh_time: std::time::Duration,
    refresh_time_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        let mut s = Self {
            fractal_type: FractalType::Mandelbrot,
            max_iter: 512,
            params: FractalParams::default(),
            refresh_time: std::time::Duration::from_millis(10_000),
            refresh_time_ms: 10_000,
        };
        s.randomize_params();
        s
    }
}

impl Settings {
    fn randomize_params(&mut self) {
        self.params = FractalParams::random(self.fractal_type);
    }
}

#[derive(Clone, Copy)]
struct FractalParams {
    center: (f64, f64),
    scale: f64,
    julia_c: (f64, f64),
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            center: (0.0, 0.0),
            scale: 3.0,
            julia_c: (-0.8, 0.156),
        }
    }
}

impl FractalParams {
    fn random(fractal_type: FractalType) -> Self {
        let mut rng = rand::thread_rng();

        match fractal_type {
            FractalType::Mandelbrot => {
                let cx = rng.gen_range(-0.9..=0.2);
                let cy = rng.gen_range(-0.7..=0.7);
                let scale = rng.gen_range(0.25..=3.0);
                Self {
                    center: (cx, cy),
                    scale,
                    julia_c: (-0.8, 0.156),
                }
            }
            FractalType::Julia => {
                let cx = rng.gen_range(-0.5..=0.5);
                let cy = rng.gen_range(-0.5..=0.5);
                let scale = rng.gen_range(1.5..=3.5);
                let cr = rng.gen_range(-1.0..=1.0);
                let ci = rng.gen_range(-1.0..=1.0);
                Self {
                    center: (cx, cy),
                    scale,
                    julia_c: (cr, ci),
                }
            }
        }
    }
}

impl RandomFractalsApp {
    fn render(&mut self, ctx: &egui::Context, available_points: egui::Vec2) {
        let start = std::time::Instant::now();

        let pixels_per_point = ctx.pixels_per_point();
        let mut width = (available_points.x * pixels_per_point).round() as isize;
        let mut height = (available_points.y * pixels_per_point).round() as isize;
        if width < 1 {
            width = 1;
        }
        if height < 1 {
            height = 1;
        }

        let image = render_fractal(
            self.settings.fractal_type,
            width as usize,
            height as usize,
            self.settings.max_iter,
            self.settings.params,
        );

        let elapsed = start.elapsed().as_millis();
        self.last_render_ms = Some(elapsed);

        let texture = ctx.load_texture(
            "fractal",
            image,
            egui::TextureOptions::default(),
        );
        self.texture = Some(texture);
    }
}

fn render_fractal(
    fractal_type: FractalType,
    width: usize,
    height: usize,
    max_iter: u32,
    params: FractalParams,
) -> egui::ColorImage {
    let mut pixels = vec![egui::Color32::BLACK; width * height];
    let (cx, cy) = params.center;
    let scale = params.scale;

    for y in 0..height {
        for x in 0..width {
            let re = cx + (x as f64 / (width - 1).max(1) as f64 - 0.5) * scale;
            let im = cy + (y as f64 / (height - 1).max(1) as f64 - 0.5) * scale * (height as f64 / width as f64);

            let iters = match fractal_type {
                FractalType::Mandelbrot => mandelbrot(re, im, max_iter),
                FractalType::Julia => julia(re, im, params.julia_c.0, params.julia_c.1, max_iter),
            };

            pixels[y * width + x] = palette(iters, max_iter);
        }
    }

    egui::ColorImage {
        size: [width, height],
        pixels,
    }
}

fn mandelbrot(re0: f64, im0: f64, max_iter: u32) -> u32 {
    let mut re = 0.0;
    let mut im = 0.0;
    let mut i = 0;

    while i < max_iter {
        let re2 = re * re;
        let im2 = im * im;
        if re2 + im2 > 4.0 {
            break;
        }
        let new_im = 2.0 * re * im + im0;
        let new_re = re2 - im2 + re0;
        re = new_re;
        im = new_im;
        i += 1;
    }

    i
}

fn julia(re0: f64, im0: f64, cr: f64, ci: f64, max_iter: u32) -> u32 {
    let mut re = re0;
    let mut im = im0;
    let mut i = 0;

    while i < max_iter {
        let re2 = re * re;
        let im2 = im * im;
        if re2 + im2 > 4.0 {
            break;
        }
        let new_im = 2.0 * re * im + ci;
        let new_re = re2 - im2 + cr;
        re = new_re;
        im = new_im;
        i += 1;
    }

    i
}

fn palette(iter: u32, max_iter: u32) -> egui::Color32 {
    if iter >= max_iter {
        return egui::Color32::BLACK;
    }

    let t = iter as f32 / max_iter.max(1) as f32;
    let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
    let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
    let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
    egui::Color32::from_rgb(r, g, b)
}
