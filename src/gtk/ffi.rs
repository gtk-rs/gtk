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

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::{c_int, c_char, c_float, c_uint, c_double, c_long, c_short, c_void};

use gdk;
use gtk;
use glib;

pub type Gboolean = c_int;
pub static Gfalse:  c_int = 0;
pub static Gtrue:   c_int = !Gfalse;

pub type gpointer = *const c_void;

pub struct C_GtkWidget;
pub struct C_GtkWindow;
pub struct C_GtkLabel;
pub struct C_GtkMisc;
pub struct C_GtkButton;
pub struct C_GtkBox;
pub struct C_GtkOrientable;
pub struct C_GtkButtonBox;
pub struct C_GtkFrame;
pub struct C_GtkAspectFrame;
pub struct C_GtkFixed;
pub struct C_GtkBin;
pub struct C_GtkContainer;
pub struct C_GtkFontButton;
pub struct C_GtkToggleButton;
pub struct C_GtkCheckButton;
pub struct C_GtkMenuButton;
pub struct C_GtkColorButton;
pub struct C_GtkLinkButton;
pub struct C_GtkAdjustment;
pub struct C_GtkScaleButton;
pub struct C_GtkVolumeButton;
pub struct C_GtkGrid;
pub struct C_GtkEntryBuffer;
pub struct C_GtkEntry;
pub struct C_GtkSearchEntry;
pub struct C_GtkSwitch;
pub struct C_GtkScale;
pub struct C_GtkLevelBar;
pub struct C_GtkSearchBar;
pub struct C_GtkSpinButton;
pub struct C_GtkSpinner;
pub struct C_GtkImage;
pub struct C_GtkProgressBar;
pub struct C_GtkArrow;
pub struct C_GtkCalendar;
pub struct C_GtkAlignment;
pub struct C_GtkExpander;
pub struct C_GtkPaned;
pub struct C_GtkInfoBar;
pub struct C_GtkToolShell;
pub struct C_GtkToolbar;
pub struct C_GtkDialog;
pub struct C_GtkAboutDialog;
pub struct C_GtkMessageDialog;
pub struct C_GtkAppChooserDialog;
pub struct C_GtkColorChooserDialog;
pub struct C_GtkFileChooserDialog;
pub struct C_GtkFileChooser;
pub struct C_GtkNotebook;
pub struct C_GtkStack;
pub struct C_GtkStackSwitcher;
pub struct C_GtkRevealer;
pub struct C_GtkOverlay;
pub struct C_GtkScrollable;
pub struct C_GtkLayout;
pub struct C_GtkHeaderBar;
pub struct C_GtkFlowBox;
pub struct C_GtkFlowBoxChild;
pub struct C_GtkListBox;
pub struct C_GtkListBoxRow;
pub struct C_GtkActionBar;
pub struct C_GtkFileFilter;
pub struct C_GtkAppChooser;
pub struct C_GAppLaunchContext;
pub struct C_GAppInfo;

pub struct C_GtkToolItem;
pub struct C_GtkToolButton;
pub struct C_GtkMenuToolButton;
pub struct C_GtkToggleToolButton;
pub struct C_GtkRadioToolButton;
pub struct C_GtkSeparatorToolItem;

pub struct C_GtkMenu;
pub struct C_GMenuModel;

pub struct C_GClosure;

pub struct C_GtkColorChooser;


pub fn to_gboolean(b: bool) -> Gboolean{
    match b {
        true => Gtrue,
        false => Gfalse
    }
}

pub fn to_bool(b: Gboolean) -> bool {
    b == Gtrue
}

