use super::*;

#[derive(Debug, Default)]
pub struct CubiomesgtkApplication {}

#[glib::object_subclass]
impl ObjectSubclass for CubiomesgtkApplication {
    const NAME: &'static str = "CubiomesgtkApplication";
    type Type = super::CubiomesgtkApplication;
    type ParentType = adw::Application;
}

impl ObjectImpl for CubiomesgtkApplication {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_gactions();
        obj.set_accels_for_action("app.quit", &["<primary>q"]);
    }
}

impl ApplicationImpl for CubiomesgtkApplication {
    // We connect to the activate callback to create a window when the application
    // has been launched. Additionally, this callback notifies us when the user
    // tries to launch a "second instance" of the application. When they try
    // to do that, we'll just present any existing window.
    fn activate(&self) {
        let application = self.obj();
        // Get the current window or create one if necessary
        let window = application.active_window().unwrap_or_else(|| {
            let window = CubiomesgtkWindow::new(&*application);
            window.upcast()
        });

        // Ask the window manager/compositor to present the window
        window.present();
    }
}

impl GtkApplicationImpl for CubiomesgtkApplication {}
impl AdwApplicationImpl for CubiomesgtkApplication {}