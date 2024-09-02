#[allow(clippy::incompatible_msrv)]
fn main() {
    println!("WiFi network name: ");
    let wifi_network = std::io::stdin().lines().next().unwrap().unwrap();
    println!("\nPassword: ");
    let pass = std::io::stdin().lines().next().unwrap().unwrap();
    println!("\nType (WEP or [WPA - hit Enter as default]): ");
    let _network_type = std::io::stdin().lines().next().unwrap().unwrap();
    let network_type = if _network_type.is_empty() {
        "WPA".to_string()
    } else {
        _network_type
    };

    let input =
        "WIFI:S:".to_string() + &wifi_network + ";T:" + &network_type + ";P:" + &pass + ";;";

    qr2term::print_qr(input).unwrap();
}
