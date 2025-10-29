
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, gdk};
use cubiomes::*;

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
        pub texture: RefCell<Option<gdk::Texture>>,
        pub gen: RefCell<Option<cubiomes::Generator>>,
        pub range: RefCell<Option<cubiomes::Range>>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CubiomesgtkWindow {
        const NAME: &'static str = "CubiomesgtkWindow";
        type Type = super::CubiomesgtkWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CubiomesgtkWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let mut g = Generator::new(MCVersion::MC_1_21_WD);
            glib::g_warning!("gen", "Generator::new done");
            g.set_seed(Dimension::Overworld, 728201557363502228);
            glib::g_warning!("gen", "Generator::set_seed done");
            let range = cubiomes::Range::new(
                16,
                -512/16,
                64,
                -512/16,
                1024/16,
                1,
                1024/16,
            );
            self.gen.set(Some(g));
            self.range.set(Some(range));
            self.gen();
        }
    }
    impl WidgetImpl for CubiomesgtkWindow {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            self.parent_snapshot(snapshot);
            if let Some(texture) = self.texture.borrow().as_ref() {
                snapshot.append_scaled_texture(
                    texture,
                    gtk::gsk::ScalingFilter::Nearest,
                    &gtk::graphene::Rect::new(200.0, 200.0, 500.0, 500.0)
                );
            }
        }
    }
    impl WindowImpl for CubiomesgtkWindow {}
    impl ApplicationWindowImpl for CubiomesgtkWindow {}
    impl AdwApplicationWindowImpl for CubiomesgtkWindow {}
    #[gtk::template_callbacks]
    impl CubiomesgtkWindow {
        #[template_callback]
        fn add_one(&self, _: &gtk::Button) {
            self.range.borrow_mut().as_mut().map(|r|r.x += 50);
            self.gen();
        }

        #[template_callback]
        fn remove_one(&self, _: &gtk::Button) {
            self.range.borrow_mut().as_mut().map(|r|r.x -= 50);
            self.gen();
        }

        fn gen(&self) {
            if let (Some(g), Some(range)) = (self.gen.borrow_mut().as_mut(), self.range.borrow_mut().as_mut()){
                //let mut g = Generator::new(MCVersion::MC_1_21_WD);
                glib::g_warning!("gen", "Generator::new done");
                g.set_seed(Dimension::Overworld, 728201557363502228);
                glib::g_warning!("gen", "Generator::set_seed done");
                glib::g_warning!("gen", "Range::new done");
                g.alloc_cache(range);
                glib::g_warning!("gen", "alloc_cache done");
                glib::g_warning!("gen", "gen_biomes...");
                g.gen_biomes(range).unwrap();
                glib::g_warning!("gen", "gen_biomes done");
                glib::g_warning!("gen", "############");
                let mut colors = init_biome_colors();
                let image = range.biomes_to_image(&mut colors).unwrap();
                glib::g_warning!("gen", "biomes_to_image done");
                let bytes = glib::Bytes::from_owned(image);
                let pb = gdk::gdk_pixbuf::Pixbuf::from_bytes(
                    &bytes,
                    gtk::gdk_pixbuf::Colorspace::Rgb,
                    false,
                    8,
                    range.sz,
                    range.sx,
                    range.sz * 3
                );
                let texture = gdk::Texture::for_pixbuf(&pb);
                self.texture.set(Some(texture));
            }
            
        }
    }
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
