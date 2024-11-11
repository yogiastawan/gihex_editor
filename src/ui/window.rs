use adw::subclass::prelude::*;
use gtk::gio::{ActionEntry, MenuModel};
use gtk::prelude::*;
use gtk::{gio, glib};

use super::page::tab_page_editor::TabPageEditor;

mod imp {

    use gtk::MenuButton;

    use crate::ui::menu::export_menu::ExportMenu;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/gihex/editor/ui/xml/window.ui")]
    pub struct GihexWindow {
        #[template_child]
        pub primary_menu: TemplateChild<MenuButton>,
        #[template_child]
        pub new_menu: TemplateChild<MenuButton>,
        #[template_child]
        pub tab_view: TemplateChild<adw::TabView>,
        #[template_child]
        pub page_empty: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub editor_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub break_point: TemplateChild<adw::Breakpoint>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GihexWindow {
        const NAME: &'static str = "GihexWindow";
        type Type = super::GihexWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            // register popover
            ExportMenu::ensure_type();
            TabPageEditor::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GihexWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_action_entry();
            self.obj().setup_menu();

            self.editor_stack.set_visible_child(&self.page_empty.get());

            self.tab_view.pages().connect_items_changed(glib::clone!(
                #[weak (rename_to=window)]
                self,
                move |x: &gtk::SelectionModel, _pos: u32, _removed: u32, _added: u32| {
                    println!("items changed");
                    if x.n_items() > 0 {
                        window
                            .editor_stack
                            .set_visible_child(&window.tab_view.get());
                    } else {
                        window
                            .editor_stack
                            .set_visible_child(&window.page_empty.get());
                    }
                }
            ));
        }
    }
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

    fn setup_menu(&self) {
        let builder = gtk::Builder::from_resource("/com/gihex/editor/ui/xml/main_menu.ui");
        let menu = builder
            .object::<MenuModel>("primary_menu_model")
            .expect("Cannot get primary_menu_model");
        self.imp().primary_menu.set_menu_model(Some(&menu));
        let popover = self
            .imp()
            .primary_menu
            .popover()
            .expect("Cannot get orimay menu's popover");
        popover.set_halign(gtk::Align::End);

        let builder = gtk::Builder::from_resource("/com/gihex/editor/ui/xml/new_menu.ui");
        let menu = builder
            .object::<MenuModel>("new_menu_model")
            .expect("Cannot get new_menu_model");
        self.imp().new_menu.set_menu_model(Some(&menu));
        let popover = self
            .imp()
            .new_menu
            .popover()
            .expect("Cannot get new menu's popover");
        popover.set_halign(gtk::Align::Start);
    }

    fn setup_action_entry(&self) {
        let new_page = ActionEntry::builder("new_page")
            .activate(move |win: &Self, _, _| win.new_page())
            .build();
        let new_comp = ActionEntry::builder("new_comp")
            .activate(move |win: &Self, _, _| win.new_component())
            .build();

        self.add_action_entries([new_page, new_comp])
    }

    fn new_page(&self) {
        let page = TabPageEditor::default();
        self.imp().break_point.add_setters(&[(
            &page.imp().page_split_view.get(),
            "collapsed",
            true,
        )]);

        self.imp()
            .editor_stack
            .set_visible_child(&self.imp().tab_view.get());
        let tp = self.imp().tab_view.append(&page);
        tp.set_title(&page.title());

        println!(
            "n pages: {} {} {}",
            self.imp().tab_view.n_pages(),
            page.title(),
            page.is_dirty()
        );
        self.imp().tab_view.set_selected_page(&tp);
    }
    fn new_component(&self) {}
}
