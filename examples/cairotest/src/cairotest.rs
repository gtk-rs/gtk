#![crate_type = "bin"]
#![feature(core)]

extern crate rgtk;

use std::f64::consts::PI_2;

use rgtk::*;
use rgtk::gtk::signals::{DeleteEvent, Draw};
use rgtk::gtk::DrawingArea;

use rgtk::cairo::enums::FontSlant::FontSlantNormal;
use rgtk::cairo::enums::FontWeight::FontWeightNormal;
use rgtk::cairo::Context;

fn main() {
    gtk::init();

    drawable(500, 500, &mut |cr: Context| {
        cr.set_dash(&[3., 2., 1.], 42.);
        assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 42.));

        cr.scale(500f64, 500f64);

        cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
        cr.paint();

        cr.set_line_width(0.05);

        // border
        cr.set_source_rgb(0.3, 0.3, 0.3);
        cr.rectangle(0.0, 0.0, 1.0, 1.0);
        cr.stroke();

        cr.set_line_width(0.03);

        // draw circle
        cr.arc(0.5, 0.5, 0.4, 0.0, PI_2);
        cr.stroke();


        // mouth
        let mouth_top = 0.68;
        let mouth_width = 0.38;

        let mouth_dx = 0.10;
        let mouth_dy = 0.10;

        cr.move_to( 0.50 - mouth_width/2.0, mouth_top);
        cr.curve_to(0.50 - mouth_dx,        mouth_top + mouth_dy,
                     0.50 + mouth_dx,        mouth_top + mouth_dy,
                     0.50 + mouth_width/2.0, mouth_top);

        println!("Extents: {:?}", cr.fill_extents());

        cr.stroke();

        let eye_y = 0.38;
        let eye_dx = 0.15;
        cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI_2);
        cr.fill();

        cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI_2);
        cr.fill();
    });

    drawable(500, 500, &mut |cr: Context| {
        cr.scale(500f64, 500f64);

        cr.select_font_face("Sans", FontSlantNormal, FontWeightNormal);
        cr.set_font_size(0.35);

        cr.move_to(0.04, 0.53);
        cr.show_text("Hello");

        cr.move_to(0.27, 0.65);
        cr.text_path("void");
        cr.set_source_rgb(0.5, 0.5, 1.0);
        cr.fill_preserve();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.set_line_width(0.01);
        cr.stroke();

        cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        cr.arc(0.04, 0.53, 0.02, 0.0, PI_2);
        cr.arc(0.27, 0.65, 0.02, 0.0, PI_2);
        cr.fill();
    });

    gtk::main();
}

pub fn drawable<T>(width: i32, height: i32, draw_fn: &mut T) where T: FnMut(cairo::Context) {
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    let drawing_area = Box::new(DrawingArea::new)().unwrap();

    Connect::connect(&drawing_area, Draw::new(draw_fn));

    window.set_default_size(width, height);

    Connect::connect(&window, DeleteEvent::new(&mut |_|{
        gtk::main_quit();
        true
    }));
    window.add(&drawing_area);
    window.show_all();
}
