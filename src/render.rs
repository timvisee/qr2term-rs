//! Rendering utilities.

use std::io::{self, Result as IoResult, Write};

use crossterm::style::Stylize;
pub use qrcode::types::Color::{self, Dark as QrDark, Light as QrLight};

use crate::matrix::Matrix;

/// QR barcode terminal renderer intended for terminals.
#[derive(Debug)]
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

    /// How many horizontal characters or columns in the terminal it takes to render `matrix`.
    pub fn width(&self, matrix: &Matrix<Color>) -> usize {
        return matrix.size();
    }

    /// How many vertical characters or rows or lines in the terminal it takes to render `matrix`.
    pub fn height(&self, matrix: &Matrix<Color>) -> usize {
        return matrix.size() / 2 + matrix.size() % 2;
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

#[cfg(test)]
mod tests {
    use super::*;

    mod size_tracker {
        //! Tracks how many newlines and character per line are written

        use regex::Regex;
        use std::io::Write;

        pub struct SizeTracker {
            data: Vec<u8>,
        }

        impl Write for SizeTracker {
            fn write(&mut self, data: &[u8]) -> std::result::Result<usize, std::io::Error> {
                self.data.extend(data);
                Ok(data.len())
            }

            fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
                Ok(())
            }
        }

        impl SizeTracker {
            pub fn new() -> Self {
                SizeTracker { data: vec![] }
            }

            fn without_ansi_codes(text: &str) -> String {
                let regex = Regex::new("\x1B\\[.*?m").unwrap();
                regex.replace_all(text, "").to_string()
            }

            /// Length of longest line in visible characters.
            ///
            /// Panics if data seen by tracker is not valid UTF-8.
            pub fn width(&self) -> usize {
                if self.data.len() == 0 {
                    return 0;
                }
                let data_str = std::str::from_utf8(&self.data).unwrap();
                let without_ansi_codes = Self::without_ansi_codes(data_str);
                without_ansi_codes
                    .split("\n")
                    .map(|line| line.chars().count())
                    .max()
                    .unwrap()
            }

            pub fn height(&self) -> usize {
                let newline = 10;
                self.data.iter().filter(|&elem| *elem == newline).count()
            }
        }
    }

    /// Checks that the expected, promised, and actual width and height match
    /// when rendering `pixels` to a terminal QR code.
    fn helper_width_and_height(pixels: Vec<Color>, expected_width: usize, expected_height: usize) {
        // Given: a matrix, and a renderer for that matrix.
        let matrix = Matrix::new(pixels);
        let renderer = Renderer::default();
        let mut writer = size_tracker::SizeTracker::new();

        // When: rendering the matrix
        let promised_width = renderer.width(&matrix);
        let promised_height = renderer.height(&matrix);
        renderer.render(&matrix, &mut writer).unwrap();
        let actual_height = writer.height();
        let actual_width = writer.width();

        // Then: the width & height promised by the renderer is the expected width & height,
        //       and the width & height delivered by the renderer is the expected width & height.
        assert_eq!(expected_width, promised_width);
        assert_eq!(expected_height, promised_height);
        assert_eq!(expected_width, actual_width);
        assert_eq!(expected_height, actual_height);
    }

    #[test]
    fn width_and_height() {
        helper_width_and_height(vec![], 0, 0);
        helper_width_and_height(vec![QrDark], 1, 1);
        helper_width_and_height(vec![QrDark, QrLight, QrLight, QrDark], 2, 1);
        helper_width_and_height(vec![QrDark; 3 * 3], 3, 2);
        helper_width_and_height(vec![QrLight; 4 * 4], 4, 2);
        helper_width_and_height(vec![QrLight; 5 * 5], 5, 3);
        helper_width_and_height(vec![QrDark; 21 * 21], 21, 11);
    }
}
