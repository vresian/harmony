use std::sync::OnceLock;
use glib::clone;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib::{self, subclass::InitializingObject}, CompositeTemplate};
use tokio::runtime::Runtime;
use crate::logic::remember_account_dialog::{self, RememberAccountDialog};

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
    token_entry: TemplateChild<adw::PasswordEntryRow>,
    #[template_child]
    error_label: TemplateChild<gtk::Label>
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
        self.error_label.set_label(" ");
        self.log_in_button.set_sensitive(false);

        let token = self.token_entry.get().text().to_string();
        let classes = self.token_entry.get().css_classes();

        let mut classes_str: Vec<&str> = classes
            .iter()
            .map(|gstring| gstring.as_str())
            .filter(|string| string != &"error")
            .collect();
        
        if token.is_empty() {
            classes_str.push(&"error");
            self.error_label.set_visible(true);
            self.error_label.set_label("Authorization token is required duh");
            self.log_in_button.set_sensitive(true);
            return self.token_entry.get().set_css_classes(classes_str.as_slice());
        }

        self.token_entry.set_css_classes(&classes_str);
        
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

        let entry_clone = self.token_entry.get();
        let button_clone = self.log_in_button.get();
        let button_label = button_clone.label().unwrap();
        button_clone.set_label("");
        let spinner = adw::Spinner::builder().build();
        button_clone.set_child(Some(&spinner));
        let error_label = self.error_label.get();
        let window = self.instance().clone();        

        glib::spawn_future_local(async move {
            while let Ok(response) = receiver.recv().await {
                button_clone.set_label(button_label.as_str()); 
                button_clone.set_sensitive(true);

                let mut error = "";

                if !response.is_ok() { error = "Couldn't make a GET request" }
                else {
                    let status = response.as_ref().unwrap().status();
                    if status.is_client_error() { error = "Invalid authorization token" }
                    else if status.is_server_error() { error = "Encountered a discord server error" }
                }
                
                if !error.is_empty() {
                    error_label.set_label(error);
                    error_label.set_visible(true);
                    let classes = entry_clone.css_classes();
                    let mut str_classes: Vec<&str> = classes.iter().map(|gstring| gstring.as_str()).collect();
                    str_classes.push(&"error");
                    entry_clone.set_css_classes(&str_classes);
                }
                else {
                    let dialog = RememberAccountDialog::new();
                    AdwDialogExt::present(&dialog, Some(&window.root().unwrap()));
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
