mod imp;

use adw::Application;
use adw::{gio, glib};

glib::wrapper! {
    pub struct LoginWindow(ObjectSubclass<imp::LoginWindow>)
        @extends adw::gtk::Window, adw::gtk::Widget, adw::gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, adw::gtk::Accessible, adw::gtk::Buildable, adw::gtk::ConstraintTarget,
                    adw::gtk::Native, adw::gtk::Root, adw::gtk::ShortcutManager;
}

impl LoginWindow {
    pub fn new(app: &Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
