//! Simple Mandelbrot set plotter.

use image::png::PNGEncoder;
use image::ColorType;
use num;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

type C = num::Complex<f64>

/// Mandelbrot set computation.
/// How how many iterations are needed to escape from the bounding circle?
/// Iteration step: z := z^2 + c.
/// Exit criterion: z.norm_sqr() >= 4.0
/// Returns None if max number of iterations is reached.
fn escape_time(c: C, cutoff: u32) -> Option<u32> {
    let mut z = C::new(0., 0.);
    for i in 0..cutoff {
        z = z * z + c;
        if z.norm_sqr() >= 4. {
            return Some(i);
        }
    }
    None
}

/// Parses a pair like "640x480" into two numbers of type T.
/// Returns error if either the separator is not found or the number could not be parsed correctly.
/// See tests for required error messages.
fn parse_pair<T: FromStr>(arg: &str) -> Result<(T, T), Box<dyn Error>> {
    // Assignment 2
    Err(format!("{}", "Not implemented").into())
}

#[test]
fn test_parse_pair_errors() {
    for input in &["", "10,20"] {
        assert_eq!(
            parse_pair::<i32>(input).unwrap_err().to_string(),
            format!("separator 'x' not found in argument '{}'", input)
        );
    }
    for input in &["x", "10x", "x20"] {
        assert_eq!(
            parse_pair::<i32>(input).unwrap_err().to_string(),
            format!("failed to parse pair '{}'", input)
        );
    }
}

#[test]
fn test_parse_pair_ok() {
    assert_eq!(parse_pair::<i32>("20x30").unwrap(), (20, 30));
    assert_eq!(parse_pair::<f64>("1.2x3.4").unwrap(), (1.2, 3.4));
}

/// Gets image size from command line argument.
fn parse_bounds(string: &str) -> Result<(u32, u32), Box<dyn Error>> {
    Ok(parse_pair::<u32>(string)?)
}

/// Total number of pixels in the image.
fn pixels(bounds: (u32, u32)) -> usize {
    (bounds.0 * bounds.1) as usize
}

/// Renders the whole image. Returns raw image data in row-major order.
fn render(bounds: (u32, u32), window: &Window) -> Vec<u8> {
    let mut out: Vec<u8> = vec![0; pixels(bounds)];
    for row in 0..bounds.1 {
        let start = (row * bounds.0) as usize;
        let end = ((row + 1) * bounds.0) as usize;
        render_row(&mut out[start..end], row, bounds, window);
    }
}

/// Renders a single row. Results are directly written into the passed slice to avoid excessive Vec
/// allocations.
fn render_row(out: &mut [u8], row: u32, bounds: (u32, u32), window: Window) {
    for col in 0..bounds.0 {
        let point = pixel2point(Pixel { x: col, y: row }, bounds, window);
        out[col as usize] = match escape_time(point, 255) {
            None => 0,
            Some(count) => 255 - count as u8,
        };
    }
}

/// A single pixel in the output image. (0, 0) is in the lower left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pixel {
    x: u32,
    y: u32,
}

/// A window into the complex number plane.
#[derive(Debug, Clone, PartialEq)]
struct Window {
    ul: C, // upper left
    lr: C, // lower right
}

/// Coordinate transformation between image pixels and complex numbers.
///
/// Image bounds:
/// (0,h)---------------(w,h)
///   |                   |
///   |                   |
///   |                   |
/// (0,0)---------------(w,0)
///
/// Complex plane window:
///
///              ^ im
///              |
///   UL---------|---------+
///    |         |         |
/// -------------+-------------> re
///    |         |         |
///    +---------|---------LR
///              |
fn pixel2point(pix: Pixel, bounds: (u32, u32), window: &Window) -> C {
    let width = window.lr.re - window.ul.re;
    let height = window.ul.im - window.lr.im;
    C::new(
        window.ul.re + pix.x as f64 * width / bounds.0 as f64,
        window.ul.im - pix.y as f64 * height / bounds.1 as f64,
    )
}

#[test]
fn test_pixel2point() {
    assert_eq!(
        pixel2point(
            Pixel { x: 25, y: 75 },
            100,
            &Window {
                ul: C::new(-1., 1.),
                lr: C::new(1., -1.)
            }
        ),
        C::new(-0.5, -0.5)
    );
}

/// Writes PNG image file to disk.
fn write(file: &Path, img: &[u8], bounds: (u32, u32)) -> Result<(), io::Error> {
    let encoder = PNGEncoder::new(File::create(file)?);
    encoder.encode(&img, bounds.0, bounds.1, ColorType::Gray(8))?;
    Ok(())
}

/// Driver function. Parameters are passed as found on the command line.
fn run(
    file: &Path,
    bounds: &str,
    upper_left: &str,
    lower_right: &str,
) -> Result<(), Box<dyn Error>> {
    let bounds = parse_bounds(bounds)?;
    let window = Window {
        ul: upper_left.parse()?,
        lr: lower_right.parse()?,
    };
    let img = render(bounds, &window);
    write(file, &img, bounds)?;
    Ok()

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 5 {
        eprintln!(
            "Usage: mandelbrot FILE PIXELS UPPER_LEFT LOWER_RIGHT\n\
             Example: mandelbrot out.png 1280x960 -1.2+0.35i -1+0.2i"
        );
        exit(1);
    }
    if let Err(e) = run(Path::new(&args[1]), &args[2], &args[3], &args[4]) {
        eprintln!("Error: {}", e);
        exit(2);
    }
}

#[cfg(test)]
use std::fs::{read, remove_file};

#[test]
fn test_image() -> Result<(), Box<dyn Error>> {
    // use path API to cover platform differences
    let filename = env::temp_dir().join("test.png");
    assert!(run(&filename, "64x48", "-1.2+0.35i", "-1+0.2i").is_ok());
    let reference = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("64x48.png");
    assert_eq!(read(&filename)?, read(reference)?, "PNG files differ");
    remove_file(filename)?;
    Ok(())
}
