// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

pub use glib::Error;
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


#[macro_use]
mod rt;

mod auto;

mod app_chooser;
mod builder;
mod color_button;
mod color_chooser;
mod entry_buffer;
mod file_chooser_dialog;
mod list_store;
mod lock_button;
mod message_dialog;
mod recent_chooser_dialog;
mod recent_data;
mod recent_info;
mod rectangle;
mod requisition;
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

pub mod prelude;

pub use auto::*;

pub use app_chooser::AppChooser;
pub use builder::{Builder, Buildable};
pub use color_chooser::ColorChooser;
pub use entry_buffer::EntryBuffer;
pub use lock_button::LockButton;
pub use recent_data::RecentData;
pub use recent_info::RecentInfo;
pub use rectangle::Rectangle;
pub use requisition::Requisition;
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
    events_pending,
};

pub use self::signal::{
    idle_add,
    timeout_add,
    timeout_add_seconds,
};
