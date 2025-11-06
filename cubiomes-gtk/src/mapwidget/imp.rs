use std::{cell::RefCell, sync::{Arc, Mutex, mpsc}};

use gtk::glib::{ThreadHandle, property::PropertySet};

use super::*;

#[derive(Debug, gtk::CompositeTemplate, glib::Properties)]
#[properties(wrapper_type = super::MapWidget)]
#[template(file = "src/ui/mapwidget.blp")]
pub struct MapWidget {
    pub texture: Arc<Mutex<Option<gdk::MemoryTexture>>>,
    pub generator: Arc<Mutex<Option<cubiomes::Generator>>>,
    pub range: Arc<Mutex<Option<cubiomes::Range>>>,
    #[property(get, set)]
    pub x: RefCell<f64>,
    #[property(get, set)]
    pub y: RefCell<f64>,
    #[property(get, set)]
    pub scale: RefCell<f64>,
    #[property(get, set)]
    pub regenerate: RefCell<bool>,
    #[property(get, set)]
    pub resize: RefCell<bool>,
    #[property(get, set)]
    pub ready: RefCell<bool>,
    pub dif_x: RefCell<f64>,
    pub dif_y: RefCell<f64>,
    pub thread_pool: glib::ThreadPool,
}

impl Default for MapWidget {
    fn default() -> Self {
        let thread_pool = glib::ThreadPool::exclusive(glib::num_processors()).unwrap();
        Self {
            texture: Default::default(),
            generator: Default::default(),
            range: Default::default(),
            x: Default::default(),
            y: Default::default(),
            scale: RefCell::new(1.0),
            regenerate: RefCell::new(false),
            resize: RefCell::new(false),
            ready: RefCell::new(false),
            dif_x: Default::default(),
            dif_y: Default::default(),
            thread_pool
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MapWidget {
    const NAME: &'static str = "MapWidget";
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

#[glib::derived_properties]
impl ObjectImpl for MapWidget {
    fn constructed(&self) {
        self.parent_constructed();

        
        

        let obj = self.obj();
        let mut g = Generator::new(MCVersion::MC_1_21_WD);
        glib::g_warning!("gen", "Generator::new done");
        g.set_seed(Dimension::Overworld, 728201557363502228);
        glib::g_warning!("gen", "Generator::set_seed done");
        let mut range = cubiomes::Range::new(
            4,
            -512/4,
            256,
            -512/4,
            1024/4,
            1,
            1024/4,
        );
        let d = gtk::GestureDrag::new();
        d.connect_drag_update(glib::clone!(
            #[weak(rename_to = imp)]
            self,
            move |_, x, y| {
                let old_x = imp.x.take();
                let old_y = imp.y.take();
                let dx = x - imp.dif_x.take();
                let dy = y - imp.dif_y.take();
                imp.obj().set_x(old_x - dx);
                imp.obj().set_y(old_y - dy);

                imp.dif_x.set(x);
                imp.dif_y.set(y);
            }
        ));
        d.connect_drag_end(glib::clone!(
            #[weak(rename_to = imp)]
            self,
            move |_, _, _| {
                imp.dif_x.set(0.0);
                imp.dif_y.set(0.0);
            }
        ));
        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::BOTH_AXES);
        scroll.connect_scroll(glib::clone!(
            #[weak_allow_none(rename_to = imp)]
            self,
            move |_, x, y| {
                if let Some(imp) = imp {
                    match y {
                        1.0 | 9.0 => imp.obj().set_scale(imp.scale.take()*2.0),
                        -1.0 | -9.0 => imp.obj().set_scale(imp.scale.take()/2.0),
                        _ => ()
                    }
                }
                glib::Propagation::Proceed
            }
        ));
        obj.add_controller(scroll);
        g.set_seed(Dimension::Overworld, 728201557363502228);
        g.alloc_cache(&mut range);
        self.generator.set(Some(g));
        self.range.set(Some(range));
        glib::idle_add_local(glib::clone!(
            #[weak_allow_none(rename_to = imp)]
            self,
            move || {
                if let Some(imp) = imp {
                    if imp.regenerate.take() {
                        imp.calc_cord();
                        if imp.resize.take() {
                            imp.realloc();
                        }
                        imp.generate_map();
                        //imp.obj().queue_draw();
                    }
                    glib::ControlFlow::Continue
                } else {
                    glib::ControlFlow::Break
                }
            }
        ));
        obj.connect_x_notify(|w| {
            w.set_regenerate(true);
        });
        obj.connect_y_notify(|w| {
            w.set_regenerate(true);
        });
        obj.connect_scale_notify(|w| {
            w.set_regenerate(true);
            w.set_resize(true);
        });
        obj.add_controller(d);
    }
}
impl WidgetImpl for MapWidget {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
        self.obj().set_regenerate(true);
        self.obj().set_resize(true);
    }
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        if let Ok(lock) = self.texture.try_lock() {
            if let Some(texture) = &*lock {
                snapshot.append_scaled_texture(
                    texture,
                    gtk::gsk::ScalingFilter::Nearest,
                    &gtk::graphene::Rect::new(
                        0.0,
                        0.0,
                        self.obj().width() as f32,
                        self.obj().height() as f32
                    )
                );
            }
        }
        self.parent_snapshot(snapshot);
    }
}
#[gtk::template_callbacks]
impl MapWidget {}

impl MapWidget {
    fn calc_cord(&self) {
        if let Ok(mut lock) = self.range.lock() {
            if let Some(range) = lock.as_mut().as_deref_mut() {
                let width = self.obj().width();
                let height = self.obj().height();
                let x = self.x.take();
                let y = self.y.take();
                let scale = self.scale.take();
                self.x.set(x);
                self.y.set(y);
                self.scale.set(scale);
                let block_x = x/4.0 * scale;
                let block_y = y/4.0 * scale;
                range.x = block_x as i32 - (width as f64 / 2.0 / 4.0 * scale) as i32;
                range.z = block_y as i32 - (height as f64 / 2.0 / 4.0 * scale) as i32;
                range.sx = (width as f64 / 4.0 * scale) as i32;
                range.sz = (height as f64 / 4.0 * scale) as i32;
                //range.sx = 256;
                //range.sz = 256;
            }
        }
    }

