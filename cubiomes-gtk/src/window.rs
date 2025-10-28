
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, gdk};

mod imp {
    use std::cell::RefCell;

    use gtk::glib::property::PropertySet;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/qustio/cubiomesgtk/window.ui")]
    pub struct CubiomesgtkWindow {
        // Template widgets
        #[template_child]
        pub boxx: TemplateChild<gtk::Box>,
        pub gl: RefCell<Option<gtk::GLArea>>
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

    impl ObjectImpl for CubiomesgtkWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let a = gtk::GLArea::builder()
                .width_request(100)
                .height_request(200)
                .build();
            a.connect_realize(|r| {
                r.make_current();
                // TODO make gl init or use something different
            });
            a.connect_render(|r, g| {
                // unsafe {
                //     gl::ClearColor(0.0, 0.0, 0.0, 0.0);
                //     gl::Clear(gl::CLEAR_BUFFER);
                // }
                glib::Propagation::Proceed
            });
            self.boxx.append(&a);
            
            //gdk::Texture::from_bytes(&)
            //self.gl.set(Some(a));
        }
    }
    impl WidgetImpl for CubiomesgtkWindow {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            self.parent_snapshot(snapshot);
            let bytes = glib::Bytes::from_owned([1, 2, 3]);
            let pb: gtk::gdk_pixbuf::Pixbuf = gdk::gdk_pixbuf::Pixbuf::from_bytes(
                &bytes,
                gtk::gdk_pixbuf::Colorspace::Rgb,
                false,
                8,
                1,
                1,
                0
            );
            let texture = gdk::Texture::for_pixbuf(&pb);
            snapshot.append_texture(&texture, &gtk::graphene::Rect::new(200.0, 200.0, 100.0, 100.0));
        }
    }
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
