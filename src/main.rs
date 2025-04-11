use adw::prelude::*;
use adw::{Application, ApplicationWindow};

const APP_ID: &'static str = "com.github.vresian.harmony";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Harmony")
        .build();

    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}
