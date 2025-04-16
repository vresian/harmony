use std::sync::OnceLock;
use glib::clone;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib::{self, subclass::InitializingObject}, CompositeTemplate};
use tokio::runtime::Runtime;
use crate::logic::remember_account_dialog::RememberAccountDialog;
use crate::api::discord_connection::DiscordConnection;
use std::sync::{Arc, Mutex};

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

        let button_label_pre_change = self.log_in_button.label().unwrap();

        self.log_in_button.set_child(Some(&adw::Spinner::builder().build()));

        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(
            #[strong]
            sender,
            async move { 
                let dc_conn = DiscordConnection::new(token);
                let response = dc_conn.init().await;

                sender
                    .send((dc_conn, response))
                    .await
                    .expect("The channel needs to be open.");
            }
        ));

        glib::spawn_future_local(clone!(
            #[weak(rename_to = error_label)] self.error_label.get(),
            #[weak(rename_to = log_in_button)] self.log_in_button.get(),
            #[weak(rename_to = token_entry)] self.token_entry.get(),
            #[weak(rename_to = window)] self.obj(),
            async move {
                if let Ok((dc_conn, response)) = receiver.recv().await {
                    log_in_button.set_label(button_label_pre_change.as_str()); 
                    log_in_button.set_sensitive(true);
                    
                    match response {
                        Ok(data) => {
                            println!("{}", data["username"]);
                            let dialog = RememberAccountDialog::new();
                            AdwDialogExt::present(&dialog, Some(&window.root().unwrap()));
                        },
                        Err(message) => {
                            error_label.set_label(message.as_str());
                            error_label.set_visible(true);
                            let classes = token_entry.css_classes();
                            let mut str_classes: Vec<&str> = classes.iter().map(|gstring| gstring.as_str()).collect();
                            str_classes.push(&"error");
                            token_entry.set_css_classes(&str_classes);                       
                        }
                    }
                }
            }
        ));
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
