// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(feature = "gtk_3_12")]
pub mod action_bar;
pub mod actionable;
pub mod alignment;
pub mod arrow;
pub mod bin;
pub mod box_;
pub mod button;
pub mod calendar;
pub mod combo_box;
pub mod container;
pub mod drawing_area;
pub mod editable;
pub mod entry;
pub mod fixed;
pub mod frame;
#[cfg(feature = "gtk_3_10")]
pub mod header_bar;
pub mod image;
pub mod label;
#[cfg(feature = "gtk_3_6")]
pub mod level_bar;
pub mod misc;
pub mod orientable;
pub mod progress_bar;
pub mod scrollable;
pub mod separator;
pub mod spinner;
pub mod switch;
pub mod widget;

// ---------------- cut ----------------

/*
pub use self::widget::Widget;
pub use self::builder::Builder;
pub use self::window::Window;
pub use self::label::Label;
pub use self::button::Button;
pub use self::_box::Box;
pub use self::button_box::ButtonBox;
pub use self::frame::Frame;
pub use self::aspect_frame::AspectFrame;
pub use self::fixed::Fixed;
pub use self::separator::Separator;
pub use self::font_button::FontButton;
pub use self::toggle_button::ToggleButton;
pub use self::check_button::CheckButton;
pub use self::font_chooser_dialog::FontChooserDialog;
#[cfg(feature = "gtk_3_6")]
pub use self::menu_button::MenuButton;
pub use self::color_button::ColorButton;
pub use self::link_button::LinkButton;
pub use self::adjustment::Adjustment;
pub use self::scale_button::ScaleButton;
pub use self::volume_button::VolumeButton;
pub use self::grid::Grid;
pub use self::entry_buffer::EntryBuffer;
pub use self::entry::Entry;
#[cfg(feature = "gtk_3_10")]
pub use self::search_entry::SearchEntry;
pub use self::switch::Switch;
pub use self::range::Range;
pub use self::scale::Scale;
#[cfg(feature = "gtk_3_6")]
pub use self::level_bar::LevelBar;
#[cfg(feature = "gtk_3_10")]
pub use self::search_bar::SearchBar;
pub use self::spin_button::SpinButton;
pub use self::spinner::Spinner;
pub use self::image::Image;
pub use self::progress_bar::ProgressBar;
pub use self::arrow::Arrow;
pub use self::calendar::Calendar;
pub use self::alignment::Alignment;
pub use self::expander::Expander;
pub use self::paned::Paned;
pub use self::info_bar::InfoBar;
pub use self::tool_bar::Toolbar;
pub use self::tool_item::ToolItem;
pub use self::separator_tool_item::SeparatorToolItem;
pub use self::tool_button::ToolButton;
pub use self::toggle_tool_button::ToggleToolButton;
pub use self::menu_tool_button::MenuToolButton;
pub use self::tree_path::TreePath;
pub use self::tree_iter::TreeIter;
pub use self::tree_model::TreeModel;
pub use self::list_store::ListStore;
pub use self::tree_store::TreeStore;
pub use self::dialog::Dialog;
pub use self::about_dialog::AboutDialog;
pub use self::message_dialog::MessageDialog;
pub use self::color_chooser_dialog::ColorChooserDialog;
pub use self::note_book::NoteBook;
#[cfg(feature = "gtk_3_10")]
pub use self::stack::Stack;
#[cfg(feature = "gtk_3_10")]
pub use self::stack_switcher::StackSwitcher;
#[cfg(feature = "gtk_3_10")]
pub use self::revealer::Revealer;
pub use self::overlay::Overlay;
pub use self::layout::Layout;
#[cfg(feature = "gtk_3_10")]
pub use self::header_bar::HeaderBar;
#[cfg(feature = "gtk_3_12")]
pub use self::flow_box::{FlowBox, FlowBoxChild};
#[cfg(feature = "gtk_3_10")]
pub use self::list_box::{ListBox, ListBoxRow};
#[cfg(feature = "gtk_3_12")]
pub use self::action_bar::ActionBar;
pub use self::file_filter::FileFilter;
pub use self::file_chooser_dialog::FileChooserDialog;
pub use self::app_info::AppInfo;
pub use self::app_launch_context::AppLaunchContext;
pub use self::app_chooser_dialog::AppChooserDialog;
pub use self::drawing_area::DrawingArea;
pub use self::page_setup::PageSetup;
//pub use self::pagesetupunixdialog::PageSetupUnixDialog;
pub use self::paper_size::PaperSize;
pub use self::print_settings::PrintSettings;
pub use self::recent_chooser_dialog::RecentChooserDialog;
pub use self::recent_filter::RecentFilter;
pub use self::recent_info::RecentInfo;
pub use self::recent_filter_info::RecentFilterInfo;
pub use self::recent_data::RecentData;
pub use self::recent_manager::RecentManager;
pub use self::text_view::TextView;
pub use self::text_buffer::TextBuffer;
pub use self::text_tag_table::TextTagTable;
pub use self::scrolled_window::ScrolledWindow;
pub use self::radio_button::RadioButton;
pub use self::tree_view::TreeView;
pub use self::tree_view_column::TreeViewColumn;
pub use self::menu_item::MenuItem;
pub use self::separator_menu_item::SeparatorMenuItem;
pub use self::check_menu_item::CheckMenuItem;
pub use self::scrollbar::ScrollBar;
pub use self::viewport::Viewport;
pub use self::status_bar::StatusBar;
pub use self::cell_renderer_text::CellRendererText;
pub use self::cell_renderer_toggle::CellRendererToggle;
pub use self::lock_button::LockButton;
pub use self::entry_completion::EntryCompletion;
pub use self::icon_view::IconView;
pub use self::tree_selection::TreeSelection;
pub use self::recent_chooser_widget::RecentChooserWidget;
pub use self::combo_box::ComboBox;
#[cfg(feature = "gtk_3_12")]
pub use self::popover::Popover;
pub use self::combo_box_text::ComboBoxText;
//pub use self::gtype::g_type;
pub use self::text_mark::TextMark;
pub use self::text_tag::TextTag;
pub use self::text_attributes::TextAttributes;
pub use self::text_iter::TextIter;
pub use self::text_child_anchor::TextChildAnchor;
#[cfg(feature = "gtk_3_10")]
pub use self::places_sidebar::PlacesSidebar;
pub use self::tool_palette::ToolPalette;
pub use self::tool_item_group::ToolItemGroup;
pub use self::size_group::SizeGroup;
pub use self::app_chooser_widget::AppChooserWidget;
pub use self::file_chooser_widget::FileChooserWidget;
pub use self::color_chooser_widget::ColorChooserWidget;
pub use self::font_chooser_widget::FontChooserWidget;
#[cfg(target_os = "linux")]
pub use self::socket::Socket;
pub use self::event_box::EventBox;

mod widget;
mod builder;
mod window;
mod label;
mod button;
mod _box;
mod button_box;
mod frame;
mod aspect_frame;
mod fixed;
mod separator;
mod font_button;
mod toggle_button;
mod check_button;
#[cfg(feature = "gtk_3_6")]
mod menu_button;
mod color_button;
mod link_button;
mod adjustment;
mod scale_button;
mod volume_button;
mod grid;
mod entry_buffer;
mod entry;
#[cfg(feature = "gtk_3_10")]
mod search_entry;
mod switch;
mod range;
mod scale;
#[cfg(feature = "gtk_3_6")]
mod level_bar;
#[cfg(feature = "gtk_3_10")]
mod search_bar;
mod spin_button;
mod spinner;
mod image;
mod progress_bar;
mod arrow;
mod calendar;
mod alignment;
mod expander;
mod paned;
mod info_bar;
mod tool_bar;
mod tool_item;
mod separator_tool_item;
mod tool_button;
mod toggle_tool_button;
mod menu_tool_button;
mod dialog;
mod about_dialog;
mod color_chooser_dialog;
mod font_chooser_dialog;
mod message_dialog;
mod note_book;
#[cfg(feature = "gtk_3_10")]
mod stack;
#[cfg(feature = "gtk_3_10")]
mod stack_switcher;
#[cfg(feature = "gtk_3_10")]
mod revealer;
mod overlay;
mod layout;
#[cfg(feature = "gtk_3_10")]
mod header_bar;
#[cfg(feature = "gtk_3_12")]
mod flow_box;
#[cfg(feature = "gtk_3_10")]
mod list_box;
#[cfg(feature = "gtk_3_12")]
mod action_bar;
mod file_filter;
mod file_chooser_dialog;
mod app_info;
mod app_launch_context;
mod app_chooser_dialog;
mod drawing_area;
mod page_setup;
mod paper_size;
//mod pagesetupunixdialog;
mod print_settings;
mod recent_chooser_dialog;
mod recent_filter;
mod recent_info;
mod recent_filter_info;
mod recent_data;
mod recent_manager;
mod text_view;
mod text_buffer;
mod text_tag_table;
mod scrolled_window;
mod radio_button;
mod tree_view;
mod tree_view_column;
mod tree_path;
mod tree_iter;
mod tree_model;
mod list_store;
mod tree_store;
mod menu_item;
mod separator_menu_item;
mod check_menu_item;
mod scrollbar;
mod viewport;
mod status_bar;
mod cell_renderer_text;
mod cell_renderer_toggle;
mod lock_button;
mod entry_completion;
mod icon_view;
mod tree_selection;
mod recent_chooser_widget;
mod combo_box;
#[cfg(feature = "gtk_3_12")]
mod popover;
mod combo_box_text;
//mod gtype;
mod text_mark;
mod text_tag;
mod text_attributes;
mod text_iter;
mod text_child_anchor;
#[cfg(feature = "gtk_3_10")]
mod places_sidebar;
mod tool_palette;
mod tool_item_group;
mod size_group;
mod app_chooser_widget;
mod file_chooser_widget;
mod color_chooser_widget;
mod font_chooser_widget;
#[cfg(target_os = "linux")]
mod socket;
mod event_box;
*/
