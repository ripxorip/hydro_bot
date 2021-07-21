use gtk::prelude::*;
use libhandy::prelude::*;

mod window;
use window::Window;

fn main()
{
    gtk::init().expect("Unable to start GTK3. Error");
    libhandy::init();

    let win = Window::new();

    win.start();
    gtk::main();
}
