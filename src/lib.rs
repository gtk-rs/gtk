// Copyright 2013-2015, The Gtk-rs Project Developers.
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


extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate gdk_pixbuf_sys as gdk_pixbuf_ffi;
extern crate gtk_sys as ffi;
extern crate cairo_sys as cairo_ffi;
extern crate pango_sys as pango_ffi;
#[macro_use]
extern crate glib;
extern crate gdk;
extern crate cairo;
extern crate pango;

pub use gdk::pixbuf as gdk_pixbuf;

pub use glib::ValuePublic;

pub use ffi::GtkAccelFlags as AccelFlags;
pub use ffi::GtkAlign as Align;
pub use ffi::GtkArrowPlacement as ArrowPlacement;
pub use ffi::GtkArrowType as ArrowType;
pub use ffi::GtkAttachOptions as AttachOptions;
pub use ffi::GtkBorderStyle as BorderStyle;
pub use ffi::GtkBuilderError as BuilderError;
pub use ffi::GtkButtonBoxStyle as ButtonBoxStyle;
pub use ffi::GtkButtonsType as ButtonsType;
pub use ffi::GtkCalendarDisplayOptions as CalendarDisplayOptions;
pub use ffi::GtkCellRendererState as CellRendererState;
pub use ffi::GtkCornerType as CornerType;
pub use ffi::GtkDeleteType as DeleteType;
pub use ffi::GtkDestDefaults as DestDefaults;
pub use ffi::GtkDialogFlags as DialogFlags;
pub use ffi::GtkDirectionType as DirectionType;
pub use ffi::GtkDragResult as DragResult;
pub use ffi::GtkBaselinePosition as BaselinePosition;
pub use ffi::GtkEntryIconPosition as EntryIconPosition;
pub use ffi::GtkExpanderStyle as ExpanderStyle;
pub use ffi::GtkFileChooserAction as FileChooserAction;
pub use ffi::GtkFileFilterFlags as FileFilterFlags;
pub use ffi::GtkIMPreeditStyle as IMPreeditStyle;
pub use ffi::GtkIMStatusStyle as IMStatusStyle;
pub use ffi::GtkIconSize as IconSize;
pub use ffi::GtkIconViewDropPosition as IconViewDropPosition;
pub use ffi::GtkImageType as ImageType;
pub use ffi::GtkInputHints as InputHints;
pub use ffi::GtkInputPurpose as InputPurpose;
pub use ffi::GtkJunctionSides as JunctionSides;
pub use ffi::GtkJustification as Justification;
pub use ffi::GtkLevelBarMode as LevelBarMode;
pub use ffi::GtkLicense as License;
pub use ffi::GtkMessageType as MessageType;
pub use ffi::GtkMovementStep as MovementStep;
pub use ffi::GtkNumberUpLayout as NumberUpLayout;
pub use ffi::GtkOrientation as Orientation;
pub use ffi::GtkPackType as PackType;
pub use ffi::GtkPageOrientation as PageOrientation;
pub use ffi::GtkPageSet as PageSet;
pub use ffi::GtkPathPriorityType as PathPriorityType;
pub use ffi::GtkPathType as PathType;
pub use ffi::GtkPlacesOpenFlags as PlacesOpenFlags;
pub use ffi::GtkPolicyType as PolicyType;
pub use ffi::GtkPositionType as PositionType;
pub use ffi::GtkPrintDuplex as PrintDuplex;
pub use ffi::GtkPrintPages as PrintPages;
pub use ffi::GtkPrintQuality as PrintQuality;
pub use ffi::GtkRecentFilterFlags as RecentFilterFlags;
pub use ffi::GtkRecentSortType as RecentSortType;
pub use ffi::GtkRegionFlags as RegionFlags;
pub use ffi::GtkReliefStyle as ReliefStyle;
pub use ffi::GtkResizeMode as ResizeMode;
pub use ffi::GtkResponseType as ResponseType;
pub use ffi::GtkRevealerTransitionType as RevealerTransitionType;
pub use ffi::GtkScrollStep as ScrollStep;
pub use ffi::GtkScrollType as ScrollType;
pub use ffi::GtkScrollablePolicy as ScrollablePolicy;
pub use ffi::GtkSelectionMode as SelectionMode;
pub use ffi::GtkSensitivityType as SensitivityType;
pub use ffi::GtkShadowType as ShadowType;
pub use ffi::GtkSizeGroupMode as SizeGroupMode;
pub use ffi::GtkSizeRequestMode as SizeRequestMode;
pub use ffi::GtkSortType as SortType;
pub use ffi::GtkSpinButtonUpdatePolicy as SpinButtonUpdatePolicy;
pub use ffi::GtkSpinType as SpinType;
pub use ffi::GtkStackTransitionType as StackTransitionType;
pub use ffi::GtkStateFlags as StateFlags;
pub use ffi::GtkStateType as StateType;
pub use ffi::GtkTextDirection as TextDirection;
pub use ffi::GtkTextSearchFlags as TextSearchFlags;
pub use ffi::GtkTextWindowType as TextWindowType;
pub use ffi::GtkToolPaletteDragTargets as ToolPaletteDragTargets;
pub use ffi::GtkToolbarStyle as ToolbarStyle;
pub use ffi::GtkTreeModelFlags as TreeModelFlags;
pub use ffi::GtkTreeViewColumnSizing as TreeViewColumnSizing;
pub use ffi::GtkTreeViewDropPosition as TreeViewDropPosition;
pub use ffi::GtkTreeViewGridLines as TreeViewGridLines;
pub use ffi::GtkUnit as Unit;
pub use ffi::GtkWidgetHelpType as WidgetHelpType;
pub use ffi::GtkWindowPosition as WindowPosition;
pub use ffi::GtkWindowType as WindowType;
pub use ffi::GtkWrapMode as WrapMode;

