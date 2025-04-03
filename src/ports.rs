use port_scanner::local_port_available;
///The check_ports_in_range  function takes a start and end port number and returns a vector of available ports within that range.
fn check_ports_in_range(start: u16, end: u16) -> Vec<u16> {
    let mut available_ports = Vec::new();
    for port in start..=end {
        ///The local_port_available_range is function in an external crate, that used to check for the available ports in a certain range
        if local_port_available(port) {
            available_ports.push(port);
        }
    }
    available_ports
}
