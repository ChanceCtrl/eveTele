use std::path::PathBuf;

use gtk::glib::{Receiver, Sender};

pub enum BgEvent {
    Load(PathBuf),

    // Save layout to a file
    Save(PathBuf, String),

    // Exit the main event loop
    Quit,
}
