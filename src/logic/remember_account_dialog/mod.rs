mod imp;

use adw::{gtk::prelude::*, subclass::prelude::ObjectSubclassExt};
use gtk::glib::{self, Object};
use serde_json::Value;
use adw::gtk;
use crate::api::discord_connection::DiscordConnection;

glib::wrapper! {
    pub struct RememberAccountDialog(ObjectSubclass<imp::RememberAccountDialog>)
        @extends gtk::Widget, gtk::Window, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl RememberAccountDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }
    
    pub fn pass_data(&self, data: Value, dc_conn: DiscordConnection) {
        let imp = imp::RememberAccountDialog::from_obj(self);
        
        let username = data["username"].as_str().unwrap();
        let global_name = data["global_name"].as_str().unwrap();
        
        imp.avatar.set_text(Some(global_name));
        imp.global_name_label.set_label(global_name);
        imp.username_label.set_label(username);

        let avatar = String::from(data["avatar"].as_str().unwrap());
        let user_id = String::from(data["id"].as_str().unwrap());

        imp.load_avatar(avatar, user_id);
    }

    pub fn show_dialog(&self) {
        self.present();
    }
}
