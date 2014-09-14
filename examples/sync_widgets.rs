//! # Synchronizing Widgets
//!
//! You can use signals in order to synchronize the values of widgets. In this example a spin button and a horizontal scale will get interlocked.

#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("Enter your age");
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(300, 20);

    let mut spin_button = gtk::SpinButton::new_with_range(0.0, 130.0, 1.0).unwrap();
    let slider = gtk::Scale::new_with_range(gtk::orientation::Horizontal, 0.0, 130.0, 1.0).unwrap();

    spin_button.connect(signals::ValueChanged::new(|| {
        let mut adjustment = slider.get_adjustment();
        adjustment.set_value(spin_button.get_value());
    }));

    slider.connect(signals::ValueChanged::new(|| {
        let adjustment = slider.get_adjustment();
        spin_button.set_value(adjustment.get_value());
    }));

    let mut hbox = gtk::Box::new(gtk::orientation::Horizontal, 5).unwrap();
    hbox.set_homogeneouse(true);
    hbox.add(&spin_button);
    hbox.add(&slider);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    window.add(&hbox);
    window.show_all();
    gtk::main();
}