use gtk::prelude::*;

// Refrence other files
mod app;
mod background;
mod widgets;

// Set APP_ID because GTK wants it
const APP_ID: &str = "org.eveTele";

fn main() -> gtk::glib::ExitCode {
    // Create new 'Application' called "app"
    let app = gtk::Application::builder().application_id(APP_ID).build();

    // Give "app" a window to present
    app.connect_activate(|app| app::AppStruct::new(&app));

    // Run the app
    app.run()
}

// If I write the word 'app' one more time ima kill myself
