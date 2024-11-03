use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/gihex/editor/ui/xml/new_popover.ui")]
    pub struct GihexNewPopover {}

    #[glib::object_subclass]
    impl ObjectSubclass for GihexNewPopover {
        const NAME: &'static str = "GihexNewPopover";
        type Type = super::GihexNewPopover;
        type ParentType = gtk::Popover;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GihexNewPopover {}
    impl WidgetImpl for GihexNewPopover {}

    impl PopoverImpl for GihexNewPopover {}
}

glib::wrapper! {
    pub struct GihexNewPopover(ObjectSubclass<imp::GihexNewPopover>)
        @extends gtk::Widget, gtk::Popover,
        @implements gtk::Accessible, gtk::Buildable, gtk::Native, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl GihexNewPopover {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
