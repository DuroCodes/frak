use num_complex::Complex32;
use rayon::prelude::*;
use std::ptr::addr_of_mut;

const MAX_WIDTH: usize = 3200;
const MAX_HEIGHT: usize = 2000;
const PREVIEW_SCALE: usize = 4;

#[no_mangle]
#[used]
static MEMORY_PAGES: u32 = 64;

#[no_mangle]
static mut BUFFER: [u32; MAX_WIDTH * MAX_HEIGHT] = [0; MAX_WIDTH * MAX_HEIGHT];
static mut CURRENT_QUALITY: RenderQuality = RenderQuality::Preview;

#[derive(Clone, Copy, PartialEq)]
enum RenderQuality {
    Preview,
    Full,
}

#[derive(Clone, Copy)]
enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Newton,
    RainbowNewtonSet,
    Tricorn,
    Phoenix,
    Celtic,
}

#[no_mangle]
static mut C_VALUE: Complex32 = Complex32 { re: -0.5, im: 0.5 };
static mut ROTOR: Complex32 = Complex32 { re: 1., im: 0.002 };
static mut CURRENT_FRACTAL: FractalType = FractalType::Mandelbrot;

static mut VIEW_STATE: ViewState = ViewState {
    center_x: 0.0,
    center_y: 0.0,
    zoom: 1.0,
};

#[derive(Clone)]
struct ViewState {
    center_x: f32,
    center_y: f32,
    zoom: f32,
}

#[no_mangle]
pub unsafe extern "C" fn set_fractal(fractal_type: u8) {
    CURRENT_FRACTAL = match fractal_type {
        0 => FractalType::Mandelbrot,
        1 => FractalType::Julia,
        2 => FractalType::BurningShip,
        3 => FractalType::Newton,
        4 => FractalType::RainbowNewtonSet,
        5 => FractalType::Tricorn,
        6 => FractalType::Phoenix,
        7 => FractalType::Celtic,
        _ => FractalType::Mandelbrot,
    };
}

#[no_mangle]
pub unsafe extern "C" fn set_view(x: f32, y: f32, zoom: f32) {
    VIEW_STATE.center_x = x;
    VIEW_STATE.center_y = y;
    VIEW_STATE.zoom = zoom;
}

#[no_mangle]
pub unsafe extern "C" fn set_quality(high_quality: bool) {
    CURRENT_QUALITY = match high_quality {
        true => RenderQuality::Full,
        false => RenderQuality::Preview,
    };
}

#[no_mangle]
pub unsafe extern "C" fn render() {
    match CURRENT_FRACTAL {
        FractalType::Julia => {
            if C_VALUE.norm_sqr() > 2. || C_VALUE.norm_sqr() < 0.3 {
                ROTOR *= 1. / ROTOR.norm_sqr();
            }
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::Mandelbrot => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::BurningShip => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::Newton => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::RainbowNewtonSet => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::Tricorn => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::Phoenix => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
        FractalType::Celtic => {
            let buffer_ptr = addr_of_mut!(BUFFER);
            render_frame_safe(&mut *buffer_ptr, C_VALUE, CURRENT_FRACTAL)
        }
    }
}

trait Fractal {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    );
    fn get_max_iterations(&self, zoom: f32) -> u8;
}

struct MandelbrotSet;
struct JuliaSet {
    c: Complex32,
}
struct BurningShipSet;
struct NewtonSet;
struct RainbowNewtonSet;
struct TricornSet;
struct PhoenixSet {
    c: Complex32,
}
struct CelticSet;

impl Fractal for MandelbrotSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| mandelbrot(z, max_iter),
        );
    }

    fn get_max_iterations(&self, zoom: f32) -> u8 {
        255u8.saturating_add((zoom.log2().max(0.0) * 32.0) as u8)
    }
}

impl Fractal for JuliaSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| julia(z, self.c, max_iter),
        );
    }

    fn get_max_iterations(&self, zoom: f32) -> u8 {
        match zoom {
            0.0..=1.0 => 255,
            _ => 255u8.saturating_add((zoom.log2().max(0.0) * 32.0) as u8),
        }
    }
}

impl Fractal for BurningShipSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| burning_ship(z, max_iter),
        );
    }

    fn get_max_iterations(&self, zoom: f32) -> u8 {
        255u8.saturating_add((zoom.log2().max(0.0) * 32.0) as u8)
    }
}

impl Fractal for NewtonSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| newton(z, max_iter),
        )
    }

    fn get_max_iterations(&self, _zoom: f32) -> u8 {
        32 // |zₙ₊₁ - zₙ| < ε or chaotic near ∂Gₖ
    }
}

