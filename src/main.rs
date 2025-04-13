mod ui;

use ui::login_window::LoginWindow;
use adw::prelude::*;
use adw::{Application, ApplicationWindow, gio};

const APP_ID: &'static str = "com.github.vresian.harmony";

fn build_ui(app: &Application) {
    let window = LoginWindow::new(app);

    window.present();
}

fn main() {
    gio::resources_register_include!("harmony.gresource").expect("Failed to register resources");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}
