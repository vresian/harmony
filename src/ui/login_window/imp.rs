use adw::subclass::prelude::*;
use gtk::{glib::{self, subclass::InitializingObject}, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/vresian/harmony/login_window/login_window.ui")]
pub struct LoginWindow {}

#[glib::object_subclass]
impl ObjectSubclass for LoginWindow {
    const NAME: &'static str = "HarmonyLoginWindow";
    type Type = super::LoginWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for LoginWindow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}


impl WidgetImpl for LoginWindow {}
impl WindowImpl for LoginWindow {}
impl ApplicationWindowImpl for LoginWindow {}
impl AdwApplicationWindowImpl for LoginWindow {}
