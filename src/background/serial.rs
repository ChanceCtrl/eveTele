use std::time::Duration;

use serialport;

use super::ReadVal;

// Read from port
pub fn read(read: &ReadVal) -> ReadVal {
    // Connect to the port from "read"
    let mut port = serialport::new(&read.port.clone(), 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Cannot open serial port");

    // Set up some vars to store the data we get
    let mut serial_buf: Vec<u8> = vec![0];
    let mut serial_line: String = String::new();

    // Read the data from the serial port and put it into a buf
    port.read(&mut serial_buf).expect("idk with reads");

    // Go thru each byte and save it to our string until we hit the return char
    while serial_buf[0] != 10 {
        serial_line.push(serial_buf[0] as char);
        port.read(&mut serial_buf).expect("idk with reads 2");
    }

    if &serial_line[0..4] == read.id {
        // Return the value read
        return ReadVal {
            port: read.port.clone(),
            id: read.id.clone(),
            val: serial_line[5..(serial_line.len() - 1)]
                .parse::<f64>()
                .unwrap_or(0.0),
        };
    } else {
        // Return generic value on fail and print debug
        println!("Can't parse the new line for some fucking reason lmao");

        return ReadVal {
            port: read.port.clone(),
            id: read.id.clone(),
            val: 0.0,
        };
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
