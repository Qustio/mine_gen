
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/qustio/cubiomesgtk/window.ui")]
    pub struct CubiomesgtkWindow {
        // Template widgets
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CubiomesgtkWindow {
        const NAME: &'static str = "CubiomesgtkWindow";
        type Type = super::CubiomesgtkWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CubiomesgtkWindow {}
    impl WidgetImpl for CubiomesgtkWindow {}
    impl WindowImpl for CubiomesgtkWindow {}
    impl ApplicationWindowImpl for CubiomesgtkWindow {}
    impl AdwApplicationWindowImpl for CubiomesgtkWindow {}
}

glib::wrapper! {
    pub struct CubiomesgtkWindow(ObjectSubclass<imp::CubiomesgtkWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl CubiomesgtkWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
