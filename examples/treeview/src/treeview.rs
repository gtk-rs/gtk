//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn append_text_column(tree: &mut gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("TreeView Sample");
    window.set_window_position(gtk::window_position::Center);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    // left pane

    let mut left_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::ffi::g_type_string];
    let left_store = gtk::ListStore::new(column_types).unwrap();
    let left_model = left_store.get_model().unwrap();
    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(false);
    append_text_column(&mut left_tree);

    for _ in range(0i, 10i) {
        let iter = gtk::TreeIter::new().unwrap();
        left_store.append(&iter);
        left_store.set_string(&iter, 0, "I'm in a list");
    }

    // right pane

    let mut right_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::ffi::g_type_string];
    let right_store = gtk::TreeStore::new(column_types).unwrap();
    let right_model = right_store.get_model().unwrap();
    right_tree.set_model(&right_model);
    right_tree.set_headers_visible(false);
    append_text_column(&mut right_tree);

    for _ in range(0i, 10i) {
        let iter = gtk::TreeIter::new().unwrap();
        right_store.append(&iter, None);
        right_store.set_string(&iter, 0, "I'm in a tree");

        let mut child_raw = gtk::ffi::C_GtkTreeIter;
        let child = gtk::TreeIter::wrap_pointer(&mut child_raw);
        right_store.append(&child, Some(&iter));
        right_store.set_string(&child, 0, "I'm a child node");
    }

    // display the panes

    let mut split_pane = gtk::Box::new(gtk::orientation::Horizontal, 10).unwrap();
    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk::main();
}