pub const DIALOG_MODAL: DialogFlags = ffi::GTK_DIALOG_MODAL;

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK;
pub const STYLE_PROVIDER_PRIORITY_THEME: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME;
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS;
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION;
pub const STYLE_PROVIDER_PRIORITY_USER: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER;

pub mod builder;

pub mod prelude;
#[macro_use]
mod rt;

mod auto;

mod about_dialog;
mod app_chooser;
mod color_chooser;
mod entry_buffer;
mod file_chooser_dialog;
mod list_store;
mod lock_button;
mod message_dialog;
mod paper_size;
mod recent_chooser_dialog;
mod recent_data;
mod recent_info;
mod rectangle;
#[cfg(target_os = "linux")]
mod socket;
mod text_attributes;
mod text_iter;
mod text_view;
mod tree_model;
mod tree_path;
mod tree_selection;
mod tree_store;
mod tree_view;
mod widget;
mod window;

pub use auto::*;

pub use builder::{Builder, Buildable};
pub use app_chooser::AppChooser;
pub use color_chooser::ColorChooser;
pub use entry_buffer::EntryBuffer;
pub use lock_button::LockButton;
pub use recent_data::RecentData;
pub use recent_info::RecentInfo;
pub use rectangle::Rectangle;
#[cfg(target_os = "linux")]
pub use socket::Socket;
pub use text_attributes::TextAttributes;
pub use widget::Widget;
pub use window::Window;

