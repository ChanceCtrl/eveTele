use gtk::prelude::*;
use gtk::{glib, Application};
use std::path::PathBuf;

const APP_ID: &str = "org.eveTele";

// Refrence other files
mod app;
mod bg;
mod serial_script;

pub struct GraphEntity;

// Fancy list of app actions
pub enum Event {
    Insert(GraphEntity),
    Remove(GraphEntity),
    Load(PathBuf, String),
    SyncToDisk,
    Modified,
    Delete,
    Closed,
    Quit,
}

fn main() -> glib::ExitCode {
    // Create new 'Application' called "app"
    let app = Application::builder().application_id(APP_ID).build();

    // Give "app" a window to present
    app.connect_activate(app::build_ui);

    // Run the app
    app.run()
}
