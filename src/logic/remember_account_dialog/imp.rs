use adw::subclass::prelude::*;
use adw::gtk::CompositeTemplate;
use adw::gtk::glib;
use gtk::glib::subclass::InitializingObject;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/vresian/harmony/remember_account_dialog.ui")]
pub struct RememberAccountDialog {}

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
}

impl ObjectImpl for RememberAccountDialog {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for RememberAccountDialog {}
impl WindowImpl for RememberAccountDialog {}
impl DialogImpl for RememberAccountDialog {}
impl AdwDialogImpl for RememberAccountDialog {}
