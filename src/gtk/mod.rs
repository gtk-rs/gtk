// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

/*!

Bindings and wrappers for __GTK__

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits
located in `rgtk::self::traits::*`. The various widgets implement these traits and
live in `rgtk::gtk::widgets::*` and are rexported into `rgtk::gtk::*`.

GTK Inheritance in rgtk
======================

You probably know but __Gtk+__ uses its own GObject system: inherited class and interface.

To respect this design I follow a special design on __rgtk__:

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Exemple for GtkOrientable, GtkBox, GtkButtonBox:

GtkOrientable is an interface with all the methods implemented as default method of the trait self::traits::Orientable.

GtkBox is a class with constructors implemented on the struct `gtk::Box`, and the other method as default methods of the trait `self::traits::Box`. So `gtk::Box` implements `self::traits::Orientable` and `self::traits::Box`.

GtkButtonBox is a sub-class of GtkBox, the struct `gtk::ButtonBox` implements all the methods of GtkButtonBox and the traits `self::traits::Orientable` and `self::traits::Box`.

Finally all the gtk widgets implement the trait self::traits::Widget.
*/


// These are/should be inlined
pub use self::rt::{
    init,
    main,
    main_quit,
    main_level,
    main_iteration,
    main_iteration_do,
    get_major_version,
    get_minor_version,
    get_micro_version,
    get_binary_age,
    get_interface_age,
    check_version,
    events_pending
};


/// GTK Widgets for all versions
pub use self::widgets::{
    Window,
    Label,
    Button,
    Box,
    ButtonBox,
    Frame,
    AspectFrame,
    Fixed,
    Separator,
    FontButton,
    ToggleButton,
    CheckButton,
    ColorButton,
    LinkButton,
    Adjustment,
    ScaleButton,
    VolumeButton,
    Grid,
    EntryBuffer,
    Entry,
    Switch,
    Scale,
    SpinButton,
    Spinner,
    Image,
    ProgressBar,
    Arrow,
    Calendar,
    Alignment,
    Expander,
    Paned,
    InfoBar,
    Toolbar,
    ToolItem,
    SeparatorToolItem,
    ToolButton,
    ToggleToolButton,
    MenuToolButton,
    Dialog,
    AboutDialog,
    ColorChooserDialog,
    FontChooserDialog,
    MessageDialog,
    NoteBook,
    Overlay,
    Layout,
    FileFilter,
    FileChooserDialog,
    AppInfo,
    AppLaunchContext,
    AppChooserDialog,
    DrawingArea,
    PageSetup,
    PaperSize,
    PrintSettings,
    RecentChooserDialog,
    //PageSetupUnixDialog
    RecentInfo,
    RecentFilter,
    RecentFilterInfo,
    RecentData,
    RecentManager,
    TextView,
    TextBuffer,
    TextTagTable,
    ScrolledWindow,
    RadioButton,
    TreeView,
    TreeViewColumn,
    TreePath,
    TreeIter,
    TreeModel,
    ListStore,
    TreeStore,
    MenuItem,
    SeparatorMenuItem,
    CheckMenuItem,
    ScrollBar,
    Viewport,
    StatusBar,
    CellRendererText,
    CellRendererToggle,
    LockButton,
    EntryCompletion,
    IconView,
    TreeSelection,
    RecentChooserWidget,
    ComboBox,
    GValue,
    g_type,
    ComboBoxText,
    TextMark,
    TextTag,
    TextAttributes,
    TextIter,
    TextChildAnchor,
    ToolPalette,
    ToolItemGroup,
    SizeGroup,
    AppChooserWidget,
    FileChooserWidget,
    ColorChooserWidget,
    FontChooserWidget,
    EventBox
};

#[cfg(target_os = "linux")]
pub use self::widgets::{Socket};

#[cfg(any(feature = "GTK_3_6", feature = "GTK_3_8", feature = "GTK_3_10", feature = "GTK_3_12", feature = "GTK_3_14"))]
/// GTK Widgets for versions since GTK 3.6
pub use self::widgets::{
    MenuButton,
    LevelBar,
};

#[cfg(any(feature = "GTK_3_10", feature = "GTK_3_12", feature = "GTK_3_14"))]
/// GTK Widgets for versions since GTK 3.10
pub use self::widgets::{
    SearchEntry,
    SearchBar,
    Stack,
    StackSwitcher,
    Revealer,
    HeaderBar,
    ListBox,
    ListBoxRow,
    PlacesSidebar
};

#[cfg(any(feature = "GTK_3_12", feature = "GTK_3_14"))]
/// GTK Widgets for versions since GTK 3.12
pub use self::widgets::{
    FlowBox,
    FlowBoxChild,
    ActionBar,
    Popover
};

