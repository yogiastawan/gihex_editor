use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/gihex/editor/ui/xml/export_menu.ui")]
    pub struct ExportMenu {}

    #[glib::object_subclass]
    impl ObjectSubclass for ExportMenu {
        const NAME: &'static str = "ExportMenu";
        type Type = super::ExportMenu;
        type ParentType = gtk::Popover;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExportMenu {}
    impl WidgetImpl for ExportMenu {}
    impl PopoverImpl for ExportMenu {}
}

glib::wrapper! {
    pub struct ExportMenu(ObjectSubclass<imp::ExportMenu>)
        @extends gtk::Widget, gtk::Popover,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::ShortcutManager;
}

impl ExportMenu {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for ExportMenu {
    fn default() -> Self {
        ExportMenu::new()
    }
}
