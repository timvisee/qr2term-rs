// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![warn(missing_debug_implementations, missing_docs)]

//! A stupidly simple QR code renderer, that prints text as QR code to the terminal,
//! and nothing else.
//!
//! # Examples
//! [`example.rs`](./example/example.rs):
//! ```rust
//! qr2term::print_qr("https://rust-lang.org/").unwrap();
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

pub mod matrix;
pub mod qr;
pub mod render;
pub(crate) mod util;

pub use qrcode::types::QrError;

use crate::matrix::Matrix;
use crate::render::Renderer;

/// Quiet zone size in pixels around QR code.
///
/// Should be 4, but using 2 for small terminals:
/// https://qrworld.wordpress.com/2011/08/09/the-quiet-zone/
const QUIET_ZONE_WIDTH: usize = 2;

/// Print the given `data` as QR code in the terminal.
///
/// Returns an error if generating the QR code failed.
///
/// # Examples
///
/// ```rust
/// qr2term::print_qr("https://rust-lang.org/").unwrap();
/// ```
///
/// # Panics
///
/// Panics if printing the QR code to the terminal failed.
pub fn print_qr<D: AsRef<[u8]>>(data: D) -> Result<(), QrError> {
    // Generate QR code pixel matrix
    let mut matrix = qr::Qr::from(data)?.to_matrix();
    matrix.surround(QUIET_ZONE_WIDTH, render::QrLight);

    // Render QR code to stdout
    Renderer::default().print_stdout(&matrix);
    Ok(())
}

/// Generate `String` from the given `data` as QR code.
///
/// Returns an error if generating the QR code failed.
///
/// # Examples
///
/// ```rust
/// let qr_string = qr2term::generate_qr_string("https://rust-lang.org/").unwrap();
/// print!("{}", qr_string);
/// ```
///
/// # Panics
///
/// Panics if generating the QR code string failed.
pub fn generate_qr_string<D: AsRef<[u8]>>(data: D) -> Result<String, QrError> {
    // Generate QR code pixel matrix
    let mut matrix = qr::Qr::from(data)?.to_matrix();
    matrix.surround(QUIET_ZONE_WIDTH, render::QrLight);

    // Render QR code to a String
    let mut buf = Vec::new();
    Renderer::default()
        .render(&matrix, &mut buf)
        .expect("failed to generate QR code string");
    Ok(String::from_utf8(buf).unwrap())
}
