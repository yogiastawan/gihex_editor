use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;
    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/gihex/editor/ui/xml/tab_page_editor.ui")]
    pub struct TabPageEditor {
        pub title: RefCell<String>,
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
            // register popover
            klass.bind_template();
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TabPageEditor {
        fn constructed(&self) {
            self.parent_constructed();
            self.page_split_view.set_hexpand(true);
            self.page_split_view.set_vexpand(true);
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
    @extends gtk::Widget,
    @implements gio::ActionMap ,gio::ActionGroup, gtk::Accessible, gtk::Buildable;
}

impl TabPageEditor {
    pub fn new(title: &str, is_dirty: bool) -> Self {
        let o = glib::Object::builder::<Self>().build();
        o.set_title(title);
        o.set_is_dirty(is_dirty);
        o
    }

    pub fn set_title(&self, title: &str) {
        *self.imp().title.borrow_mut() = title.to_string();
    }

    pub fn get_title(&self) -> String {
        let a = self.imp().title.borrow().clone();
        a.clone()
    }

    pub fn set_is_dirty(&self, is_dirty: bool) {
        self.imp().is_dirty.set(is_dirty);
    }

    pub fn get_is_dirty(&self) -> bool {
        self.imp().is_dirty.get()
    }
}

impl Default for TabPageEditor {
    fn default() -> Self {
        TabPageEditor::new("untitle", true)
    }
}
