// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub trait FFIWidget: Sized {
    fn unwrap_widget(&self) -> *mut ::ffi::GtkWidget;
    fn wrap_widget(widget: *mut ::ffi::GtkWidget) -> Self;
}

pub use self::style_provider::StyleProviderTrait;
pub use self::widget::WidgetTrait;
pub use self::container::ContainerTrait;
pub use self::window::WindowTrait;
pub use self::misc::MiscTrait;
pub use self::button::ButtonTrait;
pub use self::label::LabelTrait;
pub use self::_box::BoxTrait;
pub use self::orientable::OrientableTrait;
pub use self::frame::FrameTrait;
pub use self::toggle_button::ToggleButtonTrait;
pub use self::scale_button::ScaleButtonTrait;
pub use self::entry::EntryTrait;
pub use self::bin::BinTrait;
pub use self::tool_shell::ToolShellTrait;
pub use self::tool_item::ToolItemTrait;
pub use self::tool_button::ToolButtonTrait;
pub use self::toggle_tool_button::ToggleToolButtonTrait;
pub use self::dialog::DialogButtons;
pub use self::dialog::DialogTrait;
pub use self::color_chooser::ColorChooserTrait;
pub use self::scrollable::ScrollableTrait;
pub use self::file_chooser::FileChooserTrait;
pub use self::app_chooser::AppChooserTrait;
pub use self::font_chooser::FontChooserTrait;
pub use self::recent_chooser::RecentChooserTrait;
pub use self::range::RangeTrait;
pub use self::editable::EditableTrait;
pub use self::text_buffer::TextBufferTrait;
pub use self::scrolled_window::ScrolledWindowTrait;
pub use self::cell_renderer::CellRendererTrait;
pub use self::menu_shell::MenuShellTrait;
pub use self::menu_item::MenuItemTrait;
pub use self::check_menu_item::CheckMenuItemTrait;
pub use self::cell_editable::CellEditableTrait;
pub use self::actionable::ActionableTrait;
pub use self::cell_layout::CellLayoutTrait;
pub use self::combo_box::ComboBoxTrait;
pub use self::gobject::GObjectTrait;

pub use signal::{
    WidgetSignals,
    ButtonSignals,
    ComboBoxSignals,
    ToolButtonSignals,
};

pub mod style_provider;
pub mod widget;
pub mod container;
pub mod window;
pub mod misc;
pub mod button;
pub mod label;
pub mod _box;
pub mod orientable;
pub mod frame;
pub mod toggle_button;
pub mod scale_button;
pub mod entry;
pub mod bin;
pub mod tool_shell;
pub mod tool_item;
pub mod tool_button;
pub mod toggle_tool_button;
pub mod dialog;
pub mod color_chooser;
pub mod scrollable;
pub mod file_chooser;
pub mod app_chooser;
pub mod font_chooser;
pub mod recent_chooser;
pub mod range;
pub mod editable;
pub mod text_buffer;
pub mod scrolled_window;
pub mod cell_renderer;
pub mod menu_shell;
pub mod menu_item;
pub mod check_menu_item;
pub mod cell_editable;
pub mod actionable;
pub mod cell_layout;
pub mod combo_box;
pub mod gobject;
