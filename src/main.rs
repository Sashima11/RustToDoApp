mod task_object;
mod task_row;
mod window;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("todores.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk_rs.Todo1")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}