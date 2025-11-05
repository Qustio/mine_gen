use std::{cell::{Cell, RefCell}, rc::Rc};

use gtk::glib::property::{PropertySet};

use crate::mapwidget::MapWidget;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/ui/window.blp")]
pub struct CubiomesgtkWindow {
    #[template_child]
    pub map: TemplateChild<MapWidget>
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
    }
}
impl WidgetImpl for CubiomesgtkWindow {}
impl WindowImpl for CubiomesgtkWindow {}
impl ApplicationWindowImpl for CubiomesgtkWindow {}
impl AdwApplicationWindowImpl for CubiomesgtkWindow {}
#[gtk::template_callbacks]
impl CubiomesgtkWindow {
    // #[template_callback]
    // fn add_one(&self, _: &gtk::Button) {
	// 	if let Some(range) = self.range.borrow_mut().as_mut() {
	// 		range.x += 50
	// 	}
    //     self.gen();
    //     self.obj().queue_draw();
    // }

    // #[template_callback]
    // fn remove_one(&self, _: &gtk::Button) {
    //     if let Some(range) = self.range.borrow_mut().as_mut() {
	// 		range.x -= 50
	// 	}
    //     self.gen();
    //     self.obj().queue_draw();
    // }

    // fn gen(&self) {
    //     if let (Some(g), Some(range)) = (self.gen.borrow_mut().as_mut(), self.range.borrow_mut().as_mut()){
    //         g.gen_biomes(range).unwrap();
    //         let mut colors = init_biome_colors();
    //         let image = range.biomes_to_image(&mut colors).unwrap();
    //         let bytes = glib::Bytes::from_owned(image);
	// 		let texture = gdk::MemoryTexture::new(
	// 			range.sz,
	// 			range.sx,
	// 			gdk::MemoryFormat::R8g8b8,
	// 			&bytes,
	// 			(range.sz * 3) as usize
	// 		);
    //         self.texture.set(Some(texture));
    //     }
        
    // }
}
