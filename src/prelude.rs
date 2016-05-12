// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Traits and essential types inteded for blanket imports.

pub use glib::prelude::*;
pub use gio::prelude::*;

pub use auto::traits::*;

pub use app_chooser::AppChooserExt;
pub use color_chooser::ColorChooserExt;
pub use dialog::DialogExtManual;
pub use tree_sortable::TreeSortableExtManual;
pub use widget::WidgetExtManual;

pub use signal::*;
