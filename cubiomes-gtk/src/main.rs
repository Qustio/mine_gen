mod application;
mod config;
mod window;

use self::application::CubiomesgtkApplication;
use self::window::CubiomesgtkWindow;


use gtk::{gio, glib};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    #[cfg(feature = "meson")]
    {
        use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
        use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};

        glib::g_warning!("cubiomes-gtk", "GETTEXT_PACKAGE: {GETTEXT_PACKAGE}");
        glib::g_warning!("cubiomes-gtk", "LOCALEDIR: {LOCALEDIR}");
        glib::g_warning!("cubiomes-gtk", "PKGDATADIR: {PKGDATADIR}");

        // Set up gettext translations
        bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
        bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
            .expect("Unable to set the text domain encoding");
        textdomain(GETTEXT_PACKAGE)
            .expect("Unable to switch to the text domain");
        
        // Load and Register resources
        let resources = gio::Resource::load(format!("{PKGDATADIR}/cubiomesgtk.gresource"))
            .expect("Could not load resources");
        gio::resources_register(&resources);
    };
    #[cfg(not(feature = "meson"))]
    {
        // Register and include resources
        gio::resources_register_include!("cubiomesgtk.gresource")
            .expect("Failed to register resources.");
    };
    
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
