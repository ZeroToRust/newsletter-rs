use port_scanner::local_ports_available_range;
fn port_scanner() -> String {
    let mut port = "port".to_string();
for available in local_ports_available_range(8000..8080) {
    println!("Port {} is available to use", available);
    port = available.to_string();
    break;
}
port
}