extern "C" {

    //=========================================================================
    // Gtk Main Loop + events
    //=========================================================================
    pub fn gtk_init                            (argc: *const c_int, argv: *const *const *const c_char) -> ();
    pub fn gtk_main                            () -> ();
    pub fn gtk_main_quit                       () -> ();
    pub fn gtk_main_level                      () -> c_uint;
    pub fn gtk_main_iteration                  () -> Gboolean;
    pub fn gtk_main_iteration_do               (blocking: Gboolean) -> Gboolean;

    //=========================================================================
    // GtkWindow
    //=========================================================================
    pub fn gtk_window_new                      (wtype : gtk::WindowType) -> *mut C_GtkWidget;
    pub fn gtk_window_set_title                (window: *mut C_GtkWindow, title: *const c_char) -> ();
    pub fn gtk_window_get_title                (window: *mut C_GtkWindow) -> *const c_char;
    pub fn gtk_widget_hide                     (widget: *mut C_GtkWidget) -> ();
    pub fn gtk_window_set_default_size         (widget: *mut C_GtkWidget, width: c_int, height: c_int);
    // pub fn gtk_window_set_role(window: *const const C_GtkWindow, role: *const c_char) -> ();
    // pub fn gtk_window_set_startup_id(window: *const const C_GtkWindow, startup_id: *const c_char) -> ();
    // pub fn gtk_window_get_role(window: *const const C_GtkWindow) -> *const c_char;
    // pub fn gtk_window_add_accel_group(window: *const const C_GtkWindow, accel_group: *GtkAccelGroup) -> ();
    // pub fn gtk_window_remove_accel_group(window: *const const C_GtkWindow, accel_group: *GtkAccelGroup) -> ();
    // pub fn gtk_window_set_position(window: *const const C_GtkWindow,position: gtk::WindowPosition) -> ();
    // pub fn gtk_window_activate_focus(window: *const const C_GtkWindow) -> Gboolean;
    // pub fn gtk_window_set_focus(window: *const const C_GtkWindow, focus: *const const C_GtkWidget) -> ();
    // pub fn gtk_window_get_focus(window: *const const C_GtkWindow) -> *const const C_GtkWidget;
    // pub fn gtk_window_set_default(window: *const const C_GtkWindow, default_widget: *const const C_GtkWidget) -> ();
    // pub fn gtk_window_get_default_widget(window: *const const C_GtkWindow) -> *const const C_GtkWidget;
    // pub fn gtk_window_activate_default(window: *const const C_GtkWindow) -> Gboolean;
    // pub fn gtk_window_get_type() -> ();


    //=========================================================================
    // GtkWidget
    //=========================================================================
    pub fn gtk_widget_show                     (widget: *mut C_GtkWidget);
    pub fn gtk_widget_show_all                 (widget: *mut C_GtkWidget);
    pub fn gtk_widget_set_margin_right         (widget: *mut C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_set_margin_left          (widget: *mut C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_set_margin_top           (widget: *mut C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_set_margin_bottom        (widget: *mut C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_get_margin_right         (widget: *mut C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_margin_left          (widget: *mut C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_margin_top           (widget: *mut C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_margin_bottom        (widget: *mut C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_allocated_width      (widget: *mut C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_allocated_height     (widget: *mut C_GtkWidget) -> c_int;
    pub fn gtk_widget_destroy                  (widget: *mut C_GtkWidget);

    //=========================================================================
    // GtkLabel
    //=========================================================================
    pub fn gtk_label_new                       (text: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_label_set_label                 (label: *mut C_GtkLabel, text: *const c_char);
    pub fn gtk_label_set_justify               (label: *mut C_GtkLabel, jtype: gtk::Justification);
    pub fn gtk_label_set_text                  (label: *mut C_GtkLabel, stext: *const c_char) -> ();
    // pub fn gtk_label_set_attributes            (label: *const const C_GtkLabel, PangoAttrList *attrs) -> ();
    pub fn gtk_label_set_markup                (label: *mut C_GtkLabel, text: *const c_char) -> ();
    pub fn gtk_label_set_markup_with_mnemonic  (label: *mut C_GtkLabel, text: *const c_char) -> ();
    pub fn gtk_label_set_pattern               (label: *mut C_GtkLabel, text: *const c_char) -> ();
    // pub fn gtk_label_set_ellipsize             (label: *const const C_GtkLabel, PangoEllipsizeMode mode) -> ();
    pub fn gtk_label_set_width_chars           (label: *mut C_GtkLabel, n_chars: c_int) -> ();
    pub fn gtk_label_set_max_width_chars       (label: *mut C_GtkLabel, n_chars: c_int) -> ();
    pub fn gtk_label_set_line_wrap             (label: *mut C_GtkLabel, wrap: Gboolean) -> ();
    // pub fn gtk_label_set_line_wrap_mode        (label: *const const C_GtkLabel, PangoWrapMode wrap_mode);
    pub fn gtk_label_set_lines                 (label: *mut C_GtkLabel, lines: c_int) -> ();
    pub fn gtk_label_get_layout_offsets        (label: *mut C_GtkLabel, x: *const c_int, y: *const c_int) -> ();
    pub fn gtk_label_get_mnemonic_keyval       (label: *mut C_GtkLabel) -> c_uint;
    pub fn gtk_label_get_selectable            (label: *mut C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_text                  (label: *mut C_GtkLabel) -> *const c_char;
    pub fn gtk_label_new_with_mnemonic         (text: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_label_select_region             (label: *mut C_GtkLabel, start_offset: c_int, end_offset: c_int) -> ();
    // pub fn gtk_label_set_mnemonic_widget       (label: *const const C_GtkLabel, widget: *const const C_GtkWidget) -> ();
    pub fn gtk_label_set_selectable            (label: *mut C_GtkLabel, gsetting: Gboolean) -> ();
    pub fn gtk_label_set_text_with_mnemonic    (label: *mut C_GtkLabel, text: *const c_char) -> ();
    // pub fn gtk_label_get_attributes            (label: *const const C_GtkLabel) -> *PangoAttrList;
    pub fn gtk_label_get_justify               (label: *mut C_GtkLabel) -> gtk::Justification;
    // pub fn gtk_label_get_ellipsize             (label: *const const C_GtkLabel) -> PangoEllipsizeMode;
    pub fn gtk_label_get_width_chars           (label: *mut C_GtkLabel) -> c_int;
    pub fn gtk_label_get_max_width_chars       (label: *mut C_GtkLabel) -> c_int;
    pub fn gtk_label_get_label                 (label: *mut C_GtkLabel) -> *const c_char;
    // pub fn gtk_label_get_layout                (label: *const const C_GtkLabel) -> *PangoLayout;
    pub fn gtk_label_get_line_wrap             (label: *mut C_GtkLabel) -> Gboolean;
    // pub fn gtk_label_get_line_wrap_mode        (label: *const const C_GtkLabel) -> PangoWrapMode;
    pub fn gtk_label_get_lines                 (label: *mut C_GtkLabel) -> c_int;
    // pub fn gtk_label_get_mnemonic_widget       (label: *const const C_GtkLabel) -> *const const C_GtkWidget;
    pub fn gtk_label_get_selection_bounds      (label: *mut C_GtkLabel, start: *const c_int, end: *const c_int) -> Gboolean;
    pub fn gtk_label_get_use_markup            (label: *mut C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_use_underline         (label: *mut C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_single_line_mode      (label: *mut C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_angle                 (label: *mut C_GtkLabel) -> c_double;
    pub fn gtk_label_set_use_markup            (label: *mut C_GtkLabel, setting: Gboolean) -> ();
    pub fn gtk_label_set_use_underline         (label: *mut C_GtkLabel, setting: Gboolean) -> ();
    pub fn gtk_label_set_single_line_mode      (label: *mut C_GtkLabel, single_line_mod: Gboolean) -> ();
    pub fn gtk_label_set_angle                 (label: *mut C_GtkLabel, angle: c_double) -> ();
    pub fn gtk_label_get_current_uri           (label: *mut C_GtkLabel) -> *const c_char;
    pub fn gtk_label_set_track_visited_links   (label: *mut C_GtkLabel, track_links: Gboolean) -> ();
    pub fn gtk_label_get_track_visited_links   (label: *mut C_GtkLabel) -> Gboolean;

    //=========================================================================
    // GtkContainer
    //=========================================================================
    pub fn gtk_container_add                   (container: *mut C_GtkContainer, widget: *mut C_GtkWidget);
    pub fn gtk_container_remove                (container: *mut C_GtkContainer, widget: *mut C_GtkWidget) -> ();
    pub fn gtk_container_get_resize_mode       (container: *mut C_GtkContainer) -> gtk::ResizeMode;
    pub fn gtk_container_set_resize_mode       (container: *mut C_GtkContainer, resize_mode: gtk::ResizeMode) -> ();
    pub fn gtk_container_check_resize          (container: *mut C_GtkContainer) -> ();
    pub fn gtk_container_get_border_width      (container: *mut C_GtkContainer) -> c_uint;
    pub fn gtk_container_set_border_width      (container: *mut C_GtkContainer, border_width: c_uint) -> ();


    //=========================================================================
    // GtkMisc                                                               OK
    //=========================================================================

    pub fn gtk_misc_set_alignment              (misc: *mut C_GtkMisc, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_misc_set_padding                (misc: *mut C_GtkMisc, xpad: c_int, ypad: c_int) -> ();
    pub fn gtk_misc_get_alignment              (misc: *mut C_GtkMisc, xalign: *const c_float, yalign: *const c_float) -> ();
    pub fn gtk_misc_get_padding                (misc: *mut C_GtkMisc, xpad: *const c_int, ypad: *const c_int) -> ();

    //=========================================================================
    // GtkButton                                                             OK
    //=========================================================================
    pub fn gtk_button_new                      () -> *mut C_GtkWidget;
    pub fn gtk_button_new_with_label           (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_button_new_with_mnemonic        (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_button_new_from_icon_name       (icon_id: *const c_char, size: gtk::IconSize) -> *mut C_GtkWidget;
    pub fn gtk_button_new_from_stock           (stock_id: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_button_pressed                  (button: *mut C_GtkButton) -> ();
    pub fn gtk_button_released                 (button: *mut C_GtkButton) -> ();
    pub fn gtk_button_clicked                  (button: *mut C_GtkButton) -> ();
    pub fn gtk_button_enter                    (button: *mut C_GtkButton) -> ();
    pub fn gtk_button_leave                    (button: *mut C_GtkButton) -> ();
    pub fn gtk_button_set_relief               (button: *mut C_GtkButton, newstyle: gtk::ReliefStyle) -> ();
    pub fn gtk_button_get_relief               (button: *mut C_GtkButton) -> gtk::ReliefStyle;
    pub fn gtk_button_get_label                (button: *mut C_GtkButton) -> *const c_char;
    pub fn gtk_button_set_label                (button: *mut C_GtkButton, label: *const c_char) -> ();
    pub fn gtk_button_get_use_stock            (button: *mut C_GtkButton) -> Gboolean;
    pub fn gtk_button_set_use_stock            (button: *mut C_GtkButton, use_stock: Gboolean) -> ();
    pub fn gtk_button_get_use_underline        (button: *mut C_GtkButton) -> Gboolean;
    pub fn gtk_button_set_use_underline        (button: *mut C_GtkButton, use_underline: Gboolean) -> ();
    pub fn gtk_button_set_focus_on_click       (button: *mut C_GtkButton, focus_on_click: Gboolean) -> ();
    pub fn gtk_button_get_focus_on_click       (button: *mut C_GtkButton) -> Gboolean;
    pub fn gtk_button_set_alignment            (button: *mut C_GtkButton, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_button_get_alignment            (button: *mut C_GtkButton, xalign: *mut c_float, yalign: *mut c_float) -> ();
    pub fn gtk_button_set_image                (button: *mut C_GtkButton, image: *mut C_GtkWidget) -> ();
    // pub fn gtk_button_get_image                (button: *const const C_GtkButton) -> *const const C_GtkWidget;
    pub fn gtk_button_set_image_position       (button: *mut C_GtkButton, position: gtk::PositionType) -> ();
    pub fn gtk_button_get_image_position       (button: *mut C_GtkButton) -> gtk::PositionType;
    pub fn gtk_button_set_always_show_image    (button: *mut C_GtkButton, always_show: Gboolean) -> ();
    pub fn gtk_button_get_always_show_image    (button: *mut C_GtkButton) -> Gboolean;
    // pub fn gtk_button_get_event_window         (button: *const const C_GtkButton) -> *const const C_GdkWindow;

    //=========================================================================
    // GtkFileChooser                                                    NOT OK
    //=========================================================================
    // FIXME : check if memory is freed when a *const c_char is returned
    pub fn gtk_file_chooser_set_action         (chooser: *mut C_GtkFileChooser, action: gtk::FileChooserAction) -> ();
    pub fn gtk_file_chooser_get_action         (chooser: *mut C_GtkFileChooser) -> gtk::FileChooserAction;
    pub fn gtk_file_chooser_set_local_only     (chooser: *mut C_GtkFileChooser, local_only: Gboolean) -> ();
    pub fn gtk_file_chooser_get_local_only     (chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_set_select_multiple(chooser: *mut C_GtkFileChooser, select_multiple: Gboolean) -> ();
    pub fn gtk_file_chooser_get_select_multiple(chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_set_show_hidden    (chooser: *mut C_GtkFileChooser, show_hidden: Gboolean) -> ();
    pub fn gtk_file_chooser_get_show_hidden    (chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_set_do_overwrite_confirmation(chooser: *mut C_GtkFileChooser, do_overwrite_confirmation: Gboolean) -> ();
    pub fn gtk_file_chooser_get_do_overwrite_confirmation(chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_set_create_folders (chooser: *mut C_GtkFileChooser, create_folders: Gboolean) -> ();
    pub fn gtk_file_chooser_get_create_folders (chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_set_current_name   (chooser: *mut C_GtkFileChooser, name: *const c_char) -> ();
    pub fn gtk_file_chooser_get_current_name   (chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_filename       (chooser: *mut C_GtkFileChooser, filename: *const c_char) -> Gboolean;
    pub fn gtk_file_chooser_get_filename       (chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_select_filename    (chooser: *mut C_GtkFileChooser, filename: *const c_char) -> Gboolean;
    pub fn gtk_file_chooser_unselect_filename  (chooser: *mut C_GtkFileChooser, filename: *const c_char) -> ();
    pub fn gtk_file_chooser_select_all         (chooser: *mut C_GtkFileChooser) -> ();
    pub fn gtk_file_chooser_unselect_all       (chooser: *mut C_GtkFileChooser) -> ();
    pub fn gtk_file_chooser_get_filenames      (chooser: *mut C_GtkFileChooser) -> *mut glib::ffi::C_GSList;
    pub fn gtk_file_chooser_set_current_folder (chooser: *mut C_GtkFileChooser, filename: *const c_char) -> Gboolean;
    pub fn gtk_file_chooser_get_current_folder (chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_uri            (chooser: *mut C_GtkFileChooser, uri: *const c_char) -> Gboolean;
    pub fn gtk_file_chooser_get_uri            (chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_select_uri         (chooser: *mut C_GtkFileChooser, uri: *const c_char) -> Gboolean;
    pub fn gtk_file_chooser_unselect_uri       (chooser: *mut C_GtkFileChooser, uri: *const c_char) -> ();
    pub fn gtk_file_chooser_get_uris           (chooser: *mut C_GtkFileChooser) -> *mut glib::ffi::C_GSList;
    pub fn gtk_file_chooser_set_current_folder_uri(chooser: *mut C_GtkFileChooser, uri: *const c_char) -> Gboolean;
    pub fn gtk_file_chooser_get_current_folder_uri(chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_preview_widget (chooser: *mut C_GtkFileChooser, preview_widget: *mut C_GtkWidget) -> ();
    pub fn gtk_file_chooser_get_preview_widget (chooser: *mut C_GtkFileChooser) -> *mut C_GtkWidget;
    pub fn gtk_file_chooser_set_preview_widget_active(chooser: *mut C_GtkFileChooser, active: Gboolean) -> ();
    pub fn gtk_file_chooser_get_preview_widget_active(chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_set_use_preview_label(chooser: *mut C_GtkFileChooser, use_label: Gboolean) -> ();
    pub fn gtk_file_chooser_get_use_preview_label(chooser: *mut C_GtkFileChooser) -> Gboolean;
    pub fn gtk_file_chooser_get_preview_filename(chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_get_preview_uri    (chooser: *mut C_GtkFileChooser) -> *const c_char;
    pub fn gtk_file_chooser_set_extra_widget   (chooser: *mut C_GtkFileChooser, extra_widget: *mut C_GtkWidget) -> ();
    pub fn gtk_file_chooser_get_extra_widget   (chooser: *mut C_GtkFileChooser) -> *mut C_GtkWidget;
    pub fn gtk_file_chooser_add_filter         (chooser: *mut C_GtkFileChooser, filter: *mut C_GtkFileFilter) -> ();
    pub fn gtk_file_chooser_remove_filter      (chooser: *mut C_GtkFileChooser, filter: *mut C_GtkFileFilter) -> ();
    //pub fn gtk_file_chooser_list_filters       (chooser: *const const C_GtkFileChooser) -> *glib::ffi::C_GSList;
    pub fn gtk_file_chooser_set_filter         (chooser: *mut C_GtkFileChooser, filter: *mut C_GtkFileFilter) -> ();
    pub fn gtk_file_chooser_get_filter         (chooser: *mut C_GtkFileChooser) -> *mut C_GtkFileFilter;
    pub fn gtk_file_chooser_add_shortcut_folder(chooser: *mut C_GtkFileChooser, folder: *const c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn gtk_file_chooser_remove_shortcut_folder(chooser: *mut C_GtkFileChooser, folder: *const c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    //pub fn gtk_file_chooser_list_shortcut_folders(chooser: *const const C_GtkFileChooser) -> *glib::ffi::C_GSList;
    pub fn gtk_file_chooser_add_shortcut_folder_uri(chooser: *mut C_GtkFileChooser, uri: *const c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn gtk_file_chooser_remove_shortcut_folder_uri(chooser: *mut C_GtkFileChooser, uri: *const c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    //pub fn gtk_file_chooser_list_shortcut_folder_uris(chooser: *const const C_GtkFileChooser) -> *glib::ffi::C_GSList;
    //pub fn gtk_file_chooser_get_current_folder_file(chooser: *const const C_GtkFileChooser) -> *const const C_Gfile;
    //pub fn gtk_file_chooser_get_file           (chooser: *const const C_GtkFileChooser) -> *const const C_Gfile;
    //pub fn gtk_file_chooser_get_files          (chooser: *const const C_GtkFileChooser) -> *glib::ffi::C_GSList;
    //pub fn gtk_file_chooser_get_preview_file   (chooser: *const const C_GtkFileChooser) -> *const const C_Gfile;
    //pub fn gtk_file_chooser_select_file        (chooser: *const const C_GtkFileChooser, file: *const const C_Gfile, error: **glib::ffi::C_GError) -> Gboolean;
    //pub fn gtk_file_chooser_set_current_folder_file(chooser: *const const C_GtkFileChooser, file: *const const C_Gfile, error: **glib::ffi::C_GError) -> Gboolean;
    //pub fn gtk_file_chooser_set_file           (chooser: *const const C_GtkFileChooser, file: *const const C_Gfile, error: **glib::ffi::C_GError) -> Gboolean;
    //pub fn gtk_file_chooser_unselect_file        (chooser: *const const C_GtkFileChooser, file: *const const C_Gfile) -> ();

    //=========================================================================
    // GtkFileFilter                                                     NOT OK
    //=========================================================================
    pub fn gtk_file_filter_new                 () -> *mut C_GtkFileFilter;
    pub fn gtk_file_filter_set_name            (filter: *mut C_GtkFileFilter, name: *const c_char) -> ();
    pub fn gtk_file_filter_get_name            (filter: *mut C_GtkFileFilter) -> *const c_char;
    pub fn gtk_file_filter_add_mime_type       (filter: *mut C_GtkFileFilter, mime_type: *const c_char) -> ();
    pub fn gtk_file_filter_add_pattern         (filter: *mut C_GtkFileFilter, pattern: *const c_char) -> ();
    pub fn gtk_file_filter_add_pixbuf_formats  (filter: *mut C_GtkFileFilter) -> ();
    //pub fn gtk_file_filter_add_custom          (filter: *const const C_GtkFileFilter, func: *const const C_GtkFileFilterFunc, data: *const c_void, notify: *const const C_GDestroyNotif) -> ();

    //=========================================================================
    // GtkFileChooserDialog                                              NOT OK
    //=========================================================================
    //pub fn gtk_file_chooser_dialog_new         (title: *const c_char, parent: *const const C_GtkWindow, action: gtk::FileChooserAction, first_button_text: *const c_char, ...) -> *const const C_GtkWidget;
    pub fn gtk_file_chooser_dialog_new         (title: *const c_char, parent: *mut C_GtkWindow, action: gtk::FileChooserAction, button_text1: *const c_char,
        type1: gtk::ResponseType, button_text2: *const c_char, type2: gtk::ResponseType, end: *mut c_void) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkColorChooser                                                       OK
    //=========================================================================
    pub fn gtk_color_chooser_get_rgba          (chooser: *mut C_GtkColorChooser, color: *const gdk::RGBA) -> ();
    pub fn gtk_color_chooser_set_rgba          (chooser: *mut C_GtkColorChooser, color: *const gdk::RGBA) -> ();
    pub fn gtk_color_chooser_get_use_alpha     (chooser: *mut C_GtkColorChooser) -> Gboolean;
    pub fn gtk_color_chooser_set_use_alpha     (chooser: *mut C_GtkColorChooser, use_alpha: Gboolean) -> ();
    pub fn gtk_color_chooser_add_palette       (chooser: *mut C_GtkColorChooser, orientation: gtk::Orientation, colors_per_line: i32, n_colors: i32, colors: *const gdk::RGBA) -> ();

    //=========================================================================
    // GtkColorChooserDialog                                                 OK
    //=========================================================================
    pub fn gtk_color_chooser_dialog_new        (title: *const c_char, parent: *mut C_GtkWindow) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkAppLaunchContext                                               NOT OK
    //=========================================================================
    /// Error : /usr/lib/x86_64-linux-gnu/libgio-2.0.so.0: error adding symbols: DSO missing from command line
    /*pub fn g_app_launch_context_new            () -> *mut C_GAppLaunchContext;
    pub fn g_app_launch_context_setenv         (launch_context: *mut C_GAppLaunchContext, variable: *c_char, value: *c_char) -> ();
    pub fn g_app_launch_context_unsetenv       (launch_context: *mut C_GAppLaunchContext, variable: *c_char) -> ();
    pub fn g_app_launch_context_get_environment(launch_context: *mut C_GAppLaunchContext) -> **c_char;
    //pub fn g_app_launch_context_get_display    (launch_context: *mut C_GAppLaunchContext, app_info: *mut C_GAppInfo, files: *mut glib::ffi::C_GList) -> *c_char;
    //pub fn g_app_launch_context_get_startup_notify_id(launch_context: *mut C_GAppLaunchContext, app_info: *mut C_GAppInfo, files: *mut glib::ffi::C_GList) -> *c_char;
    pub fn g_app_launch_context_launch_failed  (launch_context: *mut C_GAppLaunchContext, startup_notify_id: *c_char) -> ();*/

    //=========================================================================
    // GtkAppInfo                                                        NOT OK
    //=========================================================================
    /// Error : /usr/lib/x86_64-linux-gnu/libgio-2.0.so.0: error adding symbols: DSO missing from command line
    /*pub fn g_app_info_create_from_commandline  (commande_line: *c_char, application_name: *c_char, flag: gtk::AppInfoCreateFlags, error: *mut *mut glib::ffi::C_GError) -> *mut C_GAppInfo;
    pub fn g_app_info_dup                      (app_info: *mut C_GAppInfo) -> *mut C_GAppInfo;
    pub fn g_app_info_equal                    (app_info1: *mut C_GAppInfo, app_info2: *mut C_GAppInfo) -> Gboolean;
    pub fn g_app_info_get_id                   (app_info: *mut C_GAppInfo) -> *c_char;
    pub fn g_app_info_get_name                 (app_info: *mut C_GAppInfo) -> *c_char;
    pub fn g_app_info_get_display_name         (app_info: *mut C_GAppInfo) -> *c_char;
    pub fn g_app_info_get_description          (app_info: *mut C_GAppInfo) -> *c_char;
    pub fn g_app_info_get_executable           (app_info: *mut C_GAppInfo) -> *c_char;
    pub fn g_app_info_get_commandline          (app_info: *mut C_GAppInfo) -> *c_char;
    //pub fn g_app_info_get_icon                 (app_info: *mut C_GAppInfo) -> *mut C_GIcon;
    //pub fn g_app_info_launch                   (app_info: *mut C_GAppInfo, files: *mut glib::ffi::C_GList, launch_context: *mut C_GAppLaunchContext, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_supports_files           (app_info: *mut C_GAppInfo) -> Gboolean;
    pub fn g_app_info_supports_uris            (app_info: *mut C_GAppInfo) -> Gboolean;
    //pub fn g_app_info_launch_uris              (app_info: *mut C_GAppInfo, uris: *mut glib::ffi::C_GList, launch_context: *mut C_GAppLaunchContext, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_should_show              (app_info: *mut C_GAppInfo) -> Gboolean;
    pub fn g_app_info_can_delete               (app_info: *mut C_GAppInfo) -> Gboolean;
    pub fn g_app_info_delete                   (app_info: *mut C_GAppInfo) -> Gboolean;
    pub fn g_app_info_reset_type_associations  (content_type: *c_char) -> ();
    pub fn g_app_info_set_as_default_for_type  (app_info: *mut C_GAppInfo, content_type: *c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_set_as_default_for_extension(app_info: *mut C_GAppInfo, extension: *c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_set_as_last_used_for_type(app_info: *mut C_GAppInfo, content_type: *c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_add_supports_type        (app_info: *mut C_GAppInfo, content_type: *c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_can_remove_supports_type (app_info: *mut C_GAppInfo) -> Gboolean;
    pub fn g_app_info_remove_supports_type     (app_info: *mut C_GAppInfo, content_type: *c_char, error: *mut *mut glib::ffi::C_GError) -> Gboolean;
    pub fn g_app_info_get_supported_types      (app_info: *mut C_GAppInfo) -> **c_char;
    //pub fn g_app_info_get_all                  () -> *mut glib::ffi::C_GList;
    //pub fn g_app_info_get_all_for_type         (content_type: *c_char) -> *mut glib::ffi::C_GList;
    pub fn g_app_info_get_default_for_type     (content_type: *c_char, must_support_uris: Gboolean) -> *mut C_GAppInfo;
    pub fn g_app_info_get_default_for_uri_scheme(uri_scheme: *c_char) -> *mut C_GAppInfo;
    //pub fn g_app_info_get_fallback_for_type    (content_type: *c_char) -> *mut glib::ffi::C_GList;
    //pub fn g_app_info_get_recommended_for_type (content_type: *c_char) -> *mut glib::ffi::C_GList;
    pub fn g_app_info_launch_default_for_uri   (uri: *c_char, launch_context: *mut C_GAppLaunchContext, error: *mut *mut glib::ffi::C_GError) -> Gboolean;*/

    //=========================================================================
    // GtkAppChooser                                                         OK
    //=========================================================================
    pub fn gtk_app_chooser_get_app_info        (_self: *mut C_GtkAppChooser) -> *mut C_GAppInfo;
    pub fn gtk_app_chooser_get_content_type    (_self: *mut C_GtkAppChooser) -> *const c_char;
    pub fn gtk_app_chooser_refresh             (_self: *mut C_GtkAppChooser) -> ();

    //=========================================================================
    // GtkAppChooserDialog                                               NOT OK
    //=========================================================================
    //pub fn gtk_app_chooser_dialog_new          (title: *const c_char, flags: gtk::DialogFlags, file: *const const C_Gfile) -> *const const C_GtkWidget;
    pub fn gtk_app_chooser_dialog_new_for_content_type(parent: *mut C_GtkWindow, flags: gtk::DialogFlags, content_type: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_app_chooser_dialog_get_widget   (_self: *mut C_GtkAppChooserDialog) -> *mut C_GtkWidget;
    pub fn gtk_app_chooser_dialog_set_heading  (_self: *mut C_GtkAppChooserDialog, heading: *const c_char) -> ();
    pub fn gtk_app_chooser_dialog_get_heading  (_self: *mut C_GtkAppChooserDialog) -> *const c_char;

    //=========================================================================
    // GtkMessageDialog                                                  NOT OK
    //=========================================================================
    pub fn gtk_message_dialog_new              (parent: *mut C_GtkWindow, flags: gtk::DialogFlags, _type: gtk::MessageType, buttons: gtk::ButtonsType,
        message_format: *const c_char, ...) -> *mut C_GtkWidget;
    pub fn gtk_message_dialog_new_with_markup  (parent: *mut C_GtkWindow, flags: gtk::DialogFlags, _type: gtk::MessageType, buttons: gtk::ButtonsType,
        message_format: *const c_char, ...) -> *mut C_GtkWidget;
    pub fn gtk_message_dialog_set_markup       (message_dialog: *mut C_GtkMessageDialog, str: *const c_char) -> ();
    //pub fn gtk_message_dialog_format_secondary_text(message_dialog: *const const C_GtkMessageDialog, message_format: *const c_char, ...) -> ();
    pub fn gtk_message_dialog_format_secondary_markup(message_dialog: *mut C_GtkMessageDialog, message_format: *const c_char, ...) -> ();
    pub fn gtk_message_dialog_get_message_area (message_dialog: *mut C_GtkMessageDialog) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkDialog                                                         NOT OK
    //=========================================================================
    pub fn gtk_dialog_new                      () -> *mut C_GtkWidget;
    //pub fn gtk_dialog_new_with_buttons         (title: *const c_char, parent: *const const C_GtkWindow, flags: DialogFlags, first_button_text: *const c_char, ...) -> *const const C_GtkWidget;
    pub fn gtk_dialog_run                      (dialog: *mut C_GtkDialog) -> i32;
    pub fn gtk_dialog_response                 (dialog: *mut C_GtkDialog, response_id: i32) -> ();
    pub fn gtk_dialog_add_button               (dialog: *mut C_GtkDialog, button_text: *const c_char, response_id: i32) -> *mut C_GtkWidget;
    //pub fn gtk_dialog_add_buttons              (dialog: *const const C_GtkDialog, first_button_text: *const c_char, ...) -> ();
    pub fn gtk_dialog_add_action_widget        (dialog: *mut C_GtkDialog, child: *mut C_GtkWidget, response_id: i32) -> ();
    pub fn gtk_dialog_set_default_response     (dialog: *mut C_GtkDialog, response_id: i32) -> ();
    pub fn gtk_dialog_set_response_sensitive   (dialog: *mut C_GtkDialog, response_id: i32, setting: Gboolean) -> ();
    pub fn gtk_dialog_get_response_for_widget  (dialog: *mut C_GtkDialog, widget: *mut C_GtkWidget) -> i32;
    pub fn gtk_dialog_get_widget_for_response  (dialog: *mut C_GtkDialog, response_id: i32) -> *mut C_GtkWidget;
    pub fn gtk_dialog_get_action_area          (dialog: *mut C_GtkDialog) -> *mut C_GtkWidget;
    pub fn gtk_dialog_get_content_area         (dialog: *mut C_GtkDialog) -> *mut C_GtkWidget;
    pub fn gtk_dialog_get_header_bar           (dialog: *mut C_GtkDialog) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkAboutDialog                                                    NOT OK
    //=========================================================================
    pub fn gtk_about_dialog_new                () -> *mut C_GtkWidget;
    //pub fn gtk_show_about_dialog               (parent: *GtkWindow, first_property_name: *const c_char, ...) -> ();
    pub fn gtk_about_dialog_get_program_name   (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_program_name   (about: *mut C_GtkAboutDialog, name: *const c_char) -> ();
    pub fn gtk_about_dialog_get_version        (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_version        (about: *mut C_GtkAboutDialog, version: *const c_char) -> ();
    pub fn gtk_about_dialog_get_copyright      (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_copyright      (about: *mut C_GtkAboutDialog, copyright: *const c_char) -> ();
    pub fn gtk_about_dialog_get_comments       (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_comments       (about: *mut C_GtkAboutDialog, comments: *const c_char) -> ();
    pub fn gtk_about_dialog_get_license        (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_license        (about: *mut C_GtkAboutDialog, comments: *const c_char) -> ();
    pub fn gtk_about_dialog_get_license_type   (about: *mut C_GtkAboutDialog) -> gtk::License;
    pub fn gtk_about_dialog_set_license_type   (about: *mut C_GtkAboutDialog, license_type: gtk::License) -> ();
    pub fn gtk_about_dialog_get_wrap_license   (about: *mut C_GtkAboutDialog) -> Gboolean;
    pub fn gtk_about_dialog_set_wrap_license   (about: *mut C_GtkAboutDialog, wrap_license: Gboolean) -> ();
    pub fn gtk_about_dialog_get_website        (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_website        (about: *mut C_GtkAboutDialog, website: *const c_char) -> ();
    pub fn gtk_about_dialog_get_website_label  (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_website_label  (about: *mut C_GtkAboutDialog, website_label: *const c_char) -> ();
    pub fn gtk_about_dialog_get_authors        (about: *mut C_GtkAboutDialog) -> *const *const c_char;
    pub fn gtk_about_dialog_set_authors        (about: *mut C_GtkAboutDialog, authors: *const *const c_char) -> ();
    pub fn gtk_about_dialog_get_documenters    (about: *mut C_GtkAboutDialog) -> *const *const c_char;
    pub fn gtk_about_dialog_set_documenters    (about: *mut C_GtkAboutDialog, documents: *const *const c_char) -> ();
    pub fn gtk_about_dialog_get_artists        (about: *mut C_GtkAboutDialog) -> *const *const c_char;
    pub fn gtk_about_dialog_set_artists        (about: *mut C_GtkAboutDialog, artists: *const *const c_char) -> ();
    pub fn gtk_about_dialog_get_translator_credits(about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_translator_credits(about: *mut C_GtkAboutDialog, translator_credits: *const c_char) -> ();
    //pub fn gtk_about_dialog_get_logo           (about: *const const C_GtkAboutDialog) -> *const const C_GdkPixbuf;
    //pub fn gtk_about_dialog_set_logo           (about: *const const C_GtkAboutDialog, logo: *const const C_GdkPixbuf) -> ();
    pub fn gtk_about_dialog_get_logo_icon_name (about: *mut C_GtkAboutDialog) -> *const c_char;
    pub fn gtk_about_dialog_set_logo_icon_name (about: *mut C_GtkAboutDialog, icon_name: *const c_char) -> ();
    pub fn gtk_about_dialog_add_credit_section (about: *mut C_GtkAboutDialog, section_name: *const c_char, people: *const *const c_char) -> ();

    //=========================================================================
    // GtkFontButton                                                         OK
    //=========================================================================
    pub fn gtk_font_button_new                 () -> *mut C_GtkWidget;
    pub fn gtk_font_button_new_with_font       (fontname: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_font_button_set_font_name       (font_button: *mut C_GtkFontButton, fontname: *const c_char) -> Gboolean;
    pub fn gtk_font_button_get_font_name       (font_button: *mut C_GtkFontButton) -> *const c_char;
    pub fn gtk_font_button_set_show_style      (font_button: *mut C_GtkFontButton, show_style: Gboolean) -> ();
    pub fn gtk_font_button_get_show_style      (font_button: *mut C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_show_size       (font_button: *mut C_GtkFontButton, show_size: Gboolean) -> ();
    pub fn gtk_font_button_get_show_size       (font_button: *mut C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_use_font        (font_button: *mut C_GtkFontButton, use_font: Gboolean) -> ();
    pub fn gtk_font_button_get_use_font        (font_button: *mut C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_use_size        (font_button: *mut C_GtkFontButton, use_size: Gboolean) -> ();
    pub fn gtk_font_button_get_use_size        (font_button: *mut C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_title           (font_button: *mut C_GtkFontButton, title: *const c_char) -> ();
    pub fn gtk_font_button_get_title           (font_button: *mut C_GtkFontButton) -> *const c_char;

    //=========================================================================
    // GtkToggleButton                                                       OK
    //=========================================================================
    pub fn gtk_toggle_button_new               () -> *mut C_GtkWidget;
    pub fn gtk_toggle_button_new_with_label    (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_toggle_button_new_with_mnemonic (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_toggle_button_set_mode          (toggle_button: *mut C_GtkToggleButton, draw_indicator: Gboolean) -> ();
    pub fn gtk_toggle_button_get_mode          (toggle_button: *mut C_GtkToggleButton) -> Gboolean;
    pub fn gtk_toggle_button_toggled           (toggle_button: *mut C_GtkToggleButton) -> ();
    pub fn gtk_toggle_button_get_active        (toggle_button: *mut C_GtkToggleButton) -> Gboolean;
    pub fn gtk_toggle_button_set_active        (toggle_button: *mut C_GtkToggleButton, is_active: Gboolean) -> ();
    pub fn gtk_toggle_button_get_inconsistent  (toggle_button: *mut C_GtkToggleButton) -> Gboolean;
    pub fn gtk_toggle_button_set_inconsistent  (toggle_button: *mut C_GtkToggleButton, setting: Gboolean) -> ();

    //=========================================================================
    // GtkCheckButton                                                        OK
    //=========================================================================
    pub fn gtk_check_button_new                () -> *mut C_GtkWidget;
    pub fn gtk_check_button_new_with_label     (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_check_button_new_with_mnemonic  (label: *const c_char) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkRadioButton                                                      TODO
    //=========================================================================
    // pub fn gtk_radio_button_new                (GSList *group) -> *const const C_GtkWidget;
    // pub fn gtk_radio_button_new_from_widget    (GtkRadioButton *radio_group_member) -> *const const C_GtkWidget;
    // pub fn gtk_radio_button_new_with_label     (GSList *group, const gchar *label) -> *const const C_GtkWidget;
    // pub fn tk_radio_button_new_with_label_from_widget (GtkRadioButton *radio_group_member, const gchar *label) -> *const const C_GtkWidget;
    // pub fn gtk_radio_button_new_with_mnemonic  (GSList *group, const gchar *label) -> *const const C_GtkWidget;
    // pub fn gtk_radio_button_new_with_mnemonic_from_widget(GtkRadioButton *radio_group_member,  const gchar *label) -> *const const C_GtkWidget;
    // pub fn gtk_radio_button_set_group          (GtkRadioButton *radio_button, GSList *group) -> ();
    // pub fn gtk_radio_button_get_group          (GtkRadioButton *radio_button) -> *GSList;
    // pub fn gtk_radio_button_join_group         (GtkRadioButton *radio_button, GtkRadioButton *group_source) -> ();

    //=========================================================================
    // GtkMenuButton                                                         OK
    //=========================================================================
    pub fn gtk_menu_button_new                 () -> *mut C_GtkWidget;
    pub fn gtk_menu_button_set_popup           (menu_button: *mut C_GtkMenuButton, popup: *mut C_GtkWidget) -> ();
    // pub fn gtk_menu_button_get_popup           (menu_button: *const const C_GtkMenuButton) -> *const const C_GtkMenu;
    // pub fn gtk_menu_button_set_menu_model      (menu_button: *const const C_GtkMenuButton, menu_model: *const const C_GMenuModel) -> ();
    // pub fn gtk_menu_button_get_menu_model      (menu_button: *const const C_GtkMenuButton) -> C_GMenuModel;
    pub fn gtk_menu_button_set_direction       (menu_button: *mut C_GtkMenuButton, direction: gtk::ArrowType) -> ();
    pub fn gtk_menu_button_get_direction       (menu_button: *mut C_GtkMenuButton) -> gtk::ArrowType;
    pub fn gtk_menu_button_set_align_widget    (menu_button: *mut C_GtkMenuButton, align_widget: *mut C_GtkWidget) -> ();
    // pub fn gtk_menu_button_get_align_widget    (menu_button: *const const C_GtkMenuButton) -> *const const C_GtkWidget;

    //=========================================================================
    // GtkColorButton                                                        OK
    //=========================================================================
    pub fn gtk_color_button_new                () -> *mut C_GtkWidget;
    pub fn gtk_color_button_new_with_color     (color: *const gdk::Color) -> *mut C_GtkWidget;
    pub fn gtk_color_button_new_with_rgba      (rgba: *const gdk::RGBA) -> *mut C_GtkWidget;
    pub fn gtk_color_button_set_color          (button: *mut C_GtkColorButton, color: *const gdk::Color) -> ();
    pub fn gtk_color_button_get_color          (button: *mut C_GtkColorButton, color: *const gdk::Color) -> ();
    pub fn gtk_color_button_set_alpha          (button: *mut C_GtkColorButton, alpha: u16) -> ();
    pub fn gtk_color_button_get_alpha          (button: *mut C_GtkColorButton) -> u16;
    pub fn gtk_color_button_set_rgba           (button: *mut C_GtkColorButton, rgba: *const gdk::RGBA) -> ();
    pub fn gtk_color_button_get_rgba           (button: *mut C_GtkColorButton, rgba: *const gdk::RGBA) -> ();
    pub fn gtk_color_button_set_use_alpha      (button: *mut C_GtkColorButton, use_alpha: Gboolean) -> ();
    pub fn gtk_color_button_get_use_alpha      (button: *mut C_GtkColorButton) -> Gboolean;
    pub fn gtk_color_button_set_title          (button: *mut C_GtkColorButton, title: *const c_char) -> ();
    pub fn gtk_color_button_get_title          (button: *mut C_GtkColorButton) -> *const c_char;

    //=========================================================================
    // GtkLinkButton                                                         OK
    //=========================================================================
    pub fn gtk_link_button_new                 (uri: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_link_button_new_with_label      (uri: *const c_char, label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_link_button_get_uri             (link_button: *mut C_GtkLinkButton) -> *const c_char;
    pub fn gtk_link_button_set_uri             (link_button: *mut C_GtkLinkButton, uri: *const c_char) -> ();
    pub fn gtk_link_button_get_visited         (link_button: *mut C_GtkLinkButton) -> Gboolean;
    pub fn gtk_link_button_set_visited         (link_button: *mut C_GtkLinkButton, visited: Gboolean) -> ();

    //=========================================================================
    // GtkScaleButton
    //=========================================================================
    pub fn gtk_scale_button_new                (size: gtk::IconSize, min: c_double, max: c_double, step: c_double, icons: *const *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_scale_button_set_adjustment     (button: *mut C_GtkScaleButton, adjustment: *mut C_GtkAdjustment) -> ();
    // pub fn gtk_scale_button_set_icons          (button: *const const C_GtkScaleButton, icons: *const *const c_char) -> ();
    pub fn gtk_scale_button_set_value          (button: *mut C_GtkScaleButton, value: c_double) -> ();
    pub fn gtk_scale_button_get_adjustment     (button: *mut C_GtkScaleButton) -> *mut C_GtkAdjustment;
    pub fn gtk_scale_button_get_value          (button: *mut C_GtkScaleButton) -> c_double;
    // pub fn gtk_scale_button_get_popup          (button: *const const C_GtkScaleButton) -> *const const C_GtkWidget;
    // pub fn gtk_scale_button_get_plus_button    (button: *const const C_GtkScaleButton) -> *const const C_GtkWidget;
    // pub fn gtk_scale_button_get_minus_button   (button: *const const C_GtkScaleButton) -> *const const C_GtkWidget;

    //=========================================================================
    // GtkVolumeButton                                                       OK
    //=========================================================================
    pub fn gtk_volume_button_new               () -> *mut C_GtkWidget;

    //=========================================================================
    // GtkBox
    //=========================================================================
    pub fn gtk_box_new                         (orientation: gtk::Orientation, spacing: c_int) -> *mut C_GtkWidget;
    pub fn gtk_box_pack_start                  (gbox: *mut C_GtkBox, child: *mut C_GtkWidget, expand: Gboolean, fill: Gboolean, padding: c_uint) -> ();
    pub fn gtk_box_pack_end                    (gbox: *mut C_GtkBox, child: *mut C_GtkWidget, expand: Gboolean, fill: Gboolean, padding: c_uint) -> ();
    pub fn gtk_box_get_homogeneous             (gbox: *mut C_GtkBox) -> Gboolean;
    pub fn gtk_box_set_homogeneous             (gbox: *mut C_GtkBox, homogeneous: Gboolean) -> ();
    pub fn gtk_box_get_spacing                 (gbox: *mut C_GtkBox) -> c_int;
    pub fn gtk_box_set_spacing                 (gbox: *mut C_GtkBox, spacing: c_int) -> ();
    pub fn gtk_box_reorder_child               (gbox: *mut C_GtkBox, child: *mut C_GtkWidget, position: c_int) -> ();
    pub fn gtk_box_query_child_packing         (gbox: *mut C_GtkBox, child: *mut C_GtkWidget, expand: *mut Gboolean, fill: *mut Gboolean, padding: *mut c_uint, pack_type: *mut gtk::PackType) -> ();
    pub fn gtk_box_set_child_packing           (gbox: *mut C_GtkBox, child: *mut C_GtkWidget, expand: Gboolean, fill: Gboolean, padding: c_uint, pack_type: gtk::PackType) -> ();
    // pub fn gtk_box_get_baseline_position       (gbox: *const const C_GtkBox) -> gtk::BaselinePosition;
    // pub fn gtk_box_set_baseline_position       (gbox: *const const C_GtkBox, position: gtk::BaselinePosition) -> ();

    //=========================================================================
    // GtkOrientable                                                         OK
    //=========================================================================
    pub fn gtk_orientable_get_orientation      (orientable: *mut C_GtkOrientable) -> gtk::Orientation;
    pub fn gtk_orientable_set_orientation      (orientable: *mut C_GtkOrientable,  orientation: gtk::Orientation) -> ();

    //=========================================================================
    // GtkButtonBox                                                          OK
    //=========================================================================
    pub fn gtk_button_box_new                  (orientation: gtk::Orientation) -> *mut C_GtkWidget;
    pub fn gtk_button_box_get_layout           (widget: *mut C_GtkButtonBox) -> gtk::ButtonBoxStyle;
    pub fn gtk_button_box_get_child_secondary  (widget: *mut C_GtkButtonBox, child : *mut C_GtkWidget) -> Gboolean;
    pub fn gtk_button_box_get_child_non_homogeneous(Gwidget: *mut C_GtkButtonBox, child : *mut C_GtkWidget) -> Gboolean;
    pub fn gtk_button_box_set_layout           (widget: *mut C_GtkButtonBox, layout_style: gtk::ButtonBoxStyle) -> ();
    pub fn gtk_button_box_set_child_secondary  (widget: *mut C_GtkButtonBox, child : *mut C_GtkWidget, is_secondary: Gboolean) -> ();
    pub fn gtk_button_box_set_child_non_homogeneous(widget: *mut C_GtkButtonBox, child : *mut C_GtkWidget, non_homogeneous: Gboolean) -> ();

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
    pub fn gtk_frame_new                       (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_frame_set_label                 (frame: *mut C_GtkFrame, label: *const c_char) -> ();
    pub fn gtk_frame_set_label_widget          (frame: *mut C_GtkFrame, label_widget: *mut C_GtkWidget) -> ();
    pub fn gtk_frame_set_label_align           (frame: *mut C_GtkFrame, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_frame_set_shadow_type           (frame: *mut C_GtkFrame, st_type: gtk::ShadowType) -> ();
    pub fn gtk_frame_get_label                 (frame: *mut C_GtkFrame) -> *const c_char;
    pub fn gtk_frame_get_label_align           (frame: *mut C_GtkFrame, xalign: *mut c_float, yalign: *mut c_float) -> ();
    // pub fn gtk_frame_get_label_widget          (frame: *const const C_GtkFrame) -> *const const C_GtkWidget;
    pub fn gtk_frame_get_shadow_type           (frame: *mut C_GtkFrame) -> gtk::ShadowType;

    //=========================================================================
    // GtkAspectFrame                                                        OK
    //=========================================================================
    pub fn gtk_aspect_frame_new                (label: *const c_char, xalign: c_float, yalign: c_float, ratio: c_float, obey_child: Gboolean) -> *mut C_GtkWidget;
    pub fn gtk_aspect_frame_set                (aspect_frame: *mut C_GtkAspectFrame, xalign: c_float, yalign: c_float, ratio: c_float, obey_child: Gboolean) -> ();

    //=========================================================================
    // GtkFixed                                                              OK
    //=========================================================================
    pub fn gtk_fixed_new                       () -> *mut C_GtkWidget;
    pub fn gtk_fixed_put                       (fixed: *mut C_GtkFixed, widget: *mut C_GtkWidget, x: c_int, y: c_int) -> ();
    pub fn gtk_fixed_move                      (fixed: *mut C_GtkFixed, widget: *mut C_GtkWidget, x: c_int, y: c_int) -> ();

    //=========================================================================
    // GtkBin                                                                OK
    //=========================================================================
    pub fn gtk_bin_get_child                   (bin: *mut C_GtkBin) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkSeparator                                                          OK
    //=========================================================================
    pub fn gtk_separator_new                   (orientation: gtk::Orientation) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkAdjustment                                                         OK
    //=========================================================================
    pub fn gtk_adjustment_new                  (value: c_double, lower: c_double, upper: c_double, step_increment: c_double, page_increment: c_double, page_size: c_double) -> *mut C_GtkAdjustment;
    pub fn gtk_adjustment_get_value            (adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_set_value            (adjustment: *mut C_GtkAdjustment, value: c_double) -> ();
    pub fn gtk_adjustment_clamp_page           (adjustment: *mut C_GtkAdjustment, lower: c_double, upper: c_double) -> ();
    pub fn gtk_adjustment_changed              (adjustment: *mut C_GtkAdjustment) -> ();
    pub fn gtk_adjustment_value_changed        (adjustment: *mut C_GtkAdjustment) -> ();
    pub fn gtk_adjustment_configure            (adjustment: *mut C_GtkAdjustment, value: c_double, lower: c_double, upper: c_double, tep_increment: c_double, page_increment: c_double, page_size: c_double) -> ();
    pub fn gtk_adjustment_get_lower            (adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_page_increment   (adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_page_size        (adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_step_increment   (adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_minimum_increment(adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_upper            (adjustment: *mut C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_set_lower            (adjustment: *mut C_GtkAdjustment, lower: c_double) -> ();
    pub fn gtk_adjustment_set_page_increment   (adjustment: *mut C_GtkAdjustment, page_increment: c_double) -> ();
    pub fn gtk_adjustment_set_page_size        (adjustment: *mut C_GtkAdjustment, page_size: c_double) -> ();
    pub fn gtk_adjustment_set_step_increment   (adjustment: *mut C_GtkAdjustment, step_increment: c_double) -> ();
    pub fn gtk_adjustment_set_upper            (adjustment: *mut C_GtkAdjustment, upper: c_double) -> ();

    //=========================================================================
    // GtkGrid
    //=========================================================================
    pub fn gtk_grid_new                        () -> *mut C_GtkWidget;
    pub fn gtk_grid_attach                     (grid: *mut C_GtkGrid, child: *mut C_GtkWidget, left: c_int, top: c_int, width: c_int, height: c_int) -> ();
    pub fn gtk_grid_attach_next_to             (grid: *mut C_GtkGrid, child: *mut C_GtkWidget, sibling: *mut C_GtkWidget, side: gtk::PositionType, width: c_int, height: c_int) -> ();
    // pub fn gtk_grid_get_child_at               (grid: *const const C_GtkGrid, left: c_int, top: c_int) -> *const const C_GtkWidget;
    pub fn gtk_grid_insert_row                 (grid: *mut C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_insert_column              (grid: *mut C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_remove_row                 (grid: *mut C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_remove_column              (grid: *mut C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_insert_next_to             (grid: *mut C_GtkGrid, sibling: *mut C_GtkWidget, side: gtk::PositionType) -> ();
    pub fn gtk_grid_set_row_homogeneous        (grid: *mut C_GtkGrid, homogeneous: Gboolean) -> ();
    pub fn gtk_grid_get_row_homogeneous        (grid: *mut C_GtkGrid) -> Gboolean;
    pub fn gtk_grid_set_row_spacing            (grid: *mut C_GtkGrid, spacing: c_uint) -> ();
    pub fn gtk_grid_get_row_spacing            (grid: *mut C_GtkGrid) -> c_uint;
    pub fn gtk_grid_set_column_homogeneous     (grid: *mut C_GtkGrid, homogeneous: Gboolean) -> ();
    pub fn gtk_grid_get_column_homogeneous     (grid: *mut C_GtkGrid) -> Gboolean;
    pub fn gtk_grid_set_column_spacing         (grid: *mut C_GtkGrid, spacing: c_uint) -> ();
    pub fn gtk_grid_get_column_spacing         (grid: *mut C_GtkGrid) -> c_uint;
    pub fn gtk_grid_get_baseline_row           (grid: *mut C_GtkGrid) -> c_int;
    pub fn gtk_grid_set_baseline_row           (grid: *mut C_GtkGrid, row: c_int) -> ();
    // pub fn gtk_grid_get_row_baseline_position  (grid: *const const C_GtkGrid, row: c_int);
    // pub fn gtk_grid_set_row_baseline_position  (grid: *const const C_GtkGrid, row: c_int, pos: gtk::BaselinePosition) -> ();

    //=========================================================================
    // GtkEntryBuffer                                                        OK
    //=========================================================================
    pub fn gtk_entry_buffer_new                (initial_chars: *const c_char, n_initial_chars: c_int) -> *mut C_GtkEntryBuffer;
    pub fn gtk_entry_buffer_get_text           (buffer: *mut C_GtkEntryBuffer) -> *const c_char;
    pub fn gtk_entry_buffer_set_text           (buffer: *mut C_GtkEntryBuffer, chars: *const c_char, n_chars: c_int) -> ();
    pub fn gtk_entry_buffer_get_bytes          (buffer: *mut C_GtkEntryBuffer) -> c_long;
    pub fn gtk_entry_buffer_get_length         (buffer: *mut C_GtkEntryBuffer) -> c_uint;
    pub fn gtk_entry_buffer_get_max_length     (buffer: *mut C_GtkEntryBuffer) -> c_int;
    pub fn gtk_entry_buffer_set_max_length     (buffer: *mut C_GtkEntryBuffer, max_length: c_int) -> ();
    pub fn gtk_entry_buffer_insert_text        (buffer: *mut C_GtkEntryBuffer, position: c_uint, chars: *const c_char, n_chars: c_int);
    pub fn gtk_entry_buffer_delete_text        (buffer: *mut C_GtkEntryBuffer, position: c_uint, n_char: c_uint) -> c_uint;
    pub fn gtk_entry_buffer_emit_deleted_text  (buffer: *mut C_GtkEntryBuffer, position: c_uint, n_chars: c_uint) -> ();
    pub fn gtk_entry_buffer_emit_inserted_text (buffer: *mut C_GtkEntryBuffer, position: c_uint, chars: *const c_char, n_chars: c_int) -> ();

    //=========================================================================
    // GtkEntry
    //=========================================================================
    pub fn gtk_entry_new                       () -> *mut C_GtkWidget;
    pub fn gtk_entry_new_with_buffer           (buffer: *mut C_GtkEntryBuffer) -> *mut C_GtkWidget;
    pub fn gtk_entry_get_buffer                (entry: *mut C_GtkEntry) -> *mut C_GtkEntryBuffer;
    pub fn gtk_entry_set_buffer                (entry: *mut C_GtkEntry, buffer: *mut C_GtkEntryBuffer) -> ();
    pub fn gtk_entry_set_text                  (entry: *mut C_GtkEntry, text: *const c_char) -> ();
    pub fn gtk_entry_get_text                  (entry: *mut C_GtkEntry) -> *const c_char;
    pub fn gtk_entry_get_text_length           (entry: *mut C_GtkEntry) -> c_short;
    // pub fn gtk_entry_get_text_area             (entry: *const const C_GtkEntry, text_area: *GdkRectangle) -> ();
    pub fn gtk_entry_set_visibility            (entry: *mut C_GtkEntry, visible: Gboolean) -> ();
    pub fn gtk_entry_set_invisible_char        (entry: *mut C_GtkEntry, ch: c_int) -> ();
    pub fn gtk_entry_unset_invisible_char      (entry: *mut C_GtkEntry) -> ();
    pub fn gtk_entry_set_max_length            (entry: *mut C_GtkEntry, max: c_int) -> ();
    pub fn gtk_entry_get_activates_default     (entry: *mut C_GtkEntry) -> Gboolean;
    pub fn gtk_entry_get_has_frame             (entry: *mut C_GtkEntry) -> Gboolean;
    // pub fn gtk_entry_get_inner_border          (entry: *const const C_GtkEntry) -> *const const C_GtkBorder;
    pub fn gtk_entry_get_width_chars           (entry: *mut C_GtkEntry) -> c_int;
    pub fn gtk_entry_set_activates_default     (entry: *mut C_GtkEntry, setting: Gboolean) -> ();
    pub fn gtk_entry_set_has_frame             (entry: *mut C_GtkEntry, setting: Gboolean) -> ();
    // pub fn gtk_entry_set_inner_border          (entry: *const const C_GtkEntry, border: *const const C_GtkBorder) -> ();
    pub fn gtk_entry_set_width_chars           (entry: *mut C_GtkEntry, n_chars: c_int) -> ();
    pub fn gtk_entry_get_invisible_char        (entry: *mut C_GtkEntry) -> c_uint;
    pub fn gtk_entry_set_alignment             (entry: *mut C_GtkEntry, xalign: c_float) -> ();
    pub fn gtk_entry_get_alignment             (entry: *mut C_GtkEntry) -> *const c_float;
    pub fn gtk_entry_set_placeholder_text      (entry: *mut C_GtkEntry, text: *const c_char) -> ();
    pub fn gtk_entry_get_placeholder_text      (entry: *mut C_GtkEntry) -> *const c_char;
    pub fn gtk_entry_set_overwrite_mode        (entry: *mut C_GtkEntry, overwrite: Gboolean) -> ();
    pub fn gtk_entry_get_overwrite_mode        (entry: *mut C_GtkEntry) -> Gboolean;
    // pub fn gtk_entry_get_layout                (entry: *const const C_GtkEntry) -> *PangoLayout;
    pub fn gtk_entry_get_layout_offsets        (entry: *mut C_GtkEntry, x: *const c_int,  y: *const c_int) -> ();
    pub fn gtk_entry_layout_index_to_text_index(entry: *mut C_GtkEntry, layout_index: c_int) -> c_int;
    pub fn gtk_entry_text_index_to_layout_index(entry: *mut C_GtkEntry,  text_index: c_int) -> c_int;
    // pub fn gtk_entry_set_attributes            (entry: *const const C_GtkEntry, attrs: *PangoAttrList) -> ();
    // pub fn gtk_entry_get_attributes            (entry: *const const C_GtkEntry) -> *PangoAttrList;
    pub fn gtk_entry_get_max_length            (entry: *mut C_GtkEntry) -> c_int;
    pub fn gtk_entry_get_visibility            (entry: *mut C_GtkEntry) -> Gboolean;
    // pub fn gtk_entry_set_completion            (entry: *const const C_GtkEntry, completion: *const const C_GtkEntryCompletion) -> ();
    // pub fn gtk_entry_get_completion            (entry: *const const C_GtkEntry) -> *GtkEntryCompletion;
    pub fn gtk_entry_set_cursor_hadjustment    (entry: *mut C_GtkEntry, adjustment: *mut C_GtkAdjustment) -> ();
    pub fn gtk_entry_get_cursor_hadjustment    (entry: *mut C_GtkEntry) -> *mut C_GtkAdjustment;
    pub fn gtk_entry_set_progress_fraction     (entry: *mut C_GtkEntry, fraction: c_double) -> ();
    pub fn gtk_entry_get_progress_fraction     (entry: *mut C_GtkEntry) -> c_double;
    pub fn gtk_entry_set_progress_pulse_step   (entry: *mut C_GtkEntry, fraction: c_double) -> ();
    pub fn gtk_entry_get_progress_pulse_step   (entry: *mut C_GtkEntry) -> c_double;
    pub fn gtk_entry_progress_pulse            (entry: *mut C_GtkEntry) -> ();
    // pub fn gtk_entry_im_context_filter_keypress(entry: *const const C_GtkEntry, event: *GdkEventKey) -> Gboolean;
    pub fn gtk_entry_reset_im_context          (entry: *mut C_GtkEntry) -> ();
    // pub fn gtk_entry_get_tabs                  (entry: *const const C_GtkEntry) -> *PangoTabArray;
    // pub fn gtk_entry_set_tabs                  (entry: *const const C_GtkEntry, tabs: *PangoTabArray) -> ();
    // pub fn gtk_entry_set_icon_from_pixbuf      (entry: *const const C_GtkEntry, icon_pos: gtk::EntryIconPosition, pixbuf: *GdkPixbuf) -> ();
    pub fn gtk_entry_set_icon_from_stock       (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition, stock_id: *const c_char) -> ();
    pub fn gtk_entry_set_icon_from_icon_name   (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition, icon_name: *const c_char) -> ();
    // pub fn gtk_entry_set_icon_from_gicon       (entry: *const const C_GtkEntry, icon_pos: gtk::EntryIconPosition, icon: *GIcon) -> ();
    pub fn gtk_entry_get_icon_storage_type     (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> gtk::ImageType;
    // pub fn gtk_entry_get_icon_pixbuf           (entry: *const const C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> *const const C_GdkPixbuf;
    pub fn gtk_entry_get_icon_stock            (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> *const c_char;
    pub fn gtk_entry_get_icon_name             (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> *const c_char;
    // pub fn gtk_entry_get_icon_gicon            (entry: *const const C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> *GIcon;
    pub fn gtk_entry_set_icon_activatable      (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition, activatable: Gboolean) -> ();
    pub fn gtk_entry_get_icon_activatable      (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> Gboolean;
    pub fn gtk_entry_set_icon_sensitive        (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition, sensitive: Gboolean) -> ();
    pub fn gtk_entry_get_icon_sensitive        (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> Gboolean;
    pub fn gtk_entry_get_icon_at_pos           (entry: *mut C_GtkEntry, x: c_int, y: c_int) -> c_int;
    pub fn gtk_entry_set_icon_tooltip_text     (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition, tooltip: *const c_char) -> ();
    pub fn gtk_entry_get_icon_tooltip_text     (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> *const c_char;
    pub fn gtk_entry_set_icon_tooltip_markup   (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition, tooltip: *const c_char) -> ();
    pub fn gtk_entry_get_icon_tooltip_markup   (entry: *mut C_GtkEntry, icon_pos: gtk::EntryIconPosition) -> *const c_char;
    // pub fn gtk_entry_set_icon_drag_source      (entry: *const const C_GtkEntry, icon_pos: gtk::EntryIconPosition, target_list: *GtkTargetList, actions: GdkDragAction) -> ();
    pub fn gtk_entry_get_current_icon_drag_source(entry: *mut C_GtkEntry) -> c_int;
    // pub fn gtk_entry_get_icon_area             (entry: *const const C_GtkEntry, icon_pos: gtk::EntryIconPosition, icon_area: *GdkRectangle) -> ();
    pub fn gtk_entry_set_input_purpose         (entry: *mut C_GtkEntry, purpose: gtk::InputPurpose) -> ();
    pub fn gtk_entry_get_input_purpose         (entry: *mut C_GtkEntry) -> gtk::InputPurpose;
    pub fn gtk_entry_set_input_hints           (entry: *mut C_GtkEntry, hints: gtk::InputHints) -> ();
    pub fn gtk_entry_get_input_hints           (entry: *mut C_GtkEntry) -> gtk::InputHints;

    //=========================================================================
    // GtkSearchEntry                                                        OK
    //=========================================================================
    pub fn gtk_search_entry_new                () -> *mut C_GtkWidget;

    //=========================================================================
    // GtkSwitch                                                             OK
    //=========================================================================
    pub fn gtk_switch_new                      () -> *mut C_GtkWidget;
    pub fn gtk_switch_set_active               (switch: *mut C_GtkSwitch,  is_active: Gboolean) -> ();
    pub fn gtk_switch_get_active               (switch: *mut C_GtkSwitch) -> Gboolean;

    //=========================================================================
    // GtkScale
    //=========================================================================
    pub fn gtk_scale_new                       (orientation: gtk::Orientation, adjustment: *mut C_GtkAdjustment) -> *mut C_GtkWidget;
    pub fn gtk_scale_new_with_range            (orientation: gtk::Orientation, min: c_double, max: c_double, step: c_double) -> *mut C_GtkWidget;
    pub fn gtk_scale_set_digits                (scale: *mut C_GtkScale, digits: c_int) -> ();
    pub fn gtk_scale_set_draw_value            (scale: *mut C_GtkScale, draw_value: Gboolean) -> ();
    pub fn gtk_scale_set_has_origin            (scale: *mut C_GtkScale, has_origin: Gboolean) -> ();
    pub fn gtk_scale_set_value_pos             (scale: *mut C_GtkScale, position: gtk::PositionType) -> ();
    pub fn gtk_scale_get_digits                (scale: *mut C_GtkScale) -> c_int;
    pub fn gtk_scale_get_draw_value            (scale: *mut C_GtkScale) -> Gboolean;
    pub fn gtk_scale_get_has_origin            (scale: *mut C_GtkScale) -> Gboolean;
    pub fn gtk_scale_get_value_pos             (scale: *mut C_GtkScale) -> gtk::PositionType;
    // pub fn gtk_scale_get_layout                (scale: *const const C_GtkScale) -> *PangoLayout;
    pub fn gtk_scale_get_layout_offsets        (scale: *mut C_GtkScale, x: *const c_int, y: *const c_int) -> ();
    pub fn gtk_scale_add_mark                  (scale: *mut C_GtkScale, value: c_double, position: gtk::PositionType, markup: *const c_char) -> ();
    pub fn gtk_scale_clear_marks               (scale: *mut C_GtkScale) -> ();

    //=========================================================================
    // GtkLevelBar
    //=========================================================================
    pub fn gtk_level_bar_new                   () -> *mut C_GtkWidget;
    pub fn gtk_level_bar_new_for_interval      (min_value: c_double, max_value: c_double) -> *mut C_GtkWidget;
    pub fn gtk_level_bar_set_mode              (bar: *mut C_GtkLevelBar, mode: gtk::LevelBarMode) -> ();
    pub fn gtk_level_bar_get_mode              (bar: *mut C_GtkLevelBar) -> gtk::LevelBarMode;
    pub fn gtk_level_bar_set_value             (bar: *mut C_GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_value             (bar: *mut C_GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_min_value         (bar: *mut C_GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_min_value         (bar: *mut C_GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_max_value         (bar: *mut C_GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_max_value         (bar: *mut C_GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_inverted          (bar: *mut C_GtkLevelBar, inverted: Gboolean) -> ();
    pub fn gtk_level_bar_get_inverted          (bar: *mut C_GtkLevelBar) -> Gboolean;
    pub fn gtk_level_bar_add_offset_value      (bar: *mut C_GtkLevelBar, name: *const c_char, value: c_double) -> ();
    pub fn gtk_level_bar_remove_offset_value   (bar: *mut C_GtkLevelBar, name: *const c_char) -> ();
    pub fn gtk_level_bar_get_offset_value      (bar: *mut C_GtkLevelBar, name: *const c_char, value: *const c_double) -> Gboolean;

    //=========================================================================
    // GtkSearchBar
    //=========================================================================
    pub fn gtk_search_bar_new                  () -> *mut C_GtkWidget;
    pub fn gtk_search_bar_connect_entry        (bar: *mut C_GtkSearchBar, entry: *mut C_GtkEntry) -> ();
    pub fn gtk_search_bar_get_search_mode      (bar: *mut C_GtkSearchBar) -> Gboolean;
    pub fn gtk_search_bar_set_search_mode      (bar: *mut C_GtkSearchBar, search_mode: Gboolean) -> ();
    pub fn gtk_search_bar_get_show_close_button(bar: *mut C_GtkSearchBar) -> Gboolean;
    pub fn gtk_search_bar_set_show_close_button(bar: *mut C_GtkSearchBar, visible: Gboolean) -> ();
    // pub fn gtk_search_bar_handle_event         (bar: *const const C_GtkSearchBar, event: *GdkEvent) -> Gboolean;

    //=========================================================================
    // GtkSpinButton
    //=========================================================================
    pub fn gtk_spin_button_configure           (spin_button: *mut C_GtkSpinButton, adjustment: *mut C_GtkAdjustment, climb_rate: c_double, digits: c_uint) -> ();
    pub fn gtk_spin_button_new                 (adjustment: *mut C_GtkAdjustment, climb_rate: c_double, digits: c_uint) -> *mut C_GtkWidget;
    pub fn gtk_spin_button_new_with_range      (min: c_double, max: c_double, step: c_double) -> *mut C_GtkWidget;
    pub fn gtk_spin_button_set_adjustment      (spin_button: *mut C_GtkSpinButton, adjustment: *mut C_GtkAdjustment) -> ();
    pub fn gtk_spin_button_get_adjustment      (spin_button: *mut C_GtkSpinButton) -> *mut C_GtkAdjustment;
    pub fn gtk_spin_button_set_digits          (spin_button: *mut C_GtkSpinButton, digits: c_uint) -> ();
    pub fn gtk_spin_button_set_increments      (spin_button: *mut C_GtkSpinButton, step: c_double, page: c_double) -> ();
    pub fn gtk_spin_button_set_range           (spin_button: *mut C_GtkSpinButton, min: c_double, max: c_double);
    pub fn gtk_spin_button_get_value_as_int    (spin_button: *mut C_GtkSpinButton) -> c_int;
    pub fn gtk_spin_button_set_value           (spin_button: *mut C_GtkSpinButton, value: c_double) -> ();
    pub fn gtk_spin_button_set_update_policy   (spin_button: *mut C_GtkSpinButton, policy: gtk::SpinButtonUpdatePolicy) -> ();
    pub fn gtk_spin_button_set_numeric         (spin_button: *mut C_GtkSpinButton, gnumeric: Gboolean) -> ();
    pub fn gtk_spin_button_spin                (spin_button: *mut C_GtkSpinButton, direction: gtk::SpinType, increment: c_double) -> ();
    pub fn gtk_spin_button_set_wrap            (spin_button: *mut C_GtkSpinButton, wrap: Gboolean) -> ();
    pub fn gtk_spin_button_set_snap_to_ticks   (spin_button: *mut C_GtkSpinButton, snap_to_ticks: Gboolean) -> ();
    pub fn gtk_spin_button_update              (spin_button: *mut C_GtkSpinButton) -> ();
    pub fn gtk_spin_button_get_digits          (spin_button: *mut C_GtkSpinButton) -> c_uint;
    pub fn gtk_spin_button_get_increments      (spin_button: *mut C_GtkSpinButton, step: *mut c_double, page: *mut c_double) -> ();
    pub fn gtk_spin_button_get_numeric         (spin_button: *mut C_GtkSpinButton) -> Gboolean;
    pub fn gtk_spin_button_get_range           (spin_button: *mut C_GtkSpinButton, min: *mut c_double, max: *mut c_double) -> ();
    pub fn gtk_spin_button_get_snap_to_ticks   (spin_button: *mut C_GtkSpinButton) -> Gboolean;
    pub fn gtk_spin_button_get_update_policy   (spin_button: *mut C_GtkSpinButton) -> gtk::SpinButtonUpdatePolicy;
    pub fn gtk_spin_button_get_value           (spin_button: *mut C_GtkSpinButton) -> c_double;
    pub fn gtk_spin_button_get_wrap            (spin_button: *mut C_GtkSpinButton) -> Gboolean;

    //=========================================================================
    // GtkSpinner                                                            OK
    //=========================================================================
    pub fn gtk_spinner_new                     () -> *mut C_GtkWidget;
    pub fn gtk_spinner_start                   (spinner: *mut C_GtkSpinner) -> ();
    pub fn gtk_spinner_stop                    (spinner: *mut C_GtkSpinner) -> ();

    //=========================================================================
    // GtkImage
    //=========================================================================
    pub fn gtk_image_new_from_file             (filename: *const c_char) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkProgressBar
    //=========================================================================
    pub fn gtk_progress_bar_new                () -> *mut C_GtkWidget;
    pub fn gtk_progress_bar_pulse              (pbar: *mut C_GtkProgressBar) -> ();
    pub fn gtk_progress_bar_set_fraction       (pbar: *mut C_GtkProgressBar, fraction: c_double) -> ();
    pub fn gtk_progress_bar_get_fraction       (pbar: *mut C_GtkProgressBar) -> c_double;
    pub fn gtk_progress_bar_set_inverted       (pbar: *mut C_GtkProgressBar, inverted: Gboolean) -> ();
    pub fn gtk_progress_bar_get_inverted       (pbar: *mut C_GtkProgressBar) -> Gboolean;
    pub fn gtk_progress_bar_set_show_text      (pbar: *mut C_GtkProgressBar, show_text: Gboolean) -> ();
    pub fn gtk_progress_bar_get_show_text      (pbar: *mut C_GtkProgressBar) -> Gboolean;
    pub fn gtk_progress_bar_set_text           (pbar: *mut C_GtkProgressBar, text: *const c_char) -> ();
    pub fn gtk_progress_bar_get_text           (pbar: *mut C_GtkProgressBar) -> *const c_char;
    // pub fn gtk_progress_bar_set_ellipsize      (pbar: *const const C_GtkProgressBar, mode: PangoEllipsizeMode) -> ();
    // pub fn gtk_progress_bar_get_ellipsize      (pbar: *const const C_GtkProgressBar) -> PangoEllipsizeMode
    pub fn gtk_progress_bar_set_pulse_step     (pbar: *mut C_GtkProgressBar, fraction: c_double) -> ();
    pub fn gtk_progress_bar_get_pulse_step     (pbar: *mut C_GtkProgressBar) -> c_double;

    //=========================================================================
    // GtkArrow                                                              OK
    //=========================================================================
    pub fn gtk_arrow_new                       (arrow_type: gtk::ArrowType, shadow_type: gtk::ShadowType) -> *mut C_GtkWidget;
    pub fn gtk_arrow_set                       (arrow: *mut C_GtkArrow,arrow_type: gtk::ArrowType, shadow_type: gtk::ShadowType) -> ();

    //=========================================================================
    // GtkCalendar
    //=========================================================================
    pub fn gtk_calendar_new                    () -> *mut C_GtkWidget;
    pub fn gtk_calendar_select_month           (calendar: *mut C_GtkCalendar, month: c_uint, year: c_uint) -> ();
    pub fn gtk_calendar_select_day             (calendar: *mut C_GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_mark_day               (calendar: *mut C_GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_unmark_day             (calendar: *mut C_GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_get_day_is_marked      (calendar: *mut C_GtkCalendar, day: c_uint) -> Gboolean;
    pub fn gtk_calendar_clear_marks            (calendar: *mut C_GtkCalendar) -> ();
    pub fn gtk_calendar_get_display_options    (calendar: *mut C_GtkCalendar) -> gtk::CalendarDisplayOptions;
    pub fn gtk_calendar_set_display_options    (calendar: *mut C_GtkCalendar, flags: gtk::CalendarDisplayOptions) -> ();
    pub fn gtk_calendar_get_date               (calendar: *mut C_GtkCalendar, year: *mut c_uint, month: *mut c_uint, day: *mut c_uint) -> ();
    // pub fn gtk_calendar_set_detail_func        (calendar: *const const C_GtkCalendar, GtkCalendarDetailFunc func, gpointer data, GDestroyNotify destroy) -> ();
    pub fn gtk_calendar_get_detail_width_chars (calendar: *mut C_GtkCalendar) -> c_int;
    pub fn gtk_calendar_set_detail_width_chars (calendar: *mut C_GtkCalendar, chars: c_int) -> ();
    pub fn gtk_calendar_get_detail_height_rows (calendar: *mut C_GtkCalendar) -> c_int;
    pub fn gtk_calendar_set_detail_height_rows (calendar: *mut C_GtkCalendar, rows: c_int) -> ();

    //=========================================================================
    // GtkAlignments                                                         OK
    //=========================================================================
    pub fn gtk_alignment_new                   (xalign: c_float, yalign: c_float, xscale: c_float, yscale: c_float) -> *mut C_GtkWidget;
    pub fn gtk_alignment_set                   (alignment: *mut C_GtkAlignment, xalign: c_float, yalign: c_float, xscale: c_float, yscale: c_float) -> ();
    pub fn gtk_alignment_get_padding           (alignment: *mut C_GtkAlignment, padding_top: *mut c_uint, padding_bottom: *mut c_uint, padding_left: *mut c_uint, padding_right: *mut c_uint) -> ();
    pub fn gtk_alignment_set_padding           (alignment: *mut C_GtkAlignment, padding_top: c_uint, padding_bottom: c_uint, padding_left: c_uint, padding_right: c_uint) -> ();

    //=========================================================================
    // GtkExpander                                                           OK
    //=========================================================================
    pub fn gtk_expander_new                    (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_expander_new_with_mnemonic      (label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_expander_set_expanded           (expander: *mut C_GtkExpander, expanded: Gboolean) -> ();
    pub fn gtk_expander_get_expanded           (expander: *mut C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_spacing            (expander: *mut C_GtkExpander, spacing: c_int) -> ();
    pub fn gtk_expander_get_spacing            (expander: *mut C_GtkExpander) -> c_int;
    pub fn gtk_expander_set_label              (expander: *mut C_GtkExpander, label: *const c_char) -> ();
    pub fn gtk_expander_get_label              (expander: *mut C_GtkExpander) -> *const c_char;
    pub fn gtk_expander_set_use_underline      (expander: *mut C_GtkExpander, use_underline: Gboolean)-> ();
    pub fn gtk_expander_get_use_underline      (expander: *mut C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_use_markup         (expander: *mut C_GtkExpander, use_markup: Gboolean) -> ();
    pub fn gtk_expander_get_use_markup         (expander: *mut C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_label_widget       (expander: *mut C_GtkExpander, label_widget: *mut C_GtkWidget) -> ();
    pub fn gtk_expander_get_label_widget       (expander: *mut C_GtkExpander) -> *mut C_GtkWidget;
    pub fn gtk_expander_set_label_fill         (expander: *mut C_GtkExpander, label_fill: Gboolean) -> ();
    pub fn gtk_expander_get_label_fill         (expander: *mut C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_resize_toplevel    (expander: *mut C_GtkExpander, resize_toplevel: Gboolean) -> ();
    pub fn gtk_expander_get_resize_toplevel    (expander: *mut C_GtkExpander) -> Gboolean;

    //=========================================================================
    // GtkPaned                                                              OK
    //=========================================================================
    pub fn gtk_paned_new                       (orientation: gtk::Orientation) -> *mut C_GtkWidget;
    pub fn gtk_paned_add1                      (paned: *mut C_GtkPaned, child: *mut C_GtkWidget) -> ();
    pub fn gtk_paned_add2                      (paned: *mut C_GtkPaned, child: *mut C_GtkWidget) -> ();
    pub fn gtk_paned_pack1                     (paned: *mut C_GtkPaned, child: *mut C_GtkWidget, resize: Gboolean, schrink: Gboolean) -> ();
    pub fn gtk_paned_pack2                     (paned: *mut C_GtkPaned, child: *mut C_GtkWidget, resize: Gboolean, schrink: Gboolean) -> ();
    // pub fn gtk_paned_get_child1                (paned: *const const C_GtkPaned) -> *const const C_GtkWidget;
    // pub fn gtk_paned_get_child2                (paned: *const const C_GtkPaned) -> *const const C_GtkWidget;
    pub fn gtk_paned_set_position              (paned: *mut C_GtkPaned, position: c_int) -> ();
    pub fn gtk_paned_get_position              (paned: *mut C_GtkPaned) -> c_int;
    pub fn gtk_paned_get_handle_window         (paned: *mut C_GtkPaned) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkInfoBar
    //=========================================================================
    pub fn gtk_info_bar_new                    () -> *mut C_GtkWidget;
    // pub fn gtk_info_bar_new_with_buttons       (const gchar *first_button_text, ...) -> *const const C_GtkWidget;
    pub fn gtk_info_bar_add_action_widget      (info_bar: *mut C_GtkInfoBar, child: *mut C_GtkWidget, response_id: c_int);
    pub fn gtk_info_bar_add_button             (info_bar: *mut C_GtkInfoBar, button_text: *const c_char, response_id: c_int) -> *mut C_GtkWidget;
    // pub fn gtk_info_bar_add_buttons            (GtkInfoBar *info_bar, const gchar *first_button_text, ...) -> ();
    pub fn gtk_info_bar_set_response_sensitive (info_bar: *mut C_GtkInfoBar, response_id: c_int, setting: Gboolean) -> ();
    pub fn gtk_info_bar_set_default_response   (info_bar: *mut C_GtkInfoBar, response_id: c_int) -> ();
    pub fn gtk_info_bar_response               (info_bar: *mut C_GtkInfoBar, response_id: c_int) -> ();
    pub fn gtk_info_bar_set_message_type       (info_bar: *mut C_GtkInfoBar, message_type: gtk::MessageType) -> ();
    pub fn gtk_info_bar_get_message_type       (info_bar: *mut C_GtkInfoBar) -> gtk::MessageType;
    // pub fn gtk_info_bar_get_action_area        (info_bar: *const const C_GtkInfoBar) -> *const const C_GtkWidget;
    // pub fn gtk_info_bar_get_content_area       (info_bar: *const const C_GtkInfoBar) -> *const const C_GtkWidget;
    pub fn gtk_info_bar_get_show_close_button  (info_bar: *mut C_GtkInfoBar) -> Gboolean;
    pub fn gtk_info_bar_set_show_close_button  (info_bar: *mut C_GtkInfoBar, setting: Gboolean) -> ();

    //=========================================================================
    // GtkToolShell
    //=========================================================================
    // pub fn gtk_tool_shell_get_ellipsize_mode   (shell: *const const C_GtkToolShell) -> PangoEllipsizeMode;
    pub fn gtk_tool_shell_get_icon_size        (shell: *mut C_GtkToolShell) -> gtk::IconSize;
    pub fn gtk_tool_shell_get_orientation      (shell: *mut C_GtkToolShell) -> gtk::Orientation;
    pub fn gtk_tool_shell_get_relief_style     (shell: *mut C_GtkToolShell) -> gtk::ReliefStyle;
    pub fn gtk_tool_shell_get_style            (shell: *mut C_GtkToolShell) -> gtk::ToolbarStyle;
    pub fn gtk_tool_shell_get_text_alignment   (shell: *mut C_GtkToolShell) -> c_float;
    pub fn gtk_tool_shell_get_text_orientation (shell: *mut C_GtkToolShell) -> gtk::Orientation;
    pub fn gtk_tool_shell_rebuild_menu         (shell: *mut C_GtkToolShell) -> ();
    // pub fn gtk_tool_shell_get_text_size_group  (shell: *const const C_GtkToolShell) -> *GtkSizeGroup;

    //=========================================================================
    // GtkToolBar
    //=========================================================================
    pub fn gtk_toolbar_new                     () -> *mut C_GtkWidget;
    pub fn gtk_toolbar_insert                  (toolbar: *mut C_GtkToolbar, item: *mut C_GtkToolItem, pos: c_int) -> ();
    pub fn gtk_toolbar_get_item_index          (toolbar: *mut C_GtkToolbar, item: *mut C_GtkToolItem) -> c_int;
    pub fn gtk_toolbar_get_n_items             (toolbar: *mut C_GtkToolbar) -> c_int;
    pub fn gtk_toolbar_get_nth_item            (toolbar: *mut C_GtkToolbar, n: c_int) -> *mut C_GtkToolItem;
    pub fn gtk_toolbar_get_drop_index          (toolbar: *mut C_GtkToolbar, x: c_int, y: c_int) -> c_int;
    pub fn gtk_toolbar_set_drop_highlight_item (toolbar: *mut C_GtkToolbar, item: *mut C_GtkToolItem, index: c_int) -> ();
    pub fn gtk_toolbar_set_show_arrow          (toolbar: *mut C_GtkToolbar, show_array: Gboolean) -> ();
    pub fn gtk_toolbar_unset_icon_size         (toolbar: *mut C_GtkToolbar) -> ();
    pub fn gtk_toolbar_get_show_arrow          (toolbar: *mut C_GtkToolbar) -> Gboolean;
    pub fn gtk_toolbar_get_style               (toolbar: *mut C_GtkToolbar) -> gtk::ToolbarStyle;
    pub fn gtk_toolbar_get_icon_size           (toolbar: *mut C_GtkToolbar) -> gtk::IconSize;
    pub fn gtk_toolbar_get_relief_style        (toolbar: *mut C_GtkToolbar) -> gtk::ReliefStyle;
    pub fn gtk_toolbar_set_style               (toolbar: *mut C_GtkToolbar, style: gtk::ToolbarStyle) -> ();
    pub fn gtk_toolbar_set_icon_size           (toolbar: *mut C_GtkToolbar, icon_size: gtk::IconSize) -> ();
    pub fn gtk_toolbar_unset_style             (toolbar: *mut C_GtkToolbar) -> ();

    //=========================================================================
    // GtkToolItem
    //=========================================================================
    pub fn gtk_tool_item_new                   () -> *mut C_GtkWidget;
    pub fn gtk_tool_item_set_homogeneous       (tool_item: *mut C_GtkToolItem, homogeneous: Gboolean) -> ();
    pub fn gtk_tool_item_get_homogeneous       (tool_item: *mut C_GtkToolItem) -> Gboolean;
    pub fn gtk_tool_item_set_expand            (tool_item: *mut C_GtkToolItem, expand: Gboolean) -> ();
    pub fn gtk_tool_item_get_expand            (tool_item: *mut C_GtkToolItem) -> Gboolean;
    pub fn gtk_tool_item_set_tooltip_text      (tool_item: *mut C_GtkToolItem, text: *const c_char) -> ();
    pub fn gtk_tool_item_set_tooltip_markup    (tool_item: *mut C_GtkToolItem, markup: *const c_char) -> ();
    pub fn gtk_tool_item_set_use_drag_window   (tool_item: *mut C_GtkToolItem, use_drag_window: Gboolean) -> ();
    pub fn gtk_tool_item_get_use_drag_window   (tool_item: *mut C_GtkToolItem) -> Gboolean;
    pub fn gtk_tool_item_set_visible_horizontal(tool_item: *mut C_GtkToolItem, visible_horizontal: Gboolean) -> ();
    pub fn gtk_tool_item_get_visible_horizontal(tool_item: *mut C_GtkToolItem) -> Gboolean;
    pub fn gtk_tool_item_set_visible_vertical  (tool_item: *mut C_GtkToolItem, visible_vertical: Gboolean) -> ();
    pub fn gtk_tool_item_get_visible_vertical  (tool_item: *mut C_GtkToolItem) -> Gboolean;
    pub fn gtk_tool_item_set_is_important      (tool_item: *mut C_GtkToolItem, is_important: Gboolean) -> ();
    pub fn gtk_tool_item_get_is_important      (tool_item: *mut C_GtkToolItem) -> Gboolean;
    // pub fn gtk_tool_item_get_ellipsize_mode    (tool_item: *const const C_GtkToolItem) -> PangoEllipsizeMode;
    pub fn gtk_tool_item_get_icon_size         (tool_item: *mut C_GtkToolItem) -> gtk::IconSize;
    pub fn gtk_tool_item_get_orientation       (tool_item: *mut C_GtkToolItem) -> gtk::Orientation;
    pub fn gtk_tool_item_get_toolbar_style     (tool_item: *mut C_GtkToolItem) -> gtk::ToolbarStyle;
    pub fn gtk_tool_item_get_relief_style      (tool_item: *mut C_GtkToolItem) -> gtk::ReliefStyle;
    pub fn gtk_tool_item_get_text_alignment    (tool_item: *mut C_GtkToolItem) -> c_float;
    pub fn gtk_tool_item_get_text_orientation  (tool_item: *mut C_GtkToolItem) -> gtk::Orientation;
    // pub fn gtk_tool_item_retrieve_proxy_menu_item(tool_item: *const const C_GtkToolItem) -> *const const C_GtkWidget;
    // pub fn gtk_tool_item_get_proxy_menu_item   (tool_item: *const const C_GtkToolItem, menu_item_id: *const c_char) -> *const const C_GtkWidget;
    // pub fn gtk_tool_item_set_proxy_menu_item   (tool_item: *const const C_GtkToolItem, menu_item_id: *const c_char, menu_item: *const const C_GtkWidget) -> ();
    pub fn gtk_tool_item_rebuild_menu          (tool_item: *mut C_GtkToolItem) -> ();
    pub fn gtk_tool_item_toolbar_reconfigured  (tool_item: *mut C_GtkToolItem) -> ();
    // pub fn gtk_tool_item_get_text_size_group   (tool_item: *const const C_GtkToolItem) -> *GtkSizeGroup;

    //=========================================================================
    // GtkSeparatorToolItem
    //=========================================================================
    pub fn gtk_separator_tool_item_new         () -> *mut C_GtkWidget;
    pub fn gtk_separator_tool_item_set_draw    (item: *mut C_GtkSeparatorToolItem, draw: Gboolean) -> ();
    pub fn gtk_separator_tool_item_get_draw    (item: *mut C_GtkSeparatorToolItem) -> Gboolean;

    //=========================================================================
    // GtkToolButton
    //=========================================================================
    pub fn gtk_tool_button_new                 (icon_widget: *mut C_GtkWidget, label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_tool_button_new_from_stock      (stock_id: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_tool_button_set_label           (button: *mut C_GtkToolButton, label: *const c_char) -> ();
    pub fn gtk_tool_button_get_label           (button: *mut C_GtkToolButton) -> *const c_char;
    pub fn gtk_tool_button_set_use_underline   (button: *mut C_GtkToolButton, use_underline: Gboolean) -> ();
    pub fn gtk_tool_button_get_use_underline   (button: *mut C_GtkToolButton) -> Gboolean;
    pub fn gtk_tool_button_set_stock_id        (button: *mut C_GtkToolButton, stock_id: *const c_char) -> ();
    pub fn gtk_tool_button_get_stock_id        (button: *mut C_GtkToolButton) -> *const c_char;
    pub fn gtk_tool_button_set_icon_name       (button: *mut C_GtkToolButton, icon_name: *const c_char) -> ();
    pub fn gtk_tool_button_get_icon_name       (button: *mut C_GtkToolButton) -> *const c_char;
    // pub fn gtk_tool_button_set_icon_widget     (button: *const const C_GtkToolButton, icon_widget: *const const C_GtkWidget) -> ();
    // pub fn gtk_tool_button_get_icon_widget     (button: *const const C_GtkToolButton) -> *const const C_GtkWidget;
    pub fn gtk_tool_button_set_label_widget    (button: *mut C_GtkToolButton, label_widget: *mut C_GtkWidget) -> ();
    pub fn gtk_tool_button_get_label_widget    (button: *mut C_GtkToolButton) -> *mut C_GtkWidget;

    //=========================================================================
    // GtkToggleToolButton
    //=========================================================================
    pub fn gtk_toggle_tool_button_new          () -> *mut C_GtkWidget;
    pub fn gtk_toggle_tool_button_new_from_stock(stock_id: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_toggle_tool_button_set_active   (button: *mut C_GtkToggleToolButton, is_active: Gboolean) -> ();
    pub fn gtk_toggle_tool_button_get_active   (button: *mut C_GtkToggleToolButton) -> Gboolean;


    //=========================================================================
    // GtkRadioToolButton
    //=========================================================================
    // pub fn gtk_radio_tool_button_new           (GSList *group) -> *const const C_GtkWidget;
    // pub fn gtk_radio_tool_button_new_from_stock(GSList *group, const gchar *stock_id) -> *const const C_GtkWidget;
    // pub fn gtk_radio_tool_button_new_from_widget(GtkRadioToolButton *group) -> *const const C_GtkWidget;
    // pub fn gtk_radio_tool_button_new_with_stock_from_widget(GtkRadioToolButton *group, const gchar *stock_id) -> *const const C_GtkWidget;
    // pub fn gtk_radio_tool_button_get_group     (GtkRadioToolButton *button) -> *GSList;
    // pub fn gtk_radio_tool_button_set_group     (GtkRadioToolButton *button, GSList *group) -> ();

    //=========================================================================
    // GtkMenuToolButton
    //=========================================================================
    pub fn gtk_menu_tool_button_new            (icon_widget: *mut C_GtkWidget, label: *const c_char) -> *mut C_GtkWidget;
    pub fn gtk_menu_tool_button_new_from_stock (stock_id: *const c_char) -> *mut C_GtkWidget;
    // pub fn gtk_menu_tool_button_set_menu       (button: *const const C_GtkMenuToolButton, menu: *const const C_GtkWidget);
    // pub fn gtk_menu_tool_button_get_menu       (button: *const const C_GtkMenuToolButton) -> *const const C_GtkWidget;
    pub fn gtk_menu_tool_button_set_arrow_tooltip_text(button: *mut C_GtkMenuToolButton, text: *const c_char) -> ();
    pub fn gtk_menu_tool_button_set_arrow_tooltip_markup(button: *mut C_GtkMenuToolButton, markup: *const c_char) -> ();

    //=========================================================================
    // GtkNoteBook                                                           OK
    //=========================================================================
    pub fn gtk_notebook_new               () -> *mut C_GtkWidget;
    pub fn gtk_notebook_append_page       (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget) -> c_int;
    pub fn gtk_notebook_append_page_menu  (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget, menu_label: *mut C_GtkWidget) -> c_int;
    pub fn gtk_notebook_prepend_page      (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget) -> c_int;
    pub fn gtk_notebook_prepend_page_menu (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget, menu_label: *mut C_GtkWidget) -> c_int;
    pub fn gtk_notebook_insert_page       (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget, position: c_int) -> c_int;
    pub fn gtk_notebook_insert_page_menu  (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget, menu_label: *mut C_GtkWidget, position: c_int) -> c_int;
    pub fn gtk_notebook_remove_page       (notebook: *mut C_GtkNotebook, page_num: c_int);
    pub fn gtk_notebook_set_group_name    (notebook: *mut C_GtkNotebook, group_name: *const c_char);
    pub fn gtk_notebook_get_group_name    (notebook: *mut C_GtkNotebook) -> *const c_char;
    pub fn gtk_notebook_get_current_page  (notebook: *mut C_GtkNotebook) -> c_int;
    pub fn gtk_notebook_get_nth_page      (notebook: *mut C_GtkNotebook, page_num: c_int) -> *mut C_GtkWidget;
    pub fn gtk_notebook_get_n_pages       (notebook: *mut C_GtkNotebook) -> c_int;
    pub fn gtk_notebook_page_num          (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> c_int;
    pub fn gtk_notebook_set_current_page  (notebook: *mut C_GtkNotebook, page_num: c_int);
    pub fn gtk_notebook_next_page         (notebook: *mut C_GtkNotebook);
    pub fn gtk_notebook_prev_page         (notebook: *mut C_GtkNotebook);
    pub fn gtk_notebook_set_show_border   (notebook: *mut C_GtkNotebook, show_border: Gboolean);
    pub fn gtk_notebook_get_show_border   (notebook: *mut C_GtkNotebook) -> Gboolean;
    pub fn gtk_notebook_set_show_tabs     (notebook: *mut C_GtkNotebook, show_tabs: Gboolean);
    pub fn gtk_notebook_get_show_tabs     (notebook: *mut C_GtkNotebook) -> Gboolean;
    pub fn gtk_notebook_set_tab_pos       (notebook: *mut C_GtkNotebook, pos: gtk::PositionType);
    pub fn gtk_notebook_get_tab_pos       (notebook: *mut C_GtkNotebook) -> gtk::PositionType;
    pub fn gtk_notebook_set_scrollable    (notebook: *mut C_GtkNotebook, scrollable: Gboolean);
    pub fn gtk_notebook_get_scrollable    (notebook: *mut C_GtkNotebook) -> Gboolean;
    pub fn gtk_notebook_get_tab_hborder   (notebook: *mut C_GtkNotebook) -> u16;
    pub fn gtk_notebook_get_tab_vborder   (notebook: *mut C_GtkNotebook) -> u16;
    pub fn gtk_notebook_popup_enable      (notebook: *mut C_GtkNotebook);
    pub fn gtk_notebook_popup_disable     (notebook: *mut C_GtkNotebook);
    pub fn gtk_notebook_get_tab_label     (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> *mut C_GtkWidget;
    pub fn gtk_notebook_set_tab_label     (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_label: *mut C_GtkWidget);
    pub fn gtk_notebook_set_tab_label_text(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, tab_text: *const c_char);
    pub fn gtk_notebook_get_tab_label_text(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> *const c_char;
    pub fn gtk_notebook_get_menu_label    (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> *mut C_GtkWidget;
    pub fn gtk_notebook_set_menu_label    (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, menu_label: *mut C_GtkWidget);
    pub fn gtk_notebook_set_menu_label_text(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, menu_text: *const c_char);
    pub fn gtk_notebook_get_menu_label_text(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> *const c_char;
    pub fn gtk_notebook_reorder_child     (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, position: c_int);
    pub fn gtk_notebook_get_tab_reorderable(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> Gboolean;
    pub fn gtk_notebook_set_tab_reorderable(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, reorderable: Gboolean);
    pub fn gtk_notebook_get_tab_detachable(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget) -> Gboolean;
    pub fn gtk_notebook_set_tab_detachable(notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, detachable: Gboolean);
    pub fn gtk_notebook_get_action_widget (notebook: *mut C_GtkNotebook,pack_type: gtk::PackType) -> *mut C_GtkWidget;
    pub fn gtk_notebook_set_action_widget (notebook: *mut C_GtkNotebook, child: *mut C_GtkWidget, pack_type: gtk::PackType);


    //=========================================================================
    // GtkStack                                                              OK
    //=========================================================================
    pub fn gtk_stack_new                     () -> *mut C_GtkWidget;
    pub fn gtk_stack_add_named               (stack: *mut C_GtkStack, child: *mut C_GtkWidget, name: *const c_char);
    pub fn gtk_stack_add_titled              (stack: *mut C_GtkStack, child: *mut C_GtkWidget, name: *const c_char, title: *const c_char);
    pub fn gtk_stack_set_visible_child       (stack: *mut C_GtkStack, child: *mut C_GtkWidget);
    pub fn gtk_stack_get_visible_child       (stack: *mut C_GtkStack) -> *mut C_GtkWidget;
    pub fn gtk_stack_set_visible_child_name  (stack: *mut C_GtkStack, name: *const c_char);
    pub fn gtk_stack_get_visible_child_name  (stack: *mut C_GtkStack) -> *const c_char;
    pub fn gtk_stack_set_visible_child_full  (stack: *mut C_GtkStack, name: *const c_char, transition: gtk::StackTransitionType);
    pub fn gtk_stack_set_homogeneous         (stack: *mut C_GtkStack, homogeneous: Gboolean);
    pub fn gtk_stack_get_homogeneous         (stack: *mut C_GtkStack) -> Gboolean;
    pub fn gtk_stack_set_transition_duration (stack: *mut C_GtkStack, duration: c_uint);
    pub fn gtk_stack_get_transition_duration (stack: *mut C_GtkStack) -> c_uint;
    pub fn gtk_stack_set_transition_type     (stack: *mut C_GtkStack, transition: gtk::StackTransitionType);
    pub fn gtk_stack_get_transition_type     (stack: *mut C_GtkStack) -> gtk::StackTransitionType;

    //=========================================================================
    // GtkStackSwitcher                                                      OK
    //=========================================================================
    pub fn gtk_stack_switcher_new       () -> *mut C_GtkWidget;
    pub fn gtk_stack_switcher_set_stack (switcher: *mut C_GtkStackSwitcher, stack: *mut C_GtkStack);
    pub fn gtk_stack_switcher_get_stack (switcher: *mut C_GtkStackSwitcher) -> *mut C_GtkWidget;


    //=========================================================================
    // GtkRevealer                                                           OK
    //=========================================================================
    pub fn gtk_revealer_new                     () -> *mut C_GtkWidget;
    pub fn gtk_revealer_get_reveal_child        (revealer: *mut C_GtkRevealer) -> Gboolean;
    pub fn gtk_revealer_set_reveal_child        (revealer: *mut C_GtkRevealer, reveal_child: Gboolean);
    pub fn gtk_revealer_get_child_revealed      (revealer: *mut C_GtkRevealer) -> Gboolean;
    pub fn gtk_revealer_get_transition_duration (revealer: *mut C_GtkRevealer) -> c_uint;
    pub fn gtk_revealer_set_transition_duration (revealer: *mut C_GtkRevealer, duration: c_uint);
    pub fn gtk_revealer_set_transition_type     (revealer: *mut C_GtkRevealer, transition: gtk::RevealerTransitionType);
    pub fn gtk_revealer_get_transition_type     (revealer: *mut C_GtkRevealer) -> gtk::RevealerTransitionType;

    //=========================================================================
    // GtkOverlay                                                            OK
    //=========================================================================
    pub fn gtk_overlay_new () -> *mut C_GtkWidget;
    pub fn gtk_overlay_add_overlay (overlay: *mut C_GtkOverlay, widget: *mut C_GtkWidget);

    //=========================================================================
    // GtkScrollable                                                         OK
    //=========================================================================
    pub fn gtk_scrollable_get_hadjustment        (scrollable: *mut C_GtkScrollable) -> *mut C_GtkAdjustment;
    pub fn gtk_scrollable_set_hadjustment        (scrollable: *mut C_GtkScrollable, hadjustment: *mut C_GtkAdjustment);
    pub fn gtk_scrollable_get_vadjustment        (scrollable: *mut C_GtkScrollable) -> *mut C_GtkAdjustment;
    pub fn gtk_scrollable_set_vadjustment        (scrollable: *mut C_GtkScrollable, vadjustment: *mut C_GtkAdjustment);
    pub fn gtk_scrollable_get_hscroll_policy     (scrollable: *mut C_GtkScrollable) -> gtk::ScrollablePolicy;
    pub fn gtk_scrollable_set_hscroll_policy     (scrollable: *mut C_GtkScrollable, policy: gtk::ScrollablePolicy);
    pub fn gtk_scrollable_get_vscroll_policy     (scrollable: *mut C_GtkScrollable) -> gtk::ScrollablePolicy;
    pub fn gtk_scrollable_set_vscroll_policy     (scrollable: *mut C_GtkScrollable, policy: gtk::ScrollablePolicy);

    //=========================================================================
    // GtkLayout
    //=========================================================================
    pub fn gtk_layout_new             (hadjustment: *mut C_GtkAdjustment, vadjustment: *mut C_GtkAdjustment) -> *mut C_GtkWidget;
    pub fn gtk_layout_put             (layout: *mut C_GtkLayout, child_widget: *mut C_GtkWidget, x: c_int, y: c_int);
    pub fn gtk_layout_move            (layout: *mut C_GtkLayout, child_widget: *mut C_GtkWidget, x: c_int, y: c_int);
    pub fn gtk_layout_set_size        (layout: *mut C_GtkLayout, width: c_uint, height: c_uint);
    pub fn gtk_layout_get_size        (layout: *mut C_GtkLayout, width: *mut c_uint, height: *mut c_uint);
    // pub fn gtk_layout_get_bin_window  (layout: *const const C_GtkLayout) -> *const const C_GdkWindow;

    //=========================================================================
    // GtkHeaderBar                                                          OK
    //=========================================================================
    pub fn gtk_header_bar_new               () -> *mut C_GtkWidget;
    pub fn gtk_header_bar_set_title         (bar: *mut C_GtkHeaderBar, title: *const c_char);
    pub fn gtk_header_bar_get_title         (bar: *mut C_GtkHeaderBar) -> *const c_char;
    pub fn gtk_header_bar_set_subtitle      (bar: *mut C_GtkHeaderBar, subtitle: *const c_char);
    pub fn gtk_header_bar_get_subtitle      (bar: *mut C_GtkHeaderBar) -> *const c_char;
    pub fn gtk_header_bar_set_custom_title  (bar: *mut C_GtkHeaderBar, title_widget: *mut C_GtkWidget);
    pub fn gtk_header_bar_get_custom_title  (bar: *mut C_GtkHeaderBar) -> *mut C_GtkWidget;
    pub fn gtk_header_bar_pack_start        (bar: *mut C_GtkHeaderBar, child: *mut C_GtkWidget);
    pub fn gtk_header_bar_pack_end          (bar: *mut C_GtkHeaderBar, child: *mut C_GtkWidget);
    pub fn gtk_header_bar_get_show_close_button (bar: *mut C_GtkHeaderBar) -> Gboolean;
    pub fn gtk_header_bar_set_show_close_button (bar: *mut C_GtkHeaderBar, setting: Gboolean);

    //=========================================================================
    // GtkFlowBox                                                            OK
    //=========================================================================
    pub fn gtk_flow_box_new                       () -> *mut C_GtkWidget;
    pub fn gtk_flow_box_set_homogeneous           (_box: *mut C_GtkFlowBox, homogeneous: Gboolean);
    pub fn gtk_flow_box_get_homogeneous           (_box: *mut C_GtkFlowBox) -> Gboolean;
    pub fn gtk_flow_box_set_row_spacing           (_box: *mut C_GtkFlowBox, spacing: c_uint);
    pub fn gtk_flow_box_get_row_spacing           (_box: *mut C_GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_column_spacing        (_box: *mut C_GtkFlowBox, spacing: c_uint);
    pub fn gtk_flow_box_get_column_spacing        (_box: *mut C_GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_min_children_per_line (_box: *mut C_GtkFlowBox, n_children: c_uint);
    pub fn gtk_flow_box_get_min_children_per_line (_box: *mut C_GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_max_children_per_line (_box: *mut C_GtkFlowBox, n_children: c_uint);
    pub fn gtk_flow_box_get_max_children_per_line (_box: *mut C_GtkFlowBox) -> c_uint;
    pub fn gtk_flow_box_set_activate_on_single_click (_box: *mut C_GtkFlowBox, single: Gboolean);
    pub fn gtk_flow_box_get_activate_on_single_click (_box: *mut C_GtkFlowBox) -> Gboolean;
    pub fn gtk_flow_box_insert                       (_box: *mut C_GtkFlowBox, widget: *mut C_GtkWidget, position: c_int);
    pub fn gtk_flow_box_get_child_at_index           (_box: *mut C_GtkFlowBox, idx: c_int) -> *mut C_GtkFlowBoxChild;
    pub fn gtk_flow_box_select_child                 (_box: *mut C_GtkFlowBox, child: *mut C_GtkFlowBoxChild);
    pub fn gtk_flow_box_unselect_child               (_box: *mut C_GtkFlowBox, child: *mut C_GtkFlowBoxChild);
    pub fn gtk_flow_box_select_all                   (_box: *mut C_GtkFlowBox);
    pub fn gtk_flow_box_unselect_all                 (_box: *mut C_GtkFlowBox);
    pub fn gtk_flow_box_set_selection_mode           (_box: *mut C_GtkFlowBox, mode: gtk::SelectionMode);
    pub fn gtk_flow_box_get_selection_mode           (_box: *mut C_GtkFlowBox) -> gtk::SelectionMode;
    pub fn gtk_flow_box_set_hadjustment              (_box: *mut C_GtkFlowBox, adjustment: *mut C_GtkAdjustment);
    pub fn gtk_flow_box_set_vadjustment              (_box: *mut C_GtkFlowBox, adjustment: *mut C_GtkAdjustment);
    // void                  gtk_flow_box_selected_foreach             (_box: *const const C_GtkFlowBox, GtkFlowBoxForeachFunc func, gpointer           data);
    // GList                *gtk_flow_box_get_selected_children        (_box: *const const C_GtkFlowBox);

    //=========================================================================
    // GtkFlowBoxChild                                                       OK
    //=========================================================================
    pub fn gtk_flow_box_child_new () -> *mut C_GtkWidget;
    pub fn gtk_flow_box_child_get_index (child: *mut C_GtkFlowBoxChild) -> c_int;
    pub fn gtk_flow_box_child_is_selected (child: *mut C_GtkFlowBoxChild) -> Gboolean;
    pub fn gtk_flow_box_child_changed (child: *mut C_GtkFlowBoxChild);

    //=========================================================================
    // GtkListBox                                                            OK
    //=========================================================================
    pub fn gtk_list_box_new                          () -> *mut C_GtkWidget;
    pub fn gtk_list_box_prepend                      (list_box: *mut C_GtkListBox, child: *mut C_GtkWidget);
    pub fn gtk_list_box_insert                       (list_box: *mut C_GtkListBox, child: *mut C_GtkWidget, position: c_int);
    pub fn gtk_list_box_get_selected_row             (list_box: *mut C_GtkListBox) -> *mut C_GtkListBoxRow;
    pub fn gtk_list_box_get_row_at_index             (list_box: *mut C_GtkListBox, index: c_int) -> *mut C_GtkListBoxRow;
    pub fn gtk_list_box_get_row_at_y                 (list_box: *mut C_GtkListBox, y: c_int) -> *mut C_GtkListBoxRow;
    pub fn gtk_list_box_select_row                   (list_box: *mut C_GtkListBox, row: *mut C_GtkListBoxRow);
    pub fn gtk_list_box_set_placeholder              (list_box: *mut C_GtkListBox, placeholder: *mut C_GtkWidget);
    pub fn gtk_list_box_set_adjustment               (list_box: *mut C_GtkListBox, adjustment: *mut C_GtkAdjustment);
    pub fn gtk_list_box_get_adjustment               (list_box: *mut C_GtkListBox) -> *mut C_GtkAdjustment;
    pub fn gtk_list_box_set_selection_mode           (list_box: *mut C_GtkListBox, mode: gtk::SelectionMode);
    pub fn gtk_list_box_get_selection_mode           (list_box: *mut C_GtkListBox) -> gtk::SelectionMode;
    // pub fn gtk_list_box_invalidate_filter            (list_box: *const const C_GtkListBox);
    // pub fn gtk_list_box_invalidate_sort              (list_box: *const const C_GtkListBox);
    pub fn gtk_list_box_invalidate_headers           (list_box: *mut C_GtkListBox);
    pub fn gtk_list_box_set_activate_on_single_click (list_box: *mut C_GtkListBox, simgle: Gboolean);
    pub fn gtk_list_box_get_activate_on_single_click (list_box: *mut C_GtkListBox) -> Gboolean;
    pub fn gtk_list_box_drag_unhighlight_row         (list_box: *mut C_GtkListBox);
    pub fn gtk_list_box_drag_highlight_row           (list_box: *mut C_GtkListBox, row: *mut C_GtkListBoxRow);


    //=========================================================================
    // GtkListBoxRow                                                         OK
    //=========================================================================
    pub fn gtk_list_box_row_new         () -> *mut C_GtkWidget;
    pub fn gtk_list_box_row_changed     (row: *mut C_GtkListBoxRow);
    pub fn gtk_list_box_row_get_header  (row: *mut C_GtkListBoxRow) -> *mut C_GtkWidget;
    pub fn gtk_list_box_row_set_header  (row: *mut C_GtkListBoxRow, header: *mut C_GtkWidget);
    pub fn gtk_list_box_row_get_index   (row: *mut C_GtkListBoxRow) -> c_int;

    //=========================================================================
    // GtkActionBar                                                          OK
    //=========================================================================
    pub fn gtk_action_bar_new               () -> *mut C_GtkWidget;
    pub fn gtk_action_bar_get_center_widget (action_bar: *mut C_GtkActionBar) -> *mut C_GtkWidget;
    pub fn gtk_action_bar_set_center_widget (action_bar: *mut C_GtkActionBar, center_widget: *mut C_GtkWidget);
    pub fn gtk_action_bar_pack_start        (action_bar: *mut C_GtkActionBar, child: *mut C_GtkWidget);
    pub fn gtk_action_bar_pack_end          (action_bar: *mut C_GtkActionBar, child: *mut C_GtkWidget);

    //=========================================================================
    // GtkDrawingArea
    //=========================================================================
    pub fn gtk_drawing_area_new                 () -> *mut C_GtkWidget;

    //=========================================================================
    // Glue fixe code
    //=========================================================================
    pub fn glue_signal_connect(g_object: *mut C_GtkWidget,
                               signal: *const c_char,
                               func: Option<extern "C" fn()>,
                               user_data: *const c_void);
    pub fn g_signal_connect_data(instance: gpointer,
                                 detailed_signal: *const c_char,
                                 c_hanlder: Option<extern "C" fn()>,
                                 data: gpointer,
                                 destroy_data: Option<extern "C" fn(gpointer, *const C_GClosure)>,
                                 connect_flags: i32);

    //=========================================================================
    // GTK Casts functions
    //=========================================================================
    pub fn cast_GtkWindow(widget: *mut C_GtkWidget) -> *mut C_GtkWindow;
    pub fn cast_GtkBin(widget: *mut C_GtkWidget) -> *mut C_GtkBin;
    pub fn cast_GtkButton(widget: *mut C_GtkWidget) -> *mut C_GtkButton;
    pub fn cast_GtkContainer(widget: *mut C_GtkWidget) -> *mut C_GtkContainer;
    pub fn cast_GtkFrame(widget: *mut C_GtkWidget) -> *mut C_GtkFrame;
    pub fn cast_GtkLabel(widget: *mut C_GtkWidget) -> *mut C_GtkLabel;
    pub fn cast_GtkMisc(widget: *mut C_GtkWidget) -> *mut C_GtkMisc;
    pub fn cast_GtkOrientable(widget: *mut C_GtkWidget) -> *mut C_GtkOrientable;
    pub fn cast_GtkBox(widget: *mut C_GtkWidget) -> *mut C_GtkBox;
    pub fn cast_GtkFixed(widget: *mut C_GtkWidget) -> *mut C_GtkFixed;
    pub fn cast_GtkButtonBox(widget: *mut C_GtkWidget) -> *mut C_GtkButtonBox;
    pub fn cast_GtkAspectFrame(widget: *mut C_GtkWidget) -> *mut C_GtkAspectFrame;
    pub fn cast_GtkFontButton(widget: *mut C_GtkWidget) -> *mut C_GtkFontButton;
    pub fn cast_GtkToggleButton(widget: *mut C_GtkWidget) -> *mut C_GtkToggleButton;
    pub fn cast_GtkCheckButton(widget: *mut C_GtkWidget) -> *mut C_GtkCheckButton;
    pub fn cast_GtkMenuButton(widget: *mut C_GtkWidget) -> *mut C_GtkMenuButton;
    pub fn cast_GtkColorButton(widget: *mut C_GtkWidget) -> *mut C_GtkColorButton;
    pub fn cast_GtkLinkButton(widget: *mut C_GtkWidget) -> *mut C_GtkLinkButton;
    pub fn cast_GtkScaleButton(widget: *mut C_GtkWidget) -> *mut C_GtkScaleButton;
    pub fn cast_GtkGrid(widget: *mut C_GtkWidget) -> *mut C_GtkGrid;
    pub fn cast_GtkEntry(widget: *mut C_GtkWidget) -> *mut C_GtkEntry;
    pub fn cast_GtkSwitch(widget: *mut C_GtkWidget) -> *mut C_GtkSwitch;
    pub fn cast_GtkScale(widget: *mut C_GtkWidget) -> *mut C_GtkScale;
    pub fn cast_GtkLevelBar(widget: *mut C_GtkWidget) -> *mut C_GtkLevelBar;
    pub fn cast_GtkSearchBar(widget: *mut C_GtkWidget) -> *mut C_GtkSearchBar;
    pub fn cast_GtkSpinButton(widget: *mut C_GtkWidget) -> *mut C_GtkSpinButton;
    pub fn cast_GtkSpinner(widget: *mut C_GtkWidget) -> *mut C_GtkSpinner;
    pub fn cast_GtkProgressBar(widget: *mut C_GtkWidget) -> *mut C_GtkProgressBar;
    pub fn cast_GtkArrow(widget: *mut C_GtkWidget) -> *mut C_GtkArrow;
    pub fn cast_GtkCalendar(widget: *mut C_GtkWidget) -> *mut C_GtkCalendar;
    pub fn cast_GtkAlignment(widget: *mut C_GtkWidget) -> *mut C_GtkAlignment;
    pub fn cast_GtkExpander(widget: *mut C_GtkWidget) -> *mut C_GtkExpander;
    pub fn cast_GtkPaned(widget: *mut C_GtkWidget) -> *mut C_GtkPaned;
    pub fn cast_GtkInfoBar(widget: *mut C_GtkWidget) -> *mut C_GtkInfoBar;
    pub fn cast_GtkToolShell(widget: *mut C_GtkWidget) -> *mut C_GtkToolShell;
    pub fn cast_GtkToolbar(widget: *mut C_GtkWidget) -> *mut C_GtkToolbar;
    pub fn cast_GtkToolItem(widget: *mut C_GtkWidget) -> *mut C_GtkToolItem;
    pub fn cast_GtkToolButton(widget: *mut C_GtkWidget) -> *mut C_GtkToolButton;
    pub fn cast_GtkSeparatorToolItem(widget: *mut C_GtkWidget) -> *mut C_GtkSeparatorToolItem;
    pub fn cast_GtkMenuToolButton(widget: *mut C_GtkWidget) -> *mut C_GtkMenuToolButton;
    pub fn cast_GtkToggleToolButton(widget: *mut C_GtkWidget) -> *mut C_GtkToggleToolButton;
    pub fn cast_GtkRadioToolButton(widget: *mut C_GtkWidget) -> *mut C_GtkRadioToolButton;
    pub fn cast_GtkDialog(widget: *mut C_GtkWidget) -> *mut C_GtkDialog;
    pub fn cast_GtkAboutDialog(widget: *mut C_GtkWidget) -> *mut C_GtkAboutDialog;
    pub fn cast_GtkMessageDialog(widget: *mut C_GtkWidget) -> *mut C_GtkMessageDialog;
    pub fn cast_GtkColorChooserDialog(widget: *mut C_GtkWidget) -> *mut C_GtkColorChooserDialog;
    pub fn cast_GtkColorChooser(widget: *mut C_GtkWidget) -> *mut C_GtkColorChooser;
    pub fn cast_GtkAdjustment(widget: *mut C_GtkWidget) -> *mut C_GtkAdjustment;
    pub fn cast_GtkNotebook(widget: *mut C_GtkWidget) -> *mut C_GtkNotebook;
    pub fn cast_GtkStack(widget: *mut C_GtkWidget) -> *mut C_GtkStack;
    pub fn cast_GtkStackSwitcher(widget: *mut C_GtkWidget) -> *mut C_GtkStackSwitcher;
    pub fn cast_GtkRevealer(widget: *mut C_GtkWidget) -> *mut C_GtkRevealer;
    pub fn cast_GtkOverlay(widget: *mut C_GtkWidget) -> *mut C_GtkOverlay;
    pub fn cast_GtkScrollable(widget: *mut C_GtkWidget) -> *mut C_GtkScrollable;
    pub fn cast_GtkLayout(widget: *mut C_GtkWidget) -> *mut C_GtkLayout;
    pub fn cast_GtkHeaderBar(widget: *mut C_GtkWidget) -> *mut C_GtkHeaderBar;
    pub fn cast_GtkFlowBox(widget: *mut C_GtkWidget) -> *mut C_GtkFlowBox;
    pub fn cast_GtkFlowBoxChild(widget: *mut C_GtkWidget) -> *mut C_GtkFlowBoxChild;
    pub fn cast_GtkListBox(widget: *mut C_GtkWidget) -> *mut C_GtkListBox;
    pub fn cast_GtkListBoxRow(widget: *mut C_GtkWidget) -> *mut C_GtkListBoxRow;
    pub fn cast_GtkActionBar(widget: *mut C_GtkWidget) -> *mut C_GtkActionBar;
    pub fn cast_GtkFileFilter(widget: *mut C_GtkWidget) -> *mut C_GtkFileFilter;
    pub fn cast_GtkFileChooser(widget: *mut C_GtkWidget) -> *mut C_GtkFileChooser;
    pub fn cast_GtkAppChooser(widget: *mut C_GtkWidget) -> *mut C_GtkAppChooser;
    pub fn cast_GtkAppChooserDialog(widget: *mut C_GtkWidget) -> *mut C_GtkAppChooserDialog;
    pub fn cast_GtkAppInfo(widget: *mut C_GtkWidget) -> *mut C_GAppInfo;
    pub fn cast_GtkAppLaunchContext(widget: *mut C_GtkWidget) -> *mut C_GAppLaunchContext;
}
