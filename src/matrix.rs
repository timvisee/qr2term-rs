//! Matrix types representing 2D barcode.

use crate::util;

/// A square 2D matrix representing a barcode.
#[derive(Debug)]
pub struct Matrix<T> {
    pixels: Vec<T>,
}

impl<T> Matrix<T> {
    /// Construct a new QR matrix from given pixels.
    ///
    /// # Panics
    ///
    /// Panics if given pixel map does not have a length that is a multiple of 2.
    pub fn new(pixels: Vec<T>) -> Self {
        // Assert pixels being multiple of 2
        util::usize_sqrt(pixels.len());

        Self { pixels }
    }

    /// Get the width and height of the QR code in pixels.
    pub fn size(&self) -> usize {
        util::usize_sqrt(self.pixels.len())
    }

    /// Get the pixel matrix.
    pub fn pixels(&self) -> &[T] {
        &self.pixels
    }

    /// Surround this matrix with `quiet` pixels having the specified `thickness`.
    pub fn surround(&mut self, thickness: usize, quiet: T)
    where
        T: Copy,
    {
        // Calculate widths
        let width = self.size();
        let out_width = width + thickness * 2;

        // Build the new pixel matrix, move given matrix in the center
        let mut out = vec![quiet; out_width.pow(2)];
        for vec_row in 0..width {
            for vec_col in 0..width {
                let vec_pos = width * vec_row + vec_col;
                let out_row = vec_row + thickness;
                let out_col = vec_col + thickness;
                let out_pos = out_row * out_width + out_col;
                out[out_pos] = self.pixels[vec_pos];
            }
        }

        self.pixels = out;
    }
}

#[cfg(test)]
mod tests {
    use qrcode::types::Color::{Dark as QrDark, Light as QrLight};

    use super::*;

    /// Printing a matrix with the number of pixels not being a multiple of 2 fails.
    #[test]
    #[should_panic]
    fn matrix_incorrect_size() {
        Matrix::new(vec![QrDark, QrDark, QrLight, QrLight, QrLight, QrDark]);
    }

    #[test]
    fn surround_quiet_normal() {
        let input = vec![
            0, 1, 2, //
            3, 4, 5, //
            6, 7, 8,
        ];
        let expected = vec![
            9, 9, 9, 9, 9, 9, 9, 9, 9, //
            9, 9, 9, 9, 9, 9, 9, 9, 9, //
            9, 9, 9, 9, 9, 9, 9, 9, 9, //
            9, 9, 9, 0, 1, 2, 9, 9, 9, //
            9, 9, 9, 3, 4, 5, 9, 9, 9, //
            9, 9, 9, 6, 7, 8, 9, 9, 9, //
            9, 9, 9, 9, 9, 9, 9, 9, 9, //
            9, 9, 9, 9, 9, 9, 9, 9, 9, //
            9, 9, 9, 9, 9, 9, 9, 9, 9,
        ];
        let mut matrix = Matrix::new(input);
        matrix.surround(3, 9);
        let actual = matrix.pixels();
        assert_eq!(expected, actual);
    }

    #[test]
    fn surround_quiet_empty() {
        let mut matrix = Matrix::new(vec![]);
        matrix.surround(3, 7);
        let actual = matrix.pixels();
        let expected = vec![7; (3 * 2) * (3 * 2)];
        assert_eq!(expected, actual);
    }
}
