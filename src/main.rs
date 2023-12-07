use gtk::prelude::*;
use gtk::{glib, Application};
use std::path::PathBuf;

// Set APP_ID because GTK wants it
const APP_ID: &str = "org.eveTele";

// Refrence other files
mod app;
mod bg;
mod serial_script;

fn main() -> glib::ExitCode {
    // Create new 'Application' called "app"
    let app = Application::builder().application_id(APP_ID).build();

    // Give "app" a window to present
    app.connect_activate(|app| app::App::new(&app));

    // Run the app
    app.run()
}
