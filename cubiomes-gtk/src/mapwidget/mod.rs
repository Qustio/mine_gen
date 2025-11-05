use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, gdk};
use cubiomes::*;

mod imp;

glib::wrapper! {
    pub struct MapWidget(ObjectSubclass<imp::MapWidget>)
        @extends gtk::Widget,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl MapWidget {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("scale", 1)
            .build()
    }
}
