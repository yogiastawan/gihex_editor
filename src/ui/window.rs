use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {

    use gtk::Label;

    use crate::ui::new_popover::GihexNewPopover;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/gihex/editor/ui/xml/window.ui")]
    pub struct GihexWindow {
        // #[template_child]
        // pub label: TemplateChild<Label>,
        // #[template_child]
        // pub new_menu_popover: TemplateChild<GihexNewPopover>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GihexWindow {
        const NAME: &'static str = "GihexWindow";
        type Type = super::GihexWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            // register popover
            GihexNewPopover::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GihexWindow {}
    impl WidgetImpl for GihexWindow {}

    impl WindowImpl for GihexWindow {}
    impl ApplicationWindowImpl for GihexWindow {}

    impl AdwApplicationWindowImpl for GihexWindow {}
}

glib::wrapper! {
    pub struct GihexWindow(ObjectSubclass<imp::GihexWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                        gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GihexWindow {
    pub fn new<APP: IsA<gtk::Application>>(app: &APP) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
