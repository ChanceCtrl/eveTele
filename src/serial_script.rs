use std::time::Duration;

use serialport;

// Read from port
pub fn test(port_name: String) -> String {
    // Connect to port
    let mut port = serialport::new(&port_name.clone(), 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Cannot open serial port");

    println!("{}", &port_name);

    // Read from a serial port
    let mut serial_buf: Vec<u8> = vec![0];
    loop {
        let mut serial_line: String = String::new();
        port.read(&mut serial_buf).expect("idk with reads");

        while serial_buf[0] != 10 {
            serial_line.push(serial_buf[0] as char);
            port.read(&mut serial_buf).expect("idk with reads 2");
        }

        println!("{}", serial_line.to_string());
    }
}

// List all ports
pub fn list_ports() -> Vec<String> {
    let mut str_p: Vec<String> = Vec::new();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        str_p.push(p.port_name);
    }

    print!("List: ");
    println!("{:?}", &str_p);

    return str_p;
}

pub fn read(port: String) -> String {
    return "fuck".to_string();
}