impl Fractal for RainbowNewtonSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| rainbow_newton(z, max_iter),
        )
    }

    fn get_max_iterations(&self, _zoom: f32) -> u8 {
        32 // |zₙ₊₁ - zₙ| < ε or chaotic near ∂Gₖ
    }
}

impl Fractal for TricornSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| tricorn(z, max_iter),
        )
    }

    fn get_max_iterations(&self, zoom: f32) -> u8 {
        255u8.saturating_add((zoom.log2().max(0.0) * 32.0) as u8)
    }
}

impl Fractal for PhoenixSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| phoenix(z, self.c, max_iter),
        )
    }

    fn get_max_iterations(&self, zoom: f32) -> u8 {
        255u8.saturating_add((zoom.log2().max(0.0) * 32.0) as u8)
    }
}

impl Fractal for CelticSet {
    fn render(
        &self,
        buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
        view: &ViewState,
        quality: RenderQuality,
    ) {
        let (width, height, stride) = get_dimensions(quality);
        let (scale, x0, y0) = get_view_transforms(width, height, view);
        let max_iter = self.get_max_iterations(view.zoom);

        render_parallel(
            buffer,
            width,
            height,
            stride,
            scale,
            x0,
            y0,
            max_iter,
            |z, max_iter| celtic(z, max_iter),
        );
    }

    fn get_max_iterations(&self, zoom: f32) -> u8 {
        255u8.saturating_add((zoom.log2().max(0.0) * 32.0) as u8)
    }
}

fn get_dimensions(quality: RenderQuality) -> (usize, usize, usize) {
    match quality {
        RenderQuality::Preview => (
            MAX_WIDTH / PREVIEW_SCALE,
            MAX_HEIGHT / PREVIEW_SCALE,
            PREVIEW_SCALE,
        ),
        RenderQuality::Full => (MAX_WIDTH, MAX_HEIGHT, 1),
    }
}

fn get_view_transforms(width: usize, height: usize, view: &ViewState) -> (f32, f32, f32) {
    let scale = 3.2 / (height as f32) / view.zoom;
    let x0 = view.center_x - (width as f32 * scale / 2.0);
    let y0 = view.center_y - (height as f32 * scale / 2.0);
    (scale, x0, y0)
}

fn render_parallel<F>(
    buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
    width: usize,
    height: usize,
    stride: usize,
    scale: f32,
    x0: f32,
    y0: f32,
    max_iter: u8,
    compute_point: F,
) where
    F: Fn(Complex32, u8) -> f32 + Send + Sync,
{
    let chunks: Vec<_> = (0..height)
        .into_par_iter()
        .map(|y| {
            let mut row = Vec::with_capacity(width * stride);
            let zy = (y as f32) * scale + y0;

            for x in 0..width {
                let zx = (x as f32) * scale + x0;
                let z = Complex32::new(zx, zy);
                let val = pixel(compute_point(z, max_iter));

                for _ in 0..stride {
                    row.push(val);
                }
            }
            (y * stride, row)
        })
        .collect();

    for (y_start, row) in chunks {
        for dy in 0..stride {
            let y = y_start + dy;
            if y < MAX_HEIGHT {
                for (x, &val) in row.iter().enumerate() {
                    if x < MAX_WIDTH {
                        buffer[y * MAX_WIDTH + x] = val;
                    }
                }
            }
        }
    }
}

