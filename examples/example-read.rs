#[allow(clippy::incompatible_msrv)]
fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    qr2term::print_qr(line).unwrap();
}
