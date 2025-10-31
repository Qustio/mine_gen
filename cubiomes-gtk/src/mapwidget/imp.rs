use std::{cell::{Cell, RefCell}, rc::Rc};

use gtk::glib::property::{PropertySet};

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/qustio/cubiomesgtk/mapwidget.ui")]
pub struct MapWidget {
    pub texture: RefCell<Option<gdk::Texture>>,
    pub generator: RefCell<Option<cubiomes::Generator>>,
    pub range: RefCell<Option<cubiomes::Range>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MapWidget {
    const NAME: &'static str = "CubiomesgtkWindow";
    type Type = super::MapWidget;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MapWidget {
    fn constructed(&self) {
        self.parent_constructed();
        let mut g = Generator::new(MCVersion::MC_1_21_WD);
        glib::g_warning!("gen", "Generator::new done");
        g.set_seed(Dimension::Overworld, 728201557363502228);
        glib::g_warning!("gen", "Generator::set_seed done");
        let mut range = cubiomes::Range::new(
            16,
            -512/16,
            256,
            -512/16,
            1024/16,
            1,
            1024/16,
        );
        let d = gtk::GestureDrag::new();
        let save_x = Rc::new(Cell::new(0.0));
        let save_y = Rc::new(Cell::new(0.0));
        d.connect_drag_update(glib::clone!(
            #[weak(rename_to = imp)]
            self,
            #[weak] save_x,
            #[weak] save_y,                
            move |_, x, y| {
                let mul = 1024.0/16.0/500.0;
                let dx = ((*save_x).get()-x)*mul;
                let dy = ((*save_y).get()-y)*mul;
                imp.range.borrow_mut().as_mut().map(|r| {
                    r.x += dx as i32;
                    r.z += dy as i32; 
                });
                save_x.set(x + (dx%1.0)/mul);
                save_y.set(y + (dy%1.0)/mul);
                imp.gen();
                imp.obj().queue_draw();
            }
        ));
        d.connect_drag_end(glib::clone!(
            move |_, x, y| {
                glib::g_warning!("gen", "enddrag {} {}", x, y);
                save_x.set(0.0);
                save_y.set(0.0);
            }
        ));
        
        g.set_seed(Dimension::Overworld, 728201557363502228);
        g.alloc_cache(&mut range);
        self.generator.set(Some(g));
        self.range.set(Some(range));
        self.generate_map();
        self.obj().add_controller(d);
    }
}
impl WidgetImpl for MapWidget {
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
#[gtk::template_callbacks]
impl MapWidget {
    #[template_callback]
    fn add_one(&self, _: &gtk::Button) {
        self.range.borrow_mut().as_mut().map(|r|r.x += 50);
        self.generate_map();
        self.obj().queue_draw();
    }

    #[template_callback]
    fn remove_one(&self, _: &gtk::Button) {
        self.range.borrow_mut().as_mut().map(|r|r.x -= 50);
        self.generate_map();
        self.obj().queue_draw();
    }

    fn generate_map(&self) {
        if let (Some(g), Some(range)) = (self.generator.borrow_mut().as_mut(), self.range.borrow_mut().as_mut()){
            g.gen_biomes(range).unwrap();
            let mut colors = init_biome_colors();
            let image = range.biomes_to_image(&mut colors).unwrap();
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