// These are/should be inlined
pub use self::rt::{
    init,
    set_initialized,
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

// ---------------- cut ----------------

/*

/// GTK Widgets for all versions
pub use self::widgets::{
    CssProvider,
    StyleContext,
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

#[cfg(gtk_3_6)]
/// GTK Widgets for versions since GTK 3.6
pub use self::widgets::{
    MenuButton,
    LevelBar,
};

#[cfg(gtk_3_10)]
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

#[cfg(gtk_3_12)]
/// GTK Widgets for versions since GTK 3.12
pub use self::widgets::{
    FlowBox,
    FlowBoxChild,
    ActionBar,
    Popover
};

/// GTK Enum types
pub use ffi::GtkAccelFlags as AccelFlags;
pub use ffi::GtkAlign as Align;
pub use ffi::GtkArrowPlacement as ArrowPlacement;
pub use ffi::GtkArrowType as ArrowType;
pub use ffi::GtkAttachOptions as AttachOptions;
pub use ffi::GtkBorderStyle as BorderStyle;
pub use ffi::GtkBuilderError as BuilderError;
pub use ffi::GtkButtonBoxStyle as ButtonBoxStyle;
pub use ffi::GtkButtonsType as ButtonsType;
pub use ffi::GtkCalendarDisplayOptions as CalendarDisplayOptions;
pub use ffi::GtkCellRendererState as CellRendererState;
pub use ffi::GtkCornerType as CornerType;
pub use ffi::GtkDeleteType as DeleteType;
pub use ffi::GtkDestDefaults as DestDefaults;
pub use ffi::GtkDialogFlags as DialogFlags;
pub use ffi::GtkDirectionType as DirectionType;
pub use ffi::GtkDragResult as DragResult;
pub use ffi::GtkEntryIconPosition as EntryIconPosition;
pub use ffi::GtkExpanderStyle as ExpanderStyle;
pub use ffi::GtkFileChooserAction as FileChooserAction;
pub use ffi::GtkFileFilterFlags as FileFilterFlags;
pub use ffi::GtkIMPreeditStyle as IMPreeditStyle;
pub use ffi::GtkIMStatusStyle as IMStatusStyle;
pub use ffi::GtkIconSize as IconSize;
pub use ffi::GtkIconViewDropPosition as IconViewDropPosition;
pub use ffi::GtkImageType as ImageType;
pub use ffi::GtkInputHints as InputHints;
pub use ffi::GtkInputPurpose as InputPurpose;
pub use ffi::GtkJunctionSides as JunctionSides;
pub use ffi::GtkJustification as Justification;
pub use ffi::GtkLevelBarMode as LevelBarMode;
pub use ffi::GtkLicense as License;
pub use ffi::GtkMessageType as MessageType;
pub use ffi::GtkMovementStep as MovementStep;
pub use ffi::GtkNumberUpLayout as NumberUpLayout;
pub use ffi::GtkOrientation as Orientation;
pub use ffi::GtkPackType as PackType;
pub use ffi::GtkPageOrientation as PageOrientation;
pub use ffi::GtkPageSet as PageSet;
pub use ffi::GtkPathPriorityType as PathPriorityType;
pub use ffi::GtkPathType as PathType;
pub use ffi::GtkPlacesOpenFlags as PlacesOpenFlags;
pub use ffi::GtkPolicyType as PolicyType;
pub use ffi::GtkPositionType as PositionType;
pub use ffi::GtkPrintPages as PrintPages;
pub use ffi::GtkRecentFilterFlags as RecentFilterFlags;
pub use ffi::GtkRecentSortType as RecentSortType;
pub use ffi::GtkRegionFlags as RegionFlags;
pub use ffi::GtkReliefStyle as ReliefStyle;
pub use ffi::GtkResizeMode as ResizeMode;
pub use ffi::GtkResponseType as ResponseType;
pub use ffi::GtkRevealerTransitionType as RevealerTransitionType;
pub use ffi::GtkScrollStep as ScrollStep;
pub use ffi::GtkScrollType as ScrollType;
pub use ffi::GtkScrollablePolicy as ScrollablePolicy;
pub use ffi::GtkSelectionMode as SelectionMode;
pub use ffi::GtkSensitivityType as SensitivityType;
pub use ffi::GtkShadowType as ShadowType;
pub use ffi::GtkSizeGroupMode as SizeGroupMode;
pub use ffi::GtkSizeRequestMode as SizeRequestMode;
pub use ffi::GtkSortType as SortType;
pub use ffi::GtkSpinButtonUpdatePolicy as SpinButtonUpdatePolicy;
pub use ffi::GtkSpinType as SpinType;
pub use ffi::GtkStackTransitionType as StackTransitionType;
pub use ffi::GtkStateFlags as StateFlags;
pub use ffi::GtkStateType as StateType;
pub use ffi::GtkTextDirection as TextDirection;
pub use ffi::GtkTextSearchFlags as TextSearchFlags;
pub use ffi::GtkTextWindowType as TextWindowType;
pub use ffi::GtkToolPaletteDragTargets as ToolPaletteDragTargets;
pub use ffi::GtkToolbarStyle as ToolbarStyle;
pub use ffi::GtkTreeModelFlags as TreeModelFlags;
pub use ffi::GtkTreeViewColumnSizing as TreeViewColumnSizing;
pub use ffi::GtkTreeViewGridLines as TreeViewGridLines;
pub use ffi::GtkUnit as Unit;
pub use ffi::GtkWidgetHelpType as WidgetHelpType;
pub use ffi::GtkWindowPosition as WindowPosition;
pub use ffi::GtkWindowType as WindowType;
pub use ffi::GtkWrapMode as WrapMode;

/// Gtk Traits
pub use self::traits::FFIWidget;
pub use self::traits::StyleProviderTrait;
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

pub use self::traits::style_provider::{
    STYLE_PROVIDER_PRIORITY_FALLBACK,
    STYLE_PROVIDER_PRIORITY_THEME,
    STYLE_PROVIDER_PRIORITY_SETTINGS,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    STYLE_PROVIDER_PRIORITY_USER
};

pub const DIALOG_MODAL: DialogFlags = ffi::GTK_DIALOG_MODAL;

/// GTK various struct
pub struct Tooltip;

#[macro_use]
mod rt;

mod macros;
mod cast;

pub mod traits;
pub mod signal;
pub mod widgets;
*/
