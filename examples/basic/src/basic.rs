//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals::{DeleteEvent};

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    Connect::connect(&window, DeleteEvent::new(&mut |&mut: _| {
        gtk::main_quit();
        true
    }));

    let button = gtk::Button::new_with_label("Click me!").unwrap();

    window.add(&button);

    window.show_all();
    gtk::main();
}