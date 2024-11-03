mod ui;

use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{gio, glib};
use ui::application::GihexEditorApp;

const APP_ID: &str = "com.gihex.editor";
const APP_NAME: &str = "Gihex Editor";
const DEVELOPER_NAME: &str = "I Gede Yogi Astawan";
const VERSION: &str = env!("CARGO_PKG_VERSION");

const GETTEXT_PACKAGE: &str = "gihex_editor";
const LOCALEDIR: &str = "/app/share/locale";
const PKGDATADIR: &str = "/app/share/ged";

fn main() -> gtk::glib::ExitCode {
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    // let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/gihex_editor.gresource")
    //     .expect("Could not load resources");
    //
    let resources = gio::resources_register_include!("gihex_editor.gresource")
        .expect("Failed to register resources.");
    // gio::resources_register(&resources);

    let app = GihexEditorApp::new(APP_ID, &ApplicationFlags::empty());

    app.run()
}
