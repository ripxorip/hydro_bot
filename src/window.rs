use gtk::prelude::*;
use libhandy::prelude::*;

pub struct Window {
    main_window: libhandy::Window,
}

impl Window
{
    pub fn new () -> Window {
        let ui_src = include_str!("window.ui");
        let builder = gtk::Builder::from_string(ui_src);

        let main_window: libhandy::Window = builder.object("mainWindow").unwrap();

        Window {
            main_window,
        }
    }

    pub fn start(&self) {
        self.main_window.show_all();
    }
}
