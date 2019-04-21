extern crate qr2term;

use qr2term::print_qr;

fn main() {
    print_qr("https://rust-lang.org/").unwrap();
}
