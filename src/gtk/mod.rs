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

#![macro_use]

pub use gtk_ffi as ffi;
pub use gtk_ffi::enums;

pub trait FFIWidget: Sized {
    fn unwrap_widget(&self) -> *mut ffi::C_GtkWidget;
    fn wrap_widget(widget: *mut ffi::C_GtkWidget) -> Self;
}

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
    //g_type,
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

#[cfg(feature = "GTK_3_6")]
/// GTK Widgets for versions since GTK 3.6
pub use self::widgets::{
    MenuButton,
    LevelBar,
};

#[cfg(feature = "GTK_3_10")]
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

#[cfg(feature = "GTK_3_12")]
/// GTK Widgets for versions since GTK 3.12
pub use self::widgets::{
    FlowBox,
    FlowBoxChild,
    ActionBar,
    Popover
};

/// GTK Enum types
pub use gtk_ffi::enums::WindowType;
pub use gtk_ffi::enums::TextDirection;
pub use gtk_ffi::enums::WindowPosition;
pub use gtk_ffi::enums::ButtonBoxStyle;
pub use gtk_ffi::enums::Orientation;
pub use gtk_ffi::enums::DirectionType;
pub use gtk_ffi::enums::CornerType;
pub use gtk_ffi::enums::ResizeMode;
pub use gtk_ffi::enums::BorderStyle;
pub use gtk_ffi::enums::SortType;
pub use gtk_ffi::enums::StateFlags;
pub use gtk_ffi::enums::DragResult;
pub use gtk_ffi::enums::AccelFlags;
pub use gtk_ffi::enums::ArrowPlacement;
pub use gtk_ffi::enums::ArrowType;
pub use gtk_ffi::enums::AttachOptions;
pub use gtk_ffi::enums::DeleteType;
pub use gtk_ffi::enums::ExpanderStyle;
pub use gtk_ffi::enums::IMPreeditStyle;
pub use gtk_ffi::enums::IMStatusStyle;
pub use gtk_ffi::enums::Justification;
pub use gtk_ffi::enums::MovementStep;
pub use gtk_ffi::enums::PackType;
pub use gtk_ffi::enums::PathPriorityType;
pub use gtk_ffi::enums::PathType;
pub use gtk_ffi::enums::PolicyType;
pub use gtk_ffi::enums::PositionType;
pub use gtk_ffi::enums::ReliefStyle;
pub use gtk_ffi::enums::ScrollStep;
pub use gtk_ffi::enums::ScrollType;
pub use gtk_ffi::enums::SelectionMode;
pub use gtk_ffi::enums::ShadowType;
pub use gtk_ffi::enums::StateType;
pub use gtk_ffi::enums::ToolbarStyle;
pub use gtk_ffi::enums::JunctionSides;
pub use gtk_ffi::enums::RegionFlags;
pub use gtk_ffi::enums::IconSize;
pub use gtk_ffi::enums::EntryIconPosition;
pub use gtk_ffi::enums::InputHints;
pub use gtk_ffi::enums::InputPurpose;
pub use gtk_ffi::enums::ImageType;
pub use gtk_ffi::enums::SpinType;
pub use gtk_ffi::enums::SpinButtonUpdatePolicy;
pub use gtk_ffi::enums::LevelBarMode;
pub use gtk_ffi::enums::CalendarDisplayOptions;
pub use gtk_ffi::enums::MessageType;
pub use gtk_ffi::enums::License;
pub use gtk_ffi::enums::ResponseType;
pub use gtk_ffi::enums::DialogFlags;
pub use gtk_ffi::enums::FileChooserAction;
pub use gtk_ffi::enums::ButtonsType;
pub use gtk_ffi::enums::StackTransitionType;
pub use gtk_ffi::enums::RevealerTransitionType;
pub use gtk_ffi::enums::ScrollablePolicy;
pub use gtk_ffi::enums::FileFilterFlags;
pub use gtk_ffi::enums::AppInfoCreateFlags;
pub use gtk_ffi::enums::SizeRequestMode;
pub use gtk_ffi::enums::Align;
pub use gtk_ffi::enums::GConnectFlags;
pub use gtk_ffi::enums::BuilderError;
pub use gtk_ffi::enums::PageOrientation;
pub use gtk_ffi::enums::Unit;
pub use gtk_ffi::enums::NumberUpLayout;
pub use gtk_ffi::enums::PrintPages;
pub use gtk_ffi::enums::PageSet;
pub use gtk_ffi::enums::RecentSortType;
pub use gtk_ffi::enums::RecentFilterFlags;
pub use gtk_ffi::enums::WidgetHelpType;
pub use gtk_ffi::enums::TextWindowType;
pub use gtk_ffi::enums::WrapMode;
pub use gtk_ffi::enums::TreeViewGridLines;
pub use gtk_ffi::enums::TreeViewColumnSizing;
pub use gtk_ffi::enums::CellRendererState;
pub use gtk_ffi::enums::TreeModelFlags;
pub use gtk_ffi::enums::IconViewDropPosition;
pub use gtk_ffi::enums::SensitivityType;
pub use gtk_ffi::enums::GType;
pub use gtk_ffi::enums::TextSearchFlags;
pub use gtk_ffi::enums::PlacesOpenFlags;
pub use gtk_ffi::enums::ToolPaletteDragTargets;
pub use gtk_ffi::enums::DestDefaults;
pub use gtk_ffi::enums::SizeGroupMode;
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
pub use self::traits::DialogButtons;
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

mod traits;
pub mod signals;
pub mod widgets;
pub mod types;

