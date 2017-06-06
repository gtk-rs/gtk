// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Traits and essential types inteded for blanket imports.

pub use glib::prelude::*;

pub use auto::traits::*;

pub use app_chooser::AppChooserExt;
pub use assistant::AssistantExtManual;
pub use clipboard::ClipboardExtManual;
pub use color_button::ColorButtonExtManual;
pub use color_chooser::ColorChooserExtManual;
pub use css_provider::CssProviderExtManual;
pub use dialog::DialogExtManual;
pub use drag_context::DragContextExtManual;
pub use list_store::ListStoreExtManual;
pub use menu::MenuExtManual;
pub use notebook::NotebookExtManual;
pub use switch::SwitchExtManual;
pub use text_buffer::TextBufferExtManual;
pub use tree_model_filter::TreeModelFilterExtManual;
pub use tree_sortable::TreeSortableExtManual;
pub use tree_store::TreeStoreExtManual;
pub use widget::WidgetExtManual;

pub use signal::*;
