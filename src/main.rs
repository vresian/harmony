mod ui;

use ui::login_window::LoginWindow;
use adw::prelude::*;
use adw::{Application, ApplicationWindow, gio, gtk::{CssProvider, gdk}};

const APP_ID: &'static str = "com.github.vresian.harmony";

fn build_ui(app: &Application) {
    let window = LoginWindow::new(app);

    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("css/login_window.css"));

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display"), 
        &provider, adw::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn main() {
    gio::resources_register_include!("harmony.gresource").expect("Failed to register resources");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}
