#![cfg_attr(not(feature = "GTK_3_10"), allow(unused_variables, unused_mut))]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals::{Clicked, KeyPressEvent, DeleteEvent};
use rgtk::gdk::enums::modifier_type;

/// Expands to its argument if GTK+ 3.10 support is configured and to `()` otherwise
#[macro_export]
#[cfg(not(feature = "GTK_3_10"))]
macro_rules! with_gtk_3_10 {
    ($ex:expr) => (
        ()
    );
    ($bl:block) => {
        ()
    }
}

/// Expands to its argument if GTK+ 3.10 support is configured and to `()` otherwise
#[macro_export]
#[cfg(feature = "GTK_3_10")]
macro_rules! with_gtk_3_10 {
    ($ex:expr) => (
        $ex
    );
    ($bl:block) => {
        $bl
    }
}

fn main() {
    gtk::init();
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    let mut frame = gtk::Frame::new(Some("Yep a frame")).unwrap();
    let mut _box = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();
    let mut v_box = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();
    let mut button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal).unwrap();
    let mut label = gtk::Label::new("Yeah a wonderful label too !").unwrap();
    let button = gtk::Button::new_with_label("Whattttt a button !").unwrap();
    let button_about = gtk::Button::new_with_label("About?").unwrap();
    let button_recent = gtk::Button::new_with_label("Choose a recent one !").unwrap();
    let button_font = gtk::Button::new_with_label("Choose a font !").unwrap();
    let app_button = gtk::Button::new_with_label("App ?").unwrap();
    let file_button = gtk::Button::new_with_label("file ?").unwrap();
    let font_button = gtk::FontButton::new().unwrap();
    let toggle_button = gtk::ToggleButton::new_with_label("Toggle Me !").unwrap();
    let check_button = gtk::CheckButton::new_with_label("Labeled check button").unwrap();
    let color_button = gtk::ColorButton::new().unwrap();
    let menu_button = with_gtk_3_10!(
        gtk::MenuButton::new().unwrap()
    );
    let link_button = gtk::LinkButton::new("www.rust-lang.org").unwrap();
    let mut volume_button = gtk::VolumeButton::new().unwrap();
    let mut entry = gtk::Entry::new().unwrap();
    let search_entry = with_gtk_3_10!(
        gtk::SearchEntry::new().unwrap()
    );
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal).unwrap();
    let separator2 = gtk::Separator::new(gtk::Orientation::Horizontal).unwrap();
    let switch = gtk::Switch::new().unwrap();
    let mut switch2 = gtk::Switch::new().unwrap();
    let scale = gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0., 100., 1.).unwrap();
    let mut level_bar = with_gtk_3_10!(
        gtk::LevelBar::new_for_interval(0., 100.).unwrap()
    );
    let spin_button = gtk::SpinButton::new_with_range(0., 100., 1.).unwrap();
    let mut spinner = gtk::Spinner::new().unwrap();
    let image = gtk::Image::new_from_file("./test/resources/gtk.jpg").unwrap();
    let mut progress_bar = gtk::ProgressBar::new().unwrap();
    let arrow = gtk::Arrow::new(gtk::ArrowType::Right, gtk::ShadowType::EtchedOut).unwrap();
    let calendar = gtk::Calendar::new().unwrap();
    let mut info_bar = gtk::InfoBar::new().unwrap();
    let tmp_button = with_gtk_3_10!(
        gtk::Button::new_from_icon_name("edit-clear", gtk::IconSize::Button).unwrap()
    );

    println!("test");

    with_gtk_3_10! {{
        info_bar.show_close_button(true);
    }}

    /*info_bar.connect(signals::Response::new(|response_id| {
        info_bar.hide()
    }));*/ //TODO: Why does this not work?

    progress_bar.set_fraction(0.7);
    spinner.start();
    with_gtk_3_10! {{
        level_bar.set_value(37.);
    }}
    switch2.set_active(true);
    frame.set_border_width(10);
    _box.set_border_width(5);
    entry.set_placeholder("An Entry with a placeholder !");
    volume_button.set_orientation(gtk::Orientation::Horizontal);
    label.set_justify(gtk::Justification::Left);
    window.set_title("Yeah a beautiful window with rgtk !");
    window.set_window_position(gtk::WindowPosition::Center);
    window.add(&frame);

    Connect::connect(&button, Clicked::new(&mut || {

        let dialog = gtk::Dialog::with_buttons(
            "Hello!", None, gtk::DialogFlags::Modal,
            [("No", 0), ("Yes", 1), ("Yes!", 2)]);

        let ret = dialog.run();

        dialog.destroy();

        entry.set_text(&format!("Clicked {}", ret));
    }));

    Connect::connect(&button_about, Clicked::new(&mut ||{

        let dialog = gtk::AboutDialog::new().unwrap();

        let crew = [
            "James T. Kirk",
            "Spock",
            "Leonard McCoy",
        ];

        dialog.set_authors(&crew);
        dialog.set_artists(&crew[1..]);

        println!("Authors: {:?}", dialog.get_authors());
        println!("Artists: {:?}", dialog.get_artists());
        println!("Documenters: {:?}", dialog.get_documenters());

        dialog.run();
        dialog.destroy();
    }));
    Connect::connect(&button_font, Clicked::new(&mut ||{
        let dialog = gtk::FontChooserDialog::new("Font chooser test", None).unwrap();

        dialog.run();
        dialog.destroy();
    }));
    Connect::connect(&button_recent, Clicked::new(&mut ||{
        let dialog = gtk::RecentChooserDialog::new(
            "Recent chooser test", None,
            [("Ok", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);

        dialog.run();
        dialog.destroy();
    }));
    Connect::connect(&file_button, Clicked::new(&mut ||{
        //entry.set_text("Clicked!");
        let dialog = gtk::FileChooserDialog::new(
            "Choose a file", None, gtk::FileChooserAction::Open,
            [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);

        dialog.set_select_multiple(true);

        dialog.run();

        let files = dialog.get_filenames();

        dialog.destroy();

        println!("Files: {:?}", files);
    }));
    Connect::connect(&app_button, Clicked::new(&mut ||{
        //entry.set_text("Clicked!");
        let dialog = gtk::AppChooserDialog::new_for_content_type(None, gtk::DialogFlags::Modal, "sh").unwrap();

        dialog.run();
        dialog.destroy();
    }));

    Connect::connect(&window, KeyPressEvent::new(&mut |key|{
        let keyval = unsafe { (*key).keyval };
        let keystate = unsafe { (*key).state };

        println!("key pressed: {} / {:?}", keyval, keystate);
        println!("text: {}", entry.get_text().unwrap());

        if keystate.intersects(modifier_type::ControlMask) {
            println!("You pressed Ctrl!");
        }
        false
    }));

    Connect::connect(&window, DeleteEvent::new(&mut |_|{
        gtk::main_quit();
        true
    }));

    frame.add(&_box);
    with_gtk_3_10! {{
        button_box.add(&tmp_button)
    }};
    button_box.add(&button);
    button_box.add(&button_about);
    button_box.add(&button_font);
    button_box.add(&button_recent);
    button_box.add(&file_button);
    button_box.add(&app_button);
    button_box.add(&font_button);
    button_box.add(&toggle_button);
    button_box.add(&color_button);
    button_box.add(&volume_button);
    v_box.add(&switch);
    with_gtk_3_10! {{
        v_box.add(&menu_button);
    }}
    v_box.add(&switch2);
    v_box.add(&check_button);
    v_box.add(&link_button);
    v_box.add(&spin_button);
    _box.add(&info_bar);
    _box.add(&v_box);
    _box.add(&scale);
    with_gtk_3_10! {{
        _box.add(&level_bar);
    }}
    _box.add(&button_box);
    _box.add(&progress_bar);
    _box.add(&separator);
    _box.add(&label);
    _box.add(&entry);
    _box.add(&separator2);
    with_gtk_3_10! {{
        _box.add(&search_entry);
    }}
    _box.add(&spinner);
    _box.add(&image);
    _box.add(&arrow);
    _box.add(&calendar);
    _box.set_orientation(gtk::Orientation::Vertical);
    // window.set_decorated(false);
    window.set_decorated(true);
    window.show_all();
    gtk::main();
}
