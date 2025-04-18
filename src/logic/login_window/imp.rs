use glib::clone;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib::{self, subclass::InitializingObject}, CompositeTemplate};
use crate::logic::remember_account_dialog::RememberAccountDialog;
use crate::api::discord_connection::DiscordConnection;
use crate::api::discord_connection::runtime;

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
        let token = self.token_entry.get().text().to_string();

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
                let button_label_pre_change = log_in_button.label().unwrap();
                log_in_button.set_child(Some(&adw::Spinner::builder().build()));
                log_in_button.set_sensitive(false);

                error_label.set_label("");

                let classes = token_entry.css_classes();

                let mut classes_str: Vec<&str> = classes
                    .iter()
                    .map(|gstring| gstring.as_str())
                    .filter(|string| string != &"error")
                    .collect();

                token_entry.set_css_classes(&classes_str);

                if let Ok((dc_conn, response)) = receiver.recv().await {
                    log_in_button.set_label(button_label_pre_change.as_str()); 
                    log_in_button.set_sensitive(true);

                    match response {
                        Ok(data) => {
                            let dialog = RememberAccountDialog::new();
                            dialog.pass_data(data, dc_conn);
                            AdwDialogExt::present(&dialog, Some(&window.root().unwrap()));
                        },
                        Err(message) => {
                            error_label.set_label(message.as_str());
                            error_label.set_visible(true);
                            classes_str.push(&"error");
                            token_entry.set_css_classes(&classes_str);                       
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
