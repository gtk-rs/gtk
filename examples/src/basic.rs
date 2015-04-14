//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate gtk;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
    gtk::init();

    let window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let button = gtk::Button::new_with_label("Click me!").unwrap();

    window.add(&button);

    window.show_all();
    gtk::main();
}