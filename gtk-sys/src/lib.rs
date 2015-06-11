// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gdk_sys as gdk_ffi;

pub mod enums;

use libc::{c_int, c_char, c_float, c_uint, c_double, c_long, c_short, c_void, c_ulong, time_t};

pub use glib_ffi::{
    gboolean, GFALSE, GTRUE, gpointer, GType, GObject, GPermission,
    GList, GSList, GError, GValue};

//pub type GtkAllocation = GdkRectangle;

#[repr(C)]
pub struct GtkWidget;
#[repr(C)]
pub struct GtkWindow;
#[repr(C)]
pub struct GtkLabel;
#[repr(C)]
pub struct GtkMisc;
#[repr(C)]
pub struct GtkButton;
#[repr(C)]
pub struct GtkBox;
#[repr(C)]
pub struct GtkOrientable;
#[repr(C)]
pub struct GtkRange;
#[repr(C)]
pub struct GtkButtonBox;
#[repr(C)]
pub struct GtkFrame;
#[repr(C)]
pub struct GtkAspectFrame;
#[repr(C)]
pub struct GtkFixed;
#[repr(C)]
pub struct GtkBin;
#[repr(C)]
pub struct GtkContainer;
#[repr(C)]
pub struct GtkFontButton;
#[repr(C)]
pub struct GtkToggleButton;
#[repr(C)]
pub struct GtkCheckButton;
#[repr(C)]
pub struct GtkMenuButton;
#[repr(C)]
pub struct GtkColorButton;
#[repr(C)]
pub struct GtkLinkButton;
#[repr(C)]
pub struct GtkAdjustment;
#[repr(C)]
pub struct GtkScaleButton;
#[repr(C)]
pub struct GtkVolumeButton;
#[repr(C)]
pub struct GtkGrid;
#[repr(C)]
pub struct GtkEntryBuffer;
#[repr(C)]
pub struct GtkEntry;
#[repr(C)]
pub struct GtkSearchEntry;
#[repr(C)]
pub struct GtkSwitch;
#[repr(C)]
pub struct GtkScale;
#[repr(C)]
pub struct GtkLevelBar;
#[repr(C)]
pub struct GtkSearchBar;
#[repr(C)]
pub struct GtkSpinButton;
#[repr(C)]
pub struct GtkSpinner;
#[repr(C)]
pub struct GtkImage;
#[repr(C)]
pub struct GtkProgressBar;
#[repr(C)]
pub struct GtkArrow;
#[repr(C)]
pub struct GtkCalendar;
#[repr(C)]
pub struct GtkAlignment;
#[repr(C)]
pub struct GtkExpander;
#[repr(C)]
pub struct GtkPaned;
#[repr(C)]
pub struct GtkInfoBar;
#[repr(C)]
pub struct GtkToolShell;
#[repr(C)]
pub struct GtkToolbar;
#[repr(C)]
pub struct GtkDialog;
#[repr(C)]
pub struct GtkAboutDialog;
#[repr(C)]
pub struct GtkMessageDialog;
#[repr(C)]
pub struct GtkAppChooserDialog;
#[repr(C)]
pub struct GtkColorChooserDialog;
#[repr(C)]
pub struct GtkFileChooserDialog;
#[repr(C)]
pub struct GtkFileChooser;
#[repr(C)]
pub struct GtkNotebook;
#[repr(C)]
pub struct GtkStack;
#[repr(C)]
pub struct GtkStackSwitcher;
#[repr(C)]
pub struct GtkRevealer;
#[repr(C)]
pub struct GtkOverlay;
#[repr(C)]
pub struct GtkScrollable;
#[repr(C)]
pub struct GtkLayout;
#[repr(C)]
pub struct GtkHeaderBar;
#[repr(C)]
pub struct GtkFlowBox;
#[repr(C)]
pub struct GtkFlowBoxChild;
#[repr(C)]
pub struct GtkListBox;
#[repr(C)]
pub struct GtkListBoxRow;
#[repr(C)]
pub struct GtkActionBar;
#[repr(C)]
pub struct GtkFileFilter;
#[repr(C)]
pub struct GtkAppChooser;
#[repr(C)]
pub struct GAppLaunchContext;
#[repr(C)]
pub struct GAppInfo;
#[repr(C)]
pub struct GtkFontChooser;
#[repr(C)]
pub struct GtkFontChooserDialog;
#[repr(C)]
pub struct GtkBuildable;
//pub struct GtkPageSetupUnixDialog;
#[repr(C)]
pub struct GtkPrintSettings;
#[repr(C)]
pub struct GtkPageSetup;
#[repr(C)]
pub struct GtkPaperSize;
#[repr(C)]
pub struct GtkRecentData {
    pub display_name: *mut c_char,
    pub description: *mut c_char,
    pub mime_type: *mut c_char,
    pub app_name: *mut c_char,
    pub app_exec: *mut c_char,
    pub groups: *mut *mut c_char,
    pub is_private: gboolean
}
#[repr(C)]
pub struct GtkRecentInfo;
#[repr(C)]
pub struct GtkRecentFilter;
#[repr(C)]
pub struct GtkRecentFilterInfo {
    pub contains: enums::RecentFilterFlags,
    pub uri: *const c_char,
    pub display_name: *const c_char,
    pub mime_type: *const c_char,
    pub applications: *const *const c_char,
    pub groups: *const *const c_char,
    pub age: c_int
}
#[repr(C)]
pub struct GtkRecentManager;
#[repr(C)]
pub struct GtkRecentChooser;
#[repr(C)]
pub struct GtkRecentChooserDialog;
#[repr(C)]
pub struct GtkCellRendererClass;
#[repr(C)]
pub struct GtkCellRenderer;
#[repr(C)]
pub struct GtkCellLayout;
#[repr(C)]
pub struct GtkTreeModel;
#[repr(C)]
pub struct GtkListStore;
#[repr(C)]
pub struct GtkTreeStore;
#[repr(C)]
pub struct GtkTreePath;
#[repr(C)]
pub struct GtkTreeIter {
    pub stamp: c_int,
    pub user_data: *mut c_void,
    pub user_data2: *mut c_void,
    pub user_data3: *mut c_void
}
#[repr(C)]
pub struct GtkCellEditable;
#[repr(C)]
pub struct GtkCellRendererText;
#[repr(C)]
pub struct GtkCellRendererToggle;
#[repr(C)]
pub struct GtkToolItem;
#[repr(C)]
pub struct GtkToolItemGroup;
#[repr(C)]
pub struct GtkToolPalette;
#[repr(C)]
pub struct GtkToolButton;
#[repr(C)]
pub struct GtkMenuToolButton;
#[repr(C)]
pub struct GtkToggleToolButton;
#[repr(C)]
pub struct GtkRadioToolButton;
#[repr(C)]
pub struct GtkSeparatorToolItem;
#[repr(C)]
pub struct GtkMenu;
#[repr(C)]
pub struct GMenuModel;
#[repr(C)]
pub struct GClosure;
#[repr(C)]
pub struct GtkColorChooser;
#[repr(C)]
pub struct GtkEditable;
#[repr(C)]
pub struct GtkRadioButton;
// not useful to implement for the moment
#[repr(C)]
pub struct GtkBuilder;
#[repr(C)]
pub struct GtkTextTag;
#[repr(C)]
pub struct GtkTextAttributes;
#[repr(C)]
pub struct GtkTextView;
#[repr(C)]
pub struct GtkTextBuffer;
#[repr(C)]
pub struct GtkTextTagTable;
#[repr(C)]
pub struct GtkScrolledWindow;
#[repr(C)]
pub struct GtkPlacesSidebar;
#[repr(C)]
pub struct GtkIconSize;
#[repr(C)]
pub struct GtkTextMark;
#[repr(C)]
pub struct GtkTextIter;
#[repr(C)]
pub struct GtkTextChildAnchor;
#[repr(C)]
pub struct GtkTreeView;
#[repr(C)]
pub struct GtkTreeViewColumn;
#[repr(C)]
pub struct GtkMenuShell;
#[repr(C)]
pub struct GtkMenuItem;
#[repr(C)]
pub struct GtkCheckMenuItem;
#[repr(C)]
pub struct GtkViewport;
#[repr(C)]
pub struct GtkStatusbar;
#[repr(C)]
pub struct GtkLockButton;
#[repr(C)]
pub struct GtkActionable;
#[repr(C)]
pub struct GtkEntryCompletion;
#[repr(C)]
pub struct GtkIconView;
#[repr(C)]
pub struct GtkTreeSelection;
#[repr(C)]
pub struct GtkRecentChooserWidget;
#[repr(C)]
pub struct GtkComboBox;
#[repr(C)]
pub struct GtkComboBoxText;
#[repr(C)]
pub struct GtkPopover;
#[repr(C)]
pub struct GtkAppChooserWidget;
#[repr(C)]
pub struct GtkSizeGroup;
#[repr(C)]
pub struct GtkFileChooserWidget;
#[repr(C)]
pub struct GtkColorChooserWidget;
#[repr(C)]
pub struct GtkFontChooserWidget;
#[repr(C)]
pub struct GtkMovementStep;
#[repr(C)]
pub struct GtkSocket;
#[repr(C)]
pub struct GtkEventBox;
#[repr(C)]
pub struct GtkTooltip;

//pub type GtkTreeModelForeachFunc = fn(model: *mut GtkTreeModel, path: *mut GtkTreePath, iter: *mut GtkTreeIter, data: gpointer) -> gboolean;

