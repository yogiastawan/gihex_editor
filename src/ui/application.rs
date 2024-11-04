use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    gio::{self, ActionEntry, ApplicationFlags},
    glib,
};

use crate::{APP_ID, APP_NAME, DEVELOPER_NAME, VERSION};

mod imp {
    use crate::ui::window::GihexWindow;

    use super::*;

    #[derive(Debug, Default)]
    pub struct GihexEditorApp {}

    #[glib::object_subclass]
    impl ObjectSubclass for GihexEditorApp {
        const NAME: &'static str = "GihexEditorApp";
        type Type = super::GihexEditorApp;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GihexEditorApp {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_action_entry();
            obj.setup_action_shortcut();
        }
    }

    impl ApplicationImpl for GihexEditorApp {
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = GihexWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for GihexEditorApp {}
    impl AdwApplicationImpl for GihexEditorApp {}
}

glib::wrapper! {
    pub struct GihexEditorApp(ObjectSubclass<imp::GihexEditorApp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}
impl GihexEditorApp {
    pub fn new(id: &str, flags: &ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", id)
            .property("flags", flags)
            .build()
    }

    fn setup_action_entry(&self) {
        let quit_action = ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let about_action = ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        let new_page = ActionEntry::builder("new_page")
            .activate(move |app: &Self, _, _| app.new_page())
            .build();
        let new_comp = ActionEntry::builder("new_comp")
            .activate(move |app: &Self, _, _| app.new_component())
            .build();

        self.add_action_entries([quit_action, about_action, new_page, new_comp]);
    }

    fn setup_action_shortcut(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("app.about", &["<primary>h"]);
        self.set_accels_for_action("app.new_page", &["<primary>n"]);
        self.set_accels_for_action("app.new_comp", &["<primary><shift>n"]);
    }

    fn show_about(&self) {
        let window = self.active_window();
        if window.is_none() {
            #[cfg(debug_assertions)]
            eprintln!("Cannot activate window");

            return;
        }

        let about = adw::AboutDialog::builder()
            .application_name(APP_NAME)
            .application_icon(APP_ID)
            .developer_name(DEVELOPER_NAME)
            .version(VERSION)
            .developers([DEVELOPER_NAME])
            .translator_credits("I Gede Yogi Astawan (yogi.astawan@gmail.com)")
            .copyright("© 2024 I Gede Yogi Astawan")
            .build();

        // about.present(Some(&window));
        about.present(window.as_ref());
    }

    fn new_page(&self) {}

    fn new_component(&self) {}
}