fn render_frame_safe(
    buffer: &mut [u32; MAX_WIDTH * MAX_HEIGHT],
    c: Complex32,
    fractal_type: FractalType,
) {
    let view = unsafe { VIEW_STATE.clone() };
    let quality = unsafe { CURRENT_QUALITY };

    let fractal: Box<dyn Fractal> = match fractal_type {
        FractalType::Mandelbrot => Box::new(MandelbrotSet),
        FractalType::Julia => Box::new(JuliaSet { c }),
        FractalType::BurningShip => Box::new(BurningShipSet),
        FractalType::Newton => Box::new(NewtonSet),
        FractalType::RainbowNewtonSet => Box::new(RainbowNewtonSet),
        FractalType::Tricorn => Box::new(TricornSet),
        FractalType::Phoenix => Box::new(PhoenixSet { c }),
        FractalType::Celtic => Box::new(CelticSet),
    };

    fractal.render(buffer, &view, quality);
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match (h as u32) / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn pixel(i: f32) -> u32 {
    if i == 0.0 {
        return 0xFF000000;
    }

    let hue = (i * 15.0).rem_euclid(360.0);
    let saturation = 0.9;
    let value = 1.0 - (i * 0.015).cos() * 0.1;

    let (r, g, b) = hsv_to_rgb(hue, saturation, value);
    u32::from_be_bytes([0xff, r, g, b])
}

fn julia(mut z: Complex32, c: Complex32, max_i: u8) -> f32 {
    for i in 1..max_i {
        if z.norm_sqr() > 16.0 {
            let log_zn = z.norm_sqr().ln() / 2.0;
            let nu = log_zn / 4.0_f32.ln();
            return (i as f32) + 1.0 - nu;
        }
        z = z * z + c;
    }
    0.0
}

fn mandelbrot(c: Complex32, max_i: u8) -> f32 {
    let mut z = Complex32::new(0.0, 0.0);
    for i in 1..max_i {
        if z.norm_sqr() > 16.0 {
            let log_zn = z.norm_sqr().ln() / 2.0;
            let nu = log_zn / 4.0_f32.ln();
            return (i as f32) + 1.0 - nu;
        }
        z = z * z + c;
    }
    0.0
}

fn burning_ship(c: Complex32, max_i: u8) -> f32 {
    let mut z = Complex32::new(0.0, 0.0);
    for i in 1..max_i {
        if z.norm_sqr() > 16.0 {
            let log_zn = z.norm_sqr().ln() / 2.0;
            let nu = log_zn / 4.0_f32.ln();
            return (i as f32) + 1.0 - nu;
        }
        z = Complex32::new(z.re.abs(), z.im.abs());
        z = z * z + c;
    }
    0.0
}

fn newton(mut z: Complex32, max_i: u8) -> f32 {
    const ROOTS: [(Complex32, f32); 3] = [
        (Complex32::new(1.0, 0.0), 85.0),            // 1
        (Complex32::new(-0.5, 0.866025404), 170.0),  // ω^2
        (Complex32::new(-0.5, -0.866025404), 255.0), // ω
    ];
    const EPSILON: f32 = 1e-3;

    for _ in 1..max_i {
        let z2 = z * z;
        let z3 = z2 * z;
        let next = z - (z3 - Complex32::new(1.0, 0.0)) / (Complex32::new(3.0, 0.0) * z2);

        if (next - z).norm_sqr() < 1e-6 {
            return ROOTS
                .iter()
                .find(|(root, _)| (z - root).norm() < EPSILON)
                .map_or(0.0, |&(_, color)| color);
        }

        z = next;
    }
    0.0
}

fn rainbow_newton(mut z: Complex32, max_i: u8) -> f32 {
    for i in 1..max_i {
        let z2 = z * z;
        let z3 = z2 * z;
        let next = z - (z3 - Complex32::new(1.0, 0.0)) / (Complex32::new(3.0, 0.0) * z2);

        let diff = (next - z).norm_sqr();
        if diff < 1e-6 {
            let angle = z.arg();
            let base = (i as f32) * 5.0;
            return base + angle.rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU;
        }
        z = next;
    }
    0.0
}

fn tricorn(c: Complex32, max_i: u8) -> f32 {
    let mut z = Complex32::new(0.0, 0.0);
    for i in 1..max_i {
        if z.norm_sqr() > 16.0 {
            let log_zn = z.norm_sqr().ln() / 2.0;
            let nu = log_zn / 4.0_f32.ln();
            return (i as f32) + 1.0 - nu;
        }
        z = Complex32::new(z.re, -z.im) * Complex32::new(z.re, -z.im) + c;
    }
    0.0
}

fn phoenix(z: Complex32, c: Complex32, max_i: u8) -> f32 {
    let mut z_n = z;
    let mut z_n_1 = Complex32::new(0.0, 0.0);
    let p = Complex32::new(-0.5, 0.0);

    for i in 1..max_i {
        if z_n.norm_sqr() > 16.0 {
            let log_zn = z_n.norm_sqr().ln() / 2.0;
            let nu = log_zn / 4.0_f32.ln();
            return (i as f32) + 1.0 - nu;
        }
        let temp = z_n;
        z_n = z_n * z_n + c + p * z_n_1;
        z_n_1 = temp;
    }
    0.0
}

fn celtic(c: Complex32, max_i: u8) -> f32 {
    let mut z = Complex32::new(0.0, 0.0);
    for i in 1..max_i {
        if z.norm_sqr() > 16.0 {
            let log_zn = z.norm_sqr().ln() / 2.0;
            let nu = log_zn / 4.0_f32.ln(); //
            return (i as f32) + 1.0 - nu;
        }
        z = Complex32::new((z * z + c).re.abs(), (z * z + c).im);
    }
    0.0
}
