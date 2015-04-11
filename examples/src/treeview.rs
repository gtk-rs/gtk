//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk;

use gtk::Connect;
use gtk::traits::*;
use gtk::signals::DeleteEvent;

fn append_text_column(tree: &mut gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();

    window.set_title("TreeView Sample");
    window.set_window_position(gtk::WindowPosition::Center);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk::main_quit();
        true
    }));

    // test Value

    let hello = String::from("Hello world !");
    let value = glib::Value::new().unwrap();

    value.init(glib::Type::String);
    value.set(&hello);
    println!("gvalue.get example : {}", value.get::<String>());

    // left pane

    let mut left_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::Type::String];
    let left_store = gtk::ListStore::new(&column_types).unwrap();
    let left_model = left_store.get_model().unwrap();

    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(false);
    append_text_column(&mut left_tree);

    for _ in 0..10 {
        let mut iter = gtk::TreeIter::new().unwrap();
        left_store.append(&mut iter);
        left_store.set_string(&iter, 0, "I'm in a list");
    }

    // right pane

    let mut right_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::Type::String];
    let right_store = gtk::TreeStore::new(&column_types).unwrap();
    let right_model = right_store.get_model().unwrap();

    right_tree.set_model(&right_model);
    right_tree.set_headers_visible(false);
    append_text_column(&mut right_tree);

    for _ in 0..10 {
        let mut iter = gtk::TreeIter::new().unwrap();

        right_store.append(&mut iter, None);
        right_store.set_value(&iter, 0, &value);

        let mut child_iter = gtk::TreeIter::new().unwrap();

        right_store.append(&mut child_iter, Some(&iter));
        right_store.set_string(&child_iter, 0, "I'm a child node");
    }

    // display the panes

    let mut split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk::main();
}