/// GTK Enum types
pub use self::enums::window_type::WindowType;
pub use self::enums::text_direction::TextDirection;
pub use self::enums::window_position::WindowPosition;
pub use self::enums::button_box_style::ButtonBoxStyle;
pub use self::enums::orientation::Orientation;
pub use self::enums::direction_type::DirectionType;
pub use self::enums::corner_type::CornerType;
pub use self::enums::resize_mode::ResizeMode;
pub use self::enums::border_style::BorderStyle;
pub use self::enums::sort_type::SortType;
pub use self::enums::state_flags::StateFlags;
pub use self::enums::drag_result::DragResult;
pub use self::enums::accel_flags::AccelFlags;
pub use self::enums::arrow_placement::ArrowPlacement;
pub use self::enums::arrow_type::ArrowType;
pub use self::enums::attach_options::AttachOptions;
pub use self::enums::delete_type::DeleteType;
pub use self::enums::expander_style::ExpanderStyle;
pub use self::enums::im_preedit_style::IMPreeditStyle;
pub use self::enums::im_status_style::IMStatusStyle;
pub use self::enums::justification::Justification;
pub use self::enums::movement_step::MovementStep;
pub use self::enums::pack_type::PackType;
pub use self::enums::path_priority_type::PathPriorityType;
pub use self::enums::path_type::PathType;
pub use self::enums::policy_type::PolicyType;
pub use self::enums::position_type::PositionType;
pub use self::enums::relief_style::ReliefStyle;
pub use self::enums::scroll_step::ScrollStep;
pub use self::enums::scroll_type::ScrollType;
pub use self::enums::selection_mode::SelectionMode;
pub use self::enums::shadow_type::ShadowType;
pub use self::enums::state_type::StateType;
pub use self::enums::toolbar_style::ToolbarStyle;
pub use self::enums::junction_sides::JunctionSides;
pub use self::enums::region_flags::RegionFlags;
pub use self::enums::icon_size::IconSize;
pub use self::enums::entry_icon_position::EntryIconPosition;
pub use self::enums::input_hints::InputHints;
pub use self::enums::input_purpose::InputPurpose;
pub use self::enums::image_type::ImageType;
pub use self::enums::spin_type::SpinType;
pub use self::enums::spin_button_update_policy::SpinButtonUpdatePolicy;
pub use self::enums::level_bar_mode::LevelBarMode;
pub use self::enums::calendar_display_options::CalendarDisplayOptions;
pub use self::enums::message_type::MessageType;
pub use self::enums::license::License;
pub use self::enums::response_type::ResponseType;
pub use self::enums::dialog_flags::DialogFlags;
pub use self::enums::file_chooser_action::FileChooserAction;
pub use self::enums::buttons_type::ButtonsType;
pub use self::enums::stack_transition_type::StackTransitionType;
pub use self::enums::revealer_transition_type::RevealerTransitionType;
pub use self::enums::scrollable_policy::ScrollablePolicy;
pub use self::enums::file_filter_flags::FileFilterFlags;
pub use self::enums::app_info_create_flags::AppInfoCreateFlags;
pub use self::enums::size_request_mode::SizeRequestMode;
pub use self::enums::align::Align;
pub use self::enums::g_connect_flags::GConnectFlags;
pub use self::enums::builder_error::BuilderError;
pub use self::enums::page_orientation::PageOrientation;
pub use self::enums::unit::Unit;
pub use self::enums::number_up_layout::NumberUpLayout;
pub use self::enums::print_pages::PrintPages;
pub use self::enums::page_set::PageSet;
pub use self::enums::recent_sort_type::RecentSortType;
pub use self::enums::recent_filter_flags::RecentFilterFlags;
pub use self::enums::widget_help_type::WidgetHelpType;
pub use self::enums::text_window_type::TextWindowType;
pub use self::enums::wrap_mode::WrapMode;
pub use self::enums::tree_view_grid_lines::TreeViewGridLines;
pub use self::enums::tree_view_column_sizing::TreeViewColumnSizing;
pub use self::enums::cell_renderer_state::CellRendererState;
pub use self::enums::tree_model_flags::TreeModelFlags;
pub use self::enums::icon_view_drop_position::IconViewDropPosition;
pub use self::enums::sensitivity_type::SensitivityType;
pub use self::enums::g_type_enum::GType;
pub use self::enums::text_search_flags::TextSearchFlags;
pub use self::enums::places_open_flags::PlacesOpenFlags;
pub use self::enums::tool_palette_drag_targets::ToolPaletteDragTargets;
pub use self::enums::dest_defaults::DestDefaults;
pub use self::enums::size_group_mode::SizeGroupMode;
pub use self::traits::GObjectTrait;

/// Gtk Traits
pub use self::traits::BoxTrait;
pub use self::traits::ActionableTrait;
pub use self::traits::AppChooserTrait;
pub use self::traits::BinTrait;
pub use self::traits::ButtonTrait;
pub use self::traits::CellEditableTrait;
pub use self::traits::CellLayoutTrait;
pub use self::traits::CellRendererTrait;
pub use self::traits::CheckMenuItemTrait;
pub use self::traits::ColorChooserTrait;
pub use self::traits::ComboBoxTrait;
pub use self::traits::ContainerTrait;
pub use self::traits::DialogTrait;
pub use self::traits::EditableTrait;
pub use self::traits::EntryTrait;
pub use self::traits::FileChooserTrait;
pub use self::traits::FontChooserTrait;
pub use self::traits::FrameTrait;
pub use self::traits::LabelTrait;
pub use self::traits::MenuItemTrait;
pub use self::traits::MenuShellTrait;
pub use self::traits::MiscTrait;
pub use self::traits::OrientableTrait;
pub use self::traits::RangeTrait;
pub use self::traits::RecentChooserTrait;
pub use self::traits::ScaleButtonTrait;
pub use self::traits::ScrollableTrait;
pub use self::traits::ScrolledWindowTrait;
pub use self::traits::TextBufferTrait;
pub use self::traits::ToggleButtonTrait;
pub use self::traits::ToggleToolButtonTrait;
pub use self::traits::ToolButtonTrait;
pub use self::traits::ToolItemTrait;
pub use self::traits::ToolShellTrait;
pub use self::traits::WidgetTrait;
pub use self::traits::WindowTrait;

/// GTK various struct
pub use self::types::{
    Tooltip,
};

mod macros;
mod cast;
mod rt;

mod enums;
mod traits;
pub mod signals;
pub mod widgets;
pub mod types;

#[doc(hidden)]
pub mod ffi;
