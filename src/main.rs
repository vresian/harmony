mod logic;
mod api;

use logic::login_window::LoginWindow;
use adw::prelude::*;
use adw::{Application, gio, gtk::{CssProvider, gdk}};

const APP_ID: &'static str = "com.github.vresian.harmony";

fn build_ui(app: &Application) {
    let window = LoginWindow::new(app);

    window.present();
}

fn load_css_files(paths: Vec<&str>) {
    let screen = gdk::Display::default().expect("Could not connect to a display");

    for path in paths {
        let provider = CssProvider::new();
        provider.load_from_string(path);

        gtk::style_context_add_provider_for_display(
            &screen, &provider, adw::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        )
    }
}

fn main() {
    gio::resources_register_include!("harmony.gresource").expect("Failed to register resources");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    
    app.connect_startup(|_| load_css_files(vec![
        include_str!("styles/login_window.css"),
        include_str!("styles/remember_account_dialog.css")
    ]));
    app.connect_activate(build_ui);

    app.run();
}
