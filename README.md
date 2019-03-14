# Rust library: qr2term
A stupidly simple QR code renderer for in the terminal.

[`example.rs`](./example/example.rs):
```rust
extern crate qr2term;

use qr2term::print_qr;

fn main() {
    print_qr("https://rust-lang.org/");
}
```

Produces:
![qr2term example screenshot](./res/qr2term-example.png)

This library is based on [`qair`](https://code.willemp.be/willem/qair),
which didn't provide the renderer as a library on it's own.
Credits for the actual renderer go to it's developer.

## License
This project is licensed under the MPL 2.0 license.
Check out the [LICENSE](LICENSE) file for more information.
