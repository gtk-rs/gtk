// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # GTK+ 3 bindings
//!
//! This library contains safe Rust bindings for [GTK+ 3](http://www.gtk.org), a
//! multi-platform GUI toolkit. It's a part of [Gtk-rs](http://gtk-rs.org/).
//!
//! The library is a work in progress: expect missing bindings and breaking
//! changes. A steadily increasing share of the code is machine-generated from
//! `GObject` introspection metadata. The API docs were converted from the
//! upstream ones so until they've all been reviewed there will be incongruities
//! with actual Rust APIs.
//!
//! See also:
//!
//! - [Gtk-rs documentation overview](http://gtk-rs.org/docs/)
//!
//! - [General `GLib` family types and object system overview](../glib/index.html)
//!
//! - [GTK+ documentation](http://www.gtk.org/documentation.php)
//!
//! # Hello World
//!
//! ```no_run
//! extern crate gtk;
//! use gtk::prelude::*;
//! use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
//!
//! fn main() {
//!     if gtk::init().is_err() {
//!         println!("Failed to initialize GTK.");
//!         return;
//!     }
//!     MessageDialog::new(None::<&Window>,
//!                        DialogFlags::empty(),
//!                        MessageType::Info,
//!                        ButtonsType::Ok,
//!                        "Hello World").run();
//! }
//! ```
//!
//! # Initialization
//!
//! GTK+ needs to be initialized before use by calling [`init`](fn.init.html) or
//! [`Application::new`](struct.Application.html#method.new). You only need to
//! do it once and there is no 'finalize'.
//!
//! # The main loop
//!
//! In a typical GTK+ application you set up the UI, assign signal handlers
//! and run the main event loop:
//!
//! ```no_run
//! extern crate gtk;
//! extern crate gio;
//!
//! // To import all needed traits.
//! use gtk::prelude::*;
//! use gio::prelude::*;
//!
//! use std::env;
//!
//! fn main() {
//!     let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
//!                                       gio::ApplicationFlags::FLAGS_NONE)
//!                                  .expect("Application::new failed");
//!     uiapp.connect_activate(|app| {
//!         // We create the main window.
//!         let win = gtk::ApplicationWindow::new(app);
//!
//!         // Then we set its size and a title.
//!         win.set_default_size(320, 200);
//!         win.set_title("Basic example");
//!
//!         // Don't forget to make all widgets visible.
//!         win.show_all();
//!     });
//!     uiapp.run(&env::args().collect::<Vec<_>>());
//! }
//! ```
//!
//! # Threads
//!
//! GTK+ is not thread-safe. Accordingly, none of this crate's structs implement
//! `Send` or `Sync`.
//!
//! The thread where `init` was called is considered the main thread. OS X has
//! its own notion of the main thread and `init` must be called on that thread.
//! After successful initialization, calling any `gtk` or `gdk` functions
//! (including `init`) from other threads will `panic`.
//!
//! Any thread can schedule a closure to be run by the main loop on the main
//! thread via [`glib::idle_add`](../glib/source/fn.idle_add.html) or
//! [`glib::timeout_add`](../glib/source/fn.timeout_add.html). This crate has
//! versions of those functions without the `Send` bound, which may only be
//! called from the main thread: [`idle_add`](fn.idle_add.html),
//! [`timeout_add`](fn.timeout_add.html).
//!
//! # Panics
//!
//! This and the `gdk` crate have some run-time safety and contract checks:
//!
//! - Any constructor or free function will panic if called before `init` or on
//! a non-main thread.
//!
//! - Any `&str` or `&Path` parameter with an interior null (`\0`) character will
//! cause a panic.
//!
//! - Some functions will panic if supplied out-of-range integer parameters. All
//! such cases will be documented individually but they're not yet.
//!
//! **A panic in a closure will abort the process.**
//!
//! # Crate features
//!
//! ## Library versions
//!
//! By default this crate provides only GTK+ 3.14 APIs. You can access more
//! modern APIs by selecting one of the following features: `v3_14`, `v3_16`, etc.
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [dependencies.gtk]
//! version = "0.x.y"
//! features = ["v3_16"]
//! ```
//!
//! **Take care when choosing the version to target: some of your users might
//! not have easy access to the latest ones.** The higher the version, the fewer
//! users will have it installed.
//!
//! ## Lgpl-docs
//!
//! The Gtk-rs crates come with API docs missing because of licensing
//! incompatibilty. You can embed those docs locally via the `embed-lgpl-docs`
//! feature, e.g.
//!
//! ```shell
//! > cargo doc --features embed-lgpl-docs
//! ```
//!
//! Its counterpart `purge-lgpl-docs` removes those docs regardless of edits.
//!
//! These features **rewrite the crate sources** so it's sufficient to enable
//! them once. **Omitting them in the following cargo invocations will not undo
//! their effects!**

#![cfg_attr(feature = "cargo-clippy", allow(let_unit_value))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(derive_hash_xor_eq))]
#![allow(deprecated)]

extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

extern crate glib_sys;
extern crate gio_sys;
extern crate gdk_sys;
extern crate gdk_pixbuf_sys;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate cairo_sys;
extern crate pango_sys;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate cairo;
extern crate pango;
extern crate atk;

#[cfg(feature = "futures")]
extern crate fragile;
#[cfg(feature = "futures")]
extern crate futures;

pub use glib::{
    Cast,
    Continue,
    Error,
    IsA,
    Object,
    StaticType,
    ToValue,
    Type,
    TypedValue,
    Value,
};

pub mod xlib;

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;


#[macro_use]
mod rt;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
#[cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
mod auto;

mod app_chooser;
mod application;
mod application_window;
mod buildable;
mod builder;
mod border;
mod cell_renderer_pixbuf;
mod clipboard;
mod color_button;
mod color_chooser;
mod combo_box;
mod dialog;
mod drag_context;
mod entry_buffer;
mod entry_completion;
mod enums;
mod file_chooser_dialog;
mod fixed;
#[cfg(any(feature = "v3_18", feature = "dox"))]
mod flow_box;
mod im_context_simple;
mod invisible;
#[cfg(any(feature = "v3_16", feature = "dox"))]
mod list_box;
mod list_store;
mod menu;
mod message_dialog;
mod notebook;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod pad_action_entry;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod pad_controller;
mod page_range;
mod print_settings;
mod radio_button;
mod radio_menu_item;
mod radio_tool_button;
mod recent_chooser_dialog;
mod recent_data;
mod requisition;
mod response_type;
mod selection_data;
mod signal;
mod switch;
mod target_entry;
mod target_list;
mod text_buffer;
mod text_iter;
mod tree_model_filter;
mod tree_model_sort;
mod tree_row_reference;
mod tree_sortable;
mod tree_path;
mod tree_store;
mod widget;
mod window;

#[macro_use]
#[cfg(feature = "subclassing")]
pub mod subclass;

pub mod prelude;

pub use auto::*;
pub use auto::functions::*;
pub use rt::*;
pub use signal::*;
pub use prelude::*;

pub use gdk::Rectangle as Allocation;
pub use gdk::Rectangle;

pub use app_chooser::AppChooser;
pub use border::Border;
pub use entry_buffer::EntryBuffer;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use pad_action_entry::PadActionEntry;
pub use page_range::PageRange;
pub use recent_data::RecentData;
pub use requisition::Requisition;
pub use response_type::ResponseType;
pub use target_entry::TargetEntry;
pub use tree_sortable::SortColumn;
pub use widget::TickCallbackId;
