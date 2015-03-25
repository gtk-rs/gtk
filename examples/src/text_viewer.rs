//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

#![crate_type = "bin"]
#![feature(core)]

extern crate rgtk;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::num::FromPrimitive;

use rgtk::*;
use rgtk::gtk::signals::{Clicked, DeleteEvent};

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title("Text File Viewer");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);

    let mut toolbar = gtk::Toolbar::new().unwrap();

    let open_icon = gtk::Image::new_from_icon_name("document-open", gtk::IconSize::SmallToolbar).unwrap();
    let text_view = gtk::TextView::new().unwrap();

    let mut open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open")).unwrap();
    open_button.set_is_important(true);
    Connect::connect(&open_button, Clicked::new(&mut || {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            "Open File", None, gtk::FileChooserAction::Open,
            [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
        let response: Option<gtk::ResponseType> = FromPrimitive::from_i32(file_chooser.run());

        match response {
            Some(gtk::ResponseType::Ok) => {
                let filename = file_chooser.get_filename().unwrap();
                let file = File::open(&filename).unwrap();

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view.get_buffer().unwrap().set_text(&contents);

            },
            _ => {}
        };

        file_chooser.destroy();
    }));

    toolbar.add(&open_button);

    let mut scroll = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    vbox.pack_start(&toolbar, false, true, 0);
    vbox.pack_start(&scroll, true, true, 0);

    window.add(&vbox);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk::main_quit();
        true
    }));

    window.show_all();
    gtk::main();
}
