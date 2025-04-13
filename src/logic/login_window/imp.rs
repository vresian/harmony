use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib::{self, subclass::InitializingObject, GString}, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/vresian/harmony/login_window.ui")]
pub struct LoginWindow {
    #[template_child]
    log_in_button: TemplateChild<gtk::Button>,
    #[template_child]
    token_entry: TemplateChild<adw::PasswordEntryRow>
}

#[glib::object_subclass]
impl ObjectSubclass for LoginWindow {
    const NAME: &'static str = "HarmonyLoginWindow";
    type Type = super::LoginWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
        class.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl LoginWindow {
    #[template_callback]
    fn handle_log_in_attempt(&self, _: &gtk::Button) {
        let token = self.token_entry.get().text().to_string();
        
        let classes = self.token_entry.get().css_classes();

        let mut classes_str: Vec<&str> = classes.
            iter()
            .map(|gstring| gstring.as_str())
            .filter(|string| string != &"error")
            .collect();
        
        if token.is_empty() {
            classes_str.push(&"error");
            return self.token_entry.get().set_css_classes(classes_str.as_slice());
        }

        self.token_entry.set_css_classes(classes_str.as_slice());
        println!("{}", self.token_entry.get().text());
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
