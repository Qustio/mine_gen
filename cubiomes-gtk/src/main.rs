mod application;
mod config;
mod mapwidget;
mod window;

use self::application::CubiomesgtkApplication;
use self::window::CubiomesgtkWindow;

use gtk::{gio, glib};
use gtk::prelude::*;

use config::{GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};

fn main() -> glib::ExitCode {
	// Set up gettext translations
	bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
	bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
		.expect("Unable to set the text domain encoding");
	textdomain(GETTEXT_PACKAGE)
		.expect("Unable to switch to the text domain");

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
