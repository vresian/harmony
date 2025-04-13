use std::sync::OnceLock;
use glib::clone;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib::{self, subclass::InitializingObject, GString}, CompositeTemplate, gio};
use tokio::runtime::Runtime;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

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

        let mut classes_str: Vec<&str> = classes
            .iter()
            .map(|gstring| gstring.as_str())
            .filter(|string| string != &"error")
            .collect();
        
        if token.is_empty() {
            classes_str.push(&"error");
            return self.token_entry.get().set_css_classes(classes_str.as_slice());
        }

        self.token_entry.set_css_classes(classes_str.as_slice());
        
        let (sender, receiver) = async_channel::bounded(1);
        
        runtime().spawn(clone!(
            #[strong]
            sender,
            async move {
                let client = reqwest::Client::new();

                let response = client
                    .get("https://discord.com/api/v9/users/@me")
                    .header(reqwest::header::AUTHORIZATION, token)
                    .send()
                    .await;

                sender
                    .send(response)
                    .await
                    .expect("The channel needs to be open.");
            }
        ));
            
        let entry = self.token_entry.get();

        glib::spawn_future_local(async move {
            while let Ok(response) = receiver.recv().await {
                if let Ok(response) = response {
                    if response.status().is_client_error() {
                        let classes = entry.css_classes();
                        let mut str_classes: Vec<&str> = classes.iter().map(|gstring| gstring.as_str()).collect();
                        str_classes.push(&"error");
                        entry.set_css_classes(&str_classes);
                    }
                } else {
                    println!("Could not make a `GET` request.");
                }
            }
        });
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
