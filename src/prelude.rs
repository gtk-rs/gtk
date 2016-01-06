// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(gtk_3_4)]
pub use auto::traits::ActionableExt;
pub use auto::traits::BinExt;
pub use auto::traits::BoxExt;
pub use auto::traits::ButtonExt;
pub use auto::traits::CellAreaExt;
pub use auto::traits::CellEditableExt;
pub use auto::traits::CellLayoutExt;
pub use auto::traits::CellRendererExt;
pub use auto::traits::CheckButtonExt;
pub use auto::traits::ComboBoxExt;
pub use auto::traits::ContainerExt;
pub use auto::traits::DialogExt;
pub use auto::traits::EditableExt;
pub use auto::traits::EntryExt;
pub use auto::traits::FileChooserExt;
pub use auto::traits::FontChooserExt;
pub use auto::traits::FrameExt;
pub use auto::traits::MenuItemExt;
pub use auto::traits::MenuShellExt;
pub use auto::traits::MiscExt;
pub use auto::traits::OrientableExt;
#[cfg(gtk_3_12)]
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
pub use auto::traits::TreeSortableExt;

pub use app_chooser::AppChooserExt;
pub use color_chooser::ColorChooserExt;
pub use tree_model::TreeModelExt;
pub use widget::WidgetExt;
pub use window::WindowExt;
