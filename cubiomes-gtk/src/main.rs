
mod application;
mod config;
mod window;

use self::application::CubiomesgtkApplication;
use self::window::CubiomesgtkWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR, PORTABLE};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib};
use gtk::prelude::*;
use std::path::PathBuf;

fn main() -> glib::ExitCode {
    let bin = std::env::current_exe().unwrap();
    let gettext_package = if PORTABLE {
        bin.parent().unwrap().parent().unwrap().join(GETTEXT_PACKAGE)
    } else {
        PathBuf::from(GETTEXT_PACKAGE)
    };
    let localedir = if PORTABLE {
        bin.parent().unwrap().parent().unwrap().join(LOCALEDIR)
    } else {
        PathBuf::from(LOCALEDIR)
    };
    let pkgdatadir = if PORTABLE {
        bin.parent().unwrap().parent().unwrap().join(PKGDATADIR)
    } else {
        PathBuf::from(PKGDATADIR)
    };
    glib::g_warning!("cubiomes-gtk", "GETTEXT_PACKAGE: {gettext_package:?}");
    glib::g_warning!("cubiomes-gtk", "LOCALEDIR: {localedir:?}");
    glib::g_warning!("cubiomes-gtk", "PKGDATADIR: {pkgdatadir:?}");
    // Set up gettext translations
    bindtextdomain(gettext_package.to_str().unwrap(), localedir).expect("Unable to bind the text domain");
    bind_textdomain_codeset(gettext_package.to_str().unwrap(), "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(gettext_package.to_str().unwrap()).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(pkgdatadir.join("cubiomesgtk.gresource"))
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = CubiomesgtkApplication::new("com.qustio.cubiomesgtk", &gio::ApplicationFlags::empty());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    app.run()
}
