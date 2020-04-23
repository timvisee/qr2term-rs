//! QR code type.

use qrcode::{types::Color, QrCode};

use super::QrError;
use crate::Matrix;

/// Raw QR code.
#[allow(missing_debug_implementations)]
pub struct Qr {
    code: QrCode,
}

impl Qr {
    /// Construct a new QR code.
    pub fn from<D: AsRef<[u8]>>(data: D) -> Result<Self, QrError> {
        Ok(Self {
            // TODO: error handle here!
            code: QrCode::new(data.as_ref())?,
        })
    }

    /// Create pixel matrix from this QR code.
    pub fn to_matrix(&self) -> Matrix<Color> {
        Matrix::new(self.code.to_colors())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generating QR codes for text that is too large should fail.
    #[test]
    #[should_panic]
    fn print_qr_too_long() {
        Qr::from(&String::from_utf8(vec![b'a'; 8000]).unwrap()).unwrap();
    }
}