extern "C" {

    //=========================================================================
    // Gtk Main Loop + events
    //=========================================================================
    pub fn gtk_init                            (argc: *const c_int, argv: *const *const *const c_char) -> ();
    pub fn gtk_init_check                      (argc: *const c_int, argv: *const *const *const c_char) -> gboolean;
    pub fn gtk_main                            () -> ();
    pub fn gtk_main_quit                       () -> ();
    pub fn gtk_main_level                      () -> c_uint;
    pub fn gtk_main_iteration                  () -> gboolean;
    pub fn gtk_main_iteration_do               (blocking: gboolean) -> gboolean;
    pub fn gtk_events_pending                  () -> gboolean;

    //=========================================================================
    // GtkWindow
    //=========================================================================
    pub fn gtk_window_get_type                 () -> GType;
    pub fn gtk_window_new                      (wtype : enums::WindowType) -> *mut GtkWidget;
    pub fn gtk_window_set_title                (window: *mut GtkWindow, title: *const c_char) -> ();
    pub fn gtk_window_get_title                (window: *mut GtkWindow) -> *const c_char;
    pub fn gtk_window_set_default_size         (widget: *mut GtkWindow, width: c_int, height: c_int);
    pub fn gtk_window_set_position             (window: *mut GtkWindow, position: enums::WindowPosition) -> ();
    pub fn gtk_window_set_decorated            (window: *mut GtkWindow, setting: gboolean) -> ();
    pub fn gtk_window_set_titlebar             (window: *mut GtkWindow, titlebar: *mut GtkWidget) -> ();
    pub fn gtk_window_set_type_hint            (window: *mut GtkWindow, hint: gdk_ffi::enums::WindowTypeHint);
    pub fn gtk_window_move                     (window: *mut GtkWindow, x: c_int, y: c_int);

    // pub fn gtk_window_set_role(window: *const const GtkWindow, role: *const c_char) -> ();
    // pub fn gtk_window_set_startup_id(window: *const const GtkWindow, startup_id: *const c_char) -> ();
    // pub fn gtk_window_get_role(window: *const const GtkWindow) -> *const c_char;
    // pub fn gtk_window_add_accel_group(window: *const const GtkWindow, accel_group: *GtkAccelGroup) -> ();
    // pub fn gtk_window_remove_accel_group(window: *const const GtkWindow, accel_group: *GtkAccelGroup) -> ();
    // pub fn gtk_window_activate_focus(window: *const const GtkWindow) -> gboolean;
    // pub fn gtk_window_set_focus(window: *const const GtkWindow, focus: *const const GtkWidget) -> ();
    // pub fn gtk_window_get_focus(window: *const const GtkWindow) -> *const const GtkWidget;
    // pub fn gtk_window_set_default(window: *const const GtkWindow, default_widget: *const const GtkWidget) -> ();
    // pub fn gtk_window_get_default_widget(window: *const const GtkWindow) -> *const const GtkWidget;
    // pub fn gtk_window_activate_default(window: *const const GtkWindow) -> gboolean;
    // pub fn gtk_window_get_type() -> ();

    //=========================================================================
    // GType
    //=========================================================================
    pub fn g_type_name                         (_type: GType) -> *const c_char;
    pub fn g_type_from_name                    (name: *const c_char) -> GType;
    pub fn g_type_parent                       (_type: GType) -> GType;
    pub fn g_type_depth                        (_type: GType) -> c_uint;
    pub fn g_type_next_base                    (leaf_type: GType, root_type: GType) -> GType;
    pub fn g_type_is_a                         (_type: GType, is_a_type: GType) -> gboolean;
    pub fn g_type_children                     (_type: GType, n_children: *mut c_uint) -> *mut GType;
    pub fn g_type_interfaces                   (_type: GType, n_interfaces: *mut c_uint) -> *mut GType;
    pub fn g_type_interface_prerequisites      (interface_type: GType, n_prerequisites: *mut c_uint) -> *mut GType;
    pub fn g_type_interface_add_prerequisite   (interface_type: GType, prerequisite_type: GType);
    pub fn g_type_fundamental_next             () -> GType;
    pub fn g_type_fundamental                  (type_id: GType) -> GType;
    pub fn g_type_ensure                       (_type: GType);
    pub fn g_type_get_type_registration_serial () -> c_uint;

    //=========================================================================
    // GtkWidget                                                         NOT OK
    //=========================================================================
    pub fn gtk_widget_get_type                 () -> GType;
    //pub fn gtk_widget_new                      (type: GType, first_property_name: *const c_char, ...) -> *mut GtkWidget;
    pub fn gtk_widget_destroyed                (widget: *mut GtkWidget, widget_pointer: *mut *mut GtkWidget);
    pub fn gtk_widget_show                     (widget: *mut GtkWidget);
    pub fn gtk_widget_show_now                 (widget: *mut GtkWidget);
    pub fn gtk_widget_show_all                 (widget: *mut GtkWidget);
    pub fn gtk_widget_hide                     (widget: *mut GtkWidget);
    pub fn gtk_widget_map                      (widget: *mut GtkWidget);
    pub fn gtk_widget_unmap                    (widget: *mut GtkWidget);
    pub fn gtk_widget_realize                  (widget: *mut GtkWidget);
    pub fn gtk_widget_unrealize                (widget: *mut GtkWidget);
    //pub fn gtk_widget_draw                     (widget: *mut GtkWidget, cr: *mut cairo_t);
    pub fn gtk_widget_queue_draw               (widget: *mut GtkWidget);
    pub fn gtk_widget_queue_resize             (widget: *mut GtkWidget);
    pub fn gtk_widget_queue_resize_no_redraw   (widget: *mut GtkWidget);
    //pub fn gtk_widget_get_frame_clock          (widget: *mut GtkWidget) -> *mut GdkFrameClock;
    //pub fn gtk_widget_add_tick_callback        (widget: *mut GtkWidget, callback: GtkTickCallback, user_data: gpointer, notify: GDestroyNotify) -> c_uint;
    //pub fn gtk_widget_remove_tick_callback     (widget: *mut GtkWidget, id: c_uint);
    pub fn gtk_widget_get_scale_factor         (widget: *mut GtkWidget) -> c_int;
    //pub fn gtk_widget_size_request             (widget: *mut GtkWidget, requisition: *mut GtkRequisition);
    //pub fn gtk_widget_get_child_requisition    (widget: *mut GtkWidget, requisition: *mut GtkRequisition);
    //pub fn gtk_widget_size_allocate            (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
    //pub fn gtk_widget_size_allocate_with_baseline(widget: *mut GtkWidget, allocation: *mut GtkAllocation, baseline: c_int);
    //pub fn gtk_widget_add_accelerator          (widget: *mut GtkWidget, accel_signal: *const c_char, accel_group: *mut GtkAccelGroup,
        //accel_key: c_uint, accel_mods: gdk::ModifierType, accel_flags: enums::AccelFlags);
    //pub fn gtk_widget_remove_accelerator       (widget: *mut GtkWidget, accel_group: *mut GtkAccelGroup, accel_key: c_uint, accel_mods: gdk::ModifierType) -> gboolean;
    //pub fn gtk_widget_set_accel_path           (widget: *mut GtkWidget, accel_path: *const c_char, accel_group: *mut GtkAccelGroup);
    //pub fn gtk_widget_list_accel_closures      (widget: *mut GtkWidget) -> *mut glib::GList;
    //pub fn gtk_widget_can_activate_accel       (widget: *mut GtkWidget, signal_id: c_uint) -> gboolean;
    //pub fn gtk_widget_event                    (widget: *mut GtkWidget, event: *mut gdk::Event) -> gboolean;
    pub fn gtk_widget_activate                 (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_reparent                 (widget: *mut GtkWidget, new_parent: *mut GtkWidget);
    //pub fn gtk_widget_intersect                (widget: *mut GtkWidget, area: *const GdkRectangle, intersection: *mut GdkRectangle) -> gboolean;
    pub fn gtk_widget_is_focus                 (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_grab_focus               (widget: *mut GtkWidget);
    pub fn gtk_widget_grab_default             (widget: *mut GtkWidget);
    pub fn gtk_widget_set_name                 (widget: *mut GtkWidget, name: *const c_char);
    pub fn gtk_widget_get_name                 (widget: *mut GtkWidget) -> *const c_char;
    pub fn gtk_widget_set_sensitive            (widget: *mut GtkWidget, sensitive: gboolean);
    pub fn gtk_widget_set_parent               (widget: *mut GtkWidget, parent: *mut GtkWidget);
    pub fn gtk_widget_set_parent_window        (widget: *mut GtkWidget, parent_window: *mut gdk_ffi::GdkWindow);
    //pub fn gtk_widget_get_parent_window        (widget: *mut GtkWindow) -> *mut gdk::Window;
    //pub fn gtk_widget_set_events               (widget: *mut GtkWidget, events: c_int);
    //pub fn gtk_widget_get_events               (widget: *mut GtkWidget) -> c_int;
    //pub fn gtk_widget_add_events               (widget: *mut GtkWidget, events: c_int);
    //pub fn gtk_widget_set_device_events        (widget: *mut GtkWidget, device: *mut GdkDevice, events: gdk::EventMask);
    //pub fn gtk_widget_get_device_events        (widget: *mut GtkWidget, device: *mut GdkDevice) -> gdk::EventMask;
    //pub fn gtk_widget_add_device_events        (widget: *mut GtkWidget, device: *mut GdkDevice, events: gdk::EventMask);
    //pub fn gtk_widget_set_device_enabled       (widget: *mut GtkWidget, device: *mut GdkDevice, enabled: gboolean);
    //pub fn gtk_widget_get_device_enabled       (widget: *mut GtkWidget, device: *mut GdkDevice) -> gboolean;
    pub fn gtk_widget_get_toplevel             (widget: *mut GtkWidget) -> *mut GtkWidget;
    pub fn gtk_widget_get_ancestor             (widget: *mut GtkWidget, widget_type: GType) -> *mut GtkWidget;
    //pub fn gtk_widget_get_visual               (widget: *mut GtkWidget) -> *mut GdkVisual;
    //pub fn gtk_widget_set_visual               (widget: *mut GtkWidget, visual: *mut GdkVisual);
    pub fn gtk_widget_is_ancestor              (widget: *mut GtkWidget, ancestor: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_translate_coordinates    (widget: *mut GtkWidget, dest_widget: *mut GtkWidget, src_x: c_int, src_y: c_int,
        dest_x: *mut c_int, dest_y: *mut c_int) -> gboolean;
    pub fn gtk_widget_hide_on_delete           (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_direction            (widget: *mut GtkWidget, dir: enums::TextDirection);
    pub fn gtk_widget_get_direction            (widget: *mut GtkWidget) -> enums::TextDirection;
    pub fn gtk_widget_set_default_direction    (dir: enums::TextDirection);
    pub fn gtk_widget_get_default_direction    () -> enums::TextDirection;
    //pub fn gtk_widget_shape_combine_region     (widget: *mut GtkWidget, region: *mut cairo_region_t);
    //pub fn gtk_widget_input_shape_combine_region(widget: *mut GtkWidget, region: *mut cairo_region_t);
    pub fn gtk_widget_override_background_color(widget: *mut GtkWidget, state: enums::StateFlags, color: *const gdk_ffi::GdkRGBA);
    pub fn gtk_widget_override_color           (widget: *mut GtkWidget, state: enums::StateFlags, color: *const gdk_ffi::GdkRGBA);
    //pub fn gtk_widget_override_font            (widget: *mut GtkWidget, font_desc: *const PangoFontDescription);
    pub fn gtk_widget_override_symbolic_color  (widget: *mut GtkWidget, name: *const c_char, color: *const gdk_ffi::GdkRGBA);
    pub fn gtk_widget_override_cursor          (widget: *mut GtkWidget, cursor: *const gdk_ffi::GdkRGBA, secondary_cursor: *const gdk_ffi::GdkRGBA);
    //pub fn gtk_widget_create_pango_context     (widget: *mut GtkWidget) -> *mut PangoContext;
    //pub fn gtk_widget_get_pango_context        (widget: *mut GtkWidget) -> *mut PangoContext;
    //pub fn gtk_widget_create_pango_layout      (widget: *mut GtkWidget, name: *const c_char) -> *mut PangoLayout;
    pub fn gtk_widget_queue_draw_area          (widget: *mut GtkWidget, x: c_int, y: c_int, width: c_int, height: c_int);
    //pub fn gtk_widget_queue_draw_region        (widget: *mut GtkWidget, region: *const cairo_region_t);
    pub fn gtk_widget_set_app_paintable        (widget: *mut GtkWidget, app_paintable: gboolean);
    pub fn gtk_widget_set_double_buffered      (widget: *mut GtkWidget, double_buffered: gboolean);
    pub fn gtk_widget_set_redraw_on_allocate   (widget: *mut GtkWidget, redraw_on_allocate: gboolean);
    pub fn gtk_widget_mnemonic_activate        (widget: *mut GtkWidget, group_cycling: gboolean) -> gboolean;
    //pub fn gtk_widget_class_install_style_property(klass: *mut GtkWidgetClass, pspec: *mut GParamSpec);
    //pub fn gtk_widget_class_install_style_property_parser(klass: *mut GtkWidgetClass, pspec: *mut GParamSpec, parser: *mut GtkRcPropertyParser);
    //pub fn gtk_widget_class_find_style_property(klass: *mut GtkWidgetClass, property_name: *const c_char) -> *mut GParamSpec;
    //pub fn gtk_widget_class_list_style_properties(klass: *mut GtkWidgetClass, n_properties: c_uint) -> *mut *mut GParamSpec;
    //pub fn gtk_widget_region_intersect         (widget: *mut GtkWidget, region: *const cairo_region_t) -> *mut cairo_region_t;
    //pub fn gtk_widget_send_expose              (widget: *mut GtkWidget, event: *mut gdk::Event) -> c_int;
    //pub fn gtk_widget_send_focus_change        (widget: *mut GtkWidget, event: *mut gdk::Event) -> gboolean;
    //pub fn gtk_widget_style_get                (widget: *mut GtkWidget, first_property_name: *const c_char, ...);
    //pub fn gtk_widget_style_get_property       (widget: *mut GtkWidget, property_name: *const c_char, value: *mut GValue);
    //pub fn gtk_widget_style_get_valist         (widget: *mut GtkWidget, first_property_name: *const c_char, va_args: va_list);
    //pub fn gtk_widget_class_set_accessible_type(widget_class: *mut GtkWidgetClass, _type: GType);
    //pub fn gtk_widget_class_set_accessible_role(widget_class: *mut GtkWidgetClass, role: AtkRole);
    //pub fn gtk_widget_class_get_accessible     (widget_class: *mut GtkWidgetClass) -> *mut AtkObject;
    pub fn gtk_widget_child_focus              (widget: *mut GtkWidget, direction: enums::DirectionType) -> gboolean;
    pub fn gtk_widget_child_notify             (widget: *mut GtkWidget, child_property: *const c_char);
    pub fn gtk_widget_freeze_child_notify      (widget: *mut GtkWidget);
    pub fn gtk_widget_get_child_visible        (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_get_parent               (widget: *mut GtkWidget) -> *mut GtkWidget;
    //pub fn gtk_widget_get_settings             (widget: *mut GtkWidget) -> *mut GtkSettings;
    //pub fn gtk_widget_get_clipboard            (widget: *mut GtkWidget, selection: gdk::Atom) -> *mut GtkClipboard;
    //pub fn gtk_widget_get_display              (widget: *mut GtkWidget) -> *mut gdk::Display;
    //pub fn gtk_widget_get_screen               (widget: *mut GtkWidget) -> *mut gdk::Screen;
    pub fn gtk_widget_has_screen               (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_get_size_request         (widget: *mut GtkWidget, width: *mut c_int, height: *mut c_int);
    pub fn gtk_widget_set_child_visible        (widget: *mut GtkWidget, is_visible: gboolean);
    pub fn gtk_widget_set_size_request         (widget: *mut GtkWidget, width: c_int, height: c_int);
    pub fn gtk_widget_thaw_child_notify        (widget: *mut GtkWidget);
    pub fn gtk_widget_set_no_show_all          (widget: *mut GtkWidget, no_show_all: gboolean);
    pub fn gtk_widget_get_no_show_all          (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_list_mnemonic_labels     (widget: *mut GtkWidget) -> *mut GList;
    pub fn gtk_widget_add_mnemonic_label       (widget: *mut GtkWidget, label: *mut GtkWidget);
    pub fn gtk_widget_remove_mnemonic_label    (widget: *mut GtkWidget, label: *mut GtkWidget);
    pub fn gtk_widget_is_composited            (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_error_bell               (widget: *mut GtkWidget);
    pub fn gtk_widget_keynav_failed            (widget: *mut GtkWidget, direction: enums::DirectionType) -> gboolean;
    pub fn gtk_widget_get_tooltip_markup       (widget: *mut GtkWidget) -> *mut c_char;
    pub fn gtk_widget_set_tooltip_markup       (widget: *mut GtkWidget, markup: *mut c_char);
    pub fn gtk_widget_get_tooltip_text         (widget: *mut GtkWidget) -> *mut c_char;
    pub fn gtk_widget_set_tooltip_text         (widget: *mut GtkWidget, text: *mut c_char);
    //pub fn gtk_widget_get_tooltip_window       (widget: *mut GtkWidget) -> *mut GtkWindow;
    //pub fn gtk_widget_set_tooltip_window       (widget: *mut GtkWidget, custom_window: *mut GtkWindow);
    pub fn gtk_widget_get_has_tooltip          (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_has_tooltip          (widget: *mut GtkWidget, has_tooltip: gboolean);
    pub fn gtk_widget_trigger_tooltip_query    (widget: *mut GtkWidget);
    pub fn gtk_widget_get_window               (widget: *mut GtkWidget) -> *mut gdk_ffi::GdkWindow;
    //pub fn gtk_widget_register_window          (widget: *mut GtkWidget, window: *mut GtkWindow);
    //pub fn gtk_widget_unregister_window        (widget: *mut GtkWidget, window: *mut GtkWindow);
    //pub fn gtk_cairo_should_draw_window        (cr: *mut cairo_t, window: *mut GtkWindow);
    //pub fn gtk_cairo_transform_to_window       (cr: *mut cairo_t, widget: *mut GtkWidget, window: *mut GtkWindow);
    pub fn gtk_widget_get_allocated_width      (widget: *mut GtkWidget) -> c_int;
    pub fn gtk_widget_get_allocated_height     (widget: *mut GtkWidget) -> c_int;
    //pub fn gtk_widget_get_allocation           (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
    //pub fn gtk_widget_set_allocation           (widget: *mut GtkWidget, allocation: *const GtkAllocation);
    pub fn gtk_widget_get_allocated_baseline   (widget: *mut GtkWidget) -> c_int;
    pub fn gtk_widget_get_app_paintable        (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_get_can_default          (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_can_default          (widget: *mut GtkWidget, can_default: gboolean);
    pub fn gtk_widget_get_can_focus            (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_can_focus            (widget: *mut GtkWidget, can_focus: gboolean);
    pub fn gtk_widget_get_double_buffered      (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_get_has_window           (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_has_window           (widget: *mut GtkWidget, has_window: gboolean);
    pub fn gtk_widget_get_sensitive            (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_is_sensitive             (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_get_visible              (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_is_visible               (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_visible              (widget: *mut GtkWidget, visible: gboolean);
    pub fn gtk_widget_set_state_flags          (widget: *mut GtkWidget, flags: enums::StateFlags, clear: gboolean);
    pub fn gtk_widget_unset_state_flags        (widget: *mut GtkWidget, flags: enums::StateFlags);
    pub fn gtk_widget_get_state_flags          (widget: *mut GtkWidget) -> enums::StateFlags;
    pub fn gtk_widget_has_default              (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_has_focus                (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_has_visible_focus        (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_has_grab                 (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_is_drawable              (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_is_toplevel              (widget: *mut GtkWidget) -> gboolean;
    //pub fn gtk_widget_set_window               (widget: *mut GtkWidget, window: *mut gdk::GdkWindow) -> gboolean;
    pub fn gtk_widget_set_receives_default     (widget: *mut GtkWidget, receives_default: gboolean);
    pub fn gtk_widget_get_receives_default     (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_support_multidevice  (widget: *mut GtkWidget, support_multidevice: gboolean);
    pub fn gtk_widget_get_support_multidevice  (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_realized             (widget: *mut GtkWidget, realized: gboolean);
    pub fn gtk_widget_get_realized             (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_mapped               (widget: *mut GtkWidget, mapped: gboolean);
    pub fn gtk_widget_get_mapped               (widget: *mut GtkWidget) -> gboolean;
    //pub fn gtk_widget_device_is_shadowed       (widget: *mut GtkWidget, device: *mut GdkDevice) -> gboolean;
    pub fn gtk_widget_get_modifier_mask        (widget: *mut GtkWidget, intent: gdk_ffi::enums::modifier_intent::ModifierIntent) -> gdk_ffi::enums::modifier_type::ModifierType;
    //pub fn gtk_widget_insert_action_group      (widget: *mut GtkWidget, name: *mut c_char, group: *mut GActionGroup);
    pub fn gtk_widget_get_opacity              (widget: *mut GtkWidget) -> c_double;
    pub fn gtk_widget_set_opacity              (widget: *mut GtkWidget, opacity: c_double);
    //pub fn gtk_widget_get_path                 (widget: *mut GtkWidget) -> *mut GtkWidgetPath;
    //pub fn gtk_widget_get_style_context        (widget: *mut GtkWidget) -> *mut GtkStyleContext;
    pub fn gtk_widget_reset_style              (widget: *mut GtkWidget);
    //pub fn gtk_requisition_new                 () -> *mut GtkRequisition;
    //pub fn gtk_requisition_copy                (requisition: *const GtkRequisition) -> *mut GtkRequisition;
    //pub fn gtk_requisition_free                (requisition: *mut GtkRequisition);
    pub fn gtk_widget_get_preferred_height     (widget: *mut GtkWidget, minimum_height: *mut c_int, natural_height: *mut c_int);
    pub fn gtk_widget_get_preferred_width      (widget: *mut GtkWidget, minimum_width: *mut c_int, natural_width: *mut c_int);
    pub fn gtk_widget_get_preferred_height_for_width(widget: *mut GtkWidget, width: c_int, minimum_height: *mut c_int, natural_height: *mut c_int);
    pub fn gtk_widget_get_preferred_width_for_height(widget: *mut GtkWidget, height: c_int, minimum_width: *mut c_int, natural_width: *mut c_int);
    pub fn gtk_widget_get_preferred_height_and_baseline_for_width(widget: *mut GtkWidget, width: c_int, minimum_height: *mut c_int,
        natural_height: *mut c_int, minimum_baseline: *mut c_int, natural_baseline: *mut c_int);
    pub fn gtk_widget_get_request_mode         (widget: *mut GtkWidget) -> enums::SizeRequestMode;
    //pub fn gtk_widget_get_preferred_size       (widget: *mut GtkWidget, minimum_size: *mut GtkRequisition, natural_size: *mut GtkRequisition);
    //pub fn gtk_distribute_natural_allocation   (extra_space: c_int, n_requested_sizes: c_uint, sizes: *mut GtkRequestedSizes) -> c_int;
    pub fn gtk_widget_get_halign               (widget: *mut GtkWidget) -> enums::Align;
    pub fn gtk_widget_set_halign               (widget: *mut GtkWidget, align: enums::Align);
    pub fn gtk_widget_get_valign               (widget: *mut GtkWidget) -> enums::Align;
    pub fn gtk_widget_get_valign_with_baseline (widget: *mut GtkWidget) -> enums::Align;
    pub fn gtk_widget_set_valign               (widget: *mut GtkWidget, align: enums::Align);
    pub fn gtk_widget_get_margin_start         (widget: *mut GtkWidget) -> c_int;
    pub fn gtk_widget_set_margin_start         (widget: *mut GtkWidget, margin: c_int);
    pub fn gtk_widget_get_margin_end           (widget: *mut GtkWidget) -> c_int;
    pub fn gtk_widget_set_margin_end           (widget: *mut GtkWidget, margin: c_int);
    pub fn gtk_widget_get_margin_top           (widget: *mut GtkWidget) -> c_int;
    pub fn gtk_widget_set_margin_top           (widget: *mut GtkWidget, margin: c_int);
    pub fn gtk_widget_get_margin_bottom        (widget: *mut GtkWidget) -> c_int;
    pub fn gtk_widget_set_margin_bottom        (widget: *mut GtkWidget, margin: c_int);
    pub fn gtk_widget_get_hexpand              (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_hexpand              (widget: *mut GtkWidget, expand: gboolean);
    pub fn gtk_widget_get_hexpand_set          (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_hexpand_set          (widget: *mut GtkWidget, expand: gboolean);
    pub fn gtk_widget_get_vexpand              (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_vexpand              (widget: *mut GtkWidget, expand: gboolean);
    pub fn gtk_widget_get_vexpand_set          (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_set_vexpand_set          (widget: *mut GtkWidget, expand: gboolean);
    pub fn gtk_widget_queue_compute_expand     (widget: *mut GtkWidget);
    pub fn gtk_widget_compute_expand           (widget: *mut GtkWidget, orientation: enums::Orientation) -> gboolean;
    pub fn gtk_widget_init_template            (widget: *mut GtkWidget);
    //pub fn gtk_widget_class_set_template       (widget_class: *mut GtkWidgetClass, template_bytes: *mut GBytes);
    //pub fn gtk_widget_class_set_template_from_resource(widget_class: *mut GtkWidgetClass, resource_name: *const c_char);
    //pub fn gtk_widget_get_template_child       (widget: *mut GtkWidget, widget_type: GType, name: *const c_char) -> *mut GObject;
    //pub fn gtk_widget_class_bind_template_child_full(widget_class: *mut GtkWidgetClass, name: *const c_char, internal_child: gboolean, struct_offset: gssize);
    //pub fn gtk_widget_class_bind_template_callback_full(widget_class: *mut GtkWidgetClass, callback_name: *const c_char, callback_symbol: GCallback);
    //pub fn gtk_widget_class_set_connect_func   (widget_class: *mut GtkWidgetClass, connect_func: GtkBuilderConnectFunc,
        //connect_data: gpointer, connect_data_destroy: GDestroyNotify);

    pub fn gtk_widget_destroy                  (widget: *mut GtkWidget);
    pub fn gtk_widget_in_destruction           (widget: *mut GtkWidget) -> gboolean;
    pub fn gtk_widget_unparent                 (widget: *mut GtkWidget) -> ();

    //----------
    //- MACROS -
    //----------
    //gtk_widget_class_bind_template_child(widget_class, TypeName, member_name) -> gtk_widget_class_bind_template_child_full
    //gtk_widget_class_bind_template_child_internal(widget_class, TypeName, member_name) -> gtk_widget_class_bind_template_child_full
    //gtk_widget_class_bind_template_child_private(widget_class, TypeName, member_name) -> gtk_widget_class_bind_template_child_full
    //gtk_widget_class_bind_template_child_internal_private(widget_class, TypeName, member_name) -> gtk_widget_class_bind_template_child_full
    //gtk_widget_class_bind_template_callback(widget_class, callback) -> gtk_widget_class_bind_template_callback_full

    //-----------
    //-CALLBACKS-
    //-----------
    //pub fn GtkTickCallback                     (widget: *mut GtkWidget, frame_clock: *mut GdkFrameClock, user_data: gpointer) -> gboolean;

    //=========================================================================
    // GtkLabel                                                          NOT OK
    //=========================================================================
    pub fn gtk_label_new                       (text: *const c_char) -> *mut GtkWidget;
    pub fn gtk_label_set_label                 (label: *mut GtkLabel, text: *const c_char);
    pub fn gtk_label_set_justify               (label: *mut GtkLabel, jtype: enums::Justification);
    pub fn gtk_label_set_text                  (label: *mut GtkLabel, stext: *const c_char) -> ();
    // pub fn gtk_label_set_attributes            (label: *const const GtkLabel, PangoAttrList *attrs) -> ();
    pub fn gtk_label_set_markup                (label: *mut GtkLabel, text: *const c_char) -> ();
    pub fn gtk_label_set_markup_with_mnemonic  (label: *mut GtkLabel, text: *const c_char) -> ();
    pub fn gtk_label_set_pattern               (label: *mut GtkLabel, text: *const c_char) -> ();
    // pub fn gtk_label_set_ellipsize             (label: *const const GtkLabel, PangoEllipsizeMode mode) -> ();
    pub fn gtk_label_set_width_chars           (label: *mut GtkLabel, n_chars: c_int) -> ();
    pub fn gtk_label_set_max_width_chars       (label: *mut GtkLabel, n_chars: c_int) -> ();
    pub fn gtk_label_set_line_wrap             (label: *mut GtkLabel, wrap: gboolean) -> ();
    // pub fn gtk_label_set_line_wrap_mode        (label: *const const GtkLabel, PangoWrapMode wrap_mode);
    pub fn gtk_label_set_lines                 (label: *mut GtkLabel, lines: c_int) -> ();
    pub fn gtk_label_get_layout_offsets        (label: *mut GtkLabel, x: *const c_int, y: *const c_int) -> ();
    pub fn gtk_label_get_mnemonic_keyval       (label: *mut GtkLabel) -> c_uint;
    pub fn gtk_label_get_selectable            (label: *mut GtkLabel) -> gboolean;
    pub fn gtk_label_get_text                  (label: *mut GtkLabel) -> *const c_char;
    pub fn gtk_label_new_with_mnemonic         (text: *const c_char) -> *mut GtkWidget;
    pub fn gtk_label_select_region             (label: *mut GtkLabel, start_offset: c_int, end_offset: c_int) -> ();
    // pub fn gtk_label_set_mnemonic_widget       (label: *const const GtkLabel, widget: *const const GtkWidget) -> ();
    pub fn gtk_label_set_selectable            (label: *mut GtkLabel, gsetting: gboolean) -> ();
    pub fn gtk_label_set_text_with_mnemonic    (label: *mut GtkLabel, text: *const c_char) -> ();
    // pub fn gtk_label_get_attributes            (label: *const const GtkLabel) -> *PangoAttrList;
    pub fn gtk_label_get_justify               (label: *mut GtkLabel) -> enums::Justification;
    // pub fn gtk_label_get_ellipsize             (label: *const const GtkLabel) -> PangoEllipsizeMode;
    pub fn gtk_label_get_width_chars           (label: *mut GtkLabel) -> c_int;
    pub fn gtk_label_get_max_width_chars       (label: *mut GtkLabel) -> c_int;
    pub fn gtk_label_get_label                 (label: *mut GtkLabel) -> *const c_char;
    // pub fn gtk_label_get_layout                (label: *const const GtkLabel) -> *PangoLayout;
    pub fn gtk_label_get_line_wrap             (label: *mut GtkLabel) -> gboolean;
    // pub fn gtk_label_get_line_wrap_mode        (label: *const const GtkLabel) -> PangoWrapMode;
    pub fn gtk_label_get_lines                 (label: *mut GtkLabel) -> c_int;
    // pub fn gtk_label_get_mnemonic_widget       (label: *const const GtkLabel) -> *const const GtkWidget;
    pub fn gtk_label_get_selection_bounds      (label: *mut GtkLabel, start: *const c_int, end: *const c_int) -> gboolean;
    pub fn gtk_label_get_use_markup            (label: *mut GtkLabel) -> gboolean;
    pub fn gtk_label_get_use_underline         (label: *mut GtkLabel) -> gboolean;
    pub fn gtk_label_get_single_line_mode      (label: *mut GtkLabel) -> gboolean;
    pub fn gtk_label_get_angle                 (label: *mut GtkLabel) -> c_double;
    pub fn gtk_label_set_use_markup            (label: *mut GtkLabel, setting: gboolean) -> ();
    pub fn gtk_label_set_use_underline         (label: *mut GtkLabel, setting: gboolean) -> ();
    pub fn gtk_label_set_single_line_mode      (label: *mut GtkLabel, single_line_mod: gboolean) -> ();
    pub fn gtk_label_set_angle                 (label: *mut GtkLabel, angle: c_double) -> ();
    pub fn gtk_label_get_current_uri           (label: *mut GtkLabel) -> *const c_char;
    pub fn gtk_label_set_track_visited_links   (label: *mut GtkLabel, track_links: gboolean) -> ();
    pub fn gtk_label_get_track_visited_links   (label: *mut GtkLabel) -> gboolean;

    //=========================================================================
    // GtkEntryCompletion                                                NOT OK
    //=========================================================================
    pub fn gtk_entry_completion_new            () -> *mut GtkEntryCompletion;
    //pub fn gtk_entry_completion_new_with_area  (area: *mut GtkCellArea) -> *mut GtkEntryCompletion;
    pub fn gtk_entry_completion_get_entry      (completion: *mut GtkEntryCompletion) -> *mut GtkWidget;
    pub fn gtk_entry_completion_set_model      (completion: *mut GtkEntryCompletion, model: *mut GtkTreeModel);
    pub fn gtk_entry_completion_get_model      (completion: *mut GtkEntryCompletion) -> *mut GtkTreeModel;
    //pub fn gtk_entry_completion_set_match_func (completion: *mut GtkEntryCompletion, func: GtkEntryCompletionMatchFunc,
    //    func_data: gpointer, func_notifiy: GDestroyNotify);
    pub fn gtk_entry_completion_set_minimum_key_length(completion: *mut GtkEntryCompletion, length: c_int);
    pub fn gtk_entry_completion_get_minimum_key_length(completion: *mut GtkEntryCompletion) -> c_int;
    pub fn gtk_entry_completion_compute_prefix (completion: *mut GtkEntryCompletion, key: *const c_char) -> *mut c_char;
    pub fn gtk_entry_completion_complete       (completion: *mut GtkEntryCompletion);
    pub fn gtk_entry_completion_get_completion_prefix(completion: *mut GtkEntryCompletion) -> *const c_char;
    pub fn gtk_entry_completion_insert_prefix  (completion: *mut GtkEntryCompletion);
    pub fn gtk_entry_completion_insert_action_text(completion: *mut GtkEntryCompletion, index_: c_int, text: *const c_char);
    pub fn gtk_entry_completion_insert_action_markup(completion: *mut GtkEntryCompletion, index_: c_int, markup: *const c_char);
    pub fn gtk_entry_completion_delete_action  (completion: *mut GtkEntryCompletion, index_: c_int);
    pub fn gtk_entry_completion_set_text_column(completion: *mut GtkEntryCompletion, column: c_int);
    pub fn gtk_entry_completion_get_text_column(completion: *mut GtkEntryCompletion) -> c_int;
    pub fn gtk_entry_completion_set_inline_completion(completion: *mut GtkEntryCompletion, inline_completion: gboolean);
    pub fn gtk_entry_completion_get_inline_completion(completion: *mut GtkEntryCompletion) -> gboolean;
    pub fn gtk_entry_completion_set_inline_selection(completion: *mut GtkEntryCompletion, inline_selection: gboolean);
    pub fn gtk_entry_completion_get_inline_selection(completion: *mut GtkEntryCompletion) -> gboolean;
    pub fn gtk_entry_completion_set_popup_completion(completion: *mut GtkEntryCompletion, popup_completion: gboolean);
    pub fn gtk_entry_completion_get_popup_completion(completion: *mut GtkEntryCompletion) -> gboolean;
    pub fn gtk_entry_completion_set_popup_set_width(completion: *mut GtkEntryCompletion, popup_set_width: gboolean);
    pub fn gtk_entry_completion_get_popup_set_width(completion: *mut GtkEntryCompletion) -> gboolean;
    pub fn gtk_entry_completion_set_popup_single_match(completion: *mut GtkEntryCompletion, popup_single_match: gboolean);
    pub fn gtk_entry_completion_get_popup_single_match(completion: *mut GtkEntryCompletion) -> gboolean;

    //pub type GtkEntryCompletionMatchFunc = Option<extern "C" fn(completion: *mut GtkEntryCompletion, key: *const c_char,
    //    iter: *mut GtkTreeIter, user_data: gpointer) -> gboolean>;

    //=========================================================================
    // GtkContainer                                                          OK
    //=========================================================================
    pub fn gtk_container_get_type              () -> GType;
    pub fn gtk_container_add                   (container: *mut GtkContainer, widget: *mut GtkWidget);
    pub fn gtk_container_remove                (container: *mut GtkContainer, widget: *mut GtkWidget) -> ();
    pub fn gtk_container_get_resize_mode       (container: *mut GtkContainer) -> enums::ResizeMode;
    pub fn gtk_container_set_resize_mode       (container: *mut GtkContainer, resize_mode: enums::ResizeMode) -> ();
    pub fn gtk_container_check_resize          (container: *mut GtkContainer) -> ();
    pub fn gtk_container_get_border_width      (container: *mut GtkContainer) -> c_uint;
    pub fn gtk_container_set_border_width      (container: *mut GtkContainer, border_width: c_uint) -> ();
    pub fn gtk_container_get_children          (container: *mut GtkContainer) -> *mut GList;

    //=========================================================================
    // GtkMisc                                                               OK
    //=========================================================================
    pub fn gtk_misc_set_alignment              (misc: *mut GtkMisc, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_misc_set_padding                (misc: *mut GtkMisc, xpad: c_int, ypad: c_int) -> ();
    pub fn gtk_misc_get_alignment              (misc: *mut GtkMisc, xalign: *const c_float, yalign: *const c_float) -> ();
    pub fn gtk_misc_get_padding                (misc: *mut GtkMisc, xpad: *const c_int, ypad: *const c_int) -> ();

    //=========================================================================
    // GtkCellEditable                                                   NOT OK
    //=========================================================================
    pub fn gtk_cell_editable_get_type          () -> GType;
    //pub fn gtk_cell_editable_start_editing     (cell_editable: *mut GtkCellEditable, event: *mut GdkEvent) -> ();
    pub fn gtk_cell_editable_editing_done      (cell_editable: *mut GtkCellEditable);
    pub fn gtk_cell_editable_remove_widget     (cell_editable: *mut GtkCellEditable);

    //=========================================================================
    // GtkCellRendererText                                               NOT OK
    //=========================================================================
    pub fn gtk_cell_renderer_text_get_type     () -> GType;
    pub fn gtk_cell_renderer_text_new          () -> *mut GtkCellRenderer;
    pub fn gtk_cell_renderer_text_set_fixed_height_from_font(renderer: *mut GtkCellRendererText, number_of_rows: c_int) -> ();

    //=========================================================================
    // GtkCellRendererToggle                                             NOT OK
    //=========================================================================
    pub fn gtk_cell_renderer_toggle_get_type   () -> GType;
    pub fn gtk_cell_renderer_toggle_new        () -> *mut GtkCellRenderer;
    pub fn gtk_cell_renderer_toggle_get_radio  (renderer: *mut GtkCellRendererToggle) -> gboolean;
    pub fn gtk_cell_renderer_toggle_set_radio  (renderer: *mut GtkCellRendererToggle, radio: gboolean) -> ();
    pub fn gtk_cell_renderer_toggle_get_active (renderer: *mut GtkCellRendererToggle) -> gboolean;
    pub fn gtk_cell_renderer_toggle_set_active (renderer: *mut GtkCellRendererToggle, active: gboolean) -> ();

    //=========================================================================
    // GtkRecentChooserWidget                                                OK
    //=========================================================================
    pub fn gtk_recent_chooser_widget_new       () -> *mut GtkWidget;
    pub fn gtk_recent_chooser_widget_new_for_manager(manager: *mut GtkRecentManager) -> *mut GtkWidget;

    //=========================================================================
    // GtkPopover                                                        NOT OK
    //=========================================================================
    pub fn gtk_popover_new                     (relative_to: *mut GtkWidget) -> *mut GtkWidget;
    //pub fn gtk_popover_new_from_model          (relative_to: *mut GtkWidget, model: *mut GMenuModel) -> *mut GtkWidget;
    //pub fn gtk_popover_bind_model              (popover: *mut GtkPopover, model: *mut GMenuModel, action_namespace: *const c_char);
    pub fn gtk_popover_set_relative_to         (popover: *mut GtkPopover, relative_to: *mut GtkWidget);
    pub fn gtk_popover_get_relative_to         (popover: *mut GtkPopover) -> *mut GtkWidget;
    //pub fn gtk_popover_set_pointing_to         (popover: *mut GtkPopover, rect: *mut GdkRectangle);
    //pub fn gtk_popover_get_pointing_to         (popover: *mut GtkPopover) -> *mut GdkRectangle;
    pub fn gtk_popover_set_position            (popover: *mut GtkPopover, position: enums::PositionType);
    pub fn gtk_popover_get_position            (popover: *mut GtkPopover) -> enums::PositionType;
    pub fn gtk_popover_set_modal               (popover: *mut GtkPopover, modal: gboolean);
    pub fn gtk_popover_get_modal               (popover: *mut GtkPopover) -> gboolean;

    //=========================================================================
    // GtkTreePath                                                       NOT OK
    //=========================================================================
    pub fn gtk_tree_path_new                   () -> *mut GtkTreePath;
    pub fn gtk_tree_path_new_from_string       (path: *const c_char) -> *mut GtkTreePath;
    //pub fn gtk_tree_path_new_from_indices      (first_index: c_int, ...) -> *mut GtkTreePath;
    pub fn gtk_tree_path_new_from_indicesv     (indices: *mut c_int, length: c_ulong) -> *mut GtkTreePath;
    pub fn gtk_tree_path_to_string             (path: *mut GtkTreePath) -> *mut c_char;
    pub fn gtk_tree_path_new_first             () -> *mut GtkTreePath;
    pub fn gtk_tree_path_append_index          (path: *mut GtkTreePath, index_: c_int);
    pub fn gtk_tree_path_prepend_index         (path: *mut GtkTreePath, index_: c_int);
    pub fn gtk_tree_path_get_depth             (path: *mut GtkTreePath) -> c_int;
    pub fn gtk_tree_path_get_indices           (path: *mut GtkTreePath) -> *mut c_int;
    pub fn gtk_tree_path_get_indices_with_depth(path: *mut GtkTreePath, depth: *mut c_int) -> *mut c_int;
    pub fn gtk_tree_path_free                  (path: *mut GtkTreePath);
    pub fn gtk_tree_path_copy                  (path: *mut GtkTreePath) -> *mut GtkTreePath;
    pub fn gtk_tree_path_compare               (a: *const GtkTreePath, b: *const GtkTreePath) -> i32;
    pub fn gtk_tree_path_next                  (path: *mut GtkTreePath);
    pub fn gtk_tree_path_prev                  (path: *mut GtkTreePath);
    pub fn gtk_tree_path_up                    (path: *mut GtkTreePath) -> gboolean;
    pub fn gtk_tree_path_down                  (path: *mut GtkTreePath);
    pub fn gtk_tree_path_is_ancestor           (path: *mut GtkTreePath, descendant: *mut GtkTreePath) -> gboolean;
    pub fn gtk_tree_path_is_descendant         (path: *mut GtkTreePath, ancestor: *mut GtkTreePath) -> gboolean;

    //=========================================================================
    // GtkTreeSelection                                                  NOT OK
    //=========================================================================
    pub fn gtk_tree_selection_set_mode         (selection: *mut GtkTreeSelection, type_: enums::SelectionMode);
    pub fn gtk_tree_selection_get_mode         (selection: *mut GtkTreeSelection) -> enums::SelectionMode;
    //pub fn gtk_tree_selection_set_select_function(selection: *mut GtkTreeSelection, func: GtkTreeSelectionFunc, data: gpointer,
    //    destroy: GDestroyNotify);
    //pub fn gtk_tree_selection_get_select_function(selection: *mut GtkTreeSelection) -> GtkTreeSelectionFunc;
    pub fn gtk_tree_selection_get_user_data    (selection: *mut GtkTreeSelection) -> gpointer;
    pub fn gtk_tree_selection_get_tree_view    (selection: *mut GtkTreeSelection) -> *mut GtkTreeView;
    pub fn gtk_tree_selection_get_selected     (selection: *mut GtkTreeSelection, model: *mut *mut GtkTreeModel,
        iter: *mut GtkTreeIter) -> gboolean;
    //pub fn gtk_tree_selection_selected_foreach (selection: *mut GtkTreeSelection, func: GtkTreeSelectionForeachFunc, data: gpointer);
    //pub fn gtk_tree_selection_get_selected_rows(selection: *mut GtkTreeSelection, model: *mut *mut GtkTreeModel) -> *mut GList;
    pub fn gtk_tree_selection_count_selected_rows(selection: *mut GtkTreeSelection) -> c_int;
    pub fn gtk_tree_selection_select_path      (selection: *mut GtkTreeSelection, path: *mut GtkTreePath);
    pub fn gtk_tree_selection_unselect_path    (selection: *mut GtkTreeSelection, path: *mut GtkTreePath);
    pub fn gtk_tree_selection_path_is_selected (selection: *mut GtkTreeSelection, path: *mut GtkTreePath) -> gboolean;
    pub fn gtk_tree_selection_select_iter      (selection: *mut GtkTreeSelection, iter: *mut GtkTreeIter);
    pub fn gtk_tree_selection_unselect_iter    (selection: *mut GtkTreeSelection, iter: *mut GtkTreeIter);
    pub fn gtk_tree_selection_iter_is_selected (selection: *mut GtkTreeSelection, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_selection_select_all       (selection: *mut GtkTreeSelection);
    pub fn gtk_tree_selection_unselect_all     (selection: *mut GtkTreeSelection);
    pub fn gtk_tree_selection_select_range     (selection: *mut GtkTreeSelection, start_range: *mut GtkTreePath,
        end_range: *mut GtkTreePath);
    pub fn gtk_tree_selection_unselect_range   (selection: *mut GtkTreeSelection, start_range: *mut GtkTreePath,
        end_range: *mut GtkTreePath);

    //pub type GtkTreeSelectionFunc = fn(selection: *mut GtkTreeSelection, model: *mut GtkTreeModel, path: *mut GtkTreePath,
    //    path_currently_selected: gboolean, data: gpointer) -> gboolean;
    //pub type GtkTreeSelectionForeachFunc = fn(model: *mut GtkTreeModel, path: *mut GtkTreePath, iter: *mut GtkTreeIter,
    //    data: gpointer);


    //=========================================================================
    // GtkTreeModel                                                      NOT OK
    //=========================================================================
    pub fn gtk_tree_model_get_flags            (tree_model: *mut GtkTreeModel) -> enums::TreeModelFlags;
    pub fn gtk_tree_model_get_n_columns        (tree_model: *mut GtkTreeModel) -> c_int;
    pub fn gtk_tree_model_get_column_type      (tree_model: *mut GtkTreeModel, index_: c_int) -> GType;
    pub fn gtk_tree_model_get_iter             (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, path: *mut GtkTreePath) -> gboolean;
    pub fn gtk_tree_model_get_iter_from_string (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, path_string: *const c_char) -> gboolean;
    pub fn gtk_tree_model_get_iter_first       (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_model_get_path             (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> *mut GtkTreePath;
    pub fn gtk_tree_model_get_value            (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, column: c_int, value: *mut GValue);
    pub fn gtk_tree_model_iter_next            (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_model_iter_previous        (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_model_iter_children        (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_model_iter_has_child       (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_model_iter_n_children      (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> c_int;
    pub fn gtk_tree_model_iter_nth_child       (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter,
        n: c_int) -> gboolean;
    pub fn gtk_tree_model_iter_parent          (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, child: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_model_get_string_from_iter (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> *mut c_char;
    pub fn gtk_tree_model_ref_node             (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> ();
    pub fn gtk_tree_model_unref_node           (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter) -> ();
    //pub fn gtk_tree_model_get                  (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, ...) -> ();
    //pub fn gtk_tree_model_get_valist           (tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, va_list: var_args) -> ();
    //pub fn gtk_tree_model_foreach              (tree_model: *mut GtkTreeModel, func: GtkTreeModelForeachFunc, user_data: gpointer) -> ();
    pub fn gtk_tree_model_foreach              (tree_model: *mut GtkTreeModel, func: gpointer, user_data: gpointer) -> ();
    pub fn gtk_tree_model_row_changed          (tree_model: *mut GtkTreeModel, path: *mut GtkTreePath, iter: *mut GtkTreeIter) -> ();
    pub fn gtk_tree_model_row_inserted         (tree_model: *mut GtkTreeModel, path: *mut GtkTreePath, iter: *mut GtkTreeIter) -> ();
    pub fn gtk_tree_model_row_has_child_toggled(tree_model: *mut GtkTreeModel, path: *mut GtkTreePath, iter: *mut GtkTreeIter) -> ();
    pub fn gtk_tree_model_row_deleted          (tree_model: *mut GtkTreeModel, path: *mut GtkTreePath) -> ();
    pub fn gtk_tree_model_rows_reordered       (tree_model: *mut GtkTreeModel, path: *mut GtkTreePath, iter: *mut GtkTreeIter,
        new_order: *mut c_int) -> ();

    //=========================================================================
    // GtkListStore                                                          OK
    //=========================================================================

    pub fn gtk_list_store_newv(n_columns: c_int, column_types: *mut GType) -> *mut GtkListStore;
    pub fn gtk_list_store_set_column_types(list_store: *mut GtkListStore, n_columns: c_int, column_types: *mut GType);
    pub fn gtk_list_store_set_value(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, column: c_int, value: *mut GValue);
    pub fn gtk_list_store_set(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, ...);
    //pub fn gtk_list_store_set_valist(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, var_args: va_list);
    //pub fn gtk_list_store_set_valuesv(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, columns: *mut c_int, values: *mut GValue, n_values: c_int);
    pub fn gtk_list_store_remove(list_store: *mut GtkListStore, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_list_store_insert(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, position: c_int);
    pub fn gtk_list_store_insert_before(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, sibling: *mut GtkTreeIter);
    pub fn gtk_list_store_insert_after(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, sibling: *mut GtkTreeIter);
    //pub fn gtk_list_store_insert_with_valuesv(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, position: c_int, columns: *mut c_int, values: *mut GtkGValue, n_values: c_int);
    pub fn gtk_list_store_prepend(list_store: *mut GtkListStore, iter: *mut GtkTreeIter);
    pub fn gtk_list_store_append(list_store: *mut GtkListStore, iter: *mut GtkTreeIter);
    pub fn gtk_list_store_clear(list_store: *mut GtkListStore);
    pub fn gtk_list_store_iter_is_valid(list_store: *mut GtkListStore, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_list_store_reorder(list_store: *mut GtkListStore, new_order: *mut c_int);
    pub fn gtk_list_store_swap(list_store: *mut GtkListStore, a: *mut GtkTreeIter, b: *mut GtkTreeIter);
    pub fn gtk_list_store_move_before(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, position: *mut GtkTreeIter);
    pub fn gtk_list_store_move_after(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, position: *mut GtkTreeIter);

    //=========================================================================
    // GtkTreeStore                                                          OK
    //=========================================================================

    pub fn gtk_tree_store_newv(n_columns: c_int, column_types: *mut GType) -> *mut GtkTreeStore;
    pub fn gtk_tree_store_set_column_types(tree_store: *mut GtkTreeStore, n_columns: c_int, column_types: *mut GType);
    pub fn gtk_tree_store_set_value(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, column: c_int, value: *mut GValue);
    pub fn gtk_tree_store_set(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, ...);
    //pub fn gtk_tree_store_set_valist(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, var_args: va_list);
    //pub fn gtk_tree_store_set_valuesv(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, columns: *mut c_int, values: *mut GValue, n_values: c_int);
    pub fn gtk_tree_store_remove(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_store_insert(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter, position: c_int);
    pub fn gtk_tree_store_insert_before(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter, sibling: *mut GtkTreeIter);
    pub fn gtk_tree_store_insert_after(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter, sibling: *mut GtkTreeIter);
    //pub fn gtk_tree_store_insert_with_valuesv(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter, position: c_int, columns: *mut c_int, values: *mut GValue, n_values: c_int);
    pub fn gtk_tree_store_prepend(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter);
    pub fn gtk_tree_store_append(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, parent: *mut GtkTreeIter);
    pub fn gtk_tree_store_is_ancestor(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, descendent: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_store_iter_depth(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter) -> c_int;
    pub fn gtk_tree_store_clear(tree_store: *mut GtkTreeStore);
    pub fn gtk_tree_store_iter_is_valid(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_tree_store_reorder(tree_store: *mut GtkTreeStore, parent: *mut GtkTreeIter, new_order: *mut c_int);
    pub fn gtk_tree_store_swap(tree_store: *mut GtkTreeStore, a: *mut GtkTreeIter, b: *mut GtkTreeIter);
    pub fn gtk_tree_store_move_before(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, position: *mut GtkTreeIter);
    pub fn gtk_tree_store_move_after(tree_store: *mut GtkTreeStore, iter: *mut GtkTreeIter, position: *mut GtkTreeIter);

    //=========================================================================
    // GtkTreeIter                                                           OK
    //=========================================================================
    pub fn gtk_tree_iter_copy(iter: *mut GtkTreeIter) -> *mut GtkTreeIter;
    pub fn gtk_tree_iter_free(iter: *mut GtkTreeIter);

    //=========================================================================
    // GtkCellRenderer                                                   NOT OK
    //=========================================================================
    pub fn gtk_cell_renderer_get_type          () -> GType;
    //pub fn gtk_cell_renderer_class_set_accessible_type(renderer_class: *mut GtkCellRendererClass, _type: GType);
    //pub fn gtk_cell_renderer_get_aligned_area  (cell: *mut GtkCellRenderer, widget: *mut GtkWidget, flags: enums::CellRendererState,
    //    cell_area: *const GdkRectangle, aligned_area: *mut GdkRectangle);
    //pub fn gtk_cell_renderer_render            (cell: *mut GtkCellRenderer, cr: *mut cairo_t, widget: *mut GtkWidget,
    //    background_area: *const GdkRectangle, cell_area: *const GdkRectangle, flags: enums::CellRendererState);
    //pub fn gtk_cell_renderer_activate          (cell: *mut GtkCellRenderer, event: *mut GdkEvent, widget: *mut GtkWidget, path: *const c_char,
    //    background_area: *const GdkRectangle, cell_area: *const GdkRectangle, flags: enums::CellRendererState) -> gboolean;
    //pub fn gtk_cell_renderer_start_editing     (cell: *mut GtkCellRenderer, event: *mut GdkEvent, widget: *mut GtkWidget, path: *const c_char,
    //    background_area: *const GdkRectangle, cell_area: *const GdkRectangle, flags: enums::CellRendererState) -> *mut GtkCellEditable;
    pub fn gtk_cell_renderer_stop_editing      (cell: *mut GtkCellRenderer, canceled: gboolean) -> ();
    pub fn gtk_cell_renderer_get_fixed_size    (cell: *mut GtkCellRenderer, width: *mut c_int, height: *mut c_int) -> ();
    pub fn gtk_cell_renderer_set_fixed_size    (cell: *mut GtkCellRenderer, width: c_int, height: c_int) -> ();
    pub fn gtk_cell_renderer_get_visible       (cell: *mut GtkCellRenderer) -> gboolean;
    pub fn gtk_cell_renderer_set_visible       (cell: *mut GtkCellRenderer, visible: gboolean) -> ();
    pub fn gtk_cell_renderer_get_sensitive     (cell: *mut GtkCellRenderer) -> gboolean;
    pub fn gtk_cell_renderer_set_sensitive     (cell: *mut GtkCellRenderer, sensitive: gboolean) -> ();
    pub fn gtk_cell_renderer_get_alignment     (cell: *mut GtkCellRenderer, xalign: *mut c_float, yalign: *mut c_float) -> ();
    pub fn gtk_cell_renderer_set_alignment     (cell: *mut GtkCellRenderer, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_cell_renderer_get_padding       (cell: *mut GtkCellRenderer, xpad: *mut c_int, ypad: *mut c_int) -> ();
    pub fn gtk_cell_renderer_set_padding       (cell: *mut GtkCellRenderer, xpad: c_int, ypad: c_int) -> ();
    pub fn gtk_cell_renderer_get_state         (cell: *mut GtkCellRenderer, widget: *mut GtkWidget, cell_state: enums::CellRendererState) -> enums::StateFlags;
    pub fn gtk_cell_renderer_is_activatable    (cell: *mut GtkCellRenderer) -> gboolean;
    pub fn gtk_cell_renderer_get_preferred_height(cell: *mut GtkCellRenderer, widget: *mut GtkWidget, minimum_size: *mut c_int,
        natural_size: *mut c_int) -> ();
    pub fn gtk_cell_renderer_get_preferred_height_for_width(cell: *mut GtkCellRenderer, widget: *mut GtkWidget, width: c_int,
        minimum_size: *mut c_int, natural_size: *mut c_int) -> ();
    //pub fn gtk_cell_renderer_get_preferred_size(cell: *mut GtkCellRenderer, widget: *mut GtkWidget, minimum_size: *mut GtkRequisition,
    //    natural_size: *mut GtkRequisition) -> ();
    pub fn gtk_cell_renderer_get_preferred_width(cell: *mut GtkCellRenderer, widget: *mut GtkWidget, minimum_size: *mut c_int,
        natural_size: *mut c_int) -> ();
    pub fn gtk_cell_renderer_get_preferred_width_for_height(cell: *mut GtkCellRenderer, widget: *mut GtkWidget, height: c_int,
        minimum_size: *mut c_int, natural_size: *mut c_int) -> ();
    pub fn gtk_cell_renderer_get_request_mode(cell: *mut GtkCellRenderer) -> enums::SizeRequestMode;

    //=========================================================================
    // GtkCelllayout                                                     NOT OK
    //=========================================================================
    pub fn gtk_cell_layout_get_type            () -> GType;
    pub fn gtk_cell_layout_pack_start          (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer, expand: gboolean);
    pub fn gtk_cell_layout_pack_end            (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer, expand: gboolean);
    //pub fn gtk_cell_layout_get_area            (cell_layout: *mut GtkCellLayout) -> *mut GtkCellArea;
    pub fn gtk_cell_layout_get_cells           (cell_layout: *mut GtkCellLayout) -> *mut GList;
    pub fn gtk_cell_layout_reorder             (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer, position: c_int);
    pub fn gtk_cell_layout_clear               (cell_layout: *mut GtkCellLayout);
    //pub fn gtk_cell_layout_set_attributes      (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer, ...);
    pub fn gtk_cell_layout_add_attribute       (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer, attribute: *const c_char,
        column: c_int);
    //pub fn gtk_cell_layout_set_cell_data_func  (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer,
    //    func: GtkCellLayoutDataFunc, func_data: gpointer, destroy: GDestroyNotify);
    pub fn gtk_cell_layout_clear_attributes    (cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer);
    //pub type GtkCellLayoutDataFunc = Option<extern "C" fn(cell_layout: *mut GtkCellLayout, cell: *mut GtkCellRenderer,
    //    tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, data: gpointer)>;

    //=========================================================================
    // GtkButton                                                         NOT OK
    //=========================================================================
    pub fn gtk_button_get_type                 () -> GType;
    pub fn gtk_button_new                      () -> *mut GtkWidget;
    pub fn gtk_button_new_with_label           (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_button_new_with_mnemonic        (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_button_new_from_icon_name       (icon_id: *const c_char, size: enums::IconSize) -> *mut GtkWidget;
    pub fn gtk_button_new_from_stock           (stock_id: *const c_char) -> *mut GtkWidget;
    pub fn gtk_button_pressed                  (button: *mut GtkButton) -> ();
    pub fn gtk_button_released                 (button: *mut GtkButton) -> ();
    pub fn gtk_button_clicked                  (button: *mut GtkButton) -> ();
    pub fn gtk_button_enter                    (button: *mut GtkButton) -> ();
    pub fn gtk_button_leave                    (button: *mut GtkButton) -> ();
    pub fn gtk_button_set_relief               (button: *mut GtkButton, newstyle: enums::ReliefStyle) -> ();
    pub fn gtk_button_get_relief               (button: *mut GtkButton) -> enums::ReliefStyle;
    pub fn gtk_button_get_label                (button: *mut GtkButton) -> *const c_char;
    pub fn gtk_button_set_label                (button: *mut GtkButton, label: *const c_char) -> ();
    pub fn gtk_button_get_use_stock            (button: *mut GtkButton) -> gboolean;
    pub fn gtk_button_set_use_stock            (button: *mut GtkButton, use_stock: gboolean) -> ();
    pub fn gtk_button_get_use_underline        (button: *mut GtkButton) -> gboolean;
    pub fn gtk_button_set_use_underline        (button: *mut GtkButton, use_underline: gboolean) -> ();
    pub fn gtk_button_set_focus_on_click       (button: *mut GtkButton, focus_on_click: gboolean) -> ();
    pub fn gtk_button_get_focus_on_click       (button: *mut GtkButton) -> gboolean;
    pub fn gtk_button_set_alignment            (button: *mut GtkButton, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_button_get_alignment            (button: *mut GtkButton, xalign: *mut c_float, yalign: *mut c_float) -> ();
    pub fn gtk_button_set_image                (button: *mut GtkButton, image: *mut GtkWidget) -> ();
    // pub fn gtk_button_get_image                (button: *const const GtkButton) -> *const const GtkWidget;
    pub fn gtk_button_set_image_position       (button: *mut GtkButton, position: enums::PositionType) -> ();
    pub fn gtk_button_get_image_position       (button: *mut GtkButton) -> enums::PositionType;
    pub fn gtk_button_set_always_show_image    (button: *mut GtkButton, always_show: gboolean) -> ();
    pub fn gtk_button_get_always_show_image    (button: *mut GtkButton) -> gboolean;
    // pub fn gtk_button_get_event_window         (button: *const const GtkButton) -> *const const GdkWindow;

    //=========================================================================
    // GtkFileChooser                                                    NOT OK
    //=========================================================================
    pub fn gtk_file_chooser_get_type           () -> GType;
    // FIXME : check if memory is freed when a *const c_char is returned
    pub fn gtk_file_chooser_set_action         (chooser: *mut GtkFileChooser, action: enums::FileChooserAction) -> ();
    pub fn gtk_file_chooser_get_action         (chooser: *mut GtkFileChooser) -> enums::FileChooserAction;
    pub fn gtk_file_chooser_set_local_only     (chooser: *mut GtkFileChooser, local_only: gboolean) -> ();
    pub fn gtk_file_chooser_get_local_only     (chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_set_select_multiple(chooser: *mut GtkFileChooser, select_multiple: gboolean) -> ();
    pub fn gtk_file_chooser_get_select_multiple(chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_set_show_hidden    (chooser: *mut GtkFileChooser, show_hidden: gboolean) -> ();
    pub fn gtk_file_chooser_get_show_hidden    (chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_set_do_overwrite_confirmation(chooser: *mut GtkFileChooser, do_overwrite_confirmation: gboolean) -> ();
    pub fn gtk_file_chooser_get_do_overwrite_confirmation(chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_set_create_folders (chooser: *mut GtkFileChooser, create_folders: gboolean) -> ();
    pub fn gtk_file_chooser_get_create_folders (chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_set_current_name   (chooser: *mut GtkFileChooser, name: *const c_char) -> ();
    pub fn gtk_file_chooser_get_current_name   (chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_filename       (chooser: *mut GtkFileChooser, filename: *const c_char) -> gboolean;
    pub fn gtk_file_chooser_get_filename       (chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_select_filename    (chooser: *mut GtkFileChooser, filename: *const c_char) -> gboolean;
    pub fn gtk_file_chooser_unselect_filename  (chooser: *mut GtkFileChooser, filename: *const c_char) -> ();
    pub fn gtk_file_chooser_select_all         (chooser: *mut GtkFileChooser) -> ();
    pub fn gtk_file_chooser_unselect_all       (chooser: *mut GtkFileChooser) -> ();
    pub fn gtk_file_chooser_get_filenames      (chooser: *mut GtkFileChooser) -> *mut GSList;
    pub fn gtk_file_chooser_set_current_folder (chooser: *mut GtkFileChooser, filename: *const c_char) -> gboolean;
    pub fn gtk_file_chooser_get_current_folder (chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_uri            (chooser: *mut GtkFileChooser, uri: *const c_char) -> gboolean;
    pub fn gtk_file_chooser_get_uri            (chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_select_uri         (chooser: *mut GtkFileChooser, uri: *const c_char) -> gboolean;
    pub fn gtk_file_chooser_unselect_uri       (chooser: *mut GtkFileChooser, uri: *const c_char) -> ();
    pub fn gtk_file_chooser_get_uris           (chooser: *mut GtkFileChooser) -> *mut GSList;
    pub fn gtk_file_chooser_set_current_folder_uri(chooser: *mut GtkFileChooser, uri: *const c_char) -> gboolean;
    pub fn gtk_file_chooser_get_current_folder_uri(chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_preview_widget (chooser: *mut GtkFileChooser, preview_widget: *mut GtkWidget) -> ();
    pub fn gtk_file_chooser_get_preview_widget (chooser: *mut GtkFileChooser) -> *mut GtkWidget;
    pub fn gtk_file_chooser_set_preview_widget_active(chooser: *mut GtkFileChooser, active: gboolean) -> ();
    pub fn gtk_file_chooser_get_preview_widget_active(chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_set_use_preview_label(chooser: *mut GtkFileChooser, use_label: gboolean) -> ();
    pub fn gtk_file_chooser_get_use_preview_label(chooser: *mut GtkFileChooser) -> gboolean;
    pub fn gtk_file_chooser_get_preview_filename(chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_get_preview_uri    (chooser: *mut GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_extra_widget   (chooser: *mut GtkFileChooser, extra_widget: *mut GtkWidget) -> ();
    pub fn gtk_file_chooser_get_extra_widget   (chooser: *mut GtkFileChooser) -> *mut GtkWidget;
    pub fn gtk_file_chooser_add_filter         (chooser: *mut GtkFileChooser, filter: *mut GtkFileFilter) -> ();
    pub fn gtk_file_chooser_remove_filter      (chooser: *mut GtkFileChooser, filter: *mut GtkFileFilter) -> ();
    pub fn gtk_file_chooser_list_filters       (chooser: *mut GtkFileChooser) -> *mut GSList;
    pub fn gtk_file_chooser_set_filter         (chooser: *mut GtkFileChooser, filter: *mut GtkFileFilter) -> ();
    pub fn gtk_file_chooser_get_filter         (chooser: *mut GtkFileChooser) -> *mut GtkFileFilter;
    pub fn gtk_file_chooser_add_shortcut_folder(chooser: *mut GtkFileChooser, folder: *const c_char, error: *mut *mut GError) -> gboolean;
    pub fn gtk_file_chooser_remove_shortcut_folder(chooser: *mut GtkFileChooser, folder: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_file_chooser_list_shortcut_folders(chooser: *const const GtkFileChooser) -> *GSList;
    pub fn gtk_file_chooser_add_shortcut_folder_uri(chooser: *mut GtkFileChooser, uri: *const c_char, error: *mut *mut GError) -> gboolean;
    pub fn gtk_file_chooser_remove_shortcut_folder_uri(chooser: *mut GtkFileChooser, uri: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_file_chooser_list_shortcut_folder_uris(chooser: *const const GtkFileChooser) -> *GSList;
    //pub fn gtk_file_chooser_get_current_folder_file(chooser: *const const GtkFileChooser) -> *const const Gfile;
    //pub fn gtk_file_chooser_get_file           (chooser: *const const GtkFileChooser) -> *const const Gfile;
    //pub fn gtk_file_chooser_get_files          (chooser: *const const GtkFileChooser) -> *GSList;
    //pub fn gtk_file_chooser_get_preview_file   (chooser: *const const GtkFileChooser) -> *const const Gfile;
    //pub fn gtk_file_chooser_select_file        (chooser: *const const GtkFileChooser, file: *const const Gfile, error: **GError) -> gboolean;
    //pub fn gtk_file_chooser_set_current_folder_file(chooser: *const const GtkFileChooser, file: *const const Gfile, error: **GError) -> gboolean;
    //pub fn gtk_file_chooser_set_file           (chooser: *const const GtkFileChooser, file: *const const Gfile, error: **GError) -> gboolean;
    //pub fn gtk_file_chooser_unselect_file        (chooser: *const const GtkFileChooser, file: *const const Gfile) -> ();

    //=========================================================================
    // GtkFileFilter                                                     NOT OK
    //=========================================================================
    pub fn gtk_file_filter_get_type            () -> GType;
    pub fn gtk_file_filter_new                 () -> *mut GtkFileFilter;
    pub fn gtk_file_filter_set_name            (filter: *mut GtkFileFilter, name: *const c_char) -> ();
    pub fn gtk_file_filter_get_name            (filter: *mut GtkFileFilter) -> *const c_char;
    pub fn gtk_file_filter_add_mime_type       (filter: *mut GtkFileFilter, mime_type: *const c_char) -> ();
    pub fn gtk_file_filter_add_pattern         (filter: *mut GtkFileFilter, pattern: *const c_char) -> ();
    pub fn gtk_file_filter_add_pixbuf_formats  (filter: *mut GtkFileFilter) -> ();
    //pub fn gtk_file_filter_add_custom          (filter: *const const GtkFileFilter, func: *const const GtkFileFilterFunc, data: *const c_void, notify: *const const GDestroyNotif) -> ();

    //=========================================================================
    // GtkFileChooserDialog                                              NOT OK
    //=========================================================================
    //pub fn gtk_file_chooser_dialog_new         (title: *const c_char, parent: *const const GtkWindow, action: enums::FileChooserAction, first_button_text: *const c_char, ...) -> *const const GtkWidget;
    pub fn gtk_file_chooser_dialog_new         (title: *const c_char,
                                                parent: *mut GtkWindow,
                                                action: enums::FileChooserAction,
                                                first_button_text: *const c_char,
                                                ...) -> *mut GtkWidget;

    //=========================================================================
    // GtkIconView                                                       NOT OK
    //=========================================================================
    pub fn gtk_icon_view_new                   () -> *mut GtkWidget;
    //pub fn gtk_icon_view_new_with_area         (area: *mut GtkCellArea) -> *mut GtkWidget;
    pub fn gtk_icon_view_new_with_model        (model: *mut GtkTreeModel) -> *mut GtkWidget;
    pub fn gtk_icon_view_set_model             (icon_view: *mut GtkIconView, model: *mut GtkTreeModel);
    pub fn gtk_icon_view_get_model             (icon_view: *mut GtkIconView) -> *mut GtkTreeModel;
    pub fn gtk_icon_view_set_text_column       (icon_view: *mut GtkIconView, column: c_int);
    pub fn gtk_icon_view_get_text_column       (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_markup_column     (icon_view: *mut GtkIconView, column: c_int);
    pub fn gtk_icon_view_get_markup_column     (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_pixbuf_column     (icon_view: *mut GtkIconView, column: c_int);
    pub fn gtk_icon_view_get_pixbuf_column     (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_get_path_at_pos       (icon_view: *mut GtkIconView, x: c_int, y: c_int) -> *mut GtkTreePath;
    pub fn gtk_icon_view_get_item_at_pos       (icon_view: *mut GtkIconView, x: c_int, y: c_int, path: *mut *mut GtkTreePath,
        cell: *mut *mut GtkCellRenderer) -> gboolean;
    pub fn gtk_icon_view_convert_widget_to_bin_window_coords(icon_view: *mut GtkIconView, wx: c_int, wy: c_int, bx: *mut c_int,
        by: *mut c_int);
    pub fn gtk_icon_view_set_cursor            (icon_view: *mut GtkIconView, path: *mut GtkTreePath, cell: *mut GtkCellRenderer,
        start_editing: gboolean);
    pub fn gtk_icon_view_get_cursor            (icon_view: *mut GtkIconView, path: *mut *mut GtkTreePath,
        cell: *mut *mut GtkCellRenderer) -> gboolean;
    //pub fn gtk_icon_view_selected_foreach      (icon_view: *mut GtkIconView, func: GtkIconViewForeachFunc, data: gpointer);
    pub fn gtk_icon_view_set_selection_mode    (icon_view: *mut GtkIconView, mode: enums::SelectionMode);
    pub fn gtk_icon_view_get_selection_mode    (icon_view: *mut GtkIconView) -> enums::SelectionMode;
    pub fn gtk_icon_view_set_item_orientation  (icon_view: *mut GtkIconView, orientation: enums::Orientation);
    pub fn gtk_icon_view_get_item_orientation  (icon_view: *mut GtkIconView) -> enums::Orientation;
    pub fn gtk_icon_view_set_columns           (icon_view: *mut GtkIconView, column: c_int);
    pub fn gtk_icon_view_get_columns           (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_item_width        (icon_view: *mut GtkIconView, item_width: c_int);
    pub fn gtk_icon_view_get_item_width        (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_spacing           (icon_view: *mut GtkIconView, spacing: c_int);
    pub fn gtk_icon_view_get_spacing           (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_row_spacing       (icon_view: *mut GtkIconView, row_spacing: c_int);
    pub fn gtk_icon_view_get_row_spacing       (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_column_spacing    (icon_view: *mut GtkIconView, column_spacing: c_int);
    pub fn gtk_icon_view_get_column_spacing    (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_margin            (icon_view: *mut GtkIconView, margin: c_int);
    pub fn gtk_icon_view_get_margin            (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_item_padding      (icon_view: *mut GtkIconView, item_padding: c_int);
    pub fn gtk_icon_view_get_item_padding      (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_set_activate_on_single_click(icon_view: *mut GtkIconView, single: gboolean);
    pub fn gtk_icon_view_get_activate_on_single_click(icon_view: *mut GtkIconView) -> gboolean;
    //pub fn gtk_icon_view_get_cell_rect         (icon_view: *mut GtkIconView, path: *mut GtkTreePath, cell: *mut GtkCellRenderer,
    //    rect: *mut GdkRectangle) -> gboolean;
    pub fn gtk_icon_view_select_path           (icon_view: *mut GtkIconView, path: *mut GtkTreePath);
    pub fn gtk_icon_view_unselect_path         (icon_view: *mut GtkIconView, path: *mut GtkTreePath);
    pub fn gtk_icon_view_path_is_selected      (icon_view: *mut GtkIconView, path: *mut GtkTreePath) -> gboolean;
    //pub fn gtk_icon_view_get_selected_items    (icon_view: *mut GtkIconView) -> *mut GList;
    pub fn gtk_icon_view_select_all            (icon_view: *mut GtkIconView);
    pub fn gtk_icon_view_unselect_all          (icon_view: *mut GtkIconView);
    pub fn gtk_icon_view_item_activated        (icon_view: *mut GtkIconView, path: *mut GtkTreePath);
    pub fn gtk_icon_view_scroll_to_path        (icon_view: *mut GtkIconView, path: *mut GtkTreePath, use_align: gboolean,
        row_align: c_float, col_align: c_float);
    pub fn gtk_icon_view_get_visible_range     (icon_view: *mut GtkIconView, start_path: *mut *mut GtkTreePath,
        end_path: *mut *mut GtkTreePath) -> gboolean;
    //pub fn gtk_icon_view_set_tooltip_item      (icon_view: *mut GtkIconView, tooltip: *mut GtkTooltip, path: *mut GtkTreePath);
    //pub fn gtk_icon_view_set_tooltip_cell      (icon_view: *mut GtkIconView, tooltip: *mut GtkTooltip, path: *mut GtkTreePath,
    //    cell: *mut GtkCellRenderer);
    pub fn gtk_icon_view_get_tooltip_context   (icon_view: *mut GtkIconView, x: *mut c_int, y: *mut c_int, keyboard_tip: gboolean,
        model: *mut *mut GtkTreeModel, path: *mut *mut GtkTreePath, iter: *mut GtkTreeIter) -> gboolean;
    pub fn gtk_icon_view_set_tooltip_column    (icon_view: *mut GtkIconView, column: c_int);
    pub fn gtk_icon_view_get_tooltip_column    (icon_view: *mut GtkIconView) -> c_int;
    pub fn gtk_icon_view_get_item_row          (icon_view: *mut GtkIconView, path: *mut GtkTreePath) -> c_int;
    pub fn gtk_icon_view_get_item_column       (icon_view: *mut GtkIconView, path: *mut GtkTreePath) -> c_int;
    //pub fn gtk_icon_view_enable_model_drag_source(icon_view: *mut GtkIconView, start_button_mask: gdk::ModifierType,
    //    targets: *const GtkTargetEntry, n_targets: c_int, actions: GdkDragAction);
    //pub fn gtk_icon_view_enable_model_drag_dest(icon_view: *mut GtkIconView, targets: *const GtkTargetEntry, n_targets: c_int,
    //    actions: GdkDragAction);
    pub fn gtk_icon_view_unset_model_drag_source(icon_view: *mut GtkIconView);
    pub fn gtk_icon_view_unset_model_drag_dest (icon_view: *mut GtkIconView);
    pub fn gtk_icon_view_set_reorderable       (icon_view: *mut GtkIconView, reorderable: gboolean);
    pub fn gtk_icon_view_get_reorderable       (icon_view: *mut GtkIconView) -> gboolean;
    pub fn gtk_icon_view_set_drag_dest_item    (icon_view: *mut GtkIconView, path: *mut GtkTreePath, pos: enums::IconViewDropPosition);
    pub fn gtk_icon_view_get_drag_dest_item    (icon_view: *mut GtkIconView, path: *mut *mut GtkTreePath,
        pos: *mut enums::IconViewDropPosition);
    pub fn gtk_icon_view_get_dest_item_at_pos  (icon_view: *mut GtkIconView, drag_x: c_int, drag_y: c_int, path: *mut *mut GtkTreePath,
        pos: *mut enums::IconViewDropPosition);
    //pub fn gtk_icon_view_create_drag_icon      (icon_view: *mut GtkIconView, path: *mut GtkTreePath) -> *mut cairo_surface_t;

    //pub type GtkIconViewForeachFunc = fn(icon_view: *mut GtkIconView, path: *mut GtkTreePath, data: gpointer);

    //=========================================================================
    // GtkPaperSize                                                      NOT OK
    //=========================================================================
    pub fn gtk_paper_size_new                  (name: *const c_char) -> *mut GtkPaperSize;
    pub fn gtk_paper_size_new_from_ppd         (ppd_name: *const c_char, ppd_display_name: *const c_char, width: c_double, height: c_double) -> *mut GtkPaperSize;
    pub fn gtk_paper_size_new_custom           (name: *const c_char, display_name: *const c_char, width: c_double, height: c_double, unit: enums::Unit) -> *mut GtkPaperSize;
    pub fn gtk_paper_size_copy                 (other: *mut GtkPaperSize) -> *mut GtkPaperSize;
    pub fn gtk_paper_size_free                 (size: *mut GtkPaperSize);
    pub fn gtk_paper_size_is_equal             (size1: *mut GtkPaperSize, size2: *mut GtkPaperSize) -> gboolean;
    pub fn gtk_paper_size_get_paper_sizes      (include_custom: gboolean) -> *mut GList;
    pub fn gtk_paper_size_get_name             (size: *mut GtkPaperSize) -> *const c_char;
    pub fn gtk_paper_size_get_display_name     (size: *mut GtkPaperSize) -> *const c_char;
    pub fn gtk_paper_size_get_ppd_name         (size: *mut GtkPaperSize) -> *const c_char;
    pub fn gtk_paper_size_get_width            (size: *mut GtkPaperSize, unit: enums::Unit) -> c_double;
    pub fn gtk_paper_size_get_height           (size: *mut GtkPaperSize, unit: enums::Unit) -> c_double;
    pub fn gtk_paper_size_is_custom            (size: *mut GtkPaperSize) -> gboolean;
    pub fn gtk_paper_size_set_size             (size: *mut GtkPaperSize, width: c_double, height: c_double, unit: enums::Unit);
    pub fn gtk_paper_size_get_default_top_margin(size: *mut GtkPaperSize, unit: enums::Unit) -> c_double;
    pub fn gtk_paper_size_get_default_bottom_margin(size: *mut GtkPaperSize, unit: enums::Unit) -> c_double;
    pub fn gtk_paper_size_get_default_left_margin(size: *mut GtkPaperSize, unit: enums::Unit) -> c_double;
    pub fn gtk_paper_size_get_default_right_margin(size: *mut GtkPaperSize, unit: enums::Unit) -> c_double;
    pub fn gtk_paper_size_get_default          () -> *const c_char;
    //pub fn gtk_paper_size_new_from_key_file    (key_file: *mut GKeyFile, group_name: *const c_char, error: *mut *mut GError) -> *mut GtkPageSetup;
    //pub fn gtk_paper_size_to_key_file          (size: *mut GtkPaperSize, key_file: *mut GKeyFile, group_name: *const c_char) -> *mut GtkPageSetup;

    //=========================================================================
    // GtkPageSetup                                                      NOT OK
    //=========================================================================
    pub fn gtk_page_setup_new                  () -> *mut GtkPageSetup;
    pub fn gtk_page_setup_copy                 (other: *mut GtkPageSetup) -> *mut GtkPageSetup;
    pub fn gtk_page_setup_get_orientation      (setup: *mut GtkPageSetup) -> enums::PageOrientation;
    pub fn gtk_page_setup_set_orientation      (setup: *mut GtkPageSetup, orientation: enums::PageOrientation);
    pub fn gtk_page_setup_get_paper_size       (setup: *mut GtkPageSetup) -> *mut GtkPaperSize;
    pub fn gtk_page_setup_set_paper_size       (setup: *mut GtkPageSetup, size: *mut GtkPaperSize);
    pub fn gtk_page_setup_get_top_margin       (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_set_top_margin       (setup: *mut GtkPageSetup, margin: c_double, unit: enums::Unit);
    pub fn gtk_page_setup_get_bottom_margin    (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_set_bottom_margin    (setup: *mut GtkPageSetup, margin: c_double, unit: enums::Unit);
    pub fn gtk_page_setup_get_left_margin      (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_set_left_margin      (setup: *mut GtkPageSetup, margin: c_double, unit: enums::Unit);
    pub fn gtk_page_setup_get_right_margin     (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_set_right_margin     (setup: *mut GtkPageSetup, margin: c_double, unit: enums::Unit);
    pub fn gtk_page_setup_set_paper_size_and_default_margins(setup: *mut GtkPageSetup, size: *mut GtkPaperSize);
    pub fn gtk_page_setup_get_paper_width      (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_get_paper_height     (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_get_page_width       (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    pub fn gtk_page_setup_get_page_height      (setup: *mut GtkPageSetup, unit: enums::Unit) -> c_double;
    //pub fn gtk_page_setup_new_from_file        (file_name: *const c_char, error: *mut *mut GError) -> *mut GtkPageSetup;
    //pub fn gtk_page_setup_new_from_key_file    (key_file: *mut GKeyFile, group_name: *const c_char, error: *mut *mut GError) -> *mut GtkPageSetup;
    //pub fn gtk_page_setup_load_file            (setup: *mut GtkPageSetup, file_name: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_page_setup_load_key_file        (setup: *mut GtkPageSetup, key_file: *mut GKeyFile, group_name: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_page_setup_to_file              (setup: *mut GtkPageSetup, file_name: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_page_setup_to_key_file          (setup: *mut GtkPageSetup, key_file: *mut GKeyFile, group_name: *const c_char);

    //=========================================================================
    // GtkPrintSettings                                                  NOT OK
    //=========================================================================
    pub fn gtk_print_settings_new              () -> *mut GtkPrintSettings;
    pub fn gtk_print_settings_copy             (other: *mut GtkPrintSettings) -> *mut GtkPrintSettings;
    pub fn gtk_print_settings_has_key          (settings: *mut GtkPrintSettings, key: *const c_char) -> gboolean;
    pub fn gtk_print_settings_get              (settings: *mut GtkPrintSettings, key: *const c_char) -> *const c_char;
    pub fn gtk_print_settings_set              (settings: *mut GtkPrintSettings, key: *const c_char, value: *const c_char);
    pub fn gtk_print_settings_unset            (settings: *mut GtkPrintSettings, key: *const c_char);
    //pub fn gtk_print_settings_foreach          (settings: *mut GtkPrintSettings, func: GtkPrintSettingsFunc, user_data: *mut c_void);
    pub fn gtk_print_settings_get_bool         (settings: *mut GtkPrintSettings, key: *const c_char) -> gboolean;
    pub fn gtk_print_settings_set_bool         (settings: *mut GtkPrintSettings, key: *const c_char, value: gboolean);
    pub fn gtk_print_settings_get_double       (settings: *mut GtkPrintSettings, key: *const c_char) -> c_double;
    pub fn gtk_print_settings_set_double       (settings: *mut GtkPrintSettings, key: *const c_char, value: c_double);
    pub fn gtk_print_settings_get_double_with_default(settings: *mut GtkPrintSettings, key: *const c_char, def: c_double) -> c_double;
    pub fn gtk_print_settings_get_length       (settings: *mut GtkPrintSettings, key: *const c_char, unit: enums::Unit) -> c_double;
    pub fn gtk_print_settings_set_length       (settings: *mut GtkPrintSettings, key: *const c_char, value: c_double, unit: enums::Unit);
    pub fn gtk_print_settings_get_int          (settings: *mut GtkPrintSettings, key: *const c_char) -> c_int;
    pub fn gtk_print_settings_set_int          (settings: *mut GtkPrintSettings, key: *const c_char, value: c_int);
    pub fn gtk_print_settings_get_int_with_default(settings: *mut GtkPrintSettings, key: *const c_char, def: c_int) -> c_int;
    pub fn gtk_print_settings_get_printer      (settings: *mut GtkPrintSettings) -> *const c_char;
    pub fn gtk_print_settings_set_printer      (settings: *mut GtkPrintSettings, printer: *const c_char);
    pub fn gtk_print_settings_get_orientation  (settings: *mut GtkPrintSettings) -> enums::PageOrientation;
    pub fn gtk_print_settings_set_orientation  (settings: *mut GtkPrintSettings, orientation: enums::PageOrientation);
    pub fn gtk_print_settings_get_paper_size   (settings: *mut GtkPrintSettings) -> *mut GtkPaperSize;
    pub fn gtk_print_settings_set_paper_size   (settings: *mut GtkPrintSettings, paper_size: *mut GtkPaperSize);
    pub fn gtk_print_settings_get_paper_width  (settings: *mut GtkPrintSettings, unit: enums::Unit) -> c_double;
    pub fn gtk_print_settings_set_paper_width  (settings: *mut GtkPrintSettings, width: c_double, unit: enums::Unit);
    pub fn gtk_print_settings_get_paper_height (settings: *mut GtkPrintSettings, unit: enums::Unit) -> c_double;
    pub fn gtk_print_settings_set_paper_height (settings: *mut GtkPrintSettings, height: c_double, unit: enums::Unit);
    pub fn gtk_print_settings_get_use_color    (settings: *mut GtkPrintSettings) -> gboolean;
    pub fn gtk_print_settings_set_use_color    (settings: *mut GtkPrintSettings, use_color: gboolean);
    pub fn gtk_print_settings_get_collate      (settings: *mut GtkPrintSettings) -> gboolean;
    pub fn gtk_print_settings_set_collate      (settings: *mut GtkPrintSettings, collate: gboolean);
    pub fn gtk_print_settings_get_reverse      (settings: *mut GtkPrintSettings) -> gboolean;
    pub fn gtk_print_settings_set_reverse      (settings: *mut GtkPrintSettings, reverse: gboolean);
    //pub fn gtk_print_settings_get_duplex       (settings: *mut GtkPrintSettings) -> GtkPrintDuplex;
    //pub fn gtk_print_settings_set_duplex       (settings: *mut GtkPrintSettings, duplex: GtkPrintDuplex);
    //pub fn gtk_print_settings_get_quality      (settings: *mut GtkPrintSettings) -> GtkPrintQuality;
    //pub fn gtk_print_settings_set_quality      (settings: *mut GtkPrintSettings, quality: GtkPrintQuality);
    pub fn gtk_print_settings_get_n_copies     (settings: *mut GtkPrintSettings) -> c_int;
    pub fn gtk_print_settings_set_n_copies     (settings: *mut GtkPrintSettings, num_copies: c_int);
    pub fn gtk_print_settings_get_number_up    (settings: *mut GtkPrintSettings) -> enums::NumberUpLayout;
    pub fn gtk_print_settings_set_number_up    (settings: *mut GtkPrintSettings, number_up: enums::NumberUpLayout);
    pub fn gtk_print_settings_get_resolution   (settings: *mut GtkPrintSettings) -> c_int;
    pub fn gtk_print_settings_set_resolution   (settings: *mut GtkPrintSettings, resolution: c_int);
    pub fn gtk_print_settings_set_resolution_xy(settings: *mut GtkPrintSettings, resolution_x: c_int, resolution_y: c_int);
    pub fn gtk_print_settings_get_resolution_x (settings: *mut GtkPrintSettings) -> c_int;
    pub fn gtk_print_settings_get_resolution_y (settings: *mut GtkPrintSettings) -> c_int;
    pub fn gtk_print_settings_get_printer_lpi  (settings: *mut GtkPrintSettings) -> f64;
    pub fn gtk_print_settings_set_printer_lpi  (settings: *mut GtkPrintSettings, lpi: f64);
    pub fn gtk_print_settings_get_scale        (settings: *mut GtkPrintSettings) -> f64;
    pub fn gtk_print_settings_set_scale        (settings: *mut GtkPrintSettings, scale: f64);
    pub fn gtk_print_settings_get_print_pages  (settings: *mut GtkPrintSettings) -> enums::PrintPages;
    pub fn gtk_print_settings_set_print_pages  (settings: *mut GtkPrintSettings, pages: enums::PrintPages);
    //pub fn gtk_print_settings_get_page_ranges  (settings: *mut GtkPrintSettings, num_ranges: *mut c_int) -> *mut GtkPageRange;
    //pub fn gtk_print_settings_set_page_ranges  (settings: *mut GtkPrintSettings, page_ranges: *mut GtkPageRange, num_range: *mut c_int);
    pub fn gtk_print_settings_get_page_set     (settings: *mut GtkPrintSettings) -> enums::PageSet;
    pub fn gtk_print_settings_set_page_set     (settings: *mut GtkPrintSettings, page_set: enums::PageSet);
    pub fn gtk_print_settings_get_default_source(settings: *mut GtkPrintSettings) -> *const c_char;
    pub fn gtk_print_settings_set_default_source(settings: *mut GtkPrintSettings, default_source: *const c_char);
    pub fn gtk_print_settings_get_media_type   (settings: *mut GtkPrintSettings) -> *const c_char;
    pub fn gtk_print_settings_set_media_type   (settings: *mut GtkPrintSettings, media_type: *const c_char);
    pub fn gtk_print_settings_get_dither       (settings: *mut GtkPrintSettings) -> *const c_char;
    pub fn gtk_print_settings_set_dither       (settings: *mut GtkPrintSettings, dither: *const c_char);
    pub fn gtk_print_settings_get_finishings   (settings: *mut GtkPrintSettings) -> *const c_char;
    pub fn gtk_print_settings_set_finishings   (settings: *mut GtkPrintSettings, finishings: *const c_char);
    pub fn gtk_print_settings_get_output_bin   (settings: *mut GtkPrintSettings) -> *const c_char;
    pub fn gtk_print_settings_set_output_bin   (settings: *mut GtkPrintSettings, output_bin: *const c_char);
    //pub fn gtk_print_settings_new_from_file    (file_name: *const c_char, error: *mut *mut GError) -> *mut GtkPrintSettings;
    //pub fn gtk_print_settings_new_from_key_file(key_file: *mut GKeyFile, group_name: *const c_char, error: *mut *mut GError) -> *mut GtkPrintSettings;
    //pub fn gtk_print_settings_load_file        (settings: *mut GtkPrintSettings, file_name: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_print_settings_load_key_file    (settings: *mut GtkPrintSettings, key_file: *mut GKeyFile, group_name: *const c_char,
    //    error: *mut *mut GError) -> gboolean;
    //pub fn gtk_print_settings_to_file          (settings: *mut GtkPrintSettings, file_name: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_print_settings_to_key_file      (settings: *mut GtkPrintSettings, key_file: *mut GKeyFile, group_name: *const c_char) -> gboolean;

    //callbacks
    //let GtkPrintSettingsFunc = fn(key: *const c_char, value: *const c_char, user_data: *mut c_void);

    //=========================================================================
    // GtkPageSetupUnixDialog                                            NOT OK
    //=========================================================================
    /*pub fn gtk_page_setup_unix_dialog_new      (title: *const c_char, parent: *mut GtkWindow) -> *mut GtkWidget;
    pub fn gtk_page_setup_unix_dialog_set_page_setup(dialog: *mut GtkPageSetupUnixDialog, page_setup: *mut GtkPageSetup);
    pub fn gtk_page_setup_unix_dialog_get_page_setup(dialog: *mut GtkPageSetupUnixDialog) -> *mut GtkPageSetup;
    pub fn gtk_page_setup_unix_dialog_set_print_settings(dialog: *mut GtkPageSetupUnixDialog, print_settings: *mut GtkPrintSettings);
    pub fn gtk_page_setup_unix_dialog_get_print_settings(dialog: *mut GtkPageSetupUnixDialog) -> *mut GtkPrintSettings;*/

    //=========================================================================
    // GtkRecentChooser                                                  NOT OK
    //=========================================================================
    pub fn gtk_recent_chooser_get_type         () -> GType;
    pub fn gtk_recent_chooser_set_show_private (chooser: *mut GtkRecentChooser, show_private: gboolean);
    pub fn gtk_recent_chooser_get_show_private (chooser: *mut GtkRecentChooser) -> gboolean;
    pub fn gtk_recent_chooser_set_show_not_found(chooser: *mut GtkRecentChooser, show_not_found: gboolean);
    pub fn gtk_recent_chooser_get_show_not_found(chooser: *mut GtkRecentChooser) -> gboolean;
    pub fn gtk_recent_chooser_set_show_icons   (chooser: *mut GtkRecentChooser, show_icons: gboolean);
    pub fn gtk_recent_chooser_get_show_icons   (chooser: *mut GtkRecentChooser) -> gboolean;
    pub fn gtk_recent_chooser_set_select_multiple(chooser: *mut GtkRecentChooser, select_multiple: gboolean);
    pub fn gtk_recent_chooser_get_select_multiple(chooser: *mut GtkRecentChooser) -> gboolean;
    pub fn gtk_recent_chooser_set_local_only   (chooser: *mut GtkRecentChooser, local_only: gboolean);
    pub fn gtk_recent_chooser_get_local_only   (chooser: *mut GtkRecentChooser) -> gboolean;
    pub fn gtk_recent_chooser_set_limit        (chooser: *mut GtkRecentChooser, limit: c_int);
    pub fn gtk_recent_chooser_get_limit        (chooser: *mut GtkRecentChooser) -> c_int;
    pub fn gtk_recent_chooser_set_show_tips    (chooser: *mut GtkRecentChooser, show_tips: gboolean);
    pub fn gtk_recent_chooser_get_show_tips    (chooser: *mut GtkRecentChooser) -> gboolean;
    pub fn gtk_recent_chooser_set_sort_type    (chooser: *mut GtkRecentChooser, sort_type: enums::RecentSortType);
    pub fn gtk_recent_chooser_get_sort_type    (chooser: *mut GtkRecentChooser) -> enums::RecentSortType;
    //pub fn gtk_recent_chooser_set_sort_func    (chooser: *mut GtkRecentChooser, sort_func: RecentSortFunc, sort_data: *mut c_void, data_destroy: GDestroyNotify);
    //pub fn gtk_recent_chooser_set_current_uri  (chooser: *mut GtkRecentChooser, uri: *const c_char, error: *mut *mut GError);
    pub fn gtk_recent_chooser_get_current_uri  (chooser: *mut GtkRecentChooser) -> *mut c_char;
    pub fn gtk_recent_chooser_get_current_item (chooser: *mut GtkRecentChooser) -> *mut GtkRecentInfo;
    //pub fn gtk_recent_chooser_select_uri       (chooser: *mut GtkRecentChooser, uri: *const c_char, error: *mut *mut GError) -> gboolean;
    pub fn gtk_recent_chooser_unselect_uri     (chooser: *mut GtkRecentChooser, uri: *const c_char);
    pub fn gtk_recent_chooser_select_all       (chooser: *mut GtkRecentChooser);
    pub fn gtk_recent_chooser_unselect_all     (chooser: *mut GtkRecentChooser);
    pub fn gtk_recent_chooser_get_items        (chooser: *mut GtkRecentChooser) -> *mut GList;
    pub fn gtk_recent_chooser_get_uris         (chooser: *mut GtkRecentChooser, length: *mut c_long) -> *mut *mut c_char;
    pub fn gtk_recent_chooser_add_filter       (chooser: *mut GtkRecentChooser, filter: *mut GtkRecentFilter);
    pub fn gtk_recent_chooser_remove_filter    (chooser: *mut GtkRecentChooser, filter: *mut GtkRecentFilter);
    pub fn gtk_recent_chooser_list_filters     (chooser: *mut GtkRecentChooser) -> *mut GSList;
    pub fn gtk_recent_chooser_set_filter       (chooser: *mut GtkRecentChooser, filter: *mut GtkRecentFilter);
    pub fn gtk_recent_chooser_get_filter       (chooser: *mut GtkRecentChooser) -> *mut GtkRecentFilter;

    //callback
    //let GtkRecentSortFunc = fn(a: *mut GtkRecentInfo, b: *mut GtkRecentInfo, user_data: *mut c_void);

    //=========================================================================
    // GtkRecentFilter                                                   NOT OK
    //=========================================================================
    pub fn gtk_recent_filter_get_type          () -> GType;
    pub fn gtk_recent_filter_new               () -> *mut GtkRecentFilter;
    pub fn gtk_recent_filter_get_name          (filter: *mut GtkRecentFilter) -> *const c_char;
    pub fn gtk_recent_filter_set_name          (filter: *mut GtkRecentFilter, name: *const c_char);
    pub fn gtk_recent_filter_add_mime_type     (filter: *mut GtkRecentFilter, mime_type: *const c_char);
    pub fn gtk_recent_filter_add_pattern       (filter: *mut GtkRecentFilter, pattern: *const c_char);
    pub fn gtk_recent_filter_add_pixbuf_formats(filter: *mut GtkRecentFilter);
    pub fn gtk_recent_filter_add_application   (filter: *mut GtkRecentFilter, application: *const c_char);
    pub fn gtk_recent_filter_add_group         (filter: *mut GtkRecentFilter, group: *const c_char);
    pub fn gtk_recent_filter_add_age           (filter: *mut GtkRecentFilter, days: c_int);
    //pub fn gtk_recent_filter_add_custom        (filter: *mut GtkRecentFilter, needed: enums::RecentFilterFlags, func: GtkRecentFilterFunc,
        //data: *mut c_void, data_destroy: GDestroyNotify);
    pub fn gtk_recent_filter_get_needed        (filter: *mut GtkRecentFilter) -> enums::RecentFilterFlags;
    pub fn gtk_recent_filter_filter            (filter: *mut GtkRecentFilter, filter_info: *const GtkRecentFilterInfo) -> gboolean;

    //callback
    //let GtkRecentFilterFunc = fn(filter_info: *const GtkRecentFilterInfo, user_data: *mut c_void) -> gboolean;

    //=========================================================================
    // GtkRecentChooserDialog                                            NOT OK
    //=========================================================================
    pub fn gtk_recent_chooser_dialog_new       (title: *const c_char, parent: *mut GtkWindow, first_button_text: *const c_char, ...) -> *mut GtkWidget;
    pub fn gtk_recent_chooser_dialog_new_for_manager(title: *const c_char, parent: *mut GtkWindow, manager: *mut GtkRecentManager,
        first_button_text: *const c_char, ...) -> *mut GtkWidget;

    //=========================================================================
    // GtkRecentManager                                                  NOT OK
    //=========================================================================
    pub fn gtk_recent_manager_get_type         () -> GType;
    pub fn gtk_recent_manager_new              () -> *mut GtkRecentManager;
    pub fn gtk_recent_manager_get_default      () -> *mut GtkRecentManager;
    pub fn gtk_recent_manager_add_item         (manager: *mut GtkRecentManager, uri: *const c_char) -> gboolean;
    pub fn gtk_recent_manager_add_full         (manager: *mut GtkRecentManager, uri: *const c_char, recent_data: *const GtkRecentData) -> gboolean;
    //pub fn gtk_recent_manager_remove_item      (manager: *mut GtkRecentManager, uri: *const c_char, error: *mut *mut GError) -> gboolean;
    //pub fn gtk_recent_manager_lookup_item      (manager: *mut GtkRecentManager, uri: *const c_char, error: *mut *mut GError) -> *mut GtkRecentInfo;
    pub fn gtk_recent_manager_has_item         (manager: *mut GtkRecentManager, uri: *const c_char) -> gboolean;
    //pub fn gtk_recent_manager_lookup_item      (manager: *mut GtkRecentManager, uri: *const c_char, new_uri: *const c_char,
        //error: *mut *mut GError) -> *mut GtkRecentInfo;
    pub fn gtk_recent_manager_get_items        (manager: *mut GtkRecentManager) -> *mut GList;
    //pub fn gtk_recent_manager_purge_items      (manager: *mut GtkRecentManager, error: *mut *mut GError) -> gboolean;

    //=========================================================================
    // GtkRecentInfo                                                     NOT OK
    //=========================================================================
    pub fn gtk_recent_info_ref                 (info: *mut GtkRecentInfo) -> *mut GtkRecentInfo;
    pub fn gtk_recent_info_unref               (info: *mut GtkRecentInfo);
    pub fn gtk_recent_info_get_uri             (info: *mut GtkRecentInfo) -> *const c_char;
    pub fn gtk_recent_info_get_display_name    (info: *mut GtkRecentInfo) -> *const c_char;
    pub fn gtk_recent_info_get_description     (info: *mut GtkRecentInfo) -> *const c_char;
    pub fn gtk_recent_info_get_mime_type       (info: *mut GtkRecentInfo) -> *const c_char;
    pub fn gtk_recent_info_get_added           (info: *mut GtkRecentInfo) -> time_t;
    pub fn gtk_recent_info_get_modified        (info: *mut GtkRecentInfo) -> time_t;
    pub fn gtk_recent_info_get_visited         (info: *mut GtkRecentInfo) -> time_t;
    pub fn gtk_recent_info_get_private_hint    (info: *mut GtkRecentInfo) -> gboolean;
    pub fn gtk_recent_info_get_application_info(info: *mut GtkRecentInfo,
                                                app_name: *const c_char,
                                                app_exec: *mut *const c_char,
                                                count: *mut c_uint,
                                                time_: *mut time_t) -> gboolean;
    pub fn gtk_recent_info_get_applications    (info: *mut GtkRecentInfo, length: *mut c_long) -> *mut *mut c_char;
    pub fn gtk_recent_info_last_application    (info: *mut GtkRecentInfo) -> *mut c_char;
    pub fn gtk_recent_info_has_application     (info: *mut GtkRecentInfo, app_name: *const c_char) -> gboolean;
    //pub fn gtk_recent_info_create_app_info     (info: *mut GtkRecentInfo, app_name: *const c_char, error: *mut *mut GError) -> *mut GAppInfo;
    pub fn gtk_recent_info_get_groups          (info: *mut GtkRecentInfo, length: *mut c_long) -> *mut *mut c_char;
    pub fn gtk_recent_info_has_group           (info: *mut GtkRecentInfo, group_name: *const c_char) -> gboolean;
    //pub fn gtk_recent_info_get_icon            (info: *mut GtkRecentInfo, size: c_int) -> *mut enums::ffi::GdkPixbuf;
    //pub fn gtk_recent_info_get_gicon           (info: *mut GtkRecentInfo) -> *mut enums::ffi::GIcon;
    pub fn gtk_recent_info_get_short_name      (info: *mut GtkRecentInfo) -> *mut c_char;
    pub fn gtk_recent_info_get_uri_display     (info: *mut GtkRecentInfo) -> *mut c_char;
    pub fn gtk_recent_info_get_age             (info: *mut GtkRecentInfo) -> c_int;
    pub fn gtk_recent_info_is_local            (info: *mut GtkRecentInfo) -> gboolean;
    pub fn gtk_recent_info_exists              (info: *mut GtkRecentInfo) -> gboolean;
    pub fn gtk_recent_info_match               (info_a: *mut GtkRecentInfo, info_b: *mut GtkRecentInfo) -> gboolean;

    //=========================================================================
    // GtkFontChooser                                                    NOT OK
    //=========================================================================
    pub fn gtk_font_chooser_get_type           () -> GType;
    //pub fn gtk_font_chooser_get_font_family    (font_chooser: *mut GtkFontChooser) -> *mut PangoFontFamily;
    //pub fn gtk_font_chooser_get_font_face      (font_chooser: *mut GtkFontChooser) -> *mut PangoFontFace;
    pub fn gtk_font_chooser_get_font_size      (font_chooser: *mut GtkFontChooser) -> c_int;
    pub fn gtk_font_chooser_get_font           (font_chooser: *mut GtkFontChooser) -> *mut c_char;
    pub fn gtk_font_chooser_set_font           (font_chooser: *mut GtkFontChooser, font_name: *mut c_char);
    //pub fn gtk_font_chooser_get_font_desc      (font_chooser: *mut GtkFontChooser) -> *mut PangoFontDescription;
    //pub fn gtk_font_chooser_set_font_desc      (font_chooser: *mut GtkFontChooser, font_desc: *const PangoFontDescription;
    pub fn gtk_font_chooser_get_preview_text   (font_chooser: *mut GtkFontChooser) -> *mut c_char;
    pub fn gtk_font_chooser_set_preview_text   (font_chooser: *mut GtkFontChooser, text: *const c_char);
    pub fn gtk_font_chooser_get_show_preview_entry(font_chooser: *mut GtkFontChooser) -> gboolean;
    pub fn gtk_font_chooser_set_show_preview_entry(font_chooser: *mut GtkFontChooser, show_preview_entry: gboolean);
    //pub fn gtk_font_chooser_set_filter_func    (font_chooser: *mut GtkFontChooser, filter: GtkFontFilterFunc, user_data: *mut c_void,
        //destroy: GDestroyNotify);

    //function pointer
    //let GtkFontFilterFunc = fn(family: *const PangoFontFamily, face: *const PangoFontFace, data: *mut c_void) -> gboolean;

    //=========================================================================
    // GtkFontChooserDialog                                              NOT OK
    //=========================================================================
    pub fn gtk_font_chooser_dialog_new         (title: *const c_char, parent: *mut GtkWindow) -> *mut GtkWidget;

    //=========================================================================
    // GtkColorChooser                                                       OK
    //=========================================================================
    pub fn gtk_color_chooser_get_type          () -> GType;
    pub fn gtk_color_chooser_get_rgba          (chooser: *mut GtkColorChooser, color: *mut gdk_ffi::GdkRGBA) -> ();
    pub fn gtk_color_chooser_set_rgba          (chooser: *mut GtkColorChooser, color: *const gdk_ffi::GdkRGBA) -> ();
    pub fn gtk_color_chooser_get_use_alpha     (chooser: *mut GtkColorChooser) -> gboolean;
    pub fn gtk_color_chooser_set_use_alpha     (chooser: *mut GtkColorChooser, use_alpha: gboolean) -> ();
    pub fn gtk_color_chooser_add_palette       (chooser: *mut GtkColorChooser, orientation: enums::Orientation, colors_per_line: c_int, n_colors: c_int, colors: *const gdk_ffi::GdkRGBA);

    //=========================================================================
    // GtkColorChooserDialog                                                 OK
    //=========================================================================
    pub fn gtk_color_chooser_dialog_new        (title: *const c_char, parent: *mut GtkWindow) -> *mut GtkWidget;

    //=========================================================================
    // GtkComboBox                                                       NOT OK
    //=========================================================================
    pub fn gtk_combo_box_new                   () -> *mut GtkWidget;
    pub fn gtk_combo_box_new_with_entry        () -> *mut GtkWidget;
    pub fn gtk_combo_box_new_with_model        (model: *mut GtkTreeModel) -> *mut GtkWidget;
    pub fn gtk_combo_box_new_with_model_and_entry(model: *mut GtkTreeModel) -> *mut GtkWidget;
    //pub fn gtk_combo_box_new_with_area         (area: *mut GtkCellArea) -> *mut GtkWidget;
    //pub fn gtk_combo_box_new_with_area_and_entry(area: *mut GtkCellArea) -> *mut GtkWidget;
    pub fn gtk_combo_box_get_wrap_width        (combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_wrap_width        (combo_box: *mut GtkComboBox, width: c_int);
    pub fn gtk_combo_box_get_row_span_column   (combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_row_span_column   (combo_box: *mut GtkComboBox, row_span: c_int);
    pub fn gtk_combo_box_get_column_span_column(combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_column_span_column(combo_box: *mut GtkComboBox, column_span: c_int);
    pub fn gtk_combo_box_get_active            (combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_active            (combo_box: *mut GtkComboBox, active: c_int);
    pub fn gtk_combo_box_get_active_iter       (combo_box: *mut GtkComboBox) -> *mut GtkTreeIter;
    pub fn gtk_combo_box_set_active_iter       (combo_box: *mut GtkComboBox, iter: *mut GtkTreeIter);
    pub fn gtk_combo_box_get_id_column         (combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_id_column         (combo_box: *mut GtkComboBox, id_column: c_int);
    pub fn gtk_combo_box_get_active_id         (combo_box: *mut GtkComboBox) -> *const c_char;
    pub fn gtk_combo_box_set_active_id         (combo_box: *mut GtkComboBox, active_id: *const c_char) -> gboolean;
    pub fn gtk_combo_box_get_model             (combo_box: *mut GtkComboBox) -> *mut GtkTreeModel;
    pub fn gtk_combo_box_set_model             (combo_box: *mut GtkComboBox, model: *mut GtkTreeModel);
    //pub fn gtk_combo_box_popup_for_device      (combo_box: *mut GtkComboBox, device: *mut gdk::GdkDevice);
    pub fn gtk_combo_box_popup                 (combo_box: *mut GtkComboBox);
    pub fn gtk_combo_box_popdown               (combo_box: *mut GtkComboBox);
    //pub fn gtk_combo_box_get_popup_accessible  (combo_box: *mut GtkComboBox) -> *mut AtkObject;
    //pub fn gtk_combo_box_get_row_separator_func(combo_box: *mut GtkComboBox) -> GtkTreeViewRowSeparatorFunc;
    //pub fn gtk_combo_box_set_row_separator_func(combo_box: *mut GtkComboBox, func: GtkTreeViewRowSeparatorFunc,
    //    data: gpointer, destroy: GDestroyNotify);
    pub fn gtk_combo_box_get_focus_on_click    (combo_box: *mut GtkComboBox) -> gboolean;
    pub fn gtk_combo_box_set_focus_on_click    (combo_box: *mut GtkComboBox, focus_on_click: gboolean);
    pub fn gtk_combo_box_get_button_sensitivity(combo_box: *mut GtkComboBox) -> enums::SensitivityType;
    pub fn gtk_combo_box_set_button_sensitivity(combo_box: *mut GtkComboBox, sensitivity: enums::SensitivityType);
    pub fn gtk_combo_box_get_has_entry         (combo_box: *mut GtkComboBox) -> gboolean;
    pub fn gtk_combo_box_get_entry_text_column (combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_entry_text_column (combo_box: *mut GtkComboBox, text_column: c_int);
    pub fn gtk_combo_box_get_popup_fixed_width (combo_box: *mut GtkComboBox) -> c_int;
    pub fn gtk_combo_box_set_popup_fixed_width (combo_box: *mut GtkComboBox, fixed: c_int);

    //pub type GtkTreeViewRowSeparatorFunc = fn(model: *mut GtkTreeModel, iter: *mut GtkTreeIter, data: gpointer) -> gboolean;

    //=========================================================================
    // GtkComboBoxText                                                   NOT OK
    //=========================================================================
    pub fn gtk_combo_box_text_new              () -> *mut GtkWidget;
    pub fn gtk_combo_box_text_new_with_entry   () -> *mut GtkWidget;
    pub fn gtk_combo_box_text_append           (combo_box: *mut GtkComboBoxText, id: *const c_char, text: *const c_char);
    pub fn gtk_combo_box_text_prepend          (combo_box: *mut GtkComboBoxText, id: *const c_char, text: *const c_char);
    pub fn gtk_combo_box_text_insert           (combo_box: *mut GtkComboBoxText, position: c_int, id: *const c_char, text: *const c_char);
    pub fn gtk_combo_box_text_append_text      (combo_box: *mut GtkComboBoxText, text: *const c_char);
    pub fn gtk_combo_box_text_prepend_text     (combo_box: *mut GtkComboBoxText, text: *const c_char);
    pub fn gtk_combo_box_text_insert_text      (combo_box: *mut GtkComboBoxText, position: c_int, text: *const c_char);
    pub fn gtk_combo_box_text_remove           (combo_box: *mut GtkComboBoxText, position: c_int);
    pub fn gtk_combo_box_text_remove_all       (combo_box: *mut GtkComboBoxText);
    pub fn gtk_combo_box_text_get_active_text  (combo_box: *mut GtkComboBoxText) -> *mut c_char; // to free

    //=========================================================================
    // GtkAppLaunchContext                                               NOT OK
    //=========================================================================
    /// Error : /usr/lib/x86_64-linux-gnu/libgio-2.0.so.0: error adding symbols: DSO missing from command line
    /*pub fn g_app_launch_context_new            () -> *mut GAppLaunchContext;
    pub fn g_app_launch_context_setenv         (launch_context: *mut GAppLaunchContext, variable: *c_char, value: *c_char) -> ();
    pub fn g_app_launch_context_unsetenv       (launch_context: *mut GAppLaunchContext, variable: *c_char) -> ();
    pub fn g_app_launch_context_get_environment(launch_context: *mut GAppLaunchContext) -> **c_char;
    //pub fn g_app_launch_context_get_display    (launch_context: *mut GAppLaunchContext, app_info: *mut GAppInfo, files: *mut GList) -> *c_char;
    //pub fn g_app_launch_context_get_startup_notify_id(launch_context: *mut GAppLaunchContext, app_info: *mut GAppInfo, files: *mut GList) -> *c_char;
    pub fn g_app_launch_context_launch_failed  (launch_context: *mut GAppLaunchContext, startup_notify_id: *c_char) -> ();*/

    //=========================================================================
    // GtkAppInfo                                                        NOT OK
    //=========================================================================
    /// Error : /usr/lib/x86_64-linux-gnu/libgio-2.0.so.0: error adding symbols: DSO missing from command line
    /*pub fn g_app_info_create_from_commandline  (commande_line: *c_char, application_name: *c_char, flag: enums::AppInfoCreateFlags, error: *mut *mut GError) -> *mut GAppInfo;
    pub fn g_app_info_dup                      (app_info: *mut GAppInfo) -> *mut GAppInfo;
    pub fn g_app_info_equal                    (app_info1: *mut GAppInfo, app_info2: *mut GAppInfo) -> gboolean;
    pub fn g_app_info_get_id                   (app_info: *mut GAppInfo) -> *c_char;
    pub fn g_app_info_get_name                 (app_info: *mut GAppInfo) -> *c_char;
    pub fn g_app_info_get_display_name         (app_info: *mut GAppInfo) -> *c_char;
    pub fn g_app_info_get_description          (app_info: *mut GAppInfo) -> *c_char;
    pub fn g_app_info_get_executable           (app_info: *mut GAppInfo) -> *c_char;
    pub fn g_app_info_get_commandline          (app_info: *mut GAppInfo) -> *c_char;
    //pub fn g_app_info_get_icon                 (app_info: *mut GAppInfo) -> *mut GIcon;
    //pub fn g_app_info_launch                   (app_info: *mut GAppInfo, files: *mut GList, launch_context: *mut GAppLaunchContext, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_supports_files           (app_info: *mut GAppInfo) -> gboolean;
    pub fn g_app_info_supports_uris            (app_info: *mut GAppInfo) -> gboolean;
    //pub fn g_app_info_launch_uris              (app_info: *mut GAppInfo, uris: *mut GList, launch_context: *mut GAppLaunchContext, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_should_show              (app_info: *mut GAppInfo) -> gboolean;
    pub fn g_app_info_can_delete               (app_info: *mut GAppInfo) -> gboolean;
    pub fn g_app_info_delete                   (app_info: *mut GAppInfo) -> gboolean;
    pub fn g_app_info_reset_type_associations  (content_type: *c_char) -> ();
    pub fn g_app_info_set_as_default_for_type  (app_info: *mut GAppInfo, content_type: *c_char, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_set_as_default_for_extension(app_info: *mut GAppInfo, extension: *c_char, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_set_as_last_used_for_type(app_info: *mut GAppInfo, content_type: *c_char, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_add_supports_type        (app_info: *mut GAppInfo, content_type: *c_char, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_can_remove_supports_type (app_info: *mut GAppInfo) -> gboolean;
    pub fn g_app_info_remove_supports_type     (app_info: *mut GAppInfo, content_type: *c_char, error: *mut *mut GError) -> gboolean;
    pub fn g_app_info_get_supported_types      (app_info: *mut GAppInfo) -> **c_char;
    //pub fn g_app_info_get_all                  () -> *mut GList;
    //pub fn g_app_info_get_all_for_type         (content_type: *c_char) -> *mut GList;
    pub fn g_app_info_get_default_for_type     (content_type: *c_char, must_support_uris: gboolean) -> *mut GAppInfo;
    pub fn g_app_info_get_default_for_uri_scheme(uri_scheme: *c_char) -> *mut GAppInfo;
    //pub fn g_app_info_get_fallback_for_type    (content_type: *c_char) -> *mut GList;
    //pub fn g_app_info_get_recommended_for_type (content_type: *c_char) -> *mut GList;
    pub fn g_app_info_launch_default_for_uri   (uri: *c_char, launch_context: *mut GAppLaunchContext, error: *mut *mut GError) -> gboolean;*/

    //=========================================================================
    // GtkBuildable                                                      NOT OK
    //=========================================================================
    pub fn gtk_buildable_get_type              () -> GType;
    pub fn gtk_buildable_set_name              (buildable: *mut GtkBuildable, name: *const c_char);
    pub fn gtk_buildable_get_name              (buildable: *mut GtkBuildable) -> *const c_char;
    //pub fn gtk_buildable_add_child             (buildable: *mut GtkBuildable, builder: *mut GtkBuilder, child: *mut GObject, _type: *const c_char);
    //pub fn gtk_buildable_set_buildable_property(buildable: *mut GtkBuildable, builder: *mut GtkBuilder, name: *const c_char, value: *const GValue);
    //pub fn gtk_buildable_construct_child       (buildable: *mut GtkBuildable, builder: *mut GtkBuilder, name: *const c_char) -> *mut GObject;
    //pub fn gtk_buildable_custom_tag_start      (buildable: *mut GtkBuildable, builder: *mut GtkBuilder, child: *mut GObject, tag_name: *const c_char,
        //parser: *mut GMarkupParser, data: *mut c_void) -> gboolean;
    //pub fn gtk_buildable_custom_tag_end        (buildable: *mut GtkBuildable, builder: *mut GtkBuilder, child: *mut GObject, tag_name: *const c_char,
        //data: *mut c_void);
    //pub fn gtk_buildable_custom_tag_finished   (buildable: *mut GtkBuildable, builder: *mut GtkBuilder, child: *mut GObject, tag_name: *const c_char,
        //data: *mut c_void);
    pub fn gtk_buildable_parser_finished       (buildable: *mut GtkBuildable, builder: *mut GtkBuilder);
    pub fn gtk_buildable_get_internal_child    (buildable: *mut GtkBuildable, builder: *mut GtkBuilder, child_name: *const c_char);

    //=========================================================================
    // GtkAppChooser                                                         OK
    //=========================================================================
    pub fn gtk_app_chooser_get_type            () -> GType;
    pub fn gtk_app_chooser_get_app_info        (_self: *mut GtkAppChooser) -> *mut glib_ffi::GAppInfo;
    pub fn gtk_app_chooser_get_content_type    (_self: *mut GtkAppChooser) -> *const c_char;
    pub fn gtk_app_chooser_refresh             (_self: *mut GtkAppChooser) -> ();

    //=========================================================================
    // GtkAppChooserDialog                                               NOT OK
    //=========================================================================
    //pub fn gtk_app_chooser_dialog_new          (title: *const c_char, flags: enums::DialogFlags, file: *const const Gfile) -> *const const GtkWidget;
    pub fn gtk_app_chooser_dialog_new_for_content_type(parent: *mut GtkWindow, flags: enums::DialogFlags, content_type: *const c_char) -> *mut GtkWidget;
    pub fn gtk_app_chooser_dialog_get_widget   (_self: *mut GtkAppChooserDialog) -> *mut GtkWidget;
    pub fn gtk_app_chooser_dialog_set_heading  (_self: *mut GtkAppChooserDialog, heading: *const c_char) -> ();
    pub fn gtk_app_chooser_dialog_get_heading  (_self: *mut GtkAppChooserDialog) -> *const c_char;

    //=========================================================================
    // GtkMessageDialog                                                  NOT OK
    //=========================================================================
    pub fn gtk_message_dialog_get_type         () -> GType;
    pub fn gtk_message_dialog_new              (parent: *mut GtkWindow, flags: enums::DialogFlags, _type: enums::MessageType, buttons: enums::ButtonsType,
        message_format: *const c_char, ...) -> *mut GtkWidget;
    pub fn gtk_message_dialog_new_with_markup  (parent: *mut GtkWindow, flags: enums::DialogFlags, _type: enums::MessageType, buttons: enums::ButtonsType,
        message_format: *const c_char, ...) -> *mut GtkWidget;
    pub fn gtk_message_dialog_set_markup       (message_dialog: *mut GtkMessageDialog, str: *const c_char) -> ();
    //pub fn gtk_message_dialog_format_secondary_text(message_dialog: *const const GtkMessageDialog, message_format: *const c_char, ...) -> ();
    pub fn gtk_message_dialog_format_secondary_markup(message_dialog: *mut GtkMessageDialog, message_format: *const c_char, ...) -> ();
    pub fn gtk_message_dialog_get_message_area (message_dialog: *mut GtkMessageDialog) -> *mut GtkWidget;

    //=========================================================================
    // GtkDialog                                                         NOT OK
    //=========================================================================
    pub fn gtk_dialog_get_type                 () -> GType;
    pub fn gtk_dialog_new                      () -> *mut GtkWidget;
    pub fn gtk_dialog_new_with_buttons         (title: *const c_char,
                                                parent: *mut GtkWindow,
                                                flags: enums::DialogFlags,
                                                first_button_text: *const c_char,
                                                ...) -> *mut GtkWidget;
    pub fn gtk_dialog_run                      (dialog: *mut GtkDialog) -> i32;
    pub fn gtk_dialog_response                 (dialog: *mut GtkDialog, response_id: i32) -> ();
    pub fn gtk_dialog_add_button               (dialog: *mut GtkDialog, button_text: *const c_char, response_id: i32) -> *mut GtkWidget;
    pub fn gtk_dialog_add_buttons              (dialog: *mut GtkDialog,
                                                first_button_text: *const c_char,
                                                ...);
    pub fn gtk_dialog_add_action_widget        (dialog: *mut GtkDialog, child: *mut GtkWidget, response_id: i32) -> ();
    pub fn gtk_dialog_set_default_response     (dialog: *mut GtkDialog, response_id: i32) -> ();
    pub fn gtk_dialog_set_response_sensitive   (dialog: *mut GtkDialog, response_id: i32, setting: gboolean) -> ();
    pub fn gtk_dialog_get_response_for_widget  (dialog: *mut GtkDialog, widget: *mut GtkWidget) -> i32;
    pub fn gtk_dialog_get_widget_for_response  (dialog: *mut GtkDialog, response_id: i32) -> *mut GtkWidget;
    pub fn gtk_dialog_get_action_area          (dialog: *mut GtkDialog) -> *mut GtkWidget;
    pub fn gtk_dialog_get_content_area         (dialog: *mut GtkDialog) -> *mut GtkWidget;
    pub fn gtk_dialog_get_header_bar           (dialog: *mut GtkDialog) -> *mut GtkWidget;

    //=========================================================================
    // GtkAboutDialog                                                    NOT OK
    //=========================================================================
    pub fn gtk_about_dialog_get_type           () -> GType;
    pub fn gtk_about_dialog_new                () -> *mut GtkWidget;
    //pub fn gtk_show_about_dialog               (parent: *GtkWindow, first_property_name: *const c_char, ...) -> ();
    pub fn gtk_about_dialog_get_program_name   (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_program_name   (about: *mut GtkAboutDialog, name: *const c_char) -> ();
    pub fn gtk_about_dialog_get_version        (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_version        (about: *mut GtkAboutDialog, version: *const c_char) -> ();
    pub fn gtk_about_dialog_get_copyright      (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_copyright      (about: *mut GtkAboutDialog, copyright: *const c_char) -> ();
    pub fn gtk_about_dialog_get_comments       (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_comments       (about: *mut GtkAboutDialog, comments: *const c_char) -> ();
    pub fn gtk_about_dialog_get_license        (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_license        (about: *mut GtkAboutDialog, comments: *const c_char) -> ();
    pub fn gtk_about_dialog_get_license_type   (about: *mut GtkAboutDialog) -> enums::License;
    pub fn gtk_about_dialog_set_license_type   (about: *mut GtkAboutDialog, license_type: enums::License) -> ();
    pub fn gtk_about_dialog_get_wrap_license   (about: *mut GtkAboutDialog) -> gboolean;
    pub fn gtk_about_dialog_set_wrap_license   (about: *mut GtkAboutDialog, wrap_license: gboolean) -> ();
    pub fn gtk_about_dialog_get_website        (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_website        (about: *mut GtkAboutDialog, website: *const c_char) -> ();
    pub fn gtk_about_dialog_get_website_label  (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_website_label  (about: *mut GtkAboutDialog, website_label: *const c_char) -> ();
    pub fn gtk_about_dialog_get_authors        (about: *mut GtkAboutDialog) -> *const *const c_char;
    pub fn gtk_about_dialog_set_authors        (about: *mut GtkAboutDialog, authors: *const *const c_char) -> ();
    pub fn gtk_about_dialog_get_documenters    (about: *mut GtkAboutDialog) -> *const *const c_char;
    pub fn gtk_about_dialog_set_documenters    (about: *mut GtkAboutDialog, documents: *const *const c_char) -> ();
    pub fn gtk_about_dialog_get_artists        (about: *mut GtkAboutDialog) -> *const *const c_char;
    pub fn gtk_about_dialog_set_artists        (about: *mut GtkAboutDialog, artists: *const *const c_char) -> ();
    pub fn gtk_about_dialog_get_translator_credits(about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_translator_credits(about: *mut GtkAboutDialog, translator_credits: *const c_char) -> ();
    //pub fn gtk_about_dialog_get_logo           (about: *const const GtkAboutDialog) -> *const const GdkPixbuf;
    //pub fn gtk_about_dialog_set_logo           (about: *const const GtkAboutDialog, logo: *const const GdkPixbuf) -> ();
    pub fn gtk_about_dialog_get_logo_icon_name (about: *mut GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_logo_icon_name (about: *mut GtkAboutDialog, icon_name: *const c_char) -> ();
    pub fn gtk_about_dialog_add_credit_section (about: *mut GtkAboutDialog, section_name: *const c_char, people: *const *const c_char) -> ();

    //=========================================================================
    // GtkFontButton                                                         OK
    //=========================================================================
    pub fn gtk_font_button_new                 () -> *mut GtkWidget;
    pub fn gtk_font_button_new_with_font       (fontname: *const c_char) -> *mut GtkWidget;
    pub fn gtk_font_button_set_font_name       (font_button: *mut GtkFontButton, fontname: *const c_char) -> gboolean;
    pub fn gtk_font_button_get_font_name       (font_button: *mut GtkFontButton) -> *const c_char;
    pub fn gtk_font_button_set_show_style      (font_button: *mut GtkFontButton, show_style: gboolean) -> ();
    pub fn gtk_font_button_get_show_style      (font_button: *mut GtkFontButton) -> gboolean;
    pub fn gtk_font_button_set_show_size       (font_button: *mut GtkFontButton, show_size: gboolean) -> ();
    pub fn gtk_font_button_get_show_size       (font_button: *mut GtkFontButton) -> gboolean;
    pub fn gtk_font_button_set_use_font        (font_button: *mut GtkFontButton, use_font: gboolean) -> ();
    pub fn gtk_font_button_get_use_font        (font_button: *mut GtkFontButton) -> gboolean;
    pub fn gtk_font_button_set_use_size        (font_button: *mut GtkFontButton, use_size: gboolean) -> ();
    pub fn gtk_font_button_get_use_size        (font_button: *mut GtkFontButton) -> gboolean;
    pub fn gtk_font_button_set_title           (font_button: *mut GtkFontButton, title: *const c_char) -> ();
    pub fn gtk_font_button_get_title           (font_button: *mut GtkFontButton) -> *const c_char;

    //=========================================================================
    // GtkToggleButton                                                       OK
    //=========================================================================
    pub fn gtk_toggle_button_new               () -> *mut GtkWidget;
    pub fn gtk_toggle_button_new_with_label    (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_toggle_button_new_with_mnemonic (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_toggle_button_set_mode          (toggle_button: *mut GtkToggleButton, draw_indicator: gboolean) -> ();
    pub fn gtk_toggle_button_get_mode          (toggle_button: *mut GtkToggleButton) -> gboolean;
    pub fn gtk_toggle_button_toggled           (toggle_button: *mut GtkToggleButton) -> ();
    pub fn gtk_toggle_button_get_active        (toggle_button: *mut GtkToggleButton) -> gboolean;
    pub fn gtk_toggle_button_set_active        (toggle_button: *mut GtkToggleButton, is_active: gboolean) -> ();
    pub fn gtk_toggle_button_get_inconsistent  (toggle_button: *mut GtkToggleButton) -> gboolean;
    pub fn gtk_toggle_button_set_inconsistent  (toggle_button: *mut GtkToggleButton, setting: gboolean) -> ();

    //=========================================================================
    // GtkCheckButton                                                        OK
    //=========================================================================
    pub fn gtk_check_button_new                () -> *mut GtkWidget;
    pub fn gtk_check_button_new_with_label     (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_check_button_new_with_mnemonic  (label: *const c_char) -> *mut GtkWidget;

    //=========================================================================
    // GtkRadioButton                                                      TODO
    //=========================================================================
    // pub fn gtk_radio_button_new                (GSList *group) -> *const const GtkWidget;
    // pub fn gtk_radio_button_new_from_widget    (GtkRadioButton *radio_group_member) -> *const const GtkWidget;
    // pub fn gtk_radio_button_new_with_label     (GSList *group, const gchar *label) -> *const const GtkWidget;
    // pub fn tk_radio_button_new_with_label_from_widget (GtkRadioButton *radio_group_member, const gchar *label) -> *const const GtkWidget;
    // pub fn gtk_radio_button_new_with_mnemonic  (GSList *group, const gchar *label) -> *const const GtkWidget;
    // pub fn gtk_radio_button_new_with_mnemonic_from_widget(GtkRadioButton *radio_group_member,  const gchar *label) -> *const const GtkWidget;
    // pub fn gtk_radio_button_set_group          (GtkRadioButton *radio_button, GSList *group) -> ();
    // pub fn gtk_radio_button_get_group          (GtkRadioButton *radio_button) -> *GSList;
    // pub fn gtk_radio_button_join_group         (GtkRadioButton *radio_button, GtkRadioButton *group_source) -> ();

    //=========================================================================
    // GtkMenuButton                                                         OK
    //=========================================================================
    pub fn gtk_menu_button_new                 () -> *mut GtkWidget;
    pub fn gtk_menu_button_set_popup           (menu_button: *mut GtkMenuButton, popup: *mut GtkWidget) -> ();
    // pub fn gtk_menu_button_get_popup           (menu_button: *const const GtkMenuButton) -> *const const GtkMenu;
    // pub fn gtk_menu_button_set_menu_model      (menu_button: *const const GtkMenuButton, menu_model: *const const GMenuModel) -> ();
    // pub fn gtk_menu_button_get_menu_model      (menu_button: *const const GtkMenuButton) -> GMenuModel;
    pub fn gtk_menu_button_set_direction       (menu_button: *mut GtkMenuButton, direction: enums::ArrowType) -> ();
    pub fn gtk_menu_button_get_direction       (menu_button: *mut GtkMenuButton) -> enums::ArrowType;
    pub fn gtk_menu_button_set_align_widget    (menu_button: *mut GtkMenuButton, align_widget: *mut GtkWidget) -> ();
    // pub fn gtk_menu_button_get_align_widget    (menu_button: *const const GtkMenuButton) -> *const const GtkWidget;

    //=========================================================================
    // GtkColorButton                                                        OK
    //=========================================================================
    pub fn gtk_color_button_new                () -> *mut GtkWidget;
    pub fn gtk_color_button_new_with_color     (color: *const gdk_ffi::GdkColor) -> *mut GtkWidget;
    pub fn gtk_color_button_new_with_rgba      (rgba: *const gdk_ffi::GdkRGBA) -> *mut GtkWidget;
    pub fn gtk_color_button_set_color          (button: *mut GtkColorButton, color: *const gdk_ffi::GdkColor) -> ();
    pub fn gtk_color_button_get_color          (button: *mut GtkColorButton, color: *const gdk_ffi::GdkColor) -> ();
    pub fn gtk_color_button_set_alpha          (button: *mut GtkColorButton, alpha: u16) -> ();
    pub fn gtk_color_button_get_alpha          (button: *mut GtkColorButton) -> u16;
    pub fn gtk_color_button_set_rgba           (button: *mut GtkColorButton, rgba: *const gdk_ffi::GdkRGBA) -> ();
    pub fn gtk_color_button_get_rgba           (button: *mut GtkColorButton, rgba: *const gdk_ffi::GdkRGBA) -> ();
    pub fn gtk_color_button_set_use_alpha      (button: *mut GtkColorButton, use_alpha: gboolean) -> ();
    pub fn gtk_color_button_get_use_alpha      (button: *mut GtkColorButton) -> gboolean;
    pub fn gtk_color_button_set_title          (button: *mut GtkColorButton, title: *const c_char) -> ();
    pub fn gtk_color_button_get_title          (button: *mut GtkColorButton) -> *const c_char;

    //=========================================================================
    // GtkLinkButton                                                         OK
    //=========================================================================
    pub fn gtk_link_button_new                 (uri: *const c_char) -> *mut GtkWidget;
    pub fn gtk_link_button_new_with_label      (uri: *const c_char, label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_link_button_get_uri             (link_button: *mut GtkLinkButton) -> *const c_char;
    pub fn gtk_link_button_set_uri             (link_button: *mut GtkLinkButton, uri: *const c_char) -> ();
    pub fn gtk_link_button_get_visited         (link_button: *mut GtkLinkButton) -> gboolean;
    pub fn gtk_link_button_set_visited         (link_button: *mut GtkLinkButton, visited: gboolean) -> ();

    //=========================================================================
    // GtkScaleButton
    //=========================================================================
    pub fn gtk_scale_button_new                (size: enums::IconSize, min: c_double, max: c_double, step: c_double, icons: *const *const c_char) -> *mut GtkWidget;
    pub fn gtk_scale_button_set_adjustment     (button: *mut GtkScaleButton, adjustment: *mut GtkAdjustment) -> ();
    // pub fn gtk_scale_button_set_icons          (button: *const const GtkScaleButton, icons: *const *const c_char) -> ();
    pub fn gtk_scale_button_set_value          (button: *mut GtkScaleButton, value: c_double) -> ();
    pub fn gtk_scale_button_get_adjustment     (button: *mut GtkScaleButton) -> *mut GtkAdjustment;
    pub fn gtk_scale_button_get_value          (button: *mut GtkScaleButton) -> c_double;
    // pub fn gtk_scale_button_get_popup          (button: *const const GtkScaleButton) -> *const const GtkWidget;
    // pub fn gtk_scale_button_get_plus_button    (button: *const const GtkScaleButton) -> *const const GtkWidget;
    // pub fn gtk_scale_button_get_minus_button   (button: *const const GtkScaleButton) -> *const const GtkWidget;

    //=========================================================================
    // GtkVolumeButton                                                       OK
    //=========================================================================
    pub fn gtk_volume_button_new               () -> *mut GtkWidget;

    //=========================================================================
    // GtkBox
    //=========================================================================
    pub fn gtk_box_get_type                    () -> GType;
    pub fn gtk_box_new                         (orientation: enums::Orientation, spacing: c_int) -> *mut GtkWidget;
    pub fn gtk_box_pack_start                  (gbox: *mut GtkBox, child: *mut GtkWidget, expand: gboolean, fill: gboolean, padding: c_uint) -> ();
    pub fn gtk_box_pack_end                    (gbox: *mut GtkBox, child: *mut GtkWidget, expand: gboolean, fill: gboolean, padding: c_uint) -> ();
    pub fn gtk_box_get_homogeneous             (gbox: *mut GtkBox) -> gboolean;
    pub fn gtk_box_set_homogeneous             (gbox: *mut GtkBox, homogeneous: gboolean) -> ();
    pub fn gtk_box_get_spacing                 (gbox: *mut GtkBox) -> c_int;
    pub fn gtk_box_set_spacing                 (gbox: *mut GtkBox, spacing: c_int) -> ();
    pub fn gtk_box_reorder_child               (gbox: *mut GtkBox, child: *mut GtkWidget, position: c_int) -> ();
    pub fn gtk_box_query_child_packing         (gbox: *mut GtkBox, child: *mut GtkWidget, expand: *mut gboolean, fill: *mut gboolean, padding: *mut c_uint, pack_type: *mut enums::PackType) -> ();
    pub fn gtk_box_set_child_packing           (gbox: *mut GtkBox, child: *mut GtkWidget, expand: gboolean, fill: gboolean, padding: c_uint, pack_type: enums::PackType) -> ();
    // pub fn gtk_box_get_baseline_position       (gbox: *const const GtkBox) -> enums::BaselinePosition;
    // pub fn gtk_box_set_baseline_position       (gbox: *const const GtkBox, position: enums::BaselinePosition) -> ();

    //=========================================================================
    // GtkOrientable                                                         OK
    //=========================================================================
    pub fn gtk_orientable_get_type             () -> GType;
    pub fn gtk_orientable_get_orientation      (orientable: *mut GtkOrientable) -> enums::Orientation;
    pub fn gtk_orientable_set_orientation      (orientable: *mut GtkOrientable,  orientation: enums::Orientation) -> ();

    //=========================================================================
    // GtkButtonBox                                                          OK
    //=========================================================================
    pub fn gtk_button_box_new                  (orientation: enums::Orientation) -> *mut GtkWidget;
    pub fn gtk_button_box_get_layout           (widget: *mut GtkButtonBox) -> enums::ButtonBoxStyle;
    pub fn gtk_button_box_get_child_secondary  (widget: *mut GtkButtonBox, child : *mut GtkWidget) -> gboolean;
    pub fn gtk_button_box_get_child_non_homogeneous(Gwidget: *mut GtkButtonBox, child : *mut GtkWidget) -> gboolean;
    pub fn gtk_button_box_set_layout           (widget: *mut GtkButtonBox, layout_style: enums::ButtonBoxStyle) -> ();
    pub fn gtk_button_box_set_child_secondary  (widget: *mut GtkButtonBox, child : *mut GtkWidget, is_secondary: gboolean) -> ();
    pub fn gtk_button_box_set_child_non_homogeneous(widget: *mut GtkButtonBox, child : *mut GtkWidget, non_homogeneous: gboolean) -> ();

    //=========================================================================
    // GtkVersion                                                            OK
    //=========================================================================
    pub fn gtk_get_major_version               () -> c_uint;
    pub fn gtk_get_minor_version               () -> c_uint;
    pub fn gtk_get_micro_version               () -> c_uint;
    pub fn gtk_get_binary_age                  () -> c_uint;
    pub fn gtk_get_interface_age               () -> c_uint;
    pub fn gtk_check_version                   (required_major: c_uint, required_minor: c_uint, required_micro: c_uint) -> *const c_char;

    //=========================================================================
    // GtkFrame                                                              OK
    //=========================================================================
    pub fn gtk_frame_new                       (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_frame_set_label                 (frame: *mut GtkFrame, label: *const c_char) -> ();
    pub fn gtk_frame_set_label_widget          (frame: *mut GtkFrame, label_widget: *mut GtkWidget) -> ();
    pub fn gtk_frame_set_label_align           (frame: *mut GtkFrame, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_frame_set_shadow_type           (frame: *mut GtkFrame, st_type: enums::ShadowType) -> ();
    pub fn gtk_frame_get_label                 (frame: *mut GtkFrame) -> *const c_char;
    pub fn gtk_frame_get_label_align           (frame: *mut GtkFrame, xalign: *mut c_float, yalign: *mut c_float) -> ();
    // pub fn gtk_frame_get_label_widget          (frame: *const const GtkFrame) -> *const const GtkWidget;
    pub fn gtk_frame_get_shadow_type           (frame: *mut GtkFrame) -> enums::ShadowType;

    //=========================================================================
    // GtkAspectFrame                                                        OK
    //=========================================================================
    pub fn gtk_aspect_frame_new                (label: *const c_char, xalign: c_float, yalign: c_float, ratio: c_float, obey_child: gboolean) -> *mut GtkWidget;
    pub fn gtk_aspect_frame_set                (aspect_frame: *mut GtkAspectFrame, xalign: c_float, yalign: c_float, ratio: c_float, obey_child: gboolean) -> ();

    //=========================================================================
    // GtkFixed                                                              OK
    //=========================================================================
    pub fn gtk_fixed_new                       () -> *mut GtkWidget;
    pub fn gtk_fixed_put                       (fixed: *mut GtkFixed, widget: *mut GtkWidget, x: c_int, y: c_int) -> ();
    pub fn gtk_fixed_move                      (fixed: *mut GtkFixed, widget: *mut GtkWidget, x: c_int, y: c_int) -> ();

    //=========================================================================
    // GtkBin                                                                OK
    //=========================================================================
    pub fn gtk_bin_get_type                    () -> GType;
    pub fn gtk_bin_get_child                   (bin: *mut GtkBin) -> *mut GtkWidget;

    //=========================================================================
    // GtkSeparator                                                          OK
    //=========================================================================
    pub fn gtk_separator_new                   (orientation: enums::Orientation) -> *mut GtkWidget;

    //=========================================================================
    // GtkAdjustment                                                         OK
    //=========================================================================
    pub fn gtk_adjustment_get_type             () -> GType;
    pub fn gtk_adjustment_new                  (value: c_double, lower: c_double, upper: c_double, step_increment: c_double, page_increment: c_double, page_size: c_double) -> *mut GtkAdjustment;
    pub fn gtk_adjustment_get_value            (adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_set_value            (adjustment: *mut GtkAdjustment, value: c_double) -> ();
    pub fn gtk_adjustment_clamp_page           (adjustment: *mut GtkAdjustment, lower: c_double, upper: c_double) -> ();
    pub fn gtk_adjustment_changed              (adjustment: *mut GtkAdjustment) -> ();
    pub fn gtk_adjustment_value_changed        (adjustment: *mut GtkAdjustment) -> ();
    pub fn gtk_adjustment_configure            (adjustment: *mut GtkAdjustment, value: c_double, lower: c_double, upper: c_double, tep_increment: c_double, page_increment: c_double, page_size: c_double) -> ();
    pub fn gtk_adjustment_get_lower            (adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_page_increment   (adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_page_size        (adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_step_increment   (adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_minimum_increment(adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_upper            (adjustment: *mut GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_set_lower            (adjustment: *mut GtkAdjustment, lower: c_double) -> ();
    pub fn gtk_adjustment_set_page_increment   (adjustment: *mut GtkAdjustment, page_increment: c_double) -> ();
    pub fn gtk_adjustment_set_page_size        (adjustment: *mut GtkAdjustment, page_size: c_double) -> ();
    pub fn gtk_adjustment_set_step_increment   (adjustment: *mut GtkAdjustment, step_increment: c_double) -> ();
    pub fn gtk_adjustment_set_upper            (adjustment: *mut GtkAdjustment, upper: c_double) -> ();

    //=========================================================================
    // GtkGrid
    //=========================================================================
    pub fn gtk_grid_new                        () -> *mut GtkWidget;
    pub fn gtk_grid_attach                     (grid: *mut GtkGrid, child: *mut GtkWidget, left: c_int, top: c_int, width: c_int, height: c_int) -> ();
    pub fn gtk_grid_attach_next_to             (grid: *mut GtkGrid, child: *mut GtkWidget, sibling: *mut GtkWidget, side: enums::PositionType, width: c_int, height: c_int) -> ();
    // pub fn gtk_grid_get_child_at               (grid: *const const GtkGrid, left: c_int, top: c_int) -> *const const GtkWidget;
    pub fn gtk_grid_insert_row                 (grid: *mut GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_insert_column              (grid: *mut GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_remove_row                 (grid: *mut GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_remove_column              (grid: *mut GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_insert_next_to             (grid: *mut GtkGrid, sibling: *mut GtkWidget, side: enums::PositionType) -> ();
    pub fn gtk_grid_set_row_homogeneous        (grid: *mut GtkGrid, homogeneous: gboolean) -> ();
    pub fn gtk_grid_get_row_homogeneous        (grid: *mut GtkGrid) -> gboolean;
    pub fn gtk_grid_set_row_spacing            (grid: *mut GtkGrid, spacing: c_uint) -> ();
    pub fn gtk_grid_get_row_spacing            (grid: *mut GtkGrid) -> c_uint;
    pub fn gtk_grid_set_column_homogeneous     (grid: *mut GtkGrid, homogeneous: gboolean) -> ();
    pub fn gtk_grid_get_column_homogeneous     (grid: *mut GtkGrid) -> gboolean;
    pub fn gtk_grid_set_column_spacing         (grid: *mut GtkGrid, spacing: c_uint) -> ();
    pub fn gtk_grid_get_column_spacing         (grid: *mut GtkGrid) -> c_uint;
    pub fn gtk_grid_get_baseline_row           (grid: *mut GtkGrid) -> c_int;
    pub fn gtk_grid_set_baseline_row           (grid: *mut GtkGrid, row: c_int) -> ();
    // pub fn gtk_grid_get_row_baseline_position  (grid: *const const GtkGrid, row: c_int);
    // pub fn gtk_grid_set_row_baseline_position  (grid: *const const GtkGrid, row: c_int, pos: enums::BaselinePosition) -> ();

    //=========================================================================
    // GtkEntryBuffer                                                        OK
    //=========================================================================
    pub fn gtk_entry_buffer_new                (initial_chars: *const c_char, n_initial_chars: c_int) -> *mut GtkEntryBuffer;
    pub fn gtk_entry_buffer_get_text           (buffer: *mut GtkEntryBuffer) -> *const c_char;
    pub fn gtk_entry_buffer_set_text           (buffer: *mut GtkEntryBuffer, chars: *const c_char, n_chars: c_int) -> ();
    pub fn gtk_entry_buffer_get_bytes          (buffer: *mut GtkEntryBuffer) -> c_long;
    pub fn gtk_entry_buffer_get_length         (buffer: *mut GtkEntryBuffer) -> c_uint;
    pub fn gtk_entry_buffer_get_max_length     (buffer: *mut GtkEntryBuffer) -> c_int;
    pub fn gtk_entry_buffer_set_max_length     (buffer: *mut GtkEntryBuffer, max_length: c_int) -> ();
    pub fn gtk_entry_buffer_insert_text        (buffer: *mut GtkEntryBuffer, position: c_uint, chars: *const c_char, n_chars: c_int);
    pub fn gtk_entry_buffer_delete_text        (buffer: *mut GtkEntryBuffer, position: c_uint, n_char: c_uint) -> c_uint;
    pub fn gtk_entry_buffer_emit_deleted_text  (buffer: *mut GtkEntryBuffer, position: c_uint, n_chars: c_uint) -> ();
    pub fn gtk_entry_buffer_emit_inserted_text (buffer: *mut GtkEntryBuffer, position: c_uint, chars: *const c_char, n_chars: c_int) -> ();

    //=========================================================================
    // GtkEntry
    //=========================================================================
    pub fn gtk_entry_new                       () -> *mut GtkWidget;
    pub fn gtk_entry_new_with_buffer           (buffer: *mut GtkEntryBuffer) -> *mut GtkWidget;
    pub fn gtk_entry_get_buffer                (entry: *mut GtkEntry) -> *mut GtkEntryBuffer;
    pub fn gtk_entry_set_buffer                (entry: *mut GtkEntry, buffer: *mut GtkEntryBuffer) -> ();
    pub fn gtk_entry_set_text                  (entry: *mut GtkEntry, text: *const c_char) -> ();
    pub fn gtk_entry_get_text                  (entry: *mut GtkEntry) -> *const c_char;
    pub fn gtk_entry_get_text_length           (entry: *mut GtkEntry) -> c_short;
    // pub fn gtk_entry_get_text_area             (entry: *const const GtkEntry, text_area: *GdkRectangle) -> ();
    pub fn gtk_entry_set_visibility            (entry: *mut GtkEntry, visible: gboolean) -> ();
    pub fn gtk_entry_set_invisible_char        (entry: *mut GtkEntry, ch: c_int) -> ();
    pub fn gtk_entry_unset_invisible_char      (entry: *mut GtkEntry) -> ();
    pub fn gtk_entry_set_max_length            (entry: *mut GtkEntry, max: c_int) -> ();
    pub fn gtk_entry_get_activates_default     (entry: *mut GtkEntry) -> gboolean;
    pub fn gtk_entry_get_has_frame             (entry: *mut GtkEntry) -> gboolean;
    // pub fn gtk_entry_get_inner_border          (entry: *const const GtkEntry) -> *const const GtkBorder;
    pub fn gtk_entry_get_width_chars           (entry: *mut GtkEntry) -> c_int;
    pub fn gtk_entry_set_activates_default     (entry: *mut GtkEntry, setting: gboolean) -> ();
    pub fn gtk_entry_set_has_frame             (entry: *mut GtkEntry, setting: gboolean) -> ();
    // pub fn gtk_entry_set_inner_border          (entry: *const const GtkEntry, border: *const const GtkBorder) -> ();
    pub fn gtk_entry_set_width_chars           (entry: *mut GtkEntry, n_chars: c_int) -> ();
    pub fn gtk_entry_get_invisible_char        (entry: *mut GtkEntry) -> c_uint;
    pub fn gtk_entry_set_alignment             (entry: *mut GtkEntry, xalign: c_float) -> ();
    pub fn gtk_entry_get_alignment             (entry: *mut GtkEntry) -> c_float;
    pub fn gtk_entry_set_placeholder_text      (entry: *mut GtkEntry, text: *const c_char) -> ();
    pub fn gtk_entry_get_placeholder_text      (entry: *mut GtkEntry) -> *const c_char;
    pub fn gtk_entry_set_overwrite_mode        (entry: *mut GtkEntry, overwrite: gboolean) -> ();
    pub fn gtk_entry_get_overwrite_mode        (entry: *mut GtkEntry) -> gboolean;
    // pub fn gtk_entry_get_layout                (entry: *const const GtkEntry) -> *PangoLayout;
    pub fn gtk_entry_get_layout_offsets        (entry: *mut GtkEntry, x: *const c_int,  y: *const c_int) -> ();
    pub fn gtk_entry_layout_index_to_text_index(entry: *mut GtkEntry, layout_index: c_int) -> c_int;
    pub fn gtk_entry_text_index_to_layout_index(entry: *mut GtkEntry,  text_index: c_int) -> c_int;
    // pub fn gtk_entry_set_attributes            (entry: *const const GtkEntry, attrs: *PangoAttrList) -> ();
    // pub fn gtk_entry_get_attributes            (entry: *const const GtkEntry) -> *PangoAttrList;
    pub fn gtk_entry_get_max_length            (entry: *mut GtkEntry) -> c_int;
    pub fn gtk_entry_get_visibility            (entry: *mut GtkEntry) -> gboolean;
    // pub fn gtk_entry_set_completion            (entry: *const const GtkEntry, completion: *const const GtkEntryCompletion) -> ();
    // pub fn gtk_entry_get_completion            (entry: *const const GtkEntry) -> *GtkEntryCompletion;
    pub fn gtk_entry_set_cursor_hadjustment    (entry: *mut GtkEntry, adjustment: *mut GtkAdjustment) -> ();
    pub fn gtk_entry_get_cursor_hadjustment    (entry: *mut GtkEntry) -> *mut GtkAdjustment;
    pub fn gtk_entry_set_progress_fraction     (entry: *mut GtkEntry, fraction: c_double) -> ();
    pub fn gtk_entry_get_progress_fraction     (entry: *mut GtkEntry) -> c_double;
    pub fn gtk_entry_set_progress_pulse_step   (entry: *mut GtkEntry, fraction: c_double) -> ();
    pub fn gtk_entry_get_progress_pulse_step   (entry: *mut GtkEntry) -> c_double;
    pub fn gtk_entry_progress_pulse            (entry: *mut GtkEntry) -> ();
    // pub fn gtk_entry_im_context_filter_keypress(entry: *const const GtkEntry, event: *GdkEventKey) -> gboolean;
    pub fn gtk_entry_reset_im_context          (entry: *mut GtkEntry) -> ();
    // pub fn gtk_entry_get_tabs                  (entry: *const const GtkEntry) -> *PangoTabArray;
    // pub fn gtk_entry_set_tabs                  (entry: *const const GtkEntry, tabs: *PangoTabArray) -> ();
    // pub fn gtk_entry_set_icon_from_pixbuf      (entry: *const const GtkEntry, icon_pos: enums::EntryIconPosition, pixbuf: *GdkPixbuf) -> ();
    pub fn gtk_entry_set_icon_from_stock       (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition, stock_id: *const c_char) -> ();
    pub fn gtk_entry_set_icon_from_icon_name   (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition, icon_name: *const c_char) -> ();
    // pub fn gtk_entry_set_icon_from_gicon       (entry: *const const GtkEntry, icon_pos: enums::EntryIconPosition, icon: *GIcon) -> ();
    pub fn gtk_entry_get_icon_storage_type     (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> enums::ImageType;
    // pub fn gtk_entry_get_icon_pixbuf           (entry: *const const GtkEntry, icon_pos: enums::EntryIconPosition) -> *const const GdkPixbuf;
    pub fn gtk_entry_get_icon_stock            (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> *const c_char;
    pub fn gtk_entry_get_icon_name             (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> *const c_char;
    // pub fn gtk_entry_get_icon_gicon            (entry: *const const GtkEntry, icon_pos: enums::EntryIconPosition) -> *GIcon;
    pub fn gtk_entry_set_icon_activatable      (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition, activatable: gboolean) -> ();
    pub fn gtk_entry_get_icon_activatable      (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> gboolean;
    pub fn gtk_entry_set_icon_sensitive        (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition, sensitive: gboolean) -> ();
    pub fn gtk_entry_get_icon_sensitive        (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> gboolean;
    pub fn gtk_entry_get_icon_at_pos           (entry: *mut GtkEntry, x: c_int, y: c_int) -> c_int;
    pub fn gtk_entry_set_icon_tooltip_text     (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition, tooltip: *const c_char) -> ();
    pub fn gtk_entry_get_icon_tooltip_text     (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> *const c_char;
    pub fn gtk_entry_set_icon_tooltip_markup   (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition, tooltip: *const c_char) -> ();
    pub fn gtk_entry_get_icon_tooltip_markup   (entry: *mut GtkEntry, icon_pos: enums::EntryIconPosition) -> *const c_char;
    // pub fn gtk_entry_set_icon_drag_source      (entry: *const const GtkEntry, icon_pos: enums::EntryIconPosition, target_list: *GtkTargetList, actions: GdkDragAction) -> ();
    pub fn gtk_entry_get_current_icon_drag_source(entry: *mut GtkEntry) -> c_int;
    // pub fn gtk_entry_get_icon_area             (entry: *const const GtkEntry, icon_pos: enums::EntryIconPosition, icon_area: *GdkRectangle) -> ();
    pub fn gtk_entry_set_input_purpose         (entry: *mut GtkEntry, purpose: enums::InputPurpose) -> ();
    pub fn gtk_entry_get_input_purpose         (entry: *mut GtkEntry) -> enums::InputPurpose;
    pub fn gtk_entry_set_input_hints           (entry: *mut GtkEntry, hints: enums::InputHints) -> ();
    pub fn gtk_entry_get_input_hints           (entry: *mut GtkEntry) -> enums::InputHints;

    //=========================================================================
    // GtkSearchEntry                                                        OK
    //=========================================================================
    pub fn gtk_search_entry_new                () -> *mut GtkWidget;

    //=========================================================================
    // GtkSwitch                                                             OK
    //=========================================================================
    pub fn gtk_switch_new                      () -> *mut GtkWidget;
    pub fn gtk_switch_set_active               (switch: *mut GtkSwitch,  is_active: gboolean) -> ();
    pub fn gtk_switch_get_active               (switch: *mut GtkSwitch) -> gboolean;

    //=========================================================================
    // GtkScale
    //=========================================================================
    pub fn gtk_scale_new                       (orientation: enums::Orientation, adjustment: *mut GtkAdjustment) -> *mut GtkWidget;
    pub fn gtk_scale_new_with_range            (orientation: enums::Orientation, min: c_double, max: c_double, step: c_double) -> *mut GtkWidget;
    pub fn gtk_scale_set_digits                (scale: *mut GtkScale, digits: c_int) -> ();
    pub fn gtk_scale_set_draw_value            (scale: *mut GtkScale, draw_value: gboolean) -> ();
    pub fn gtk_scale_set_has_origin            (scale: *mut GtkScale, has_origin: gboolean) -> ();
    pub fn gtk_scale_set_value_pos             (scale: *mut GtkScale, position: enums::PositionType) -> ();
    pub fn gtk_scale_get_digits                (scale: *mut GtkScale) -> c_int;
    pub fn gtk_scale_get_draw_value            (scale: *mut GtkScale) -> gboolean;
    pub fn gtk_scale_get_has_origin            (scale: *mut GtkScale) -> gboolean;
    pub fn gtk_scale_get_value_pos             (scale: *mut GtkScale) -> enums::PositionType;
    // pub fn gtk_scale_get_layout                (scale: *const const GtkScale) -> *PangoLayout;
    pub fn gtk_scale_get_layout_offsets        (scale: *mut GtkScale, x: *const c_int, y: *const c_int) -> ();
    pub fn gtk_scale_add_mark                  (scale: *mut GtkScale, value: c_double, position: enums::PositionType, markup: *const c_char) -> ();
    pub fn gtk_scale_clear_marks               (scale: *mut GtkScale) -> ();

    //=========================================================================
    // GtkRange
    //=========================================================================
    pub fn gtk_range_set_adjustment            (scale: *mut GtkRange, adjustment: *mut GtkAdjustment) -> ();
    pub fn gtk_range_get_adjustment            (scale: *mut GtkRange) -> *mut GtkAdjustment;

    //=========================================================================
    // GtkLevelBar
    //=========================================================================
    pub fn gtk_level_bar_new                   () -> *mut GtkWidget;
    pub fn gtk_level_bar_new_for_interval      (min_value: c_double, max_value: c_double) -> *mut GtkWidget;
    pub fn gtk_level_bar_set_mode              (bar: *mut GtkLevelBar, mode: enums::LevelBarMode) -> ();
    pub fn gtk_level_bar_get_mode              (bar: *mut GtkLevelBar) -> enums::LevelBarMode;
    pub fn gtk_level_bar_set_value             (bar: *mut GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_value             (bar: *mut GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_min_value         (bar: *mut GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_min_value         (bar: *mut GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_max_value         (bar: *mut GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_max_value         (bar: *mut GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_inverted          (bar: *mut GtkLevelBar, inverted: gboolean) -> ();
    pub fn gtk_level_bar_get_inverted          (bar: *mut GtkLevelBar) -> gboolean;
    pub fn gtk_level_bar_add_offset_value      (bar: *mut GtkLevelBar, name: *const c_char, value: c_double) -> ();
    pub fn gtk_level_bar_remove_offset_value   (bar: *mut GtkLevelBar, name: *const c_char) -> ();
    pub fn gtk_level_bar_get_offset_value      (bar: *mut GtkLevelBar, name: *const c_char, value: *mut c_double) -> gboolean;

    //=========================================================================
    // GtkSearchBar
    //=========================================================================
    pub fn gtk_search_bar_new                  () -> *mut GtkWidget;
    pub fn gtk_search_bar_connect_entry        (bar: *mut GtkSearchBar, entry: *mut GtkEntry) -> ();
    pub fn gtk_search_bar_get_search_mode      (bar: *mut GtkSearchBar) -> gboolean;
    pub fn gtk_search_bar_set_search_mode      (bar: *mut GtkSearchBar, search_mode: gboolean) -> ();
    pub fn gtk_search_bar_get_show_close_button(bar: *mut GtkSearchBar) -> gboolean;
    pub fn gtk_search_bar_set_show_close_button(bar: *mut GtkSearchBar, visible: gboolean) -> ();
    // pub fn gtk_search_bar_handle_event         (bar: *const const GtkSearchBar, event: *GdkEvent) -> gboolean;

    //=========================================================================
    // GtkSpinButton
    //=========================================================================
    pub fn gtk_spin_button_configure           (spin_button: *mut GtkSpinButton, adjustment: *mut GtkAdjustment, climb_rate: c_double, digits: c_uint) -> ();
    pub fn gtk_spin_button_new                 (adjustment: *mut GtkAdjustment, climb_rate: c_double, digits: c_uint) -> *mut GtkWidget;
    pub fn gtk_spin_button_new_with_range      (min: c_double, max: c_double, step: c_double) -> *mut GtkWidget;
    pub fn gtk_spin_button_set_adjustment      (spin_button: *mut GtkSpinButton, adjustment: *mut GtkAdjustment) -> ();
    pub fn gtk_spin_button_get_adjustment      (spin_button: *mut GtkSpinButton) -> *mut GtkAdjustment;
    pub fn gtk_spin_button_set_digits          (spin_button: *mut GtkSpinButton, digits: c_uint) -> ();
    pub fn gtk_spin_button_set_increments      (spin_button: *mut GtkSpinButton, step: c_double, page: c_double) -> ();
    pub fn gtk_spin_button_set_range           (spin_button: *mut GtkSpinButton, min: c_double, max: c_double);
    pub fn gtk_spin_button_get_value_as_int    (spin_button: *mut GtkSpinButton) -> c_int;
    pub fn gtk_spin_button_set_value           (spin_button: *mut GtkSpinButton, value: c_double) -> ();
    pub fn gtk_spin_button_set_update_policy   (spin_button: *mut GtkSpinButton, policy: enums::SpinButtonUpdatePolicy) -> ();
    pub fn gtk_spin_button_set_numeric         (spin_button: *mut GtkSpinButton, gnumeric: gboolean) -> ();
    pub fn gtk_spin_button_spin                (spin_button: *mut GtkSpinButton, direction: enums::SpinType, increment: c_double) -> ();
    pub fn gtk_spin_button_set_wrap            (spin_button: *mut GtkSpinButton, wrap: gboolean) -> ();
    pub fn gtk_spin_button_set_snap_to_ticks   (spin_button: *mut GtkSpinButton, snap_to_ticks: gboolean) -> ();
    pub fn gtk_spin_button_update              (spin_button: *mut GtkSpinButton) -> ();
    pub fn gtk_spin_button_get_digits          (spin_button: *mut GtkSpinButton) -> c_uint;
    pub fn gtk_spin_button_get_increments      (spin_button: *mut GtkSpinButton, step: *mut c_double, page: *mut c_double) -> ();
    pub fn gtk_spin_button_get_numeric         (spin_button: *mut GtkSpinButton) -> gboolean;
    pub fn gtk_spin_button_get_range           (spin_button: *mut GtkSpinButton, min: *mut c_double, max: *mut c_double) -> ();
    pub fn gtk_spin_button_get_snap_to_ticks   (spin_button: *mut GtkSpinButton) -> gboolean;
    pub fn gtk_spin_button_get_update_policy   (spin_button: *mut GtkSpinButton) -> enums::SpinButtonUpdatePolicy;
    pub fn gtk_spin_button_get_value           (spin_button: *mut GtkSpinButton) -> c_double;
    pub fn gtk_spin_button_get_wrap            (spin_button: *mut GtkSpinButton) -> gboolean;

    //=========================================================================
    // GtkSpinner                                                            OK
    //=========================================================================
    pub fn gtk_spinner_new                     () -> *mut GtkWidget;
    pub fn gtk_spinner_start                   (spinner: *mut GtkSpinner) -> ();
    pub fn gtk_spinner_stop                    (spinner: *mut GtkSpinner) -> ();

    //=========================================================================
    // GtkImage
    //=========================================================================
    pub fn gtk_image_new() -> *mut GtkWidget;
    pub fn gtk_image_new_from_file(filename: *const c_char) -> *mut GtkWidget;
    pub fn gtk_image_new_from_pixbuf(pixbuf: *mut gdk_ffi::GdkPixbuf) -> *mut GtkWidget;
    pub fn gtk_image_new_from_icon_name(icon_name: *const c_char, size: enums::IconSize)
        -> *mut GtkWidget;
    pub fn gtk_image_set_from_file(image: *mut GtkImage, filename: *const c_char);
    pub fn gtk_image_set_from_pixbuf(image: *mut GtkImage, pixbuf: *mut gdk_ffi::GdkPixbuf);
    pub fn gtk_image_set_from_icon_name(image: *mut GtkImage, icon_name: *const c_char,
        size: enums::IconSize);
    pub fn gtk_image_get_pixbuf(image: *mut GtkImage) -> *mut gdk_ffi::GdkPixbuf;
    pub fn gtk_image_get_animation(image: *mut GtkImage) -> *mut gdk_ffi::GdkPixbufAnimation;

    //=========================================================================
    // GtkProgressBar
    //=========================================================================
    pub fn gtk_progress_bar_new                () -> *mut GtkWidget;
    pub fn gtk_progress_bar_pulse              (pbar: *mut GtkProgressBar) -> ();
    pub fn gtk_progress_bar_set_fraction       (pbar: *mut GtkProgressBar, fraction: c_double) -> ();
    pub fn gtk_progress_bar_get_fraction       (pbar: *mut GtkProgressBar) -> c_double;
    pub fn gtk_progress_bar_set_inverted       (pbar: *mut GtkProgressBar, inverted: gboolean) -> ();
    pub fn gtk_progress_bar_get_inverted       (pbar: *mut GtkProgressBar) -> gboolean;
    pub fn gtk_progress_bar_set_show_text      (pbar: *mut GtkProgressBar, show_text: gboolean) -> ();
    pub fn gtk_progress_bar_get_show_text      (pbar: *mut GtkProgressBar) -> gboolean;
    pub fn gtk_progress_bar_set_text           (pbar: *mut GtkProgressBar, text: *const c_char) -> ();
    pub fn gtk_progress_bar_get_text           (pbar: *mut GtkProgressBar) -> *const c_char;
    // pub fn gtk_progress_bar_set_ellipsize      (pbar: *const const GtkProgressBar, mode: PangoEllipsizeMode) -> ();
    // pub fn gtk_progress_bar_get_ellipsize      (pbar: *const const GtkProgressBar) -> PangoEllipsizeMode
    pub fn gtk_progress_bar_set_pulse_step     (pbar: *mut GtkProgressBar, fraction: c_double) -> ();
    pub fn gtk_progress_bar_get_pulse_step     (pbar: *mut GtkProgressBar) -> c_double;

    //=========================================================================
    // GtkArrow                                                              OK
    //=========================================================================
    pub fn gtk_arrow_new                       (arrow_type: enums::ArrowType, shadow_type: enums::ShadowType) -> *mut GtkWidget;
    pub fn gtk_arrow_set                       (arrow: *mut GtkArrow,arrow_type: enums::ArrowType, shadow_type: enums::ShadowType) -> ();

    //=========================================================================
    // GtkCalendar
    //=========================================================================
    pub fn gtk_calendar_new                    () -> *mut GtkWidget;
    pub fn gtk_calendar_select_month           (calendar: *mut GtkCalendar, month: c_uint, year: c_uint) -> ();
    pub fn gtk_calendar_select_day             (calendar: *mut GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_mark_day               (calendar: *mut GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_unmark_day             (calendar: *mut GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_get_day_is_marked      (calendar: *mut GtkCalendar, day: c_uint) -> gboolean;
    pub fn gtk_calendar_clear_marks            (calendar: *mut GtkCalendar) -> ();
    pub fn gtk_calendar_get_display_options    (calendar: *mut GtkCalendar) -> enums::CalendarDisplayOptions;
    pub fn gtk_calendar_set_display_options    (calendar: *mut GtkCalendar, flags: enums::CalendarDisplayOptions) -> ();
    pub fn gtk_calendar_get_date               (calendar: *mut GtkCalendar, year: *mut c_uint, month: *mut c_uint, day: *mut c_uint) -> ();
    // pub fn gtk_calendar_set_detail_func        (calendar: *const const GtkCalendar, GtkCalendarDetailFunc func, gpointer data, GDestroyNotify destroy) -> ();
    pub fn gtk_calendar_get_detail_width_chars (calendar: *mut GtkCalendar) -> c_int;
    pub fn gtk_calendar_set_detail_width_chars (calendar: *mut GtkCalendar, chars: c_int) -> ();
    pub fn gtk_calendar_get_detail_height_rows (calendar: *mut GtkCalendar) -> c_int;
    pub fn gtk_calendar_set_detail_height_rows (calendar: *mut GtkCalendar, rows: c_int) -> ();

    //=========================================================================
    // GtkAlignments                                                         OK
    //=========================================================================
    pub fn gtk_alignment_new                   (xalign: c_float, yalign: c_float, xscale: c_float, yscale: c_float) -> *mut GtkWidget;
    pub fn gtk_alignment_set                   (alignment: *mut GtkAlignment, xalign: c_float, yalign: c_float, xscale: c_float, yscale: c_float) -> ();
    pub fn gtk_alignment_get_padding           (alignment: *mut GtkAlignment, padding_top: *mut c_uint, padding_bottom: *mut c_uint, padding_left: *mut c_uint, padding_right: *mut c_uint) -> ();
    pub fn gtk_alignment_set_padding           (alignment: *mut GtkAlignment, padding_top: c_uint, padding_bottom: c_uint, padding_left: c_uint, padding_right: c_uint) -> ();

    //=========================================================================
    // GtkExpander                                                           OK
    //=========================================================================
    pub fn gtk_expander_new                    (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_expander_new_with_mnemonic      (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_expander_set_expanded           (expander: *mut GtkExpander, expanded: gboolean) -> ();
    pub fn gtk_expander_get_expanded           (expander: *mut GtkExpander) -> gboolean;
    pub fn gtk_expander_set_spacing            (expander: *mut GtkExpander, spacing: c_int) -> ();
    pub fn gtk_expander_get_spacing            (expander: *mut GtkExpander) -> c_int;
    pub fn gtk_expander_set_label              (expander: *mut GtkExpander, label: *const c_char) -> ();
    pub fn gtk_expander_get_label              (expander: *mut GtkExpander) -> *const c_char;
    pub fn gtk_expander_set_use_underline      (expander: *mut GtkExpander, use_underline: gboolean)-> ();
    pub fn gtk_expander_get_use_underline      (expander: *mut GtkExpander) -> gboolean;
    pub fn gtk_expander_set_use_markup         (expander: *mut GtkExpander, use_markup: gboolean) -> ();
    pub fn gtk_expander_get_use_markup         (expander: *mut GtkExpander) -> gboolean;
    pub fn gtk_expander_set_label_widget       (expander: *mut GtkExpander, label_widget: *mut GtkWidget) -> ();
    pub fn gtk_expander_get_label_widget       (expander: *mut GtkExpander) -> *mut GtkWidget;
    pub fn gtk_expander_set_label_fill         (expander: *mut GtkExpander, label_fill: gboolean) -> ();
    pub fn gtk_expander_get_label_fill         (expander: *mut GtkExpander) -> gboolean;
    pub fn gtk_expander_set_resize_toplevel    (expander: *mut GtkExpander, resize_toplevel: gboolean) -> ();
    pub fn gtk_expander_get_resize_toplevel    (expander: *mut GtkExpander) -> gboolean;

    //=========================================================================
    // GtkPaned                                                              OK
    //=========================================================================
    pub fn gtk_paned_new                       (orientation: enums::Orientation) -> *mut GtkWidget;
    pub fn gtk_paned_add1                      (paned: *mut GtkPaned, child: *mut GtkWidget) -> ();
    pub fn gtk_paned_add2                      (paned: *mut GtkPaned, child: *mut GtkWidget) -> ();
    pub fn gtk_paned_pack1                     (paned: *mut GtkPaned, child: *mut GtkWidget, resize: gboolean, schrink: gboolean) -> ();
    pub fn gtk_paned_pack2                     (paned: *mut GtkPaned, child: *mut GtkWidget, resize: gboolean, schrink: gboolean) -> ();
    // pub fn gtk_paned_get_child1                (paned: *const const GtkPaned) -> *const const GtkWidget;
    // pub fn gtk_paned_get_child2                (paned: *const const GtkPaned) -> *const const GtkWidget;
    pub fn gtk_paned_set_position              (paned: *mut GtkPaned, position: c_int) -> ();
    pub fn gtk_paned_get_position              (paned: *mut GtkPaned) -> c_int;
    pub fn gtk_paned_get_handle_window         (paned: *mut GtkPaned) -> *mut GtkWidget;

    //=========================================================================
    // GtkInfoBar
    //=========================================================================
    pub fn gtk_info_bar_new                    () -> *mut GtkWidget;
    // pub fn gtk_info_bar_new_with_buttons       (const gchar *first_button_text, ...) -> *const const GtkWidget;
    pub fn gtk_info_bar_add_action_widget      (info_bar: *mut GtkInfoBar, child: *mut GtkWidget, response_id: c_int);
    pub fn gtk_info_bar_add_button             (info_bar: *mut GtkInfoBar, button_text: *const c_char, response_id: c_int) -> *mut GtkWidget;
    // pub fn gtk_info_bar_add_buttons            (GtkInfoBar *info_bar, const gchar *first_button_text, ...) -> ();
    pub fn gtk_info_bar_set_response_sensitive (info_bar: *mut GtkInfoBar, response_id: c_int, setting: gboolean) -> ();
    pub fn gtk_info_bar_set_default_response   (info_bar: *mut GtkInfoBar, response_id: c_int) -> ();
    pub fn gtk_info_bar_response               (info_bar: *mut GtkInfoBar, response_id: c_int) -> ();
    pub fn gtk_info_bar_set_message_type       (info_bar: *mut GtkInfoBar, message_type: enums::MessageType) -> ();
    pub fn gtk_info_bar_get_message_type       (info_bar: *mut GtkInfoBar) -> enums::MessageType;
    // pub fn gtk_info_bar_get_action_area        (info_bar: *const const GtkInfoBar) -> *const const GtkWidget;
    // pub fn gtk_info_bar_get_content_area       (info_bar: *const const GtkInfoBar) -> *const const GtkWidget;
    pub fn gtk_info_bar_get_show_close_button  (info_bar: *mut GtkInfoBar) -> gboolean;
    pub fn gtk_info_bar_set_show_close_button  (info_bar: *mut GtkInfoBar, setting: gboolean) -> ();

    //=========================================================================
    // GtkToolShell
    //=========================================================================
    // pub fn gtk_tool_shell_get_ellipsize_mode   (shell: *const const GtkToolShell) -> PangoEllipsizeMode;
    pub fn gtk_tool_shell_get_icon_size        (shell: *mut GtkToolShell) -> enums::IconSize;
    pub fn gtk_tool_shell_get_orientation      (shell: *mut GtkToolShell) -> enums::Orientation;
    pub fn gtk_tool_shell_get_relief_style     (shell: *mut GtkToolShell) -> enums::ReliefStyle;
    pub fn gtk_tool_shell_get_style            (shell: *mut GtkToolShell) -> enums::ToolbarStyle;
    pub fn gtk_tool_shell_get_text_alignment   (shell: *mut GtkToolShell) -> c_float;
    pub fn gtk_tool_shell_get_text_orientation (shell: *mut GtkToolShell) -> enums::Orientation;
    pub fn gtk_tool_shell_rebuild_menu         (shell: *mut GtkToolShell) -> ();
    pub fn gtk_tool_shell_get_text_size_group  (shell: *const GtkToolShell) -> *mut GtkSizeGroup;

    //=========================================================================
    // GtkToolBar
    //=========================================================================
    pub fn gtk_toolbar_new                     () -> *mut GtkWidget;
    pub fn gtk_toolbar_insert                  (toolbar: *mut GtkToolbar, item: *mut GtkToolItem, pos: c_int) -> ();
    pub fn gtk_toolbar_get_item_index          (toolbar: *mut GtkToolbar, item: *mut GtkToolItem) -> c_int;
    pub fn gtk_toolbar_get_n_items             (toolbar: *mut GtkToolbar) -> c_int;
    pub fn gtk_toolbar_get_nth_item            (toolbar: *mut GtkToolbar, n: c_int) -> *mut GtkToolItem;
    pub fn gtk_toolbar_get_drop_index          (toolbar: *mut GtkToolbar, x: c_int, y: c_int) -> c_int;
    pub fn gtk_toolbar_set_drop_highlight_item (toolbar: *mut GtkToolbar, item: *mut GtkToolItem, index: c_int) -> ();
    pub fn gtk_toolbar_set_show_arrow          (toolbar: *mut GtkToolbar, show_array: gboolean) -> ();
    pub fn gtk_toolbar_unset_icon_size         (toolbar: *mut GtkToolbar) -> ();
    pub fn gtk_toolbar_get_show_arrow          (toolbar: *mut GtkToolbar) -> gboolean;
    pub fn gtk_toolbar_get_style               (toolbar: *mut GtkToolbar) -> enums::ToolbarStyle;
    pub fn gtk_toolbar_get_icon_size           (toolbar: *mut GtkToolbar) -> enums::IconSize;
    pub fn gtk_toolbar_get_relief_style        (toolbar: *mut GtkToolbar) -> enums::ReliefStyle;
    pub fn gtk_toolbar_set_style               (toolbar: *mut GtkToolbar, style: enums::ToolbarStyle) -> ();
    pub fn gtk_toolbar_set_icon_size           (toolbar: *mut GtkToolbar, icon_size: enums::IconSize) -> ();
    pub fn gtk_toolbar_unset_style             (toolbar: *mut GtkToolbar) -> ();

    //=========================================================================
    // GtkToolItem
    //=========================================================================
    pub fn gtk_tool_item_new                   () -> *mut GtkWidget;
    pub fn gtk_tool_item_set_homogeneous       (tool_item: *mut GtkToolItem, homogeneous: gboolean) -> ();
    pub fn gtk_tool_item_get_homogeneous       (tool_item: *mut GtkToolItem) -> gboolean;
    pub fn gtk_tool_item_set_expand            (tool_item: *mut GtkToolItem, expand: gboolean) -> ();
    pub fn gtk_tool_item_get_expand            (tool_item: *mut GtkToolItem) -> gboolean;
    pub fn gtk_tool_item_set_tooltip_text      (tool_item: *mut GtkToolItem, text: *const c_char) -> ();
    pub fn gtk_tool_item_set_tooltip_markup    (tool_item: *mut GtkToolItem, markup: *const c_char) -> ();
    pub fn gtk_tool_item_set_use_drag_window   (tool_item: *mut GtkToolItem, use_drag_window: gboolean) -> ();
    pub fn gtk_tool_item_get_use_drag_window   (tool_item: *mut GtkToolItem) -> gboolean;
    pub fn gtk_tool_item_set_visible_horizontal(tool_item: *mut GtkToolItem, visible_horizontal: gboolean) -> ();
    pub fn gtk_tool_item_get_visible_horizontal(tool_item: *mut GtkToolItem) -> gboolean;
    pub fn gtk_tool_item_set_visible_vertical  (tool_item: *mut GtkToolItem, visible_vertical: gboolean) -> ();
    pub fn gtk_tool_item_get_visible_vertical  (tool_item: *mut GtkToolItem) -> gboolean;
    pub fn gtk_tool_item_set_is_important      (tool_item: *mut GtkToolItem, is_important: gboolean) -> ();
    pub fn gtk_tool_item_get_is_important      (tool_item: *mut GtkToolItem) -> gboolean;
    // pub fn gtk_tool_item_get_ellipsize_mode    (tool_item: *const const GtkToolItem) -> PangoEllipsizeMode;
    pub fn gtk_tool_item_get_icon_size         (tool_item: *mut GtkToolItem) -> enums::IconSize;
    pub fn gtk_tool_item_get_orientation       (tool_item: *mut GtkToolItem) -> enums::Orientation;
    pub fn gtk_tool_item_get_toolbar_style     (tool_item: *mut GtkToolItem) -> enums::ToolbarStyle;
    pub fn gtk_tool_item_get_relief_style      (tool_item: *mut GtkToolItem) -> enums::ReliefStyle;
    pub fn gtk_tool_item_get_text_alignment    (tool_item: *mut GtkToolItem) -> c_float;
    pub fn gtk_tool_item_get_text_orientation  (tool_item: *mut GtkToolItem) -> enums::Orientation;
    // pub fn gtk_tool_item_retrieve_proxy_menu_item(tool_item: *const const GtkToolItem) -> *const const GtkWidget;
    // pub fn gtk_tool_item_get_proxy_menu_item   (tool_item: *const const GtkToolItem, menu_item_id: *const c_char) -> *const const GtkWidget;
    // pub fn gtk_tool_item_set_proxy_menu_item   (tool_item: *const const GtkToolItem, menu_item_id: *const c_char, menu_item: *const const GtkWidget) -> ();
    pub fn gtk_tool_item_rebuild_menu          (tool_item: *mut GtkToolItem) -> ();
    pub fn gtk_tool_item_toolbar_reconfigured  (tool_item: *mut GtkToolItem) -> ();
    pub fn gtk_tool_item_get_text_size_group   (tool_item: *const GtkToolItem) -> *mut GtkSizeGroup;

    //=========================================================================
    // GtkToolItemGroup                                                  NOT OK
    //=========================================================================
    pub fn gtk_tool_item_group_new              (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_tool_item_group_get_collapsed    (group: *mut GtkToolItemGroup) -> gboolean;
    pub fn gtk_tool_item_group_get_drop_item    (group: *mut GtkToolItemGroup, x: c_int, y: c_int) -> *mut GtkToolItem;
    //pub fn gtk_tool_item_group_get_ellipsize    (group: *mut GtkToolItemGroup) -> PangoEllipsizeMode;
    pub fn gtk_tool_item_group_get_item_position(group: *mut GtkToolItemGroup, item: *mut GtkToolItem) -> c_int;
    pub fn gtk_tool_item_group_get_n_items      (group: *mut GtkToolItemGroup) -> c_uint;
    pub fn gtk_tool_item_group_get_label        (group: *mut GtkToolItemGroup) -> *const c_char;
    pub fn gtk_tool_item_group_get_label_widget (group: *mut GtkToolItemGroup) -> *mut GtkWidget;
    pub fn gtk_tool_item_group_get_nth_item     (group: *mut GtkToolItemGroup, index: c_uint) -> *mut GtkToolItem;
    pub fn gtk_tool_item_group_get_header_relief(group: *mut GtkToolItemGroup) -> enums::ReliefStyle;
    pub fn gtk_tool_item_group_insert           (group: *mut GtkToolItemGroup, item: *mut GtkToolItem, position: c_int);
    pub fn gtk_tool_item_group_set_collapsed    (group: *mut GtkToolItemGroup, collapsed: gboolean);
    //pub fn gtk_tool_item_group_set_ellipsize    (group: *mut GtkToolItemGroup, ellipsize: PangoEllipsizeMode);
    pub fn gtk_tool_item_group_set_item_position(group: *mut GtkToolItemGroup, item: *mut GtkToolItem, position: c_int);
    pub fn gtk_tool_item_group_set_label        (group: *mut GtkToolItemGroup, label: *const c_char);
    pub fn gtk_tool_item_group_set_label_widget (group: *mut GtkToolItemGroup, label_widget: *mut GtkWidget);
    pub fn gtk_tool_item_group_set_header_relief(group: *mut GtkToolItemGroup, style: enums::ReliefStyle);

    //=========================================================================
    // GtkToolPalette                                                    NOT OK
    //=========================================================================
    pub fn gtk_tool_palette_new                () -> *mut GtkWidget;
    pub fn gtk_tool_palette_get_exclusive      (palette: *mut GtkToolPalette, group: *mut GtkToolItemGroup) -> gboolean;
    pub fn gtk_tool_palette_set_exclusive      (palette: *mut GtkToolPalette, group: *mut GtkToolItemGroup, exclusive: gboolean);
    pub fn gtk_tool_palette_get_expand         (palette: *mut GtkToolPalette, group: *mut GtkToolItemGroup) -> gboolean;
    pub fn gtk_tool_palette_set_expand         (palette: *mut GtkToolPalette, group: *mut GtkToolItemGroup, expand: gboolean);
    pub fn gtk_tool_palette_get_group_position (palette: *mut GtkToolPalette, group: *mut GtkToolItemGroup) -> c_int;
    pub fn gtk_tool_palette_set_group_position (palette: *mut GtkToolPalette, group: *mut GtkToolItemGroup, position: c_int);
    pub fn gtk_tool_palette_get_icon_size      (palette: *mut GtkToolPalette) -> enums::IconSize;
    pub fn gtk_tool_palette_set_icon_size      (palette: *mut GtkToolPalette, icon_size: enums::IconSize);
    pub fn gtk_tool_palette_unset_icon_size    (palette: *mut GtkToolPalette);
    pub fn gtk_tool_palette_get_style          (palette: *mut GtkToolPalette) -> enums::ToolbarStyle;
    pub fn gtk_tool_palette_set_style          (palette: *mut GtkToolPalette, style: enums::ToolbarStyle);
    pub fn gtk_tool_palette_unset_style        (palette: *mut GtkToolPalette);
    //pub fn gtk_tool_palette_add_drag_dest      (palette: *mut GtkToolPalette, widget: *mut GtkWidget, flags: enums::DestDefaults,
    //    targets: enums::ToolPaletteDragTargets, actions: GdkDragAction);
    //pub fn gtk_tool_palette_get_drag_item      (palette: *mut GtkToolPalette, selection: *const GtkSelectionData) -> *mut GtkWidget;
    //pub fn gtk_tool_palette_get_drag_target_group() -> *const GtkTargetEntry;
    //pub fn gtk_tool_palette_get_drag_target_item() -> *const GtkTargetEntry;
    pub fn gtk_tool_palette_get_drop_group     (palette: *mut GtkToolPalette, x: c_int, y: c_int) -> *mut GtkToolItemGroup;
    pub fn gtk_tool_palette_get_drop_item      (palette: *mut GtkToolPalette, x: c_int, y: c_int) -> *mut GtkToolItem;
    pub fn gtk_tool_palette_set_drag_source    (palette: *mut GtkToolPalette, targets: enums::ToolPaletteDragTargets);

    //=========================================================================
    // GtkSeparatorToolItem
    //=========================================================================
    pub fn gtk_separator_tool_item_new         () -> *mut GtkWidget;
    pub fn gtk_separator_tool_item_set_draw    (item: *mut GtkSeparatorToolItem, draw: gboolean) -> ();
    pub fn gtk_separator_tool_item_get_draw    (item: *mut GtkSeparatorToolItem) -> gboolean;

    //=========================================================================
    // GtkToolButton
    //=========================================================================
    pub fn gtk_tool_button_new                 (icon_widget: *mut GtkWidget, label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_tool_button_new_from_stock      (stock_id: *const c_char) -> *mut GtkWidget;
    pub fn gtk_tool_button_set_label           (button: *mut GtkToolButton, label: *const c_char) -> ();
    pub fn gtk_tool_button_get_label           (button: *mut GtkToolButton) -> *const c_char;
    pub fn gtk_tool_button_set_use_underline   (button: *mut GtkToolButton, use_underline: gboolean) -> ();
    pub fn gtk_tool_button_get_use_underline   (button: *mut GtkToolButton) -> gboolean;
    pub fn gtk_tool_button_set_stock_id        (button: *mut GtkToolButton, stock_id: *const c_char) -> ();
    pub fn gtk_tool_button_get_stock_id        (button: *mut GtkToolButton) -> *const c_char;
    pub fn gtk_tool_button_set_icon_name       (button: *mut GtkToolButton, icon_name: *const c_char) -> ();
    pub fn gtk_tool_button_get_icon_name       (button: *mut GtkToolButton) -> *const c_char;
    // pub fn gtk_tool_button_set_icon_widget     (button: *const const GtkToolButton, icon_widget: *const const GtkWidget) -> ();
    // pub fn gtk_tool_button_get_icon_widget     (button: *const const GtkToolButton) -> *const const GtkWidget;
    pub fn gtk_tool_button_set_label_widget    (button: *mut GtkToolButton, label_widget: *mut GtkWidget) -> ();
    pub fn gtk_tool_button_get_label_widget    (button: *mut GtkToolButton) -> *mut GtkWidget;

    //=========================================================================
    // GtkToggleToolButton
    //=========================================================================
    pub fn gtk_toggle_tool_button_new          () -> *mut GtkWidget;
    pub fn gtk_toggle_tool_button_new_from_stock(stock_id: *const c_char) -> *mut GtkWidget;
    pub fn gtk_toggle_tool_button_set_active   (button: *mut GtkToggleToolButton, is_active: gboolean) -> ();
    pub fn gtk_toggle_tool_button_get_active   (button: *mut GtkToggleToolButton) -> gboolean;


    //=========================================================================
    // GtkRadioToolButton
    //=========================================================================
    // pub fn gtk_radio_tool_button_new           (GSList *group) -> *const const GtkWidget;
    // pub fn gtk_radio_tool_button_new_from_stock(GSList *group, const gchar *stock_id) -> *const const GtkWidget;
    // pub fn gtk_radio_tool_button_new_from_widget(GtkRadioToolButton *group) -> *const const GtkWidget;
    // pub fn gtk_radio_tool_button_new_with_stock_from_widget(GtkRadioToolButton *group, const gchar *stock_id) -> *const const GtkWidget;
    // pub fn gtk_radio_tool_button_get_group     (GtkRadioToolButton *button) -> *GSList;
    // pub fn gtk_radio_tool_button_set_group     (GtkRadioToolButton *button, GSList *group) -> ();

    //=========================================================================
    // GtkMenuToolButton
    //=========================================================================
    pub fn gtk_menu_tool_button_new            (icon_widget: *mut GtkWidget, label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_menu_tool_button_new_from_stock (stock_id: *const c_char) -> *mut GtkWidget;
    // pub fn gtk_menu_tool_button_set_menu       (button: *const const GtkMenuToolButton, menu: *const const GtkWidget);
    // pub fn gtk_menu_tool_button_get_menu       (button: *const const GtkMenuToolButton) -> *const const GtkWidget;
    pub fn gtk_menu_tool_button_set_arrow_tooltip_text(button: *mut GtkMenuToolButton, text: *const c_char) -> ();
    pub fn gtk_menu_tool_button_set_arrow_tooltip_markup(button: *mut GtkMenuToolButton, markup: *const c_char) -> ();

    //=========================================================================
    // GtkNoteBook                                                           OK
    //=========================================================================
    pub fn gtk_notebook_new               () -> *mut GtkWidget;
    pub fn gtk_notebook_append_page       (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget) -> c_int;
    pub fn gtk_notebook_append_page_menu  (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget, menu_label: *mut GtkWidget) -> c_int;
    pub fn gtk_notebook_prepend_page      (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget) -> c_int;
    pub fn gtk_notebook_prepend_page_menu (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget, menu_label: *mut GtkWidget) -> c_int;
    pub fn gtk_notebook_insert_page       (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget, position: c_int) -> c_int;
    pub fn gtk_notebook_insert_page_menu  (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget, menu_label: *mut GtkWidget, position: c_int) -> c_int;
    pub fn gtk_notebook_remove_page       (notebook: *mut GtkNotebook, page_num: c_int);
    pub fn gtk_notebook_set_group_name    (notebook: *mut GtkNotebook, group_name: *const c_char);
    pub fn gtk_notebook_get_group_name    (notebook: *mut GtkNotebook) -> *const c_char;
    pub fn gtk_notebook_get_current_page  (notebook: *mut GtkNotebook) -> c_int;
    pub fn gtk_notebook_get_nth_page      (notebook: *mut GtkNotebook, page_num: c_int) -> *mut GtkWidget;
    pub fn gtk_notebook_get_n_pages       (notebook: *mut GtkNotebook) -> c_int;
    pub fn gtk_notebook_page_num          (notebook: *mut GtkNotebook, child: *mut GtkWidget) -> c_int;
    pub fn gtk_notebook_set_current_page  (notebook: *mut GtkNotebook, page_num: c_int);
    pub fn gtk_notebook_next_page         (notebook: *mut GtkNotebook);
    pub fn gtk_notebook_prev_page         (notebook: *mut GtkNotebook);
    pub fn gtk_notebook_set_show_border   (notebook: *mut GtkNotebook, show_border: gboolean);
    pub fn gtk_notebook_get_show_border   (notebook: *mut GtkNotebook) -> gboolean;
    pub fn gtk_notebook_set_show_tabs     (notebook: *mut GtkNotebook, show_tabs: gboolean);
    pub fn gtk_notebook_get_show_tabs     (notebook: *mut GtkNotebook) -> gboolean;
    pub fn gtk_notebook_set_tab_pos       (notebook: *mut GtkNotebook, pos: enums::PositionType);
    pub fn gtk_notebook_get_tab_pos       (notebook: *mut GtkNotebook) -> enums::PositionType;
    pub fn gtk_notebook_set_scrollable    (notebook: *mut GtkNotebook, scrollable: gboolean);
    pub fn gtk_notebook_get_scrollable    (notebook: *mut GtkNotebook) -> gboolean;
    pub fn gtk_notebook_get_tab_hborder   (notebook: *mut GtkNotebook) -> u16;
    pub fn gtk_notebook_get_tab_vborder   (notebook: *mut GtkNotebook) -> u16;
    pub fn gtk_notebook_popup_enable      (notebook: *mut GtkNotebook);
    pub fn gtk_notebook_popup_disable     (notebook: *mut GtkNotebook);
    pub fn gtk_notebook_get_tab_label     (notebook: *mut GtkNotebook, child: *mut GtkWidget) -> *mut GtkWidget;
    pub fn gtk_notebook_set_tab_label     (notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_label: *mut GtkWidget);
    pub fn gtk_notebook_set_tab_label_text(notebook: *mut GtkNotebook, child: *mut GtkWidget, tab_text: *const c_char);
    pub fn gtk_notebook_get_tab_label_text(notebook: *mut GtkNotebook, child: *mut GtkWidget) -> *const c_char;
    pub fn gtk_notebook_get_menu_label    (notebook: *mut GtkNotebook, child: *mut GtkWidget) -> *mut GtkWidget;
    pub fn gtk_notebook_set_menu_label    (notebook: *mut GtkNotebook, child: *mut GtkWidget, menu_label: *mut GtkWidget);
    pub fn gtk_notebook_set_menu_label_text(notebook: *mut GtkNotebook, child: *mut GtkWidget, menu_text: *const c_char);
    pub fn gtk_notebook_get_menu_label_text(notebook: *mut GtkNotebook, child: *mut GtkWidget) -> *const c_char;
    pub fn gtk_notebook_reorder_child     (notebook: *mut GtkNotebook, child: *mut GtkWidget, position: c_int);
    pub fn gtk_notebook_get_tab_reorderable(notebook: *mut GtkNotebook, child: *mut GtkWidget) -> gboolean;
    pub fn gtk_notebook_set_tab_reorderable(notebook: *mut GtkNotebook, child: *mut GtkWidget, reorderable: gboolean);
    pub fn gtk_notebook_get_tab_detachable(notebook: *mut GtkNotebook, child: *mut GtkWidget) -> gboolean;
    pub fn gtk_notebook_set_tab_detachable(notebook: *mut GtkNotebook, child: *mut GtkWidget, detachable: gboolean);
    pub fn gtk_notebook_get_action_widget (notebook: *mut GtkNotebook,pack_type: enums::PackType) -> *mut GtkWidget;
    pub fn gtk_notebook_set_action_widget (notebook: *mut GtkNotebook, child: *mut GtkWidget, pack_type: enums::PackType);


    //=========================================================================
    // GtkStack                                                              OK
    //=========================================================================
    pub fn gtk_stack_new                     () -> *mut GtkWidget;
    pub fn gtk_stack_add_named               (stack: *mut GtkStack, child: *mut GtkWidget, name: *const c_char);
    pub fn gtk_stack_add_titled              (stack: *mut GtkStack, child: *mut GtkWidget, name: *const c_char, title: *const c_char);
    pub fn gtk_stack_set_visible_child       (stack: *mut GtkStack, child: *mut GtkWidget);
    pub fn gtk_stack_get_visible_child       (stack: *mut GtkStack) -> *mut GtkWidget;
    pub fn gtk_stack_set_visible_child_name  (stack: *mut GtkStack, name: *const c_char);
    pub fn gtk_stack_get_visible_child_name  (stack: *mut GtkStack) -> *const c_char;
    pub fn gtk_stack_set_visible_child_full  (stack: *mut GtkStack, name: *const c_char, transition: enums::StackTransitionType);
    pub fn gtk_stack_set_homogeneous         (stack: *mut GtkStack, homogeneous: gboolean);
    pub fn gtk_stack_get_homogeneous         (stack: *mut GtkStack) -> gboolean;
    pub fn gtk_stack_set_transition_duration (stack: *mut GtkStack, duration: c_uint);
    pub fn gtk_stack_get_transition_duration (stack: *mut GtkStack) -> c_uint;
    pub fn gtk_stack_set_transition_type     (stack: *mut GtkStack, transition: enums::StackTransitionType);
    pub fn gtk_stack_get_transition_type     (stack: *mut GtkStack) -> enums::StackTransitionType;

    //=========================================================================
    // GtkStackSwitcher                                                      OK
    //=========================================================================
    pub fn gtk_stack_switcher_new       () -> *mut GtkWidget;
    pub fn gtk_stack_switcher_set_stack (switcher: *mut GtkStackSwitcher, stack: *mut GtkStack);
    pub fn gtk_stack_switcher_get_stack (switcher: *mut GtkStackSwitcher) -> *mut GtkWidget;


    //=========================================================================
    // GtkRevealer                                                           OK
    //=========================================================================
    pub fn gtk_revealer_new                     () -> *mut GtkWidget;
    pub fn gtk_revealer_get_reveal_child        (revealer: *mut GtkRevealer) -> gboolean;
    pub fn gtk_revealer_set_reveal_child        (revealer: *mut GtkRevealer, reveal_child: gboolean);
    pub fn gtk_revealer_get_child_revealed      (revealer: *mut GtkRevealer) -> gboolean;
    pub fn gtk_revealer_get_transition_duration (revealer: *mut GtkRevealer) -> c_uint;
    pub fn gtk_revealer_set_transition_duration (revealer: *mut GtkRevealer, duration: c_uint);
    pub fn gtk_revealer_set_transition_type     (revealer: *mut GtkRevealer, transition: enums::RevealerTransitionType);
    pub fn gtk_revealer_get_transition_type     (revealer: *mut GtkRevealer) -> enums::RevealerTransitionType;

    //=========================================================================
    // GtkOverlay                                                            OK
    //=========================================================================
    pub fn gtk_overlay_new () -> *mut GtkWidget;
    pub fn gtk_overlay_add_overlay (overlay: *mut GtkOverlay, widget: *mut GtkWidget);

    //=========================================================================
    // GtkScrollable                                                         OK
    //=========================================================================
    pub fn gtk_scrollable_get_type               () -> GType;
    pub fn gtk_scrollable_get_hadjustment        (scrollable: *mut GtkScrollable) -> *mut GtkAdjustment;
    pub fn gtk_scrollable_set_hadjustment        (scrollable: *mut GtkScrollable, hadjustment: *mut GtkAdjustment);
    pub fn gtk_scrollable_get_vadjustment        (scrollable: *mut GtkScrollable) -> *mut GtkAdjustment;
    pub fn gtk_scrollable_set_vadjustment        (scrollable: *mut GtkScrollable, vadjustment: *mut GtkAdjustment);
    pub fn gtk_scrollable_get_hscroll_policy     (scrollable: *mut GtkScrollable) -> enums::ScrollablePolicy;
    pub fn gtk_scrollable_set_hscroll_policy     (scrollable: *mut GtkScrollable, policy: enums::ScrollablePolicy);
    pub fn gtk_scrollable_get_vscroll_policy     (scrollable: *mut GtkScrollable) -> enums::ScrollablePolicy;
    pub fn gtk_scrollable_set_vscroll_policy     (scrollable: *mut GtkScrollable, policy: enums::ScrollablePolicy);

    //=========================================================================
    // GtkLayout
    //=========================================================================
    pub fn gtk_layout_new             (hadjustment: *mut GtkAdjustment, vadjustment: *mut GtkAdjustment) -> *mut GtkWidget;
    pub fn gtk_layout_put             (layout: *mut GtkLayout, child_widget: *mut GtkWidget, x: c_int, y: c_int);
    pub fn gtk_layout_move            (layout: *mut GtkLayout, child_widget: *mut GtkWidget, x: c_int, y: c_int);
    pub fn gtk_layout_set_size        (layout: *mut GtkLayout, width: c_uint, height: c_uint);
    pub fn gtk_layout_get_size        (layout: *mut GtkLayout, width: *mut c_uint, height: *mut c_uint);
    // pub fn gtk_layout_get_bin_window  (layout: *const const GtkLayout) -> *const const GdkWindow;

    //=========================================================================
    // GtkHeaderBar                                                          OK
    //=========================================================================
    pub fn gtk_header_bar_get_type          () -> GType;
    pub fn gtk_header_bar_new               () -> *mut GtkWidget;
    pub fn gtk_header_bar_set_title         (bar: *mut GtkHeaderBar, title: *const c_char);
    pub fn gtk_header_bar_get_title         (bar: *mut GtkHeaderBar) -> *const c_char;
    pub fn gtk_header_bar_set_subtitle      (bar: *mut GtkHeaderBar, subtitle: *const c_char);
    pub fn gtk_header_bar_get_subtitle      (bar: *mut GtkHeaderBar) -> *const c_char;
    pub fn gtk_header_bar_set_custom_title  (bar: *mut GtkHeaderBar, title_widget: *mut GtkWidget);
    pub fn gtk_header_bar_get_custom_title  (bar: *mut GtkHeaderBar) -> *mut GtkWidget;
    pub fn gtk_header_bar_pack_start        (bar: *mut GtkHeaderBar, child: *mut GtkWidget);
    pub fn gtk_header_bar_pack_end          (bar: *mut GtkHeaderBar, child: *mut GtkWidget);
    pub fn gtk_header_bar_get_show_close_button (bar: *mut GtkHeaderBar) -> gboolean;
    pub fn gtk_header_bar_set_show_close_button (bar: *mut GtkHeaderBar, setting: gboolean);

    //=========================================================================
    // GtkFlowBox                                                            OK
    //=========================================================================
    pub fn gtk_flow_box_new                       () -> *mut GtkWidget;
    pub fn gtk_flow_box_set_homogeneous           (_box: *mut GtkFlowBox, homogeneous: gboolean);
    pub fn gtk_flow_box_get_homogeneous           (_box: *mut GtkFlowBox) -> gboolean;
    pub fn gtk_flow_box_set_row_spacing           (_box: *mut GtkFlowBox, spacing: c_uint);
    pub fn gtk_flow_box_get_row_spacing           (_box: *mut GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_column_spacing        (_box: *mut GtkFlowBox, spacing: c_uint);
    pub fn gtk_flow_box_get_column_spacing        (_box: *mut GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_min_children_per_line (_box: *mut GtkFlowBox, n_children: c_uint);
    pub fn gtk_flow_box_get_min_children_per_line (_box: *mut GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_max_children_per_line (_box: *mut GtkFlowBox, n_children: c_uint);
    pub fn gtk_flow_box_get_max_children_per_line (_box: *mut GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_activate_on_single_click (_box: *mut GtkFlowBox, single: gboolean);
    pub fn gtk_flow_box_get_activate_on_single_click (_box: *mut GtkFlowBox) -> gboolean;
    pub fn gtk_flow_box_insert                       (_box: *mut GtkFlowBox, widget: *mut GtkWidget, position: c_int);
    pub fn gtk_flow_box_get_child_at_index           (_box: *mut GtkFlowBox, idx: c_int) -> *mut GtkFlowBoxChild;
    pub fn gtk_flow_box_select_child                 (_box: *mut GtkFlowBox, child: *mut GtkFlowBoxChild);
    pub fn gtk_flow_box_unselect_child               (_box: *mut GtkFlowBox, child: *mut GtkFlowBoxChild);
    pub fn gtk_flow_box_select_all                   (_box: *mut GtkFlowBox);
    pub fn gtk_flow_box_unselect_all                 (_box: *mut GtkFlowBox);
    pub fn gtk_flow_box_set_selection_mode           (_box: *mut GtkFlowBox, mode: enums::SelectionMode);
    pub fn gtk_flow_box_get_selection_mode           (_box: *mut GtkFlowBox) -> enums::SelectionMode;
    pub fn gtk_flow_box_set_hadjustment              (_box: *mut GtkFlowBox, adjustment: *mut GtkAdjustment);
    pub fn gtk_flow_box_set_vadjustment              (_box: *mut GtkFlowBox, adjustment: *mut GtkAdjustment);
    // void                  gtk_flow_box_selected_foreach             (_box: *const const GtkFlowBox, GtkFlowBoxForeachFunc func, gpointer           data);
    // GList                *gtk_flow_box_get_selected_children        (_box: *const const GtkFlowBox);

    //=========================================================================
    // GtkFlowBoxChild                                                       OK
    //=========================================================================
    pub fn gtk_flow_box_child_new () -> *mut GtkWidget;
    pub fn gtk_flow_box_child_get_index (child: *mut GtkFlowBoxChild) -> c_int;
    pub fn gtk_flow_box_child_is_selected (child: *mut GtkFlowBoxChild) -> gboolean;
    pub fn gtk_flow_box_child_changed (child: *mut GtkFlowBoxChild);

    //=========================================================================
    // GtkListBox                                                            OK
    //=========================================================================
    pub fn gtk_list_box_new                          () -> *mut GtkWidget;
    pub fn gtk_list_box_prepend                      (list_box: *mut GtkListBox, child: *mut GtkWidget);
    pub fn gtk_list_box_insert                       (list_box: *mut GtkListBox, child: *mut GtkWidget, position: c_int);
    pub fn gtk_list_box_get_selected_row             (list_box: *mut GtkListBox) -> *mut GtkListBoxRow;
    pub fn gtk_list_box_get_row_at_index             (list_box: *mut GtkListBox, index: c_int) -> *mut GtkListBoxRow;
    pub fn gtk_list_box_get_row_at_y                 (list_box: *mut GtkListBox, y: c_int) -> *mut GtkListBoxRow;
    pub fn gtk_list_box_select_row                   (list_box: *mut GtkListBox, row: *mut GtkListBoxRow);
    pub fn gtk_list_box_set_placeholder              (list_box: *mut GtkListBox, placeholder: *mut GtkWidget);
    pub fn gtk_list_box_set_adjustment               (list_box: *mut GtkListBox, adjustment: *mut GtkAdjustment);
    pub fn gtk_list_box_get_adjustment               (list_box: *mut GtkListBox) -> *mut GtkAdjustment;
    pub fn gtk_list_box_set_selection_mode           (list_box: *mut GtkListBox, mode: enums::SelectionMode);
    pub fn gtk_list_box_get_selection_mode           (list_box: *mut GtkListBox) -> enums::SelectionMode;
    // pub fn gtk_list_box_invalidate_filter            (list_box: *const const GtkListBox);
    // pub fn gtk_list_box_invalidate_sort              (list_box: *const const GtkListBox);
    pub fn gtk_list_box_invalidate_headers           (list_box: *mut GtkListBox);
    pub fn gtk_list_box_set_activate_on_single_click (list_box: *mut GtkListBox, simgle: gboolean);
    pub fn gtk_list_box_get_activate_on_single_click (list_box: *mut GtkListBox) -> gboolean;
    pub fn gtk_list_box_drag_unhighlight_row         (list_box: *mut GtkListBox);
    pub fn gtk_list_box_drag_highlight_row           (list_box: *mut GtkListBox, row: *mut GtkListBoxRow);

    //=========================================================================
    // GtkListBoxRow                                                         OK
    //=========================================================================
    pub fn gtk_list_box_row_new         () -> *mut GtkWidget;
    pub fn gtk_list_box_row_changed     (row: *mut GtkListBoxRow);
    pub fn gtk_list_box_row_get_header  (row: *mut GtkListBoxRow) -> *mut GtkWidget;
    pub fn gtk_list_box_row_set_header  (row: *mut GtkListBoxRow, header: *mut GtkWidget);
    pub fn gtk_list_box_row_get_index   (row: *mut GtkListBoxRow) -> c_int;

    //=========================================================================
    // GtkActionBar                                                          OK
    //=========================================================================
    pub fn gtk_action_bar_new               () -> *mut GtkWidget;
    pub fn gtk_action_bar_get_center_widget (action_bar: *mut GtkActionBar) -> *mut GtkWidget;
    pub fn gtk_action_bar_set_center_widget (action_bar: *mut GtkActionBar, center_widget: *mut GtkWidget);
    pub fn gtk_action_bar_pack_start        (action_bar: *mut GtkActionBar, child: *mut GtkWidget);
    pub fn gtk_action_bar_pack_end          (action_bar: *mut GtkActionBar, child: *mut GtkWidget);

    //=========================================================================
    // GtkDrawingArea
    //=========================================================================
    pub fn gtk_drawing_area_new                 () -> *mut GtkWidget;

    //=========================================================================
    // GtkDrawingArea                                                        OK
    //=========================================================================
    pub fn gtk_editable_select_region        (editable: *mut GtkEditable, start_pos: c_int, end_pos: c_int);
    pub fn gtk_editable_get_selection_bounds (editable: *mut GtkEditable, start_pos: *mut c_int, end_pos: *mut c_int) -> gboolean;
    pub fn gtk_editable_insert_text          (editable: *mut GtkEditable, new_text: *const c_char, new_text_length: c_int, position: *mut c_int);
    pub fn gtk_editable_delete_text          (editable: *mut GtkEditable, start_pos: c_int, end_pos: c_int);
    pub fn gtk_editable_get_chars            (editable: *mut GtkEditable, start_pos: c_int, end_pos: c_int) -> *const c_char;
    pub fn gtk_editable_cut_clipboard        (editable: *mut GtkEditable);
    pub fn gtk_editable_copy_clipboard       (editable: *mut GtkEditable);
    pub fn gtk_editable_paste_clipboard      (editable: *mut GtkEditable);
    pub fn gtk_editable_delete_selection     (editable: *mut GtkEditable);
    pub fn gtk_editable_set_position         (editable: *mut GtkEditable, position: c_int);
    pub fn gtk_editable_get_position         (editable: *mut GtkEditable) -> c_int;
    pub fn gtk_editable_set_editable         (editable: *mut GtkEditable, is_editable: gboolean);
    pub fn gtk_editable_get_editable         (editable: *mut GtkEditable) -> gboolean;

    //=========================================================================
    // GtkTextMark                                                           OK
    //=========================================================================
    pub fn gtk_text_mark_new                 (name: *const c_char, left_gravity: gboolean) -> *mut GtkTextMark;
    pub fn gtk_text_mark_set_visible         (mark: *mut GtkTextMark, setting: gboolean);
    pub fn gtk_text_mark_get_visible         (mark: *mut GtkTextMark) -> gboolean;
    pub fn gtk_text_mark_get_deleted         (mark: *mut GtkTextMark) -> gboolean;
    pub fn gtk_text_mark_get_name            (mark: *mut GtkTextMark) -> *const c_char;
    pub fn gtk_text_mark_get_buffer          (mark: *mut GtkTextMark) -> *mut GtkTextBuffer;
    pub fn gtk_text_mark_get_left_gravity    (mark: *mut GtkTextMark) -> gboolean;

    //=========================================================================
    // GtkTextAttributes                                                 NOT OK
    //=========================================================================
    pub fn gtk_text_attributes_new           () -> *mut GtkTextAttributes;
    pub fn gtk_text_attributes_copy          (src: *mut GtkTextAttributes) -> *mut GtkTextAttributes;
    pub fn gtk_text_attributes_copy_values   (src: *mut GtkTextAttributes, dest: *mut GtkTextAttributes);
    pub fn gtk_text_attributes_unref         (src: *mut GtkTextAttributes);
    pub fn gtk_text_attributes_ref           (src: *mut GtkTextAttributes) -> *mut GtkTextAttributes;

    //=========================================================================
    // GtkTextTag                                                        NOT OK
    //=========================================================================
    pub fn gtk_text_tag_new                  (name: *const c_char) -> *mut GtkTextTag;
    pub fn gtk_text_tag_get_priority         (tag: *mut GtkTextTag) -> c_int;
    pub fn gtk_text_tag_set_priority         (tag: *mut GtkTextTag, priority: c_int);
    //pub fn gtk_text_tag_event                (tag: *mut GtkTextTag, event_object: *mut GObject, event: *mut GdkEvent,
    //    iter: *const GtkTextIter);

    //=========================================================================
    // GtkTextChildAnchor                                                NOT OK
    //=========================================================================
    pub fn gtk_text_child_anchor_new         () -> *mut GtkTextChildAnchor;
    //pub fn gtk_text_child_anchor_get_widgets () -> *mut GList;
    pub fn gtk_text_child_anchor_get_deleted (anchor: *mut GtkTextChildAnchor) -> gboolean;

    //=========================================================================
    // GtkTextIter                                                       NOT OK
    //=========================================================================
    pub fn gtk_text_iter_get_buffer                     (iter: *const GtkTextIter) -> *mut GtkTextBuffer;
    pub fn gtk_text_iter_copy                           (iter: *const GtkTextIter) -> *mut GtkTextIter;
    pub fn gtk_text_iter_assign                         (iter: *mut GtkTextIter, other: *const GtkTextIter);
    pub fn gtk_text_iter_free                           (iter: *mut GtkTextIter);
    pub fn gtk_text_iter_get_offset                     (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_line                       (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_line_offset                (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_line_index                 (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_visible_line_index         (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_visible_line_offset        (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_char                       (iter: *const GtkTextIter) -> c_uint;
    pub fn gtk_text_iter_get_slice                      (start: *const GtkTextIter, end: *const GtkTextIter) -> *mut c_char;
    pub fn gtk_text_iter_get_text                       (start: *const GtkTextIter, end: *const GtkTextIter) -> *mut c_char;
    pub fn gtk_text_iter_get_visible_slice              (start: *const GtkTextIter, end: *const GtkTextIter) -> *mut c_char;
    pub fn gtk_text_iter_get_visible_text               (start: *const GtkTextIter, end: *const GtkTextIter) -> *mut c_char;
    //pub fn gtk_text_iter_get_pixbuf                     (iter: *const GtkTextIter) -> *mut GdkPixbuf;
    //pub fn gtk_text_iter_get_marks                      (iter: *const GtkTextIter) -> *mut GSList;
    //pub fn gtk_text_iter_get_toggled_tags               (iter: *const GtkTextIter, toggled_on: gboolean) -> *mut GSList;
    pub fn gtk_text_iter_get_child_anchor               (iter: *const GtkTextIter) -> *mut GtkTextChildAnchor;
    pub fn gtk_text_iter_begins_tag                     (iter: *const GtkTextIter, tag: *mut GtkTextTag) -> gboolean;
    pub fn gtk_text_iter_ends_tag                       (iter: *const GtkTextIter, tag: *mut GtkTextTag) -> gboolean;
    pub fn gtk_text_iter_toggles_tag                    (iter: *const GtkTextIter, tag: *mut GtkTextTag) -> gboolean;
    pub fn gtk_text_iter_has_tag                        (iter: *const GtkTextIter, tag: *mut GtkTextTag) -> gboolean;
    //pub fn gtk_text_iter_get_tags                       (iter: *const GtkTextIter) -> *mut GSList;
    pub fn gtk_text_iter_editable                       (iter: *const GtkTextIter, default_setting: gboolean) -> gboolean;
    pub fn gtk_text_iter_can_insert                     (iter: *const GtkTextIter, default_setting: gboolean) -> gboolean;
    pub fn gtk_text_iter_starts_word                    (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_ends_word                      (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_inside_word                    (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_starts_line                    (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_ends_line                      (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_starts_sentence                (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_ends_sentence                  (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_inside_sentence                (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_is_cursor_position             (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_get_chars_in_line              (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_bytes_in_line              (iter: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_get_attributes                 (iter: *const GtkTextIter, values: *mut GtkTextAttributes) -> gboolean;
    //pub fn gtk_text_iter_get_language                   (iter: *const GtkTextIter) -> *mut PangoLanguage;
    pub fn gtk_text_iter_is_end                         (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_is_start                       (iter: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_char                   (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_char                  (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_chars                  (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_chars                 (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_line                   (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_line                  (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_lines                  (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_lines                 (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_word_ends              (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_word_starts           (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_word_end               (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_word_start            (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_cursor_position        (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_cursor_position       (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_cursor_positions       (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_cursor_positions      (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_sentence_start        (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_sentence_starts       (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_sentence_end           (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_sentence_ends          (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_visible_word_ends      (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_visible_word_starts   (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_visible_word_end       (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_visible_word_start    (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_visible_cursor_position(iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_visible_cursor_position(iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_visible_cursor_positions(iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_visible_cursor_positions(iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_forward_visible_line           (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_backward_visible_line          (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_visible_lines          (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_backward_visible_lines         (iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_iter_set_offset                     (iter: *mut GtkTextIter, char_offset: c_int);
    pub fn gtk_text_iter_set_line                       (iter: *mut GtkTextIter, line_number: c_int);
    pub fn gtk_text_iter_set_line_offset                (iter: *mut GtkTextIter, char_on_line: c_int);
    pub fn gtk_text_iter_set_line_index                 (iter: *mut GtkTextIter, byte_on_line: c_int);
    pub fn gtk_text_iter_set_visible_line_index         (iter: *mut GtkTextIter, byte_on_line: c_int);
    pub fn gtk_text_iter_set_visible_line_offset        (iter: *mut GtkTextIter, char_on_line: c_int);
    pub fn gtk_text_iter_forward_to_end                 (iter: *mut GtkTextIter);
    pub fn gtk_text_iter_forward_to_line_end            (iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_forward_to_tag_toggle          (iter: *mut GtkTextIter, tag: *mut GtkTextTag) -> gboolean;
    pub fn gtk_text_iter_backward_to_tag_toggle         (iter: *mut GtkTextIter, tag: *mut GtkTextTag) -> gboolean;
    pub fn gtk_text_iter_equal                          (iter: *const GtkTextIter, other: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_compare                        (iter: *const GtkTextIter, other: *const GtkTextIter) -> c_int;
    pub fn gtk_text_iter_in_range                       (iter: *const GtkTextIter, start: *const GtkTextIter,
        end: *const GtkTextIter) -> gboolean;
    pub fn gtk_text_iter_order                          (first: *mut GtkTextIter, second: *mut GtkTextIter);

    //=========================================================================
    // GtkTextView                                                       NOT OK
    //=========================================================================
    pub fn gtk_text_view_new                            () -> *mut GtkWidget;
    pub fn gtk_text_view_new_with_buffer                (buffer: *mut GtkTextBuffer) -> *mut GtkWidget;
    pub fn gtk_text_view_set_buffer                     (text_view: *mut GtkTextView, buffer: *mut GtkTextBuffer);
    pub fn gtk_text_view_get_buffer                     (text_view: *mut GtkTextView) -> *mut GtkWidget;
    pub fn gtk_text_view_scroll_to_mark                 (text_view: *mut GtkTextView, mark: *mut GtkTextMark, within_margin: c_double, use_align: gboolean, x_align: c_double, y_align: c_double);
    pub fn gtk_text_view_scroll_to_iter                 (text_view: *mut GtkTextView, iter: *mut GtkTextIter, within_margin: c_double, use_align: gboolean, x_align: c_double, y_align: c_double) -> gboolean;
    pub fn gtk_text_view_scroll_mark_onscreen           (text_view: *mut GtkTextView, mark: *mut GtkTextMark);
    pub fn gtk_text_view_move_mark_onscreen             (text_view: *mut GtkTextView, mark: *mut GtkTextMark) -> gboolean;
    pub fn gtk_text_view_place_cursor_onscreen          (text_view: *mut GtkTextView) -> gboolean;
    // pub fn gtk_text_view_get_visible_rect               (text_view: *mut GtkTextView, visible_rect: *mut GdkRectangle);
    // pub fn gtk_text_view_get_iter_location              (text_view: *mut GtkTextView, iter: *const GtkTextIter, location: *mut GdkRectangle);
    // pub fn gtk_text_view_get_cursor_locations           (text_view: *mut GtkTextView, iter: *const GtkTextIter, strong: *mut GdkRectangle, weak: *mut GdkRectangle);
    pub fn gtk_text_view_get_line_at_y                  (text_view: *mut GtkTextView, target_iter: *mut GtkTextIter, y: c_int, line_top: *mut c_int);
    pub fn gtk_text_view_get_line_yrange                (text_view: *mut GtkTextView, target_iter: *const GtkTextIter, y: *mut c_int, height: *mut c_int);
    pub fn gtk_text_view_get_iter_at_location           (text_view: *mut GtkTextView, iter: *mut GtkTextIter, x: c_int, y: c_int);
    pub fn gtk_text_view_get_iter_at_position           (text_view: *mut GtkTextView, iter: *mut GtkTextIter, trailing: *mut c_int, x: c_int, y: c_int);
    pub fn gtk_text_view_buffer_to_window_coords        (text_view: *mut GtkTextView, win: enums::TextWindowType, buffer_x: c_int, buffer_y: c_int, window_x: *mut c_int, window_y: *mut c_int);
    pub fn gtk_text_view_window_to_buffer_coords        (text_view: *mut GtkTextView, win: enums::TextWindowType, window_x: c_int, window_y: c_int, buffer_x: *mut c_int, buffer_y: *mut c_int);
    // pub fn gtk_text_view_get_window () -> GdkWindow *
    // pub fn gtk_text_view_get_window_type () -> enums::TextWindowType
    pub fn gtk_text_view_set_border_window_size         (text_view: *mut GtkTextView, window_type: enums::TextWindowType, size: c_int);
    pub fn gtk_text_view_get_border_window_size         (text_view: *mut GtkTextView, window_type: enums::TextWindowType) -> c_int;
    pub fn gtk_text_view_forward_display_line           (text_view: *mut GtkTextView, iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_view_backward_display_line          (text_view: *mut GtkTextView, iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_view_forward_display_line_end       (text_view: *mut GtkTextView, iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_view_backward_display_line_start    (text_view: *mut GtkTextView, iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_view_starts_display_line            (text_view: *mut GtkTextView, iter: *mut GtkTextIter) -> gboolean;
    pub fn gtk_text_view_move_visually                  (text_view: *mut GtkTextView, iter: *mut GtkTextIter, count: c_int) -> gboolean;
    pub fn gtk_text_view_add_child_at_anchor            (text_view: *mut GtkTextView, child: *mut GtkWidget, anchor: *mut GtkTextChildAnchor);
    pub fn gtk_text_view_add_child_in_window            (text_view: *mut GtkTextView, child: *mut GtkWidget, which_window: enums::TextWindowType, x_pos: c_int, y_pos: c_int);
    pub fn gtk_text_view_move_child                     (text_view: *mut GtkTextView, child: *mut GtkWidget, x_pos: c_int, y_pos: c_int);
    pub fn gtk_text_view_set_wrap_mode                  (text_view: *mut GtkTextView, warp_mode: enums::WrapMode);
    pub fn gtk_text_view_get_wrap_mode                  (text_view: *mut GtkTextView) -> enums::WrapMode;
    pub fn gtk_text_view_set_editable                   (text_view: *mut GtkTextView, setting: gboolean);
    pub fn gtk_text_view_get_editable                   (text_view: *mut GtkTextView) -> gboolean;
    pub fn gtk_text_view_set_overwrite                  (text_view: *mut GtkTextView, overwrite: gboolean);
    pub fn gtk_text_view_get_overwrite                  (text_view: *mut GtkTextView) -> gboolean;
    pub fn gtk_text_view_set_cursor_visible             (text_view: *mut GtkTextView, setting: gboolean);
    pub fn gtk_text_view_get_cursor_visible             (text_view: *mut GtkTextView) -> gboolean;
    pub fn gtk_text_view_set_pixels_above_lines         (text_view: *mut GtkTextView, pixels_above_lines: c_int);
    pub fn gtk_text_view_get_pixels_above_lines         (text_view: *mut GtkTextView) -> c_int;
    pub fn gtk_text_view_set_pixels_below_lines         (text_view: *mut GtkTextView, pixels_below_lines: c_int);
    pub fn gtk_text_view_get_pixels_below_lines         (text_view: *mut GtkTextView) -> c_int;
    pub fn gtk_text_view_set_pixels_inside_wrap         (text_view: *mut GtkTextView, pixels_inside_wrap: c_int);
    pub fn gtk_text_view_get_pixels_inside_wrap         (text_view: *mut GtkTextView) -> c_int;
    pub fn gtk_text_view_set_justification              (text_view: *mut GtkTextView, justification: enums::Justification);
    pub fn gtk_text_view_get_justification              (text_view: *mut GtkTextView) -> enums::Justification;
    pub fn gtk_text_view_set_left_margin                (text_view: *mut GtkTextView, left_margin: c_int);
    pub fn gtk_text_view_get_left_margin                (text_view: *mut GtkTextView) -> c_int;
    pub fn gtk_text_view_set_right_margin               (text_view: *mut GtkTextView, right_margin: c_int);
    pub fn gtk_text_view_get_right_margin               (text_view: *mut GtkTextView) -> c_int;
    pub fn gtk_text_view_set_indent                     (text_view: *mut GtkTextView, indent: c_int);
    pub fn gtk_text_view_get_indent                     (text_view: *mut GtkTextView) -> c_int;
    // pub fn gtk_text_view_set_tabs ();
    // pub fn gtk_text_view_get_tabs () -> PangoTabArray
    pub fn gtk_text_view_set_accepts_tab                (text_view: *mut GtkTextView, accepts_tab: gboolean);
    pub fn gtk_text_view_get_accepts_tab                (text_view: *mut GtkTextView) -> gboolean;
    pub fn gtk_text_view_get_default_attributes         (text_view: *mut GtkTextView) -> *mut GtkTextAttributes;
    //pub fn gtk_text_view_im_context_filter_keypress     (text_view: *mut GtkTextView, event: *mut GdkEvent) -> gboolean;
    pub fn gtk_text_view_reset_im_context               (text_view: *mut GtkTextView);
    pub fn gtk_text_view_set_input_purpose              (text_view: *mut GtkTextView, purpose: enums::InputPurpose);
    pub fn gtk_text_view_get_input_purpose              (text_view: *mut GtkTextView) -> enums::InputPurpose;
    pub fn gtk_text_view_set_input_hints                (text_view: *mut GtkTextView, hints: enums::InputHints);
    pub fn gtk_text_view_get_input_hints                (text_view: *mut GtkTextView) -> enums::InputHints;

    //=========================================================================
    // GtkTextBuffer                                                     NOT OK
    //=========================================================================
    pub fn gtk_text_buffer_new      (text_tag_table: *mut GtkTextTagTable) -> *mut GtkWidget;
    pub fn gtk_text_buffer_set_text (buffer: *mut GtkTextBuffer, text: *const c_char, length: c_int);

    //=========================================================================
    // GtkTextTagTable                                                   NOT OK
    //=========================================================================
    pub fn gtk_text_tag_table_new   () -> *mut GtkTextTagTable;

    //=========================================================================
    // GtkScrolledWindow                                                 NOT OK
    //=========================================================================
    pub fn gtk_scrolled_window_new          (h_adjustment: *mut GtkAdjustment, v_adjustment: *mut GtkAdjustment) -> *mut GtkWidget;
    pub fn gtk_scrolled_window_set_policy   (scrolled_window: *mut GtkScrolledWindow, h_scrollbar_policy: enums::PolicyType, v_scrollbar_policy: enums::PolicyType);
    pub fn gtk_scrolled_window_get_min_content_width(scrolled_window: *mut GtkScrolledWindow) -> c_int;
    pub fn gtk_scrolled_window_set_min_content_width(scrolled_window: *mut GtkScrolledWindow, width: c_int);
    pub fn gtk_scrolled_window_get_min_content_height(scrolled_window: *mut GtkScrolledWindow) -> c_int;
    pub fn gtk_scrolled_window_set_min_content_height(scrolled_window: *mut GtkScrolledWindow, height: c_int);

    //=========================================================================
    // GtkPlacesSidebar                                                  NOT OK
    //=========================================================================
    pub fn gtk_places_sidebar_new              () -> *mut GtkWidget;
    pub fn gtk_places_sidebar_set_open_flags   (sidebar: *mut GtkPlacesSidebar, flags: enums::PlacesOpenFlags);
    pub fn gtk_places_sidebar_get_open_flags   (sidebar: *mut GtkPlacesSidebar) -> enums::PlacesOpenFlags;
    //pub fn gtk_places_sidebar_set_location     (sidebar: *mut GtkPlacesSidebar, location: *mut glib::File);
    //pub fn gtk_places_sidebar_get_location     (sidebar: *mut GtkPlacesSidebar) -> *mut glib::File;
    pub fn gtk_places_sidebar_set_show_desktop (sidebar: *mut GtkPlacesSidebar, show_desktop: gboolean);
    pub fn gtk_places_sidebar_get_show_desktop (sidebar: *mut GtkPlacesSidebar) -> gboolean;
    //pub fn gtk_places_sidebar_add_shortcut     (sidebar: *mut GtkPlacesSidebar, location: *mut glib::File);
    //pub fn gtk_places_sidebar_remove_shortcut  (sidebar: *mut GtkPlacesSidebar) -> *mut glib::File;
    //pub fn gtk_places_sidebar_list_shortcuts   (sidebar: *mut GtkPlacesSidebar) -> *mut GSList;
    //pub fn gtk_places_sidebar_get_nth_bookmark (sidebar: *mut GtkPlacesSidebar, n: c_int) -> *mut glib::File;
    pub fn gtk_places_sidebar_get_show_connect_to_server(sidebar: *mut GtkPlacesSidebar) -> gboolean;
    pub fn gtk_places_sidebar_set_show_connect_to_server(sidebar: *mut GtkPlacesSidebar, show_connect_to_server: gboolean);
    pub fn gtk_places_sidebar_get_local_only   (sidebar: *mut GtkPlacesSidebar) -> gboolean;
    pub fn gtk_places_sidebar_set_local_only   (sidebar: *mut GtkPlacesSidebar, local_only: gboolean);
    pub fn gtk_places_sidebar_get_show_enter_location(sidebar: *mut GtkPlacesSidebar) -> gboolean;
    pub fn gtk_places_sidebar_set_show_enter_location(sidebar: *mut GtkPlacesSidebar, show_enter_location: gboolean);

    //=========================================================================
    // GtkRadioButton                                                        OK
    //=========================================================================
    pub fn gtk_radio_button_new              (group: *mut c_void) -> *mut GtkWidget;
    pub fn gtk_radio_button_new_with_label   (group: *mut c_void, label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_radio_button_new_with_mnemonic (group: *mut c_void, label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_radio_button_join_group       (radio_button: *mut GtkRadioButton, group_source: *mut GtkRadioButton);

    //=========================================================================
    // GtkTreeView                                                       NOT OK
    //=========================================================================
    pub fn gtk_tree_view_new                           () -> *mut GtkWidget;
    pub fn gtk_tree_view_get_headers_visible           (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_headers_visible           (tree_view: *mut GtkTreeView, headers_visible: gboolean);
    pub fn gtk_tree_view_columns_autosize              (tree_view: *mut GtkTreeView);
    pub fn gtk_tree_view_get_headers_clickable         (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_headers_clickable         (tree_view: *mut GtkTreeView, setting: gboolean);
    pub fn gtk_tree_view_set_rules_hint                (tree_view: *mut GtkTreeView, setting: gboolean);
    pub fn gtk_tree_view_get_rules_hint                (tree_view: *mut GtkTreeView) -> gboolean;
    // GDK_AVAILABLE_IN_3_8
    pub fn gtk_tree_view_get_activate_on_single_click  (tree_view: *mut GtkTreeView) -> gboolean;
    // GDK_AVAILABLE_IN_3_8
    pub fn gtk_tree_view_set_activate_on_single_click  (tree_view: *mut GtkTreeView, single: gboolean);
    // GDK_AVAILABLE_IN_3_4
    pub fn gtk_tree_view_get_n_columns                 (tree_view: *mut GtkTreeView) -> c_uint;
    pub fn gtk_tree_view_scroll_to_point               (tree_view: *mut GtkTreeView, tree_x: c_int, tree_y: c_int);
    pub fn gtk_tree_view_expand_all                    (tree_view: *mut GtkTreeView);
    pub fn gtk_tree_view_collapse_all                  (tree_view: *mut GtkTreeView);
    pub fn gtk_tree_view_set_reorderable               (tree_view: *mut GtkTreeView, reorderable: gboolean);
    pub fn gtk_tree_view_get_reorderable               (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_unset_rows_drag_source        (tree_view: *mut GtkTreeView);
    pub fn gtk_tree_view_unset_rows_drag_dest          (tree_view: *mut GtkTreeView);
    pub fn gtk_tree_view_set_enable_search     (tree_view: *mut GtkTreeView, enable_search: gboolean);
    pub fn gtk_tree_view_get_enable_search     (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_get_search_column     (tree_view: *mut GtkTreeView) -> c_int;
    pub fn gtk_tree_view_set_search_column     (tree_view: *mut GtkTreeView, column: c_int);
    pub fn gtk_tree_view_get_search_entry         (tree_view: *mut GtkTreeView) -> *mut GtkEntry;
    pub fn gtk_tree_view_set_search_entry         (tree_view: *mut GtkTreeView, entry: *mut GtkEntry);
    pub fn gtk_tree_view_convert_widget_to_tree_coords       (tree_view: *mut GtkTreeView, wx: c_int, wy: c_int, tx: *mut c_int, ty: *mut c_int);
    pub fn gtk_tree_view_convert_tree_to_widget_coords       (tree_view: *mut GtkTreeView, tx: c_int, ty: c_int, wx: *mut c_int, wy: *mut c_int);
    pub fn gtk_tree_view_convert_widget_to_bin_window_coords (tree_view: *mut GtkTreeView, wx: c_int, wy: c_int, bx: *mut c_int, by: *mut c_int);
    pub fn gtk_tree_view_convert_bin_window_to_widget_coords (tree_view: *mut GtkTreeView, bx: c_int, by: c_int, wx: *mut c_int, wy: *mut c_int);
    pub fn gtk_tree_view_convert_tree_to_bin_window_coords   (tree_view: *mut GtkTreeView, tx: c_int, ty: c_int, bx: *mut c_int, by: *mut c_int);
    pub fn gtk_tree_view_convert_bin_window_to_tree_coords   (tree_view: *mut GtkTreeView, bx: c_int, by: c_int, tx: *mut c_int, ty: *mut c_int);
    pub fn gtk_tree_view_set_fixed_height_mode (tree_view: *mut GtkTreeView, enable: gboolean);
    pub fn gtk_tree_view_get_fixed_height_mode (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_hover_selection   (tree_view: *mut GtkTreeView, hover: gboolean);
    pub fn gtk_tree_view_get_hover_selection   (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_hover_expand      (tree_view: *mut GtkTreeView, expand: gboolean);
    pub fn gtk_tree_view_get_hover_expand      (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_rubber_banding    (tree_view: *mut GtkTreeView, enable: gboolean);
    pub fn gtk_tree_view_get_rubber_banding    (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_is_rubber_banding_active (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_get_grid_lines         (tree_view: *mut GtkTreeView) -> enums::TreeViewGridLines;
    pub fn gtk_tree_view_set_grid_lines         (tree_view: *mut GtkTreeView, grid_lines: enums::TreeViewGridLines);
    pub fn gtk_tree_view_get_enable_tree_lines  (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_enable_tree_lines  (tree_view: *mut GtkTreeView, enabled: gboolean);
    pub fn gtk_tree_view_set_show_expanders     (tree_view: *mut GtkTreeView, enabled: gboolean);
    pub fn gtk_tree_view_get_show_expanders     (tree_view: *mut GtkTreeView) -> gboolean;
    pub fn gtk_tree_view_set_level_indentation  (tree_view: *mut GtkTreeView, indentation: c_int);
    pub fn gtk_tree_view_get_level_indentation  (tree_view: *mut GtkTreeView) -> c_int;
    pub fn gtk_tree_view_set_tooltip_column (tree_view: *mut GtkTreeView, column: c_int);
    pub fn gtk_tree_view_get_tooltip_column (tree_view: *mut GtkTreeView) -> c_int;
    pub fn gtk_tree_view_new_with_model                (model: *mut GtkTreeModel) -> *mut GtkWidget;
    pub fn gtk_tree_view_get_model                     (tree_view: *mut GtkTreeView) -> *mut GtkTreeModel;
    pub fn gtk_tree_view_set_model                     (tree_view: *mut GtkTreeView, model: *mut GtkTreeModel);
    pub fn gtk_tree_view_get_selection                 (tree_view: *mut GtkTreeView) -> *mut GtkTreeSelection;
    // pub fn gtk_tree_view_set_tooltip_row    (tree_view: *GtkTreeView, GtkTooltip        *tooltip, GtkTreePath       *path);
    // pub fn gtk_tree_view_set_tooltip_cell   (tree_view: *GtkTreeView, GtkTooltip        *tooltip, GtkTreePath       *path, GtkTreeViewColumn *column, GtkCellRenderer   *cell);
    // pub fn gtk_tree_view_get_tooltip_context(tree_view: *GtkTreeView, gint              *x, gint              *y, gboolean           keyboard_tip, GtkTreeModel     **model, GtkTreePath      **path, GtkTreeIter       *iter) -> gboolean;
    // pub fn gtk_tree_view_set_drag_dest_row             (tree_view: *GtkTreeView, GtkTreePath *path, GtkTreeViewDropPosition  pos);
    // pub fn gtk_tree_view_get_drag_dest_row             (tree_view: *GtkTreeView, GtkTreePath **path, GtkTreeViewDropPosition *pos);
    // pub fn gtk_tree_view_get_dest_row_at_pos           (tree_view: *GtkTreeView, gint drag_x, gint drag_y, GtkTreePath **path, GtkTreeViewDropPosition *pos) -> gboolean;
    pub fn gtk_tree_view_set_cursor                    (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, focus_column: *mut GtkTreeViewColumn, start_editing: gboolean);
    // pub fn gtk_tree_view_set_cursor_on_cell            (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, focus_column: *mut GtkTreeViewColumn, focus_cell: *mut GtkCellRenderer, start_editing: gboolean);
    pub fn gtk_tree_view_get_cursor                    (tree_view: *mut GtkTreeView, path: *mut *mut GtkTreePath, focus_column: *mut *mut GtkTreeViewColumn);
    // pub fn gtk_tree_view_get_bin_window                (tree_view: *mut GtkTreeView) -> *mut GdkWindow;
    // pub fn gtk_tree_view_get_path_at_pos               (tree_view: *mut GtkTreeView, x: c_int, y: c_int, path: *mut *mut GtkTreePathh, column: *mut *mut GtkTreeViewColumn, cell_x: *mut c_int, cell_y: *mut c_int) -> gboolean
    // pub fn gtk_tree_view_get_cell_area                 (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, column: *mut GtkTreeViewColumn, rect: *mut GdkRectangle);
    // pub fn gtk_tree_view_get_background_area           (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, column: *mut GtkTreeViewColumn, rect: *mut GdkRectangle);
    // pub fn gtk_tree_view_get_visible_rect              (tree_view: *GtkTreeView, GdkRectangle              *visible_rect);
    // pub fn gtk_tree_view_get_visible_range             (tree_view: *GtkTreeView, GtkTreePath              **start_path, GtkTreePath              **end_path) -> gboolean
    // pub fn gtk_tree_view_is_blank_at_pos               (tree_view: *GtkTreeView, gint x, gint y, GtkTreePath **path, GtkTreeViewColumn **column, gint *cell_x, gint *cell_y) -> gboolean;
    // pub fn gtk_tree_view_enable_model_drag_source      (tree_view: *GtkTreeView, GdkModifierType  start_button_mask, const GtkTargetEntry *targets, gint n_targets, GdkDragAction actions);
    // pub fn gtk_tree_view_enable_model_drag_dest        (tree_view: *GtkTreeView, const GtkTargetEntry *targets, gint n_targets, GdkDragAction actions);
    // pub fn gtk_tree_view_expand_to_path                (tree_view: *mut GtkTreeView, path: *mut GtkTreePath);
    pub fn gtk_tree_view_expand_row                    (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, open_all: gboolean) -> gboolean;
    pub fn gtk_tree_view_collapse_row                  (tree_view: *mut GtkTreeView, path: *mut GtkTreePath) -> gboolean;
    // pub fn gtk_tree_view_row_expanded                  (tree_view: *mut GtkTreeView, path: *mut GtkTreePath) -> gboolean;
    // pub fn gtk_tree_view_scroll_to_cell                (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, column: *mut GtkTreeViewColumn, use_align: gboolean, row_align: c_float, col_align: c_float);
    // pub fn gtk_tree_view_row_activated                 (tree_view: *mut GtkTreeView, path: *mut GtkTreePath, column: *mut GtkTreeViewColumn);
    // pub fn gtk_tree_view_get_column                    (tree_view: *mut GtkTreeView, n: c_int) -> *mut GtkTreeViewColumn;
    // pub fn gtk_tree_view_get_columns                   (tree_view: *mut GtkTreeView) -> *mut GList;
    // pub fn gtk_tree_view_move_column_after             (tree_view: *mut GtkTreeView, column: *mut GtkTreeViewColumn, base_column: *mut GtkTreeViewColumn);
    // pub fn gtk_tree_view_set_expander_column           (tree_view: *mut GtkTreeView, column: *mut GtkTreeViewColumn);
    // pub fn gtk_tree_view_get_expander_column           (tree_view: *mut GtkTreeView) -> *mut GtkTreeViewColumn;
    pub fn gtk_tree_view_append_column                 (tree_view: *mut GtkTreeView, column: *mut GtkTreeViewColumn) -> c_int;
    // pub fn gtk_tree_view_remove_column                 (tree_view: *mut GtkTreeView, column: *mut GtkTreeViewColumn) -> c_int;
    // pub fn gtk_tree_view_insert_column                 (tree_view: *mut GtkTreeView, column: *mut GtkTreeViewColumn, position: c_int) -> c_int;

    //=========================================================================
    // GtkTreeViewColumn
    //=========================================================================
    pub fn gtk_tree_view_column_new                 () -> *mut GtkTreeViewColumn;
    pub fn gtk_tree_view_column_clear               (tree_column: *mut GtkTreeViewColumn);
    pub fn gtk_tree_view_column_set_spacing         (tree_column: *mut GtkTreeViewColumn, spacing: c_int);
    pub fn gtk_tree_view_column_get_spacing         (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_set_visible         (tree_column: *mut GtkTreeViewColumn, visible: gboolean);
    pub fn gtk_tree_view_column_get_visible         (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_set_resizable       (tree_column: *mut GtkTreeViewColumn, resizable: gboolean);
    pub fn gtk_tree_view_column_get_resizable       (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_set_sizing          (tree_column: *mut GtkTreeViewColumn, _type: enums::TreeViewColumnSizing);
    pub fn gtk_tree_view_column_get_sizing          (tree_column: *mut GtkTreeViewColumn) -> enums::TreeViewColumnSizing;
    pub fn gtk_tree_view_column_get_x_offset        (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_get_width           (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_get_fixed_width     (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_set_fixed_width     (tree_column: *mut GtkTreeViewColumn, fixed_width: c_int);
    pub fn gtk_tree_view_column_set_min_width       (tree_column: *mut GtkTreeViewColumn, min_width: c_int);
    pub fn gtk_tree_view_column_get_min_width       (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_set_max_width       (tree_column: *mut GtkTreeViewColumn, max_width: c_int);
    pub fn gtk_tree_view_column_get_max_width       (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_clicked             (tree_column: *mut GtkTreeViewColumn);
    pub fn gtk_tree_view_column_set_title           (tree_column: *mut GtkTreeViewColumn, title: *const c_char);
    pub fn gtk_tree_view_column_get_title           (tree_column: *mut GtkTreeViewColumn) -> *const c_char;
    pub fn gtk_tree_view_column_set_expand          (tree_column: *mut GtkTreeViewColumn, expand: gboolean);
    pub fn gtk_tree_view_column_get_expand          (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_set_clickable       (tree_column: *mut GtkTreeViewColumn, clickable: gboolean);
    pub fn gtk_tree_view_column_get_clickable       (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_set_widget          (tree_column: *mut GtkTreeViewColumn, widget: *mut GtkWidget);
    pub fn gtk_tree_view_column_get_widget          (tree_column: *mut GtkTreeViewColumn) -> *mut GtkWidget;
    pub fn gtk_tree_view_column_set_alignment       (tree_column: *mut GtkTreeViewColumn, xalign: c_float);
    pub fn gtk_tree_view_column_get_alignment       (tree_column: *mut GtkTreeViewColumn) -> c_float;
    pub fn gtk_tree_view_column_set_reorderable     (tree_column: *mut GtkTreeViewColumn, reorderable: gboolean);
    pub fn gtk_tree_view_column_get_reorderable     (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_set_sort_column_id  (tree_column: *mut GtkTreeViewColumn, sort_column_id: c_int);
    pub fn gtk_tree_view_column_get_sort_column_id  (tree_column: *mut GtkTreeViewColumn) -> c_int;
    pub fn gtk_tree_view_column_set_sort_indicator  (tree_column: *mut GtkTreeViewColumn, setting: gboolean);
    pub fn gtk_tree_view_column_get_sort_indicator  (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_set_sort_order      (tree_column: *mut GtkTreeViewColumn, order: enums::SortType);
    pub fn gtk_tree_view_column_get_sort_order      (tree_column: *mut GtkTreeViewColumn) -> enums::SortType;
    pub fn gtk_tree_view_column_cell_is_visible     (tree_column: *mut GtkTreeViewColumn) -> gboolean;
    pub fn gtk_tree_view_column_queue_resize        (tree_column: *mut GtkTreeViewColumn);
    pub fn gtk_tree_view_column_get_tree_view       (tree_column: *mut GtkTreeViewColumn) -> *mut GtkWidget;
    pub fn gtk_tree_view_column_get_button          (tree_column: *mut GtkTreeViewColumn) -> *mut GtkWidget;
    // pub fn gtk_tree_view_column_cell_get_size       (tree_column: *mut GtkTreeViewColumn, cell_area: *const GdkRectangle, x_offset: *mut c_int, y_offset: *mut c_int, width: *mut c_int, height: *mut c_int);
    pub fn gtk_tree_view_column_add_attribute       (tree_column: *mut GtkTreeViewColumn, cell: *mut GtkCellRenderer, attribute: *const c_char, column: c_int);
    pub fn gtk_tree_view_column_clear_attributes    (tree_column: *mut GtkTreeViewColumn, cell: *mut GtkCellRenderer);
    // pub fn gtk_tree_view_column_cell_get_position   (tree_column: *mut GtkTreeViewColumn, cell_renderer: *mut GtkCellRenderer, x_offset: *mut c_int, width: *mut c_int) -> gboolean;
    // pub fn gtk_tree_view_column_focus_cell          (tree_column: *mut GtkTreeViewColumn, cell: *mut GtkCellRenderer);
    // pub fn gtk_tree_view_column_new_with_area       (area: *mut GtkCellArea) -> *mut GtkTreeViewColumn;
    pub fn gtk_tree_view_column_pack_start          (tree_column: *mut GtkTreeViewColumn, cell: *mut GtkCellRenderer, expand: gboolean);
    pub fn gtk_tree_view_column_pack_end            (tree_column: *mut GtkTreeViewColumn, cell: *mut GtkCellRenderer, expand: gboolean);
    // pub fn gtk_tree_view_column_cell_set_cell_data  (tree_column: *mut GtkTreeViewColumn, tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, is_expander: gboolean, is_expanded: gboolean);


    //=========================================================================
    // GtkMenuShell
    //=========================================================================
    pub fn gtk_menu_shell_append         (menu_shell: *mut GtkMenuShell, child: *mut GtkWidget);
    pub fn gtk_menu_shell_prepend        (menu_shell: *mut GtkMenuShell, child: *mut GtkWidget);
    pub fn gtk_menu_shell_insert         (menu_shell: *mut GtkMenuShell, child: *mut GtkWidget, position: c_int);
    pub fn gtk_menu_shell_deactivate     (menu_shell: *mut GtkMenuShell);
    pub fn gtk_menu_shell_select_item    (menu_shell: *mut GtkMenuShell, menu_item: *mut GtkWidget);
    pub fn gtk_menu_shell_deselect       (menu_shell: *mut GtkMenuShell);
    pub fn gtk_menu_shell_activate_item  (menu_shell: *mut GtkMenuShell, menu_item: *mut GtkWidget, force_deactivate: gboolean);
    pub fn gtk_menu_shell_select_first   (menu_shell: *mut GtkMenuShell, search_sensitive: gboolean);
    pub fn gtk_menu_shell_cancel         (menu_shell: *mut GtkMenuShell);
    pub fn gtk_menu_shell_get_take_focus (menu_shell: *mut GtkMenuShell) -> gboolean;
    pub fn gtk_menu_shell_set_take_focus (menu_shell: *mut GtkMenuShell, take_focus: gboolean);
    pub fn gtk_menu_shell_get_selected_item (menu_shell: *mut GtkMenuShell) -> *mut GtkWidget;
    pub fn gtk_menu_shell_get_parent_shell  (menu_shell: *mut GtkMenuShell) -> *mut GtkWidget;
    // GDK_AVAILABLE_IN_3_6
    // pub fn gtk_menu_shell_bind_model   (menu_shell: *mut GtkMenuShell, model: *mut GMenuModel, action_namespace: *mut c_char, with_separators: gboolean);

    //=========================================================================
    // GtkMenuItem
    //=========================================================================
    pub fn gtk_menu_item_new                  () -> *mut GtkWidget;
    pub fn gtk_menu_item_new_with_label       (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_menu_item_new_with_mnemonic    (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_menu_item_set_submenu          (menu_item: *mut GtkMenuItem, submenu: *mut GtkWidget);
    pub fn gtk_menu_item_get_submenu          (menu_item: *mut GtkMenuItem) -> *mut GtkWidget;
    pub fn gtk_menu_item_select               (menu_item: *mut GtkMenuItem);
    pub fn gtk_menu_item_deselect             (menu_item: *mut GtkMenuItem);
    pub fn gtk_menu_item_activate             (menu_item: *mut GtkMenuItem);
    pub fn gtk_menu_item_toggle_size_request  (menu_item: *mut GtkMenuItem, requisition: *mut c_int);
    pub fn gtk_menu_item_toggle_size_allocate (menu_item: *mut GtkMenuItem, allocation: c_int);
    pub fn gtk_menu_item_set_accel_path    (menu_item: *mut GtkMenuItem, accel_path: *const c_char);
    pub fn gtk_menu_item_get_accel_path    (menu_item: *mut GtkMenuItem) -> *const c_char;
    pub fn gtk_menu_item_set_label         (menu_item: *mut GtkMenuItem, label: *const c_char);
    pub fn gtk_menu_item_get_label         (menu_item: *mut GtkMenuItem) -> *const c_char;
    pub fn gtk_menu_item_set_use_underline    (menu_item: *mut GtkMenuItem, setting: gboolean);
    pub fn gtk_menu_item_get_use_underline    (menu_item: *mut GtkMenuItem) -> gboolean;
    pub fn gtk_menu_item_set_reserve_indicator (Gmenu_item: *mut GtkMenuItem, reserve: gboolean);
    pub fn gtk_menu_item_get_reserve_indicator (menu_item: *mut GtkMenuItem) -> gboolean;

    //=========================================================================
    // GtkSeparatorMenuItem
    //=========================================================================
    pub fn gtk_separator_menu_item_new() -> *mut GtkWidget;

    //=========================================================================
    // GtkSeparatorMenuItem
    //=========================================================================
    pub fn gtk_check_menu_item_new               () -> *mut GtkWidget;
    pub fn gtk_check_menu_item_new_with_label    (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_check_menu_item_new_with_mnemonic (label: *const c_char) -> *mut GtkWidget;
    pub fn gtk_check_menu_item_set_active        (check_menu_item: *mut GtkCheckMenuItem, is_active: gboolean);
    pub fn gtk_check_menu_item_get_active        (check_menu_item: *mut GtkCheckMenuItem) -> gboolean;
    pub fn gtk_check_menu_item_toggled           (check_menu_item: *mut GtkCheckMenuItem);
    pub fn gtk_check_menu_item_set_inconsistent  (check_menu_item: *mut GtkCheckMenuItem, setting: gboolean);
    pub fn gtk_check_menu_item_get_inconsistent  (check_menu_item: *mut GtkCheckMenuItem) -> gboolean;
    pub fn gtk_check_menu_item_set_draw_as_radio (check_menu_item: *mut GtkCheckMenuItem, draw_as_radio: gboolean);
    pub fn gtk_check_menu_item_get_draw_as_radio (check_menu_item: *mut GtkCheckMenuItem) -> gboolean;


    //=========================================================================
    // GtkSeparatorMenuItem
    //=========================================================================
    pub fn gtk_scrollbar_new      (orientation: enums::Orientation, adjustment: *mut GtkAdjustment) -> *mut GtkWidget;

    //=========================================================================
    // GtkViewport
    //=========================================================================
    pub fn gtk_viewport_new             (hadjustment: *mut GtkAdjustment, vadjustment: *mut GtkAdjustment) -> *mut GtkWidget;
    pub fn gtk_viewport_set_shadow_type (viewport: *mut GtkViewport, ty: enums::ShadowType);
    pub fn gtk_viewport_get_shadow_type (viewport: *mut GtkViewport) -> enums::ShadowType;
    // pub fn gtk_viewport_get_bin_window  (GtkViewport   *viewport) -> *GdkWindow;
    // pub fn gtk_viewport_get_view_window (GtkViewport   *viewport) -> *GdkWindow;


    //=========================================================================
    // GtkStatusBar
    //=========================================================================
    pub fn gtk_statusbar_new            () -> *mut GtkWidget;
    pub fn gtk_statusbar_get_context_id (statusbar: *mut GtkStatusbar, context_description: *const c_char) -> c_uint;
    pub fn gtk_statusbar_push           (statusbar: *mut GtkStatusbar, context_id: c_uint, text: *const c_char) -> c_uint;
    pub fn gtk_statusbar_pop            (statusbar: *mut GtkStatusbar, context_id: c_uint);
    pub fn gtk_statusbar_remove         (statusbar: *mut GtkStatusbar, context_id: c_uint, message_id: c_uint);
    pub fn gtk_statusbar_remove_all     (statusbar: *mut GtkStatusbar, context_id: c_uint);
    pub fn gtk_statusbar_get_message_area(statusbar: *mut GtkStatusbar) -> *mut GtkWidget;

    //=========================================================================
    // GtkLockButton                                                         OK
    //=========================================================================
    pub fn gtk_lock_button_new          (permission: *mut GPermission) -> *mut GtkWidget;
    pub fn gtk_lock_button_get_permission(button: *mut GtkLockButton) -> *mut GPermission;
    pub fn gtk_lock_button_set_permission(button: *mut GtkLockButton, permission: *mut GPermission);

    //=========================================================================
    // GtkActionable                                                     NOT OK
    //=========================================================================
    pub fn gtk_actionable_get_action_name(actionable: *mut GtkActionable) -> *const c_char;
    pub fn gtk_actionable_set_action_name(actionable: *mut GtkActionable, action_name: *const c_char);
    //pub fn gtk_actionable_get_action_target_value(actionable: *mut GtkActionable) -> *mut GVariant;
    //pub fn gtk_actionable_set_action_target_value(actionable: *mut GtkActionable, target_value: *mut GVariant);
    //pub fn gtk_actionable_set_action_target(actionable: *mut GtkActionable, format_string: *const c_char, ...);
    pub fn gtk_actionable_set_detailed_action_name(actionable: *mut GtkActionable, detailed_action_name: *const c_char);

    //=========================================================================
    // GtkSizeGroup                                                      NOT OK
    //=========================================================================
    pub fn gtk_size_group_new              (mode: enums::SizeGroupMode) -> *mut GtkSizeGroup;
    pub fn gtk_size_group_set_mode         (size_group: *mut GtkSizeGroup, mode: enums::SizeGroupMode);
    pub fn gtk_size_group_get_mode         (size_group: *mut GtkSizeGroup) -> enums::SizeGroupMode;
    pub fn gtk_size_group_set_ignore_hidden(size_group: *mut GtkSizeGroup, ignore_hidden: gboolean);
    pub fn gtk_size_group_get_ignore_hidden(size_group: *mut GtkSizeGroup) -> gboolean;
    pub fn gtk_size_group_add_widget       (size_group: *mut GtkSizeGroup, widget: *mut GtkWidget);
    pub fn gtk_size_group_remove_widget    (size_group: *mut GtkSizeGroup, widget: *mut GtkWidget);
    //pub fn gtk_size_group_get_widgets      (size_group: *mut GtkSizeGroup) -> *mut GSList;

    //=========================================================================
    // GtkAppChooserWidget                                               NOT OK
    //=========================================================================
    pub fn gtk_app_chooser_widget_new      (content_type: *const c_char) -> *mut GtkWidget;
    pub fn gtk_app_chooser_widget_set_show_default(_self: *mut GtkAppChooserWidget, setting: gboolean);
    pub fn gtk_app_chooser_widget_get_show_default(_self: *mut GtkAppChooserWidget) -> gboolean;
    pub fn gtk_app_chooser_widget_set_show_recommended(_self: *mut GtkAppChooserWidget, setting: gboolean);
    pub fn gtk_app_chooser_widget_get_show_recommended(_self: *mut GtkAppChooserWidget) -> gboolean;
    pub fn gtk_app_chooser_widget_set_show_fallback(_self: *mut GtkAppChooserWidget, setting: gboolean);
    pub fn gtk_app_chooser_widget_get_show_fallback(_self: *mut GtkAppChooserWidget) -> gboolean;
    pub fn gtk_app_chooser_widget_set_show_other(_self: *mut GtkAppChooserWidget, setting: gboolean);
    pub fn gtk_app_chooser_widget_get_show_other(_self: *mut GtkAppChooserWidget) -> gboolean;
    pub fn gtk_app_chooser_widget_set_show_all(_self: *mut GtkAppChooserWidget, setting: gboolean);
    pub fn gtk_app_chooser_widget_get_show_all(_self: *mut GtkAppChooserWidget) -> gboolean;
    pub fn gtk_app_chooser_widget_set_default_text(_self: *mut GtkAppChooserWidget, text: *const c_char);
    pub fn gtk_app_chooser_widget_get_default_text(_self: *mut GtkAppChooserWidget) -> *const c_char;

    //=========================================================================
    // GtkFileChooserWidget                                                  OK
    //=========================================================================
    pub fn gtk_file_chooser_widget_new     (action: enums::FileChooserAction) -> *mut GtkWidget;

    //=========================================================================
    // GtkColorChooserWidget                                                 OK
    //=========================================================================
    pub fn gtk_color_chooser_widget_new    () -> *mut GtkWidget;

    //=========================================================================
    // GtkFontChooserWidget                                                 OK
    //=========================================================================
    pub fn gtk_font_chooser_widget_new     () -> *mut GtkWidget;

    //=========================================================================
    // GtkSocket                                                         NOT OK
    //=========================================================================
    pub fn gtk_socket_new                  () -> *mut GtkWidget;
    //pub fn gtk_socket_add_id               (socket: *mut GtkSocket, Window window);
    //pub fn gtk_socket_get_id               (socket: *mut GtkSocket) -> Window;
    //pub fn gtk_socket_get_plug_window      (socket: *mut GtkSocket) -> *mut GdkWindow;

    //=========================================================================
    // GtkEventBox                                                       NOT OK
    //=========================================================================
    pub fn gtk_event_box_new               () -> *mut GtkWidget;
    pub fn gtk_event_box_set_above_child   (event_box: *mut GtkEventBox, above_child: gboolean);
    pub fn gtk_event_box_get_above_child   (event_box: *mut GtkEventBox) -> gboolean;
    pub fn gtk_event_box_set_visible_window(event_box: *mut GtkEventBox, visible_window: gboolean);
    pub fn gtk_event_box_get_visible_window(event_box: *mut GtkEventBox) -> gboolean;

    //=========================================================================
    // Glue fixe code
    //=========================================================================
    pub fn g_signal_connect_data(instance: gpointer,
                                 detailed_signal: *const c_char,
                                 c_hanlder: Option<extern "C" fn()>,
                                 data: gpointer,
                                 destroy_data: Option<extern "C" fn(gpointer, *const GClosure)>,
                                 connect_flags: i32);


    // Not useful to implement but functions are declared at least...
    //=========================================================================
    // GtkBuilder                                                        NOT OK
    //=========================================================================
    pub fn gtk_builder_get_type                () -> GType;
    pub fn gtk_builder_new                     () -> *mut GtkBuilder;
    pub fn gtk_builder_new_from_file           (file_name: *const c_char) -> *mut GtkBuilder;
    pub fn gtk_builder_new_from_resource       (resource_path: *const c_char) -> *mut GtkBuilder;
    pub fn gtk_builder_new_from_string         (string: *const c_char, length: c_long) -> *mut GtkBuilder;
    //pub fn gtk_builder_add_callback_symbol     (builder: *mut GtkBuilder, callback_name: *const c_char, callback_symbol: GCallback);
    //pub fn gtk_builder_add_callback_symbols    (builder: *mut GtkBuilder, callback_name: *const c_char, first_callback_symbol: GCallback, ...);
    //pub fn gtk_builder_lookup_callback_symbol  (builder: *mut GtkBuilder, callback_name: *const c_char) -> GCallback;
    //pub fn gtk_builder_add_from_file           (builder: *mut GtkBuilder, file_name: *const c_char, error: *mut *mut GError) -> c_uint;
    //pub fn gtk_builder_add_from_resource       (builder: *mut GtkBuilder, resource_name: *const c_char, error: *mut *mut GError) -> c_uint;
    //pub fn gtk_builder_add_from_string         (builder: *mut GtkBuilder, buffer: *const c_char, length: c_long, error: *mut *mut GError) -> c_uint;
    //pub fn gtk_builder_add_objects_from_file   (builder: *mut GtkBuilder, file_name: *const c_char, object_ids: *mut *mut c_char, error: *mut *mut GError) -> c_uint;
    //pub fn gtk_builder_add_objects_from_string (builder: *mut GtkBuilder, buffer: *const c_char, length: c_long, object_ids: *mut *mut c_char, error: *mut *mut GError) -> c_uint;
    //pub fn gtk_builder_add_objects_from_resource(builder: *mut GtkBuilder, resource_name: *const c_char, object_ids: *mut *mut c_char, error: *mut *mut GError) -> c_uint;
    pub fn gtk_builder_get_object              (builder: *mut GtkBuilder, name: *const c_char) -> *mut GObject;
    //pub fn gtk_builder_get_objects             (builder: *mut GtkBuilder) -> *mut GSList;
    //pub fn gtk_builder_expose_object           (builder: *mut GtkBuilder, name: *const c_char, object: *mut GObject);
    //pub fn gtk_builder_connect_signals         (builder: *mut GtkBuilder, user_data: *mut c_void);
    //pub fn gtk_builder_connect_signals_full    (builder: *mut GtkBuilder, func: GtkBuilderConnectFunc, user_data: *mut c_void);
    //pub fn gtk_builder_set_translation_domain  (builder: *mut GtkBuilder, domain: *const c_char);
    //pub fn gtk_builder_get_translation_domain  (builder: *mut GtkBuilder) -> *const c_char;
    //pub fn gtk_builder_set_application         (builder: *mut GtkBuilder, application: *mut GtkApplication);
    //pub fn gtk_builder_get_application         (builder: *mut GtkBuilder) -> *mut GtkApplication;
    //pub fn gtk_builder_get_type_from_name      (builder: *mut GtkBuilder, type_name: *const c_char) -> GType;
    //pub fn gtk_builder_value_from_string       (builder: *mut GtkBuilder, pspec: *mut GParamSpec, string: *const c_char, value: *mut GValue,
        //error: *mut *mut GError) -> gboolean;
    //pub fn gtk_builder_value_from_string_type  (builder: *mut GtkBuilder, _type: GType, string: *const c_char, value: *mut GValue,
        //error: *mut *mut GError) -> gboolean;

    //function pointer
    //let GtkBuilderConnectFunc = fn(builder: *mut GtkBuilder, object: *mut GObject, signal_name: *const c_char, handler_name: *const c_char,
        //connect_object: *mut GObject, flags: enums::GConnectFlags, user_data: *mut c_void);


    //=========================================================================
    // GTK Casts functions
    //=========================================================================
    pub fn cast_GtkObject(widget: *mut GtkWidget) -> *mut GObject;
    pub fn cast_GtkObjectFromListStore(widget: *mut GtkListStore) -> *mut GObject;
    pub fn cast_GtkObjectFromTreeStore(widget: *mut GtkTreeStore) -> *mut GObject;
    pub fn cast_GtkObjectFromTreeViewColumn(widget: *mut GtkTreeViewColumn) -> *mut GObject;
    pub fn cast_GtkObjectFromTreeSelection(widget: *mut GtkTreeSelection) -> *mut GObject;
    pub fn cast_GtkWidget(widget: *mut GObject) -> *mut GtkWidget;
    pub fn cast_GtkWindow(widget: *mut GtkWidget) -> *mut GtkWindow;
    pub fn cast_GtkBin(widget: *mut GtkWidget) -> *mut GtkBin;
    pub fn cast_GtkButton(widget: *mut GtkWidget) -> *mut GtkButton;
    pub fn cast_GtkContainer(widget: *mut GtkWidget) -> *mut GtkContainer;
    pub fn cast_GtkFrame(widget: *mut GtkWidget) -> *mut GtkFrame;
    pub fn cast_GtkLabel(widget: *mut GtkWidget) -> *mut GtkLabel;
    pub fn cast_GtkMisc(widget: *mut GtkWidget) -> *mut GtkMisc;
    pub fn cast_GtkOrientable(widget: *mut GtkWidget) -> *mut GtkOrientable;
    pub fn cast_GtkRange(widget: *mut GtkWidget) -> *mut GtkRange;
    pub fn cast_GtkBox(widget: *mut GtkWidget) -> *mut GtkBox;
    pub fn cast_GtkFixed(widget: *mut GtkWidget) -> *mut GtkFixed;
    pub fn cast_GtkButtonBox(widget: *mut GtkWidget) -> *mut GtkButtonBox;
    pub fn cast_GtkAspectFrame(widget: *mut GtkWidget) -> *mut GtkAspectFrame;
    pub fn cast_GtkFontButton(widget: *mut GtkWidget) -> *mut GtkFontButton;
    pub fn cast_GtkToggleButton(widget: *mut GtkWidget) -> *mut GtkToggleButton;
    pub fn cast_GtkCheckButton(widget: *mut GtkWidget) -> *mut GtkCheckButton;
    pub fn cast_GtkMenuButton(widget: *mut GtkWidget) -> *mut GtkMenuButton;
    pub fn cast_GtkColorButton(widget: *mut GtkWidget) -> *mut GtkColorButton;
    pub fn cast_GtkLinkButton(widget: *mut GtkWidget) -> *mut GtkLinkButton;
    pub fn cast_GtkScaleButton(widget: *mut GtkWidget) -> *mut GtkScaleButton;
    pub fn cast_GtkGrid(widget: *mut GtkWidget) -> *mut GtkGrid;
    pub fn cast_GtkEntry(widget: *mut GtkWidget) -> *mut GtkEntry;
    pub fn cast_GtkSwitch(widget: *mut GtkWidget) -> *mut GtkSwitch;
    pub fn cast_GtkScale(widget: *mut GtkWidget) -> *mut GtkScale;
    pub fn cast_GtkLevelBar(widget: *mut GtkWidget) -> *mut GtkLevelBar;
    pub fn cast_GtkSearchBar(widget: *mut GtkWidget) -> *mut GtkSearchBar;
    pub fn cast_GtkSpinButton(widget: *mut GtkWidget) -> *mut GtkSpinButton;
    pub fn cast_GtkSpinner(widget: *mut GtkWidget) -> *mut GtkSpinner;
    pub fn cast_GtkProgressBar(widget: *mut GtkWidget) -> *mut GtkProgressBar;
    pub fn cast_GtkArrow(widget: *mut GtkWidget) -> *mut GtkArrow;
    pub fn cast_GtkCalendar(widget: *mut GtkWidget) -> *mut GtkCalendar;
    pub fn cast_GtkAlignment(widget: *mut GtkWidget) -> *mut GtkAlignment;
    pub fn cast_GtkExpander(widget: *mut GtkWidget) -> *mut GtkExpander;
    pub fn cast_GtkPaned(widget: *mut GtkWidget) -> *mut GtkPaned;
    pub fn cast_GtkInfoBar(widget: *mut GtkWidget) -> *mut GtkInfoBar;
    pub fn cast_GtkToolShell(widget: *mut GtkWidget) -> *mut GtkToolShell;
    pub fn cast_GtkToolbar(widget: *mut GtkWidget) -> *mut GtkToolbar;
    pub fn cast_GtkToolItem(widget: *mut GtkWidget) -> *mut GtkToolItem;
    pub fn cast_GtkToolButton(widget: *mut GtkWidget) -> *mut GtkToolButton;
    pub fn cast_GtkSeparatorToolItem(widget: *mut GtkWidget) -> *mut GtkSeparatorToolItem;
    pub fn cast_GtkMenuToolButton(widget: *mut GtkWidget) -> *mut GtkMenuToolButton;
    pub fn cast_GtkToggleToolButton(widget: *mut GtkWidget) -> *mut GtkToggleToolButton;
    pub fn cast_GtkRadioToolButton(widget: *mut GtkWidget) -> *mut GtkRadioToolButton;
    pub fn cast_GtkDialog(widget: *mut GtkWidget) -> *mut GtkDialog;
    pub fn cast_GtkAboutDialog(widget: *mut GtkWidget) -> *mut GtkAboutDialog;
    pub fn cast_GtkMessageDialog(widget: *mut GtkWidget) -> *mut GtkMessageDialog;
    pub fn cast_GtkColorChooserDialog(widget: *mut GtkWidget) -> *mut GtkColorChooserDialog;
    pub fn cast_GtkColorChooser(widget: *mut GtkWidget) -> *mut GtkColorChooser;
    pub fn cast_GtkAdjustment(widget: *mut GObject) -> *mut GtkAdjustment;
    pub fn cast_GtkNotebook(widget: *mut GtkWidget) -> *mut GtkNotebook;
    pub fn cast_GtkStack(widget: *mut GtkWidget) -> *mut GtkStack;
    pub fn cast_GtkStackSwitcher(widget: *mut GtkWidget) -> *mut GtkStackSwitcher;
    pub fn cast_GtkRevealer(widget: *mut GtkWidget) -> *mut GtkRevealer;
    pub fn cast_GtkOverlay(widget: *mut GtkWidget) -> *mut GtkOverlay;
    pub fn cast_GtkScrollable(widget: *mut GtkWidget) -> *mut GtkScrollable;
    pub fn cast_GtkLayout(widget: *mut GtkWidget) -> *mut GtkLayout;
    pub fn cast_GtkHeaderBar(widget: *mut GtkWidget) -> *mut GtkHeaderBar;
    pub fn cast_GtkFlowBox(widget: *mut GtkWidget) -> *mut GtkFlowBox;
    pub fn cast_GtkFlowBoxChild(widget: *mut GtkWidget) -> *mut GtkFlowBoxChild;
    pub fn cast_GtkListBox(widget: *mut GtkWidget) -> *mut GtkListBox;
    pub fn cast_GtkListBoxRow(widget: *mut GtkWidget) -> *mut GtkListBoxRow;
    pub fn cast_GtkActionBar(widget: *mut GtkWidget) -> *mut GtkActionBar;
    pub fn cast_GtkFileFilter(widget: *mut GObject) -> *mut GtkFileFilter;
    pub fn cast_GtkFileChooser(widget: *mut GtkWidget) -> *mut GtkFileChooser;
    pub fn cast_GtkAppChooser(widget: *mut GtkWidget) -> *mut GtkAppChooser;
    pub fn cast_GtkAppChooserDialog(widget: *mut GtkWidget) -> *mut GtkAppChooserDialog;
    pub fn cast_GtkAppInfo(widget: *mut GtkWidget) -> *mut GAppInfo;
    pub fn cast_GtkAppLaunchContext(widget: *mut GtkWidget) -> *mut GAppLaunchContext;
    pub fn cast_GtkFontChooserDialog(widget: *mut GtkWidget) -> *mut GtkFontChooserDialog;
    pub fn cast_GtkFontChooser(widget: *mut GtkWidget) -> *mut GtkFontChooser;
    pub fn cast_GtkPaperSize(widget: *mut GtkWidget) -> *mut GtkPaperSize;
    pub fn cast_GtkPageSetup(widget: *mut GObject) -> *mut GtkPageSetup;
    // pub fn cast_PageSetupUnixDialog(widget: *mut GtkWidget) -> *mut GtkPageSetupUnixDialog;
    pub fn cast_GtkPrintSettings(widget: *mut GtkWidget) -> *mut GtkPrintSettings;
    pub fn cast_GtkRecentChooserDialog(widget: *mut GtkWidget) -> *mut GtkRecentChooserDialog;
    pub fn cast_GtkRecentManager(widget: *mut GtkWidget) -> *mut GtkRecentManager;
    pub fn cast_GtkRecentChooser(widget: *mut GtkWidget) -> *mut GtkRecentChooser;
    pub fn cast_GtkRecentFilter(widget: *mut GtkWidget) -> *mut GtkRecentFilter;
    pub fn cast_GtkRecentInfo(widget: *mut GtkWidget) -> *mut GtkRecentInfo;
    pub fn cast_GtkEditable(widget: *mut GtkWidget) -> *mut GtkEditable;
    pub fn cast_GtkTextView(widget: *mut GtkWidget) -> *mut GtkTextView;
    pub fn cast_GtkTextBuffer(widget: *mut GtkWidget) -> *mut GtkTextBuffer;
    pub fn cast_GtkTextTagTable(widget: *mut GObject) -> *mut GtkTextTagTable;
    pub fn cast_GtkScrolledWindow(widget: *mut GtkWidget) -> *mut GtkScrolledWindow;
    pub fn cast_GtkRadioButton(widget: *mut GtkWidget) -> *mut GtkRadioButton;
    pub fn cast_GtkTreeView(widget: *mut GtkWidget) -> *mut GtkTreeView;
    pub fn cast_GtkCellRenderer(widget: *mut GtkWidget) -> *mut GtkCellRenderer;
    pub fn cast_GtkMenuShell(widget: *mut GtkWidget) -> *mut GtkMenuShell;
    pub fn cast_GtkMenuItem(widget: *mut GtkWidget) -> *mut GtkMenuItem;
    pub fn cast_GtkCheckMenuItem(widget: *mut GtkWidget) -> *mut GtkCheckMenuItem;
    pub fn cast_GtkViewport(widget: *mut GtkWidget) -> *mut GtkViewport;
    pub fn cast_GtkStatusbar(widget: *mut GtkWidget) -> *mut GtkStatusbar;
    pub fn cast_GtkCellEditable(widget: *mut GtkWidget) -> *mut GtkCellEditable;
    pub fn cast_GtkCellRendererText(widget: *mut GtkWidget) -> *mut GtkCellRendererText;
    pub fn cast_GtkLockButton(widget: *mut GtkWidget) -> *mut GtkLockButton;
    pub fn cast_GtkActionable(widget: *mut GtkWidget) -> *mut GtkActionable;
    pub fn cast_GtkCellLayout(widget: *mut GtkWidget) -> *mut GtkCellLayout;
    pub fn cast_GtkEntryCompletion(widget: *mut GtkWidget) -> *mut GtkEntryCompletion;
    pub fn cast_GtkEntryBuffer(obj: *mut GObject) -> *mut GtkEntryBuffer;
    pub fn cast_GtkIconView(widget: *mut GtkWidget) -> *mut GtkIconView;
    pub fn cast_GtkTreeSelection(widget: *mut GObject) -> *mut GtkTreeSelection;
    pub fn cast_GtkImage(widget: *mut GtkWidget) -> *mut GtkImage;
    pub fn cast_GtkRecentChooserWidget(widget: *mut GtkWidget) -> *mut GtkRecentChooserWidget;
    pub fn cast_GtkTreeModelFromListStore(store: *mut GtkListStore) -> *mut GtkTreeModel;
    pub fn cast_GtkListStoreFromTreeModel(store: *mut GtkTreeModel) -> *mut GtkListStore;
    pub fn cast_GtkListStore(store: *mut GObject) -> *mut GtkListStore;
    pub fn cast_GtkTreeModelFromTreeStore(store: *mut GtkTreeStore) -> *mut GtkTreeModel;
    pub fn cast_GtkTreeStoreFromTreeModel(store: *mut GtkTreeModel) -> *mut GtkTreeStore;
    pub fn cast_GtkComboBox(widget: *mut GtkWidget) -> *mut GtkComboBox;
    pub fn cast_GtkPopover(widget: *mut GtkWidget) -> *mut GtkPopover;
    pub fn cast_GtkComboBoxText(widget: *mut GtkWidget) -> *mut GtkComboBoxText;
    pub fn cast_GtkTreeStore(obj: *mut GObject) -> *mut GtkTreeStore;
    pub fn cast_GtkTreeModel(obj: *mut GObject) -> *mut GtkTreeModel;
    pub fn cast_GtkTextMark(widget: *mut GObject) -> *mut GtkTextMark;
    pub fn cast_GtkPlacesSidebar(widget: *mut GtkWidget) -> *mut GtkPlacesSidebar;
    pub fn cast_GtkToolPalette(widget: *mut GtkWidget) -> *mut GtkToolPalette;
    pub fn cast_GtkToolItemGroup(widget: *mut GtkWidget) -> *mut GtkToolItemGroup;
    pub fn cast_GtkAppChooserWidget(widget: *mut GtkWidget) -> *mut GtkAppChooserWidget;
    pub fn cast_GtkFileChooserWidget(widget: *mut GtkWidget) -> *mut GtkFileChooserWidget;
    pub fn cast_GtkColorChooserWidget(widget: *mut GtkWidget) -> *mut GtkColorChooserWidget;
    pub fn cast_GtkFontChooserWidget(widget: *mut GtkWidget) -> *mut GtkFontChooserWidget;
    pub fn cast_GtkSocket(widget: *mut GtkWidget) -> *mut GtkSocket;
    pub fn cast_GtkEventBox(widget: *mut GtkWidget) -> *mut GtkEventBox;
}
