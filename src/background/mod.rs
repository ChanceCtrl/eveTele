pub mod serial;

pub struct ReadVal {
    port: String,
    id: String,
    val: f64,
}

impl ReadVal {
    pub fn start_bg_read(port: String) {
        std::thread::spawn(move || serial::test(port));
    }

    pub fn list_ports() -> Vec<String> {
        serial::list_ports()
    }
}
