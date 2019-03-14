/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Renders a QR code to the console.

use std::sync::Arc;

use crossterm::{style, Color, TerminalOutput};
use qrcode::{
    QrCode,
    types::Color::{Dark, Light},
};

/// Print the given `text` as QR code in the terminal.
pub fn print_qr(text: &str) -> Result<(), String> {
    Renderer::new().print_qr(text)
}

///! QR barcode terminal renderer.
struct Renderer {
    /// The screen to output to.
    screen: Arc<TerminalOutput>,
}

impl Renderer {
    /// Construct a new renderer.
    pub fn new() -> Self {
        Renderer {
            screen: Arc::new(TerminalOutput::default()),
        }
    }

    /// Print the given `text` as QR code in the terminal.
    pub fn print_qr(&mut self, text: &str) -> Result<(), String> {
        let code = QrCode::new(text).map_err(|err| format!("Cannot render QR code: {}", err))?;
        let width = code.width();
        let vec = code.into_colors();

        // Should be 4, but 1 works fine and we don't have much space.
        // (see https://qrworld.wordpress.com/2011/08/09/the-quiet-zone/)
        let quiet_width = 1;

        let vec = Self::add_quiet_zone(&vec, width, quiet_width, Light);
        self.print_barcode_vec(&vec, width + (quiet_width * 2));
        Ok(())
    }

    /// Print a 2D barcode to the terminal.
    ///
    /// The barcode is given as 1D slice.
    ///
    /// # Panics
    ///
    /// Panics if `vec_width` isn't `vec.len()` squared.
    fn print_barcode_vec(&mut self, vec: &Vec<qrcode::types::Color>, vec_width: usize) {
        assert_eq!(vec.len(), vec_width * vec_width);

        for row in 0..vec_width / 2 {
            for col in 0..vec_width {
                let vec_pos = (row * 2) * vec_width + col;
                let vec_pos_below = (row * 2 + 1) * vec_width + col;
                match (vec[vec_pos], vec[vec_pos_below]) {
                    (Dark, Dark) => self.black_above_black(),
                    (Dark, Light) => self.black_above_white(),
                    (Light, Dark) => self.white_above_black(),
                    (Light, Light) => self.white_above_white(),
                };
            }
            self.newline();
        }

        // Because one character is two "pixels" above each other, the last pixel-line
        // has only white ("empty") "pixels" in case of an odd number of pixelrows.
        if vec_width % 2 == 1 {
            for col in 0..vec_width {
                let vec_pos = vec_width * (vec_width - 1) + col;
                match vec[vec_pos] {
                    Dark => self.black_above_white(),
                    Light => self.white_above_white(),
                };
            }
            self.newline()
        }
    }

    /// Surround given 2D barcode with empty space.
    ///
    /// The barcode is given as 1D slice.
    ///
    /// # Panics
    ///
    /// Panics if `vec_width` isn't `vec.len()` squared.
    fn add_quiet_zone<T: Copy>(
        vec: &Vec<T>,
        vec_width: usize,
        quiet_width: usize,
        quiet: T,
    ) -> Vec<T> {
        assert_eq!(vec.len(), vec_width * vec_width);

        let out_width = vec_width + quiet_width * 2;
        let mut out = Vec::new();
        out.resize(out_width * out_width, quiet);

        for vec_row in 0..vec_width {
            for vec_col in 0..vec_width {
                let vec_pos = vec_width * vec_row + vec_col;
                let out_row = vec_row + quiet_width;
                let out_col = vec_col + quiet_width;
                let out_pos = out_row * out_width + out_col;
                out[out_pos] = vec[vec_pos]
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
        style("▄")
            .with(Color::White)
            .on(Color::Black)
            .paint(&self.screen)
            .expect("failed to paint QR code")
    }

    /// Similar to `black_above_white`
    fn white_above_black(&self) {
        style("▄")
            .with(Color::Black)
            .on(Color::White)
            .paint(&self.screen)
            .expect("failed to paint QR code")
    }

    /// Similar to `black_above_white`
    fn black_above_black(&self) {
        style(" ")
            .with(Color::White)
            .on(Color::Black)
            .paint(&self.screen)
            .expect("failed to paint QR code")
    }

    /// Similar to `black_above_white`
    fn white_above_white(&self) {
        style(" ")
            .with(Color::Black)
            .on(Color::White)
            .paint(&self.screen)
            .expect("failed to paint QR code")
    }

    /// Print newline that does not mess up colors.
    fn newline(&mut self) {
        style("\n")
            .paint(&self.screen)
            .expect("failed to paint QR code")
    }
}

#[cfg(test)]
mod tests {
    mod print_barcode_vec {
        use super::super::*;

        #[test]
        #[should_panic]
        fn incorrect_size() {
            // Given: a barcode that is not square
            let vec = vec![Dark, Dark, Light, Light, Light, Dark];

            // When: trying rendering the vector on the terminal
            let _ = Renderer::new().print_barcode_vec(&vec, 2);

            // Then: panic occurred
        }
    }

    mod add_quiet_zone {
        use super::super::*;

        #[test]
        fn empty() {
            // Given: nothing

            // When: adding quiet zone of 3 width to an empty vector
            let actual_result = Renderer::add_quiet_zone::<i32>(&vec![], 0, 3, 7);

            // Then: the result is a vector with quiet space above,left,under,right
            let expected_result = vec![7; (3 * 2) * (3 * 2)];
            assert_eq!(expected_result, actual_result);
        }

        #[test]
        fn normal_case() {
            // Given: a normal vector
            let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

            // When: adding quiet zone of 3 width to the given fector
            let actual_result = Renderer::add_quiet_zone::<i32>(&input, 3, 3, 9);

            // Then: the result is as expected
            let expected_result = vec![
                9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                9, 9, 0, 1, 2, 9, 9, 9, 9, 9, 9, 3, 4, 5, 9, 9, 9, 9, 9, 9, 6, 7, 8, 9, 9, 9, 9, 9,
                9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            ];
            assert_eq!(expected_result, actual_result);
        }

        #[test]
        #[should_panic]
        fn incorrect_size() {
            // Given: a normal vector
            let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

            // When: the size is incorrect
            let incorrect_size = 4;
            let _ = Renderer::add_quiet_zone::<i32>(&input, incorrect_size, 3, 9);

            // Then: panic occurred
        }
    }

    mod print_qr {
        use super::super::*;

        #[test]
        fn too_long() {
            // Given: an input string exceeding the max length of data a QR code can contain
            let character_a = 97;
            let long_input = String::from_utf8(vec![character_a; 8000]).unwrap();

            // When: printing a text to a QR code to the terminal
            let actual_result = print_qr(&long_input);

            // Then: there is an error
            match actual_result {
                Ok(_) => panic!(),
                Err(_) => (),
            }
        }
    }
}
