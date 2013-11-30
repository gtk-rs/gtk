
#[feature(globs)];

extern mod rgtk;

use rgtk::*;


#[doc(hidden)]
#[cfg(target_os="macos")]
mod platform {
    #[link_args="-lglib-2.0 -lgtk-3.0 -lgobject-2.0 -lgdk-3.0 -lgtk_glue"]
    extern{}
}

#[doc(hidden)]
#[cfg(target_os="linux")]
mod platform {
    #[link_args="-lglib-2.0 -lgtk-3 -lgobject-2.0 -lgdk-3 -lgtk_glue"]
    extern{}
}


fn my_callback(widget: &mut gtk::Button, callback_data: Option<&mut GtkWidget>) {
    if callback_data.is_some() { 
        let mut i = gtk::cast::to_entry(callback_data.unwrap());
        i.set_text(widget.get_label().unwrap());
    }
}

fn main() {
    rt::init();
    println!("Major: {}, Minor: {}", gtk::version::get_major_version(), gtk::version::get_minor_version());
    let mut window = gtk::Window::new(GtkWindowTopLevel).unwrap();
    let mut frame = gtk::Frame::new(Some("Yep a frame")).unwrap();
    let mut box = gtk::Box::new(GtkOrientationHorizontal, 10).unwrap();
    let mut v_box = gtk::Box::new(GtkOrientationHorizontal, 10).unwrap();
    let mut button_box = gtk::ButtonBox::new(GtkOrientationHorizontal).unwrap();
    let mut label = gtk::Label::new("Yeah a wonderful label too !").unwrap();
    let mut button = gtk::Button::new_with_label("Whattttt a button !").unwrap();
    let font_button = gtk::FontButton::new().unwrap();
    let toggle_button = gtk::ToggleButton::new_with_label("Toggle Me !").unwrap();
    let check_button = gtk::CheckButton::new_with_label("Labeled check button").unwrap();
    let color_button = gtk::ColorButton::new().unwrap();
    let menu_button = gtk::MenuButton::new().unwrap();
    let link_button = gtk::LinkButton::new("www.rust-lang.org").unwrap();
    let mut volume_button = gtk::VolumeButton::new().unwrap();
    let mut entry = gtk::Entry::new().unwrap();    
    let search_entry = gtk::SearchEntry::new().unwrap();
    let separator = gtk::Separator::new(GtkOrientationHorizontal).unwrap();
    let separator2 = gtk::Separator::new(GtkOrientationHorizontal).unwrap();
    let switch = gtk::Switch::new().unwrap();
    let mut switch2 = gtk::Switch::new().unwrap();
    let scale = gtk::Scale::new_with_range(GtkOrientationHorizontal, 0., 100., 1.).unwrap();
    let mut level_bar = gtk::LevelBar::new_for_interval(0., 100.).unwrap();
    let spin_button = gtk::SpinButton::new_with_range(0., 100., 1.).unwrap();
    let mut spinner = gtk::Spinner::new().unwrap();
    let image = gtk::Image::new_from_file("./resources/gtk.jpg").unwrap();
    let mut progress_bar = gtk::ProgressBar::new().unwrap();
    let arrow = gtk::Arrow::new(GtkArrowRight, GtkShadowEtchedOut).unwrap();
    let calendar = gtk::Calendar::new().unwrap();
    progress_bar.set_fraction(0.7);
    spinner.start();
    level_bar.set_value(37.);
    switch2.set_active(true);
    frame.set_border_width(10);
    box.set_border_width(5);
    entry.set_placeholder("An Entry with a placeholder !");
    volume_button.set_orientation(GtkOrientationHorizontal);
    label.set_justify(GtkJustifyLeft);
    window.set_title("Yeah a beautiful window with rgtk !");
    window.add(&frame);
    button.connect_2p_widget("clicked", my_callback, Some(&entry));
    window.connect("delete-event", rt::_main_quit);
    frame.add(&box);
    button_box.add(&button);
    button_box.add(&font_button);
    button_box.add(&toggle_button);
    button_box.add(&color_button);
    button_box.add(&volume_button);
    v_box.add(&switch);
    v_box.add(&menu_button);
    v_box.add(&switch2);
    v_box.add(&check_button);
    v_box.add(&link_button);
    v_box.add(&spin_button);
    box.add(&v_box);
    box.add(&scale);
    box.add(&level_bar);
    box.add(&button_box);
    box.add(&progress_bar);
    box.add(&separator);
    box.add(&label);
    box.add(&entry);
    box.add(&separator2);
    box.add(&search_entry);
    box.add(&spinner);
    box.add(&image);
    box.add(&arrow);
    box.add(&calendar);
    box.set_orientation(GtkOrientationVertical);
    window.show_all();
    rt::_main();
}

