//! Rendering utilities.

use std::io::{self, Result as IoResult, Write};

use crossterm::style::Colorize;
use qrcode::types::Color::{self, Dark as QrDark, Light as QrLight};

use crate::matrix::Matrix;

///! QR barcode terminal renderer intended for terminals.
pub struct Renderer {}

impl Renderer {
    /// Print a matrix describing a 2D barcode to the given writer.
    pub fn render<W: Write>(&self, matrix: &Matrix<Color>, target: &mut W) -> IoResult<()> {
        let width = matrix.size();
        let pixels = matrix.pixels();

        for row in 0..width / 2 {
            for col in 0..width {
                let vec_pos = (row * 2) * width + col;
                let vec_pos_below = (row * 2 + 1) * width + col;
                match (pixels[vec_pos], pixels[vec_pos_below]) {
                    (QrDark, QrDark) => self.black_above_black(target)?,
                    (QrDark, QrLight) => self.black_above_white(target)?,
                    (QrLight, QrDark) => self.white_above_black(target)?,
                    (QrLight, QrLight) => self.white_above_white(target)?,
                };
            }
            self.newline(target)?;
        }

        // Because one character is two "pixels" above each other, the last pixel-line
        // has only white ("empty") "pixels" in case of an odd number of pixelrows.
        if width % 2 == 1 {
            for col in 0..width {
                let vec_pos = width * (width - 1) + col;
                match pixels[vec_pos] {
                    QrDark => self.black_above_white(target)?,
                    QrLight => self.white_above_white(target)?,
                };
            }
            self.newline(target)?;
        }

        Ok(())
    }

    /// Print a matrix describing a 2D barcode to the terminal.
    pub fn print_stdout(&self, matrix: &Matrix<Color>) {
        self.render(matrix, &mut io::stdout())
            .expect("failed to print QR code to stdout");
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
    fn black_above_white<W: Write>(&self, target: &mut W) -> IoResult<()> {
        write!(target, "{}", "▄".white().on_black())
    }

    /// Similar to `black_above_white`
    fn white_above_black<W: Write>(&self, target: &mut W) -> IoResult<()> {
        write!(target, "{}", "▄".black().on_white())
    }

    /// Similar to `black_above_white`
    fn black_above_black<W: Write>(&self, target: &mut W) -> IoResult<()> {
        write!(target, "{}", " ".white().on_black())
    }

    /// Similar to `black_above_white`
    fn white_above_white<W: Write>(&self, target: &mut W) -> IoResult<()> {
        write!(target, "{}", " ".black().on_white())
    }

    /// Print newline that does not mess up colors.
    fn newline<W: Write>(&self, target: &mut W) -> IoResult<()> {
        writeln!(target)
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {}
    }
}
