// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Traits and essential types inteded for blanket imports.

pub use glib::prelude::*;

pub use auto::traits::ActionableExt;
pub use auto::traits::BinExt;
pub use auto::traits::BoxExt;
pub use auto::traits::ButtonExt;
pub use auto::traits::CellAreaExt;
pub use auto::traits::CellEditableExt;
pub use auto::traits::CellLayoutExt;
pub use auto::traits::CellRendererExt;
pub use auto::traits::CellRendererTextExt;
pub use auto::traits::CheckButtonExt;
pub use auto::traits::ComboBoxExt;
pub use auto::traits::ContainerExt;
//pub use auto::traits::DialogExt;
pub use auto::traits::EditableExt;
pub use auto::traits::EntryExt;
#[cfg(feature = "v3_14")]
pub use auto::traits::EventControllerExt;
pub use auto::traits::FileChooserExt;
pub use auto::traits::FontChooserExt;
pub use auto::traits::FrameExt;
#[cfg(feature = "v3_14")]
pub use auto::traits::GestureExt;
#[cfg(feature = "v3_14")]
pub use auto::traits::GestureDragExt;
#[cfg(feature = "v3_14")]
pub use auto::traits::GestureSingleExt;
pub use auto::traits::MenuItemExt;
pub use auto::traits::MenuShellExt;
pub use auto::traits::MiscExt;
pub use auto::traits::OrientableExt;
#[cfg(feature = "v3_12")]
pub use auto::traits::PopoverExt;
pub use auto::traits::RangeExt;
pub use auto::traits::RecentChooserExt;
pub use auto::traits::ScaleButtonExt;
pub use auto::traits::ScrollableExt;
pub use auto::traits::ScrolledWindowExt;
pub use auto::traits::ToggleButtonExt;
pub use auto::traits::ToolButtonExt;
pub use auto::traits::ToolItemExt;
pub use auto::traits::ToolShellExt;
//pub use auto::traits::TreeModelExt;
//pub use auto::traits::TreeSortableExt;
//pub use auto::traits::WidgetExt;
pub use auto::traits::WindowExt;

pub use app_chooser::AppChooserExt;
pub use color_chooser::ColorChooserExt;
pub use dialog::DialogExt;
pub use tree_model::TreeModelExt;
pub use tree_sortable::TreeSortableExt;
pub use widget::WidgetExt;

pub use signal::*;
