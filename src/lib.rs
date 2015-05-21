// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

//#![macro_use]

extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate gtk_sys as ffi;
extern crate cairo_sys as cairo_ffi;
extern crate pango_sys as pango_ffi;
extern crate glib;
extern crate gdk;
extern crate cairo;
extern crate pango;

pub use glib::ValuePublic;

pub use ffi::enums::{
    AccelFlags,
    Align,
    AppInfoCreateFlags,
    ArrowPlacement,
    ArrowType,
    AttachOptions,
    BorderStyle,
    BuilderError,
    ButtonBoxStyle,
    ButtonsType,
    CalendarDisplayOptions,
    CellRendererState,
    CornerType,
    DeleteType,
    DestDefaults,
    DialogFlags,
    DirectionType,
    DragResult,
    EntryIconPosition,
    ExpanderStyle,
    FileChooserAction,
    FileFilterFlags,
    GType,
    IMPreeditStyle,
    IMStatusStyle,
    IconSize,
    IconViewDropPosition,
    ImageType,
    InputHints,
    InputPurpose,
    JunctionSides,
    Justification,
    LevelBarMode,
    License,
    MessageType,
    MovementStep,
    NumberUpLayout,
    Orientation,
    PackType,
    PageOrientation,
    PageSet,
    PathPriorityType,
    PathType,
    PlacesOpenFlags,
    PolicyType,
    PositionType,
    PrintPages,
    RecentFilterFlags,
    RecentSortType,
    RegionFlags,
    ReliefStyle,
    ResizeMode,
    ResponseType,
    RevealerTransitionType,
    ScrollStep,
    ScrollType,
    ScrollablePolicy,
    SelectionMode,
    SensitivityType,
    ShadowType,
    SizeGroupMode,
    SizeRequestMode,
    SortType,
    SpinButtonUpdatePolicy,
    SpinType,
    StackTransitionType,
    StateFlags,
    StateType,
    TextDirection,
    TextSearchFlags,
    TextWindowType,
    ToolPaletteDragTargets,
    ToolbarStyle,
    TreeModelFlags,
    TreeViewColumnSizing,
    TreeViewGridLines,
    Unit,
    WidgetHelpType,
    WindowPosition,
    WindowType,
    WrapMode,
};

pub mod builder;
pub mod object;
pub mod prelude;
pub mod rt;
pub mod widgets;

pub use builder::{Builder, Buildable};
pub use widgets::container::Container;
pub use widgets::widget::Widget;

// ---------------- cut ----------------

/*

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
    Widget,
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
    Range,
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

#[cfg(feature = "gtk_3_6")]
/// GTK Widgets for versions since GTK 3.6
pub use self::widgets::{
    MenuButton,
    LevelBar,
};

#[cfg(feature = "gtk_3_10")]
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

#[cfg(feature = "gtk_3_12")]
/// GTK Widgets for versions since GTK 3.12
pub use self::widgets::{
    FlowBox,
    FlowBoxChild,
    ActionBar,
    Popover
};

/// GTK Enum types


/// Gtk Traits
pub use self::traits::FFIWidget;
pub use self::traits::GObjectTrait;
pub use self::traits::BoxTrait;
pub use self::traits::ActionableTrait;
pub use self::traits::AppChooserTrait;
pub use self::traits::BinTrait;
pub use self::traits::ButtonTrait;
pub use self::traits::ButtonSignals;
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
pub use self::traits::ToolButtonSignals;
pub use self::traits::ToolItemTrait;
pub use self::traits::ToolShellTrait;
pub use self::traits::WidgetTrait;
pub use self::traits::WidgetSignals;
pub use self::traits::WindowTrait;

/// GTK various struct
pub struct Tooltip;

mod macros;
mod cast;

pub mod traits;
pub mod signal;
pub mod widgets;
*/
