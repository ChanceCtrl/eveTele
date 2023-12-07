use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DropDown};

use crate::serial_script;

pub fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    let wack = serial_script::list_ports();
    let v: Vec<_> = wack.iter().map(|s| s.as_str()).collect();

    let port_drop = DropDown::from_strings(v.as_slice());

    // Connect on click to drop box
    port_drop.connect_activate(move |_| {
        println!("test");
    });

    // Present window
    window.set_child(Some(&port_drop));
    window.present();
}
