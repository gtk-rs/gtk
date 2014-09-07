//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;

use std::io::{BufferedReader, File};
use std::num::FromPrimitive;

use rgtk::*;
use rgtk::gtk::signals;
use rgtk::gtk::traits::*;

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("Text File Viewer");
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(400, 300);

    let mut toolbar = gtk::Toolbar::new().unwrap();

    let open_icon = gtk::Image::new_from_icon_name("document-open", gtk::icon_size::SmallToolbar).unwrap();
    let text_view = gtk::TextView::new().unwrap();

    let mut open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open")).unwrap();
    open_button.set_is_important(true);
    open_button.connect(signals::Clicked::new(|| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new("Open File", None, gtk::file_chooser_action::Open).unwrap();
        let response: Option<gtk::ResponseType> = FromPrimitive::from_i32(file_chooser.run());

        match response {
            Some(gtk::response_type::Accept) => {
                let filename = file_chooser.get_filename().unwrap();
                let file = File::open(&Path::new(filename));

                let mut reader = BufferedReader::new(file);
                let contents = reader.read_to_string().unwrap();

                text_view.get_buffer().unwrap().set_text(contents);

            },
            _ => {}
        };

        file_chooser.destroy();
    }));

    toolbar.add(&open_button);

    let mut scroll = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll.set_policy(gtk::policy_type::Automatic, gtk::policy_type::Automatic);
    scroll.add(&text_view);

    let mut vbox = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    vbox.pack_start(&toolbar, false, true, 0);
    vbox.pack_start(&scroll, true, true, 0);

    window.add(&vbox);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    window.show_all();
    gtk::main();
}
