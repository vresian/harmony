use adw::subclass::prelude::*;
use adw::gtk::CompositeTemplate;
use adw::gtk::glib;
use glib::clone;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::WidgetExt;
use crate::api::discord_connection::{runtime, DiscordConnection};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/vresian/harmony/remember_account_dialog.ui")]
pub struct RememberAccountDialog {
    #[template_child]
    pub avatar: TemplateChild<adw::Avatar>,
    #[template_child]
    avatar_spinner: TemplateChild<adw::Spinner> ,
    #[template_child]
    pub global_name_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub username_label: TemplateChild<gtk::Label>
}

#[glib::object_subclass]
impl ObjectSubclass for RememberAccountDialog {
    const NAME: &'static str = "HarmonyRememberAccountDialog";
    type Type = super::RememberAccountDialog;
    type ParentType = adw::Dialog;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }

    fn new() -> Self {
        Self::default()
    }
}

impl ObjectImpl for RememberAccountDialog {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl RememberAccountDialog {
    pub fn load_avatar(&self, avatar: String, user_id: String) {

        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(
            #[strong]
            sender,  
            async move { 
                let response = DiscordConnection::get_profile_picture(avatar, user_id, 256).await.unwrap();

                sender
                    .send(response)
                    .await
                    .expect("The channel needs to be open.");
            }
        ));

        glib::spawn_future_local(clone!(
            #[weak(rename_to = avatar)] self.avatar.get(),
            #[weak(rename_to = avatar_spinner)] self.avatar_spinner.get(),
            async move {
                if let Ok(texture) = receiver.recv().await {
                    avatar.set_custom_image(Some(&texture));
                    avatar_spinner.set_visible(false);
                }
            }
        ));
    }
}

impl WidgetImpl for RememberAccountDialog {}
impl WindowImpl for RememberAccountDialog {}
impl AdwDialogImpl for RememberAccountDialog {}
