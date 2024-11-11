use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, glib::Properties, Default, gtk::CompositeTemplate)]
    #[properties(wrapper_type=super::TabPageEditor)]
    #[template(resource = "/com/gihex/editor/ui/xml/tab_page_editor.ui")]
    pub struct TabPageEditor {
        #[property(get,set,construct,type=String,default="untitle")]
        pub title: RefCell<String>,
        #[property(get,set,construct,type=bool,default=true)]
        pub is_dirty: Cell<bool>,
        #[template_child]
        pub page_split_view: TemplateChild<adw::OverlaySplitView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TabPageEditor {
        const NAME: &'static str = "TabPageEditor";
        type Type = super::TabPageEditor;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for TabPageEditor {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            println!("dispose page editor");
            self.page_split_view.unparent();
            unsafe {
                self.page_split_view.run_dispose();
            }
        }
    }

    impl WidgetImpl for TabPageEditor {}
}

glib::wrapper! {
    pub struct TabPageEditor(ObjectSubclass<imp::TabPageEditor>)
    @extends  gtk::Widget,
    @implements gio::ActionMap ,gio::ActionGroup, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TabPageEditor {
    pub fn new(title: &str, is_dirty: bool) -> Self {
        glib::Object::builder()
            .property("title", title)
            .property("is_dirty", is_dirty)
            .build()
    }
}

impl Default for TabPageEditor {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
