pub mod serial;

#[derive(Clone)]
pub struct ReadVal {
    pub port: String,
    pub id: String,
    pub val: f64,
}

impl ReadVal {
    pub fn new(port: String, id: String) -> ReadVal {
        ReadVal { port, id, val: 0.0 }
    }

    pub fn bg_read(&self) -> ReadVal {
        let my_g = self.clone();

        let my_g = std::thread::spawn(move || {
            let my_g = serial::read(&my_g);

            return my_g;
        })
        .join()
        .unwrap();

        my_g
    }

    pub fn list_ports() -> Vec<String> {
        serial::list_ports()
    }
}
