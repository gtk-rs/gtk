// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Traits and essential types intended for blanket imports.

pub use glib::prelude::*;

pub use auto::traits::*;

pub use app_chooser::AppChooserExt;
pub use buildable::BuildableExtManual;
pub use builder::BuilderExtManual;
pub use cell_renderer_pixbuf::CellRendererPixbufExtManual;
pub use color_button::ColorButtonExtManual;
pub use color_chooser::ColorChooserExtManual;
pub use combo_box::ComboBoxExtManual;
pub use dialog::DialogExtManual;
pub use drag_context::DragContextExtManual;
pub use entry_completion::EntryCompletionExtManual;
pub use fixed::FixedExtManual;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use flow_box::FlowBoxExtManual;
pub use im_context_simple::IMContextSimpleExtManual;
pub use invisible::InvisibleExtManual;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use list_box::ListBoxExtManual;
pub use list_store::GtkListStoreExtManual;
pub use menu::GtkMenuExtManual;
pub use notebook::NotebookExtManual;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use pad_controller::PadControllerExtManual;
pub use switch::SwitchExtManual;
pub use text_buffer::TextBufferExtManual;
pub use tree_sortable::TreeSortableExtManual;
pub use tree_store::TreeStoreExtManual;
pub use widget::WidgetExtManual;
pub use window::GtkWindowExtManual;

pub use signal::*;
