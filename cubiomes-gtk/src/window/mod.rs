use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, gdk};
use cubiomes::*;

mod imp;

glib::wrapper! {
    pub struct CubiomesgtkWindow(ObjectSubclass<imp::CubiomesgtkWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
		@implements gio::ActionGroup, gio::ActionMap, gtk::ConstraintTarget, gtk::Buildable, 
		gtk::Accessible, gtk::ShortcutManager, gtk::Root, gtk::Native;
}

impl CubiomesgtkWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
