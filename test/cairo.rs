
#![feature(globs)]

extern crate rgtk;
extern crate log;
extern crate debug;
extern crate collections;

use std::f64::consts::PI_2;
use rgtk::*;
use rgtk::gtk::signals;
use std::str;

use rgtk::cairo::enums::{
    FontSlantNormal,
    FontWeightNormal
};

#[doc(hidden)]
#[cfg(target_os="macos")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3.0")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3.0")]
    extern{}
}

#[doc(hidden)]
#[cfg(target_os="linux")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3")]
    extern{}
}

fn main() {
    gtk::init();
    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    let drawing_area = gtk::DrawingArea::new().unwrap();

    // Stolen from www.gtkforums.com/viewtopic.php?f=3&t=988&p=195286#p195286
    drawing_area.connect(signals::Draw::new(|ctx|{
        println!("BeginDraw")

        let width = drawing_area.get_allocated_width();
        let height = drawing_area.get_allocated_height();

        ctx.scale(width as f64, height as f64);

        ctx.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
        ctx.paint();

        ctx.set_line_width(0.05);

        // border
        ctx.set_source_rgb(0.3, 0.3, 0.3);
        ctx.rectangle(0.0, 0.0, 1.0, 1.0);
        ctx.stroke();

        ctx.set_line_width(0.03);

        // draw circle
        ctx.arc(0.5, 0.5, 0.4, 0.0, PI_2);
        ctx.stroke();


        // mouth
        let mouth_top = 0.68;
        let mouth_width = 0.38;

        let mouth_dx = 0.10;
        let mouth_dy = 0.10;

        ctx.move_to( 0.50 - mouth_width/2.0, mouth_top);
        ctx.curve_to(0.50 - mouth_dx,        mouth_top + mouth_dy,
                     0.50 + mouth_dx,        mouth_top + mouth_dy,
                     0.50 + mouth_width/2.0, mouth_top);

        println!("Extents: {}", ctx.fill_extents());

        ctx.stroke();

        let eye_y = 0.38;
        let eye_dx = 0.15;
        ctx.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI_2);
        ctx.fill();

        ctx.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI_2);
        ctx.fill();

    }));

    window.set_default_size(500, 500);

    window.connect(signals::DeleteEvent::new(|event_type|{
        gtk::main_quit();
        true
    }));
    window.add(&drawing_area);
    window.show_all();
    gtk::main();
}