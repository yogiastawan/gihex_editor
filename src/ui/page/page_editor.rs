use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use std::cell::{Cell, RefCell};

    use glib::Properties;

    use super::*;
    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type=super::PageEditor)]
    #[template(resource = "/com/gihex/editor/ui/xml/page_editor.ui")]
    pub struct PageEditor {
        #[property(get,set,type=String,default="untitled")]
        title: RefCell<String>,
        #[property(get,set,type=bool,default=true)]
        is_dirty: Cell<bool>,
        #[template_child]
        pub page_split_view: TemplateChild<adw::OverlaySplitView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PageEditor {
        const NAME: &'static str = "PageEditor";
        type Type = super::PageEditor;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            // register popover
            klass.bind_template();
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PageEditor {
        fn constructed(&self) {
            self.parent_constructed();
            self.page_split_view.set_hexpand(true);
            self.page_split_view.set_vexpand(true);
        }

        fn dispose(&self) {
            self.page_split_view.unparent();
            unsafe {
                self.page_split_view.run_dispose();
            }
        }
    }

    impl WidgetImpl for PageEditor {}
}

glib::wrapper! {
    pub struct PageEditor(ObjectSubclass<imp::PageEditor>)
    @extends gtk::Widget,
    @implements gio::ActionMap ,gio::ActionGroup, gtk::Accessible, gtk::Buildable;
}

impl PageEditor {
    pub fn new(title: &str, is_dirty: bool) -> Self {
        glib::Object::builder()
            .property("title", title)
            .property("is_dirty", is_dirty)
            .build()
    }
}

impl Default for PageEditor {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
