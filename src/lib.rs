// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # GTK+ 3 bindings
//!
//! This library contains safe Rust bindings for [GTK+ 3](http://www.gtk.org), a
//! multi-platform GUI toolkit. It's a part of [Gtk-rs](http://gtk-rs.org/).
//!
//! The library is a work in progress: expect missing bindings and breaking
//! changes. A steadily increasing share of the code is machine-generated from
//! GObject introspection metadata. The API docs were converted from the
//! upstream ones so until they've all been reviewed there will be incongruities
//! with actual Rust APIs.
//!
//! See also:
//!
//! - [Gtk-rs documentation overview](http://gtk-rs.org/docs/)
//!
//! - [General GLib family types and object system overview](../glib/index.html)
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
//! # extern crate gtk;
//! # use gtk::prelude::*;
//! # use gtk::{Window, WindowType};
//! fn main() {
//!     gtk::init().unwrap();
//!     // Create the main window.
//!     let window = Window::new(WindowType::Toplevel);
//!     // UI initialization.
//!     // ...
//!     // Don't forget to make all widgets visible.
//!     window.show_all();
//!     // Handle closing of the window.
//!     window.connect_delete_event(|_, _| {
//!         // Stop the main loop.
//!         gtk::main_quit();
//!         // Let the default handler destroy the window.
//!         Inhibit(false)
//!     });
//!     // Run the main loop.
//!     gtk::main();
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
//! By default this crate provides only GTK+ 3.4 APIs. You can access more
//! modern APIs by selecting one of the following features: `v3_6`, `v3_8`,
//! `v3_10`, `v3_12`, `v3_14`, `v3_16`.
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

extern crate libc;
#[macro_use]
extern crate bitflags;

extern crate glib_sys as glib_ffi;
extern crate gio_sys as gio_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate gdk_pixbuf_sys as gdk_pixbuf_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as ffi;
extern crate cairo_sys as cairo_ffi;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate cairo;
extern crate pango;

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

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;


#[macro_use]
mod rt;

mod auto;

mod app_chooser;
mod application;
mod assistant;
mod builder;
mod clipboard;
mod color_button;
mod color_chooser;
mod css_provider;
mod dialog;
mod entry_buffer;
mod enums;
mod file_chooser_dialog;
mod list_store;
mod menu;
mod message_dialog;
mod notebook;
mod recent_chooser_dialog;
mod recent_data;
mod recent_info;
mod requisition;
mod signal;
#[cfg(target_os = "linux")]
mod socket;
mod text_buffer;
mod text_iter;
mod tree_model_filter;
mod tree_sortable;
mod tree_path;
mod tree_store;
mod widget;

pub mod prelude;

pub use auto::*;
pub use rt::*;
pub use signal::*;

pub use gdk::Rectangle as Allocation;
pub use app_chooser::AppChooser;
pub use builder::Builder;
pub use color_chooser::ColorChooser;
pub use entry_buffer::EntryBuffer;
pub use recent_data::RecentData;
pub use recent_info::RecentInfo;
pub use gdk::Rectangle;
pub use requisition::Requisition;
#[cfg(target_os = "linux")]
pub use socket::Socket;
pub use tree_sortable::SortColumn;
