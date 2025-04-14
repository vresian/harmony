mod imp;

use crate::logic::login_window::imp::LoginWindow;
use adw::gtk::prelude::*;
use gtk::glib::{self, Object};
use adw::gtk;

glib::wrapper! {
    pub struct RememberAccountDialog(ObjectSubclass<imp::RememberAccountDialog>)
        @extends gtk::Widget, gtk::Window, gtk::Dialog, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl RememberAccountDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn show_dialog(&self, window: &LoginWindow) {
        self.present();
    }
}
