
#![feature(globs)]

extern crate rgtk;
extern crate log;
extern crate debug;
extern crate collections;

use log::macros::*;

use rgtk::*;
use rgtk::gtk::signals;

use collections::String;

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
    let mut drawing_area = gtk::DrawingArea::new().unwrap();

    // Stolen from www.gtkforums.com/viewtopic.php?f=3&t=988&p=195286#p195286
    drawing_area.connect(signals::Draw::new(|ctx|{
        ctx.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
        ctx.paint();

        /* set color for rectangle */
        ctx.set_source_rgb(0.42, 0.65, 0.80);
        /* set the line width */
        ctx.set_line_width(6.0);
        /* draw the rectangle's path beginning at 3,3 */
        ctx.rectangle(3.0, 3.0, 100.0, 100.0);

        println!("Is in: {} {}", ctx.in_fill(0.0, 0.0), ctx.in_fill(5.0, 5.0))

        ctx.fill_preserve();

        /* stroke the rectangle's path with the chosen color so it's actually visible */
        ctx.stroke();

        /* draw circle */
        ctx.set_source_rgb(0.17, 0.63, 0.12);
        ctx.set_line_width(2.0);
        ctx.arc(150.0, 210.0, 20.0, 0.0, 2.0*3.14);
        ctx.stroke();

        /* draw horizontal line */
        ctx.set_source_rgb(0.77, 0.16, 0.13);
        ctx.set_line_width(6.0);
        ctx.move_to(80.0,160.0);
        ctx.line_to(200.0, 160.0);
        ctx.stroke();
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