use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::str::FromStr;
use std::fs::File;
use std::env;

fn main() {

    let args : Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandelbrot.png 1024x768 -1.20,0.35 -1,0.2", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair::<usize>(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper-left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower-right corner point");

    // let bounds:(usize, usize) = (1024, 768);
    // let upper_left = Complex{re:-1.2, im:0.35};
    // let lower_right = Complex{re:-1.0, im:0.2};
    
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator:char) -> Option<(T, T)> {

    match s.find(separator) {

        None => None,
        Some(index) => {

            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }

    }
}

fn parse_complex(s : &str) -> Option<Complex<f64>> {

    match parse_pair(s, ',') {

        None => None,
        Some((re, im)) => Some(Complex{re, im})
    }
}


fn lerp(a : f64, b : f64, t : f64) -> f64 {

    a * (1.0 - t) + b * t
}


/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(bounds : (usize, usize),
                  pixel : (usize, usize),
                  upper_left : Complex<f64>,
                  lower_right : Complex<f64>) -> Complex<f64> {

    Complex{
        re:lerp(upper_left.re, lower_right.re, pixel.0 as f64 / bounds.0 as f64),
        im:lerp(upper_left.im, lower_right.im, pixel.1 as f64 / bounds.1 as f64)
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit' iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered
/// on the origin. If `c` seems to be a member (more precisely, if we
/// reached the iteration limit without being able to prove that `c` is
/// not a member), return `None`.
fn escape_time(c : Complex<f64>, limit:usize) -> Option<usize> {

    let mut z:Complex<f64> = Complex{ re: 0.0, im: 0.0 };

    for i in 0..limit {

        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None // no escape time (assumed infinite)
}

fn render(pixels : &mut [u8],
        bounds : (usize, usize),
        upper_left : Complex<f64>,
        lower_right : Complex<f64>) {

    assert!(pixels.len() == bounds.0 * bounds.1);

    for y in 0..bounds.1 {

        for x in 0..bounds.0 {

            let point = pixel_to_point(bounds, (x, y), upper_left, lower_right);

            pixels[y * bounds.0 + x] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds : (usize, usize)) -> Result<(), std::io::Error> {

    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}


#[test]
fn test_lerp() {

    assert_eq!(lerp(10.0, 20.0, 0.0), 10.0);
    assert_eq!(lerp(10.0, 20.0, 0.5), 15.0);
    assert_eq!(lerp(10.0, 20.0, 1.0), 20.0);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });
}

#[test]
fn test_parse_pair() {

    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {

    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re:1.25, im:-0.0625}));
    assert_eq!(parse_complex("0.0625,"), None);
    assert_eq!(parse_complex(",-0.0625"), None);
}