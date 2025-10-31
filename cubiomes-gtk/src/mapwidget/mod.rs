use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, gdk};
use cubiomes::*;

mod imp;

glib::wrapper! {
    pub struct MapWidget(ObjectSubclass<imp::MapWidget>)
        @extends gtk::Widget;
}

impl MapWidget {
    pub fn new() -> Self {
        glib::Object::builder()
            .build()
    }
}
