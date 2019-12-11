// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A stupidly simple QR code renderer, that prints text as QR code to the terminal,
//! and nothing else.
//!
//! # Examples
//! [`example.rs`](./example/example.rs):
//! ```rust
//! use qr2term::print_qr;
//!
//! fn main() {
//!     print_qr("https://rust-lang.org/").unwrap();
//! }
//! ```
//!
//! ![qr2term example screenshot](./res/qr2term-example.png)
//!
//! # Based on
//! This library is based on [`qair`](https://code.willemp.be/willem/qair),
//! which didn't provide the renderer as a library on it's own.
//! Credits for the actual renderer go to it's developer.
//!
//! - [https://crates.io/crates/qair](https://crates.io/crates/qair)
//! - [https://code.willemp.be/willem/qair/src/branch/master/src/console_barcode_renderer.rs](https://code.willemp.be/willem/qair/src/branch/master/src/console_barcode_renderer.rs)

use crossterm::style::Colorize;
pub use qrcode::types::QrError;
use qrcode::{
    types::Color::{self as QrColor, Dark as QrDark, Light as QrLight},
    QrCode,
};

/// Quiet zone size in pixels around QR code.
///
/// Should be 4, but using 2 for small terminals:
/// https://qrworld.wordpress.com/2011/08/09/the-quiet-zone/
const QUIET_ZONE_WIDTH: usize = 2;

/// Print the given `text` as QR code in the terminal.
///
/// Returns an error if generating the QR code failed.
///
/// # Panics
///
/// Panics if printing the QR code to the terminal failed.
pub fn print_qr(text: &str) -> Result<(), QrError> {
    Renderer::new().print_qr(text)
}

///! QR barcode terminal renderer.
struct Renderer {}

impl Renderer {
    /// Construct a new renderer.
    pub fn new() -> Self {
        Renderer {}
    }

    /// Print the given `text` as QR code in the terminal.
    ///
    /// Returns an error if generating the QR code failed.
    ///
    /// # Panics
    ///
    /// Panics if printing the QR code to the terminal failed.
    pub fn print_qr(&mut self, text: &str) -> Result<(), QrError> {
        // Generate the code, obtain the QR code colors
        let pixels = QrCode::new(text)?.into_colors();

        // Surround the code with quiet zone
        let pixels = Self::surround_quiet(&pixels, QUIET_ZONE_WIDTH, QrLight);

        // Print the code
        self.print_matrix(&pixels);
        Ok(())
    }

    /// Print a matrix describing a 2D barcode to the terminal.
    ///
    /// The barcode is given as 1D slice.
    ///
    /// # Panics
    ///
    /// Panics if the given matrix of `pixels` doens't have a length that is a multiple of 2.
    fn print_matrix(&mut self, pixels: &[QrColor]) {
        let width = usize_sqrt(pixels.len());

        for row in 0..width / 2 {
            for col in 0..width {
                let vec_pos = (row * 2) * width + col;
                let vec_pos_below = (row * 2 + 1) * width + col;
                match (pixels[vec_pos], pixels[vec_pos_below]) {
                    (QrDark, QrDark) => self.black_above_black(),
                    (QrDark, QrLight) => self.black_above_white(),
                    (QrLight, QrDark) => self.white_above_black(),
                    (QrLight, QrLight) => self.white_above_white(),
                };
            }
            self.newline();
        }

        // Because one character is two "pixels" above each other, the last pixel-line
        // has only white ("empty") "pixels" in case of an odd number of pixelrows.
        if width % 2 == 1 {
            for col in 0..width {
                let vec_pos = width * (width - 1) + col;
                match pixels[vec_pos] {
                    QrDark => self.black_above_white(),
                    QrLight => self.white_above_white(),
                };
            }
            self.newline()
        }
    }

    /// Surround a given matrix with `quiet` pixels having the specified `thickness`.
    ///
    /// The matrix is given as 1D slice.
    ///
    /// # Panics
    ///
    /// Panics if the given matrix of `pixels` doens't have a length that is a multiple of 2.
    fn surround_quiet<T: Copy>(pixels: &[T], thickness: usize, quiet: T) -> Vec<T> {
        // Calculate widths
        let width = usize_sqrt(pixels.len());
        let out_width = width + thickness * 2;

        // Build the new pixel matrix, move given matrix in the center
        let mut out = vec![quiet; out_width.pow(2)];
        for vec_row in 0..width {
            for vec_col in 0..width {
                let vec_pos = width * vec_row + vec_col;
                let out_row = vec_row + thickness;
                let out_col = vec_col + thickness;
                let out_pos = out_row * out_width + out_col;
                out[out_pos] = pixels[vec_pos]
            }
        }

        out
    }

    /// Terminal-format and print one character that show a black pixel above a white pixel.
    ///
    /// The naive approach would be to use "█", "▀", "▄", and " ".
    /// Unfortunately, "█" and "▀" are rendered on some terminals/fonts with a gap
    /// above it, so putting them under each other results in
    /// a gap between the lines. Luckily "▄" seems to be rendered
    /// without gap under it, so we workaround the problem by
    /// using color inversion (so "█" = " " inverted, and "▀" = "▄" inverted).
    /// "▄" seems to render better than "▅".
    fn black_above_white(&self) {
        print!("{}", "▄".white().on_black());
    }

    /// Similar to `black_above_white`
    fn white_above_black(&self) {
        print!("{}", "▄".black().on_white());
    }

    /// Similar to `black_above_white`
    fn black_above_black(&self) {
        print!("{}", " ".white().on_black());
    }

    /// Similar to `black_above_white`
    fn white_above_white(&self) {
        print!("{}", " ".black().on_white());
    }

    /// Print newline that does not mess up colors.
    fn newline(&mut self) {
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Printing a matrix with the number of pixels not being a multiple of 2 fails.
    #[test]
    #[should_panic]
    fn print_matrix_incorrect_size() {
        Renderer::new().print_matrix(&vec![QrDark, QrDark, QrLight, QrLight, QrLight, QrDark]);
    }

    #[test]
    fn surround_quiet_normal() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 0, 1, 2, 9, 9, 9, 9, 9, 9, 3, 4, 5, 9, 9, 9, 9, 9, 9, 6, 7, 8, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        ];
        let actual = Renderer::surround_quiet(&input, 3, 9);
        assert_eq!(expected, actual);
    }

    #[test]
    fn surround_quiet_empty() {
        let actual = Renderer::surround_quiet(&[], 3, 7);
        let expected = vec![7; (3 * 2) * (3 * 2)];
        assert_eq!(expected, actual);
    }

    /// Generating QR codes for text that is too large should fail.
    #[test]
    fn print_qr_too_long() {
        print_qr(&String::from_utf8(vec![b'a'; 8000]).unwrap())
            .err()
            .unwrap();
    }
}

/// Take the square root of the given usize.
///
/// # Panics
///
/// Panics if the given number isn't a factor of 2.
#[inline(always)]
fn usize_sqrt(num: usize) -> usize {
    let sqrt = (num as f64).sqrt() as usize;
    assert_eq!(num, sqrt * sqrt, "given number isn't a multiple of 2");
    sqrt as usize
}