    fn realloc(&self) {
        if let (Ok(lock_g), Ok( mut lock_range)) = (self.generator.lock(), self.range.lock()) {
            if let (Some(g), Some(range)) = (lock_g.as_ref().as_deref(), lock_range.as_mut().as_deref_mut()) {
                g.alloc_cache(range);
            }
        }
    }

    fn generate_map(&self) {
        let generator = self.generator.clone();
        let range = self.range.clone();
        let texture = self.texture.clone();
        let obj = self.obj().clone();

        let (s, r) = mpsc::channel::<()>();
        texture.set(None);
        _ = self.thread_pool.push(move || {
            if let (Ok(mut lock_g), Ok( mut lock_range)) = (generator.lock(), range.lock()) {
                if let (Some(generator), Some(range)) = (lock_g.as_mut().as_deref_mut(), lock_range.as_mut().as_deref_mut()) {
                    
                    generator.gen_biomes(range).unwrap();
                    let mut colors = init_biome_colors();
                    let image = range.biomes_to_image(&mut colors).unwrap();
                    let width = range.sx;
                    let height = range.sz;
                    glib::idle_add(move || {
                        let bytes = glib::Bytes::from(&image);
                        let new_texture = gdk::MemoryTexture::new(
                            width,
                            height,
                            gdk::MemoryFormat::R8g8b8,
                            &bytes,
                            width as usize * 3
                        );
                        texture.set(Some(new_texture));
                        s.send(());
                        glib::ControlFlow::Break
                    });
                }
            }
        });
        glib::idle_add_local(move || {
            if let Ok(_) = r.try_recv() {
                obj.queue_draw();
                return glib::ControlFlow::Break;
            }
            glib::ControlFlow::Continue
        });
    }
}