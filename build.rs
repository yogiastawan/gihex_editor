fn main() {
    println!("testing");
    glib_build_tools::compile_resources(
        &["src"],
        "src/gihex_editor.gresource.xml",
        "gihex_editor.gresource",
    );
}
