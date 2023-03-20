use num::Complex;
use std::str::FromStr;

fn main() {
    
    println!("Hello, world!");
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

