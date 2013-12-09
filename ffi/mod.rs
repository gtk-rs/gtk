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

use std::libc::{c_int, c_char, c_void, c_float, c_uint, c_double, c_long, c_short};

use gtk::enums::*;
use gdk;

pub type Gboolean = c_int;
pub static Gfalse:  c_int = 0;
pub static Gtrue:   c_int = !Gfalse;

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


pub struct C_GtkMenu;
pub struct C_GMenuModel;

extern "C" {

    //=========================================================================
    // Gtk Main Loop + events
    //=========================================================================
    pub fn gtk_init                            (argc: *c_int, argv: ***c_char) -> ();
    pub fn gtk_main                            () -> ();
    pub fn gtk_main_quit                       () -> ();
    pub fn gtk_main_level                      () -> c_uint;
    pub fn gtk_main_iteration                  () -> Gboolean;
    pub fn gtk_main_iteration_do               (blocking: Gboolean) -> Gboolean;

    //=========================================================================
    // GtkWindow
    //=========================================================================
    pub fn gtk_window_new                      (wtype : GtkWindowType) -> *C_GtkWidget;
    pub fn gtk_window_set_title                (window: *C_GtkWindow, title: *c_char) -> ();
    pub fn gtk_window_get_title                (window: *C_GtkWindow) -> *c_char;
    pub fn gtk_widget_hide                     (widget: *C_GtkWidget) -> ();
    // pub fn gtk_window_set_role(window: *C_GtkWindow, role: *c_char) -> ();
    // pub fn gtk_window_set_startup_id(window: *C_GtkWindow, startup_id: *c_char) -> ();
    // pub fn gtk_window_get_role(window: *C_GtkWindow) -> *c_char;
    // pub fn gtk_window_add_accel_group(window: *C_GtkWindow, accel_group: *GtkAccelGroup) -> ();
    // pub fn gtk_window_remove_accel_group(window: *C_GtkWindow, accel_group: *GtkAccelGroup) -> ();
    // pub fn gtk_window_set_position(window: *C_GtkWindow,position: GtkWindowPosition) -> ();
    // pub fn gtk_window_activate_focus(window: *C_GtkWindow) -> Gboolean;
    // pub fn gtk_window_set_focus(window: *C_GtkWindow, focus: *C_GtkWidget) -> ();
    // pub fn gtk_window_get_focus(window: *C_GtkWindow) -> *C_GtkWidget;
    // pub fn gtk_window_set_default(window: *C_GtkWindow, default_widget: *C_GtkWidget) -> ();
    // pub fn gtk_window_get_default_widget(window: *C_GtkWindow) -> *C_GtkWidget;
    // pub fn gtk_window_activate_default(window: *C_GtkWindow) -> Gboolean;
    // pub fn gtk_window_get_type() -> ();

    
    //=========================================================================
    // GtkWidget
    //=========================================================================
    pub fn gtk_widget_show                     (widget: *C_GtkWidget);
    pub fn gtk_widget_show_all                 (widget: *C_GtkWidget);
    pub fn gtk_widget_set_margin_right         (widget: *C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_set_margin_left          (widget: *C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_set_margin_top           (widget: *C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_set_margin_bottom        (widget: *C_GtkWidget, margin: c_int) -> ();
    pub fn gtk_widget_get_margin_right         (widget: *C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_margin_left          (widget: *C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_margin_top           (widget: *C_GtkWidget) -> c_int;
    pub fn gtk_widget_get_margin_bottom        (widget: *C_GtkWidget) -> c_int;

    //=========================================================================
    // GtkLabel
    //=========================================================================
    pub fn gtk_label_new                       (text: *c_char) -> *C_GtkWidget;
    pub fn gtk_label_set_label                 (label: *C_GtkLabel, text: *c_char);
    pub fn gtk_label_set_justify               (label: *C_GtkLabel, jtype: GtkJustification);
    pub fn gtk_label_set_text                  (label: *C_GtkLabel, stext: *c_char) -> ();
    // pub fn gtk_label_set_attributes            (label: *C_GtkLabel, PangoAttrList *attrs) -> ();
    pub fn gtk_label_set_markup                (label: *C_GtkLabel, text: *c_char) -> ();
    pub fn gtk_label_set_markup_with_mnemonic  (label: *C_GtkLabel, text: *c_char) -> ();
    pub fn gtk_label_set_pattern               (label: *C_GtkLabel, text: *c_char) -> ();
    // pub fn gtk_label_set_ellipsize             (label: *C_GtkLabel, PangoEllipsizeMode mode) -> ();
    pub fn gtk_label_set_width_chars           (label: *C_GtkLabel, n_chars: c_int) -> ();
    pub fn gtk_label_set_max_width_chars       (label: *C_GtkLabel, n_chars: c_int) -> ();
    pub fn gtk_label_set_line_wrap             (label: *C_GtkLabel, wrap: Gboolean) -> ();
    // pub fn gtk_label_set_line_wrap_mode        (label: *C_GtkLabel, PangoWrapMode wrap_mode);
    pub fn gtk_label_set_lines                 (label: *C_GtkLabel, lines: c_int) -> ();
    pub fn gtk_label_get_layout_offsets        (label: *C_GtkLabel, x: *c_int, y: *c_int) -> ();
    pub fn gtk_label_get_mnemonic_keyval       (label: *C_GtkLabel) -> c_uint;
    pub fn gtk_label_get_selectable            (label: *C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_text                  (label: *C_GtkLabel) -> *c_char;
    pub fn gtk_label_new_with_mnemonic         (text: *c_char) -> *C_GtkWidget;
    pub fn gtk_label_select_region             (label: *C_GtkLabel, start_offset: c_int, end_offset: c_int) -> ();
    // pub fn gtk_label_set_mnemonic_widget       (label: *C_GtkLabel, widget: *C_GtkWidget) -> ();
    pub fn gtk_label_set_selectable            (label: *C_GtkLabel, gsetting: Gboolean) -> ();
    pub fn gtk_label_set_text_with_mnemonic    (label: *C_GtkLabel, text: *c_char) -> ();
    // pub fn gtk_label_get_attributes            (label: *C_GtkLabel) -> *PangoAttrList;
    pub fn gtk_label_get_justify               (label: *C_GtkLabel) -> GtkJustification;
    // pub fn gtk_label_get_ellipsize             (label: *C_GtkLabel) -> PangoEllipsizeMode;
    pub fn gtk_label_get_width_chars           (label: *C_GtkLabel) -> c_int;
    pub fn gtk_label_get_max_width_chars       (label: *C_GtkLabel) -> c_int;
    pub fn gtk_label_get_label                 (label: *C_GtkLabel) -> *c_char;
    // pub fn gtk_label_get_layout                (label: *C_GtkLabel) -> *PangoLayout;
    pub fn gtk_label_get_line_wrap             (label: *C_GtkLabel) -> Gboolean;
    // pub fn gtk_label_get_line_wrap_mode        (label: *C_GtkLabel) -> PangoWrapMode;
    pub fn gtk_label_get_lines                 (label: *C_GtkLabel) -> c_int;
    // pub fn gtk_label_get_mnemonic_widget       (label: *C_GtkLabel) -> *C_GtkWidget;
    pub fn gtk_label_get_selection_bounds      (label: *C_GtkLabel, start: *c_int, end: *c_int) -> Gboolean;
    pub fn gtk_label_get_use_markup            (label: *C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_use_underline         (label: *C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_single_line_mode      (label: *C_GtkLabel) -> Gboolean;
    pub fn gtk_label_get_angle                 (label: *C_GtkLabel) -> c_double;
    pub fn gtk_label_set_use_markup            (label: *C_GtkLabel, setting: Gboolean) -> ();
    pub fn gtk_label_set_use_underline         (label: *C_GtkLabel, setting: Gboolean) -> ();
    pub fn gtk_label_set_single_line_mode      (label: *C_GtkLabel, single_line_mod: Gboolean) -> ();
    pub fn gtk_label_set_angle                 (label: *C_GtkLabel, angle: c_double) -> ();
    pub fn gtk_label_get_current_uri           (label: *C_GtkLabel) -> *c_char;
    pub fn gtk_label_set_track_visited_links   (label: *C_GtkLabel, track_links: Gboolean) -> ();
    pub fn gtk_label_get_track_visited_links   (label: *C_GtkLabel) -> Gboolean;

    //=========================================================================
    // GtkContainer
    //=========================================================================
    pub fn gtk_container_add                   (container: *C_GtkContainer, widget: *C_GtkWidget);
    pub fn gtk_container_remove                (container: *C_GtkContainer, widget: *C_GtkWidget) -> ();
    pub fn gtk_container_get_resize_mode       (container: *C_GtkContainer) -> GtkResizeMode;
    pub fn gtk_container_set_resize_mode       (container: *C_GtkContainer, resize_mode: GtkResizeMode) -> ();
    pub fn gtk_container_check_resize          (container: *C_GtkContainer) -> ();
    pub fn gtk_container_get_border_width      (container: *C_GtkContainer) -> c_uint;
    pub fn gtk_container_set_border_width      (container: *C_GtkContainer, border_width: c_uint) -> ();


    //=========================================================================
    // GtkMisc                                                               OK
    //=========================================================================

    pub fn gtk_misc_set_alignment              (misc: *C_GtkMisc, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_misc_set_padding                (misc: *C_GtkMisc, xpad: c_int, ypad: c_int) -> ();
    pub fn gtk_misc_get_alignment              (misc: *C_GtkMisc, xalign: *c_float, yalign: *c_float) -> ();
    pub fn gtk_misc_get_padding                (misc: *C_GtkMisc, xpad: *c_int, ypad: *c_int) -> ();

    //=========================================================================
    // GtkButton                                                             OK
    //=========================================================================
    pub fn gtk_button_new                      () -> *C_GtkWidget;
    pub fn gtk_button_new_with_label           (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_button_new_with_mnemonic        (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_button_new_from_icon_name       (icon_id: *c_char, size: GtkIconSize) -> *C_GtkWidget;
    pub fn gtk_button_new_from_stock           (stock_id: *c_char) -> *C_GtkWidget;
    pub fn gtk_button_pressed                  (button: *C_GtkButton) -> ();
    pub fn gtk_button_released                 (button: *C_GtkButton) -> ();
    pub fn gtk_button_clicked                  (button: *C_GtkButton) -> ();
    pub fn gtk_button_enter                    (button: *C_GtkButton) -> ();
    pub fn gtk_button_leave                    (button: *C_GtkButton) -> ();
    pub fn gtk_button_set_relief               (button: *C_GtkButton, newstyle: GtkReliefStyle) -> ();
    pub fn gtk_button_get_relief               (button: *C_GtkButton) -> GtkReliefStyle;
    pub fn gtk_button_get_label                (button: *C_GtkButton) -> *c_char;
    pub fn gtk_button_set_label                (button: *C_GtkButton, label: *c_char) -> ();
    pub fn gtk_button_get_use_stock            (button: *C_GtkButton) -> Gboolean;
    pub fn gtk_button_set_use_stock            (button: *C_GtkButton, use_stock: Gboolean) -> ();
    pub fn gtk_button_get_use_underline        (button: *C_GtkButton) -> Gboolean;
    pub fn gtk_button_set_use_underline        (button: *C_GtkButton, use_underline: Gboolean) -> ();
    pub fn gtk_button_set_focus_on_click       (button: *C_GtkButton, focus_on_click: Gboolean) -> ();
    pub fn gtk_button_get_focus_on_click       (button: *C_GtkButton) -> Gboolean;
    pub fn gtk_button_set_alignment            (button: *C_GtkButton, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_button_get_alignment            (button: *C_GtkButton, xalign: *c_float, yalign: *c_float) -> ();
    pub fn gtk_button_set_image                (button: *C_GtkButton, image: *C_GtkWidget) -> ();
    // pub fn gtk_button_get_image                (button: *C_GtkButton) -> *C_GtkWidget;
    pub fn gtk_button_set_image_position       (button: *C_GtkButton, position: GtkPositionType) -> ();
    pub fn gtk_button_get_image_position       (button: *C_GtkButton) -> GtkPositionType;
    pub fn gtk_button_set_always_show_image    (button: *C_GtkButton, always_show: Gboolean) -> ();
    pub fn gtk_button_get_always_show_image    (button: *C_GtkButton) -> Gboolean;
    // pub fn gtk_button_get_event_window         (button: *C_GtkButton) -> *C_GdkWindow;

    //=========================================================================
    // GtkFontButton                                                         OK
    //=========================================================================
    pub fn gtk_font_button_new                 () -> *C_GtkWidget;
    pub fn gtk_font_button_new_with_font       (fontname: *c_char) -> *C_GtkWidget;
    pub fn gtk_font_button_set_font_name       (font_button: *C_GtkFontButton, fontname: *c_char) -> Gboolean;
    pub fn gtk_font_button_get_font_name       (font_button: *C_GtkFontButton) -> *c_char;
    pub fn gtk_font_button_set_show_style      (font_button: *C_GtkFontButton, show_style: Gboolean) -> ();
    pub fn gtk_font_button_get_show_style      (font_button: *C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_show_size       (font_button: *C_GtkFontButton, show_size: Gboolean) -> ();
    pub fn gtk_font_button_get_show_size       (font_button: *C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_use_font        (font_button: *C_GtkFontButton, use_font: Gboolean) -> ();
    pub fn gtk_font_button_get_use_font        (font_button: *C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_use_size        (font_button: *C_GtkFontButton, use_size: Gboolean) -> ();
    pub fn gtk_font_button_get_use_size        (font_button: *C_GtkFontButton) -> Gboolean;
    pub fn gtk_font_button_set_title           (font_button: *C_GtkFontButton, title: *c_char) -> ();
    pub fn gtk_font_button_get_title           (font_button: *C_GtkFontButton) -> *c_char;

    //=========================================================================
    // GtkToggleButton                                                       OK
    //=========================================================================
    pub fn gtk_toggle_button_new               () -> *C_GtkWidget;
    pub fn gtk_toggle_button_new_with_label    (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_toggle_button_new_with_mnemonic (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_toggle_button_set_mode          (toggle_button: *C_GtkToggleButton, draw_indicator: Gboolean) -> ();
    pub fn gtk_toggle_button_get_mode          (toggle_button: *C_GtkToggleButton) -> Gboolean;
    pub fn gtk_toggle_button_toggled           (toggle_button: *C_GtkToggleButton) -> ();
    pub fn gtk_toggle_button_get_active        (toggle_button: *C_GtkToggleButton) -> Gboolean;
    pub fn gtk_toggle_button_set_active        (toggle_button: *C_GtkToggleButton, is_active: Gboolean) -> ();
    pub fn gtk_toggle_button_get_inconsistent  (toggle_button: *C_GtkToggleButton) -> Gboolean;
    pub fn gtk_toggle_button_set_inconsistent  (toggle_button: *C_GtkToggleButton, setting: Gboolean) -> ();

    //=========================================================================
    // GtkCheckButton                                                        OK
    //=========================================================================
    pub fn gtk_check_button_new                () -> *C_GtkWidget;
    pub fn gtk_check_button_new_with_label     (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_check_button_new_with_mnemonic  (label: *c_char) -> *C_GtkWidget;

    //=========================================================================
    // GtkRadioButton                                                      TODO  
    //=========================================================================
    // pub fn gtk_radio_button_new                (GSList *group) -> *C_GtkWidget;
    // pub fn gtk_radio_button_new_from_widget    (GtkRadioButton *radio_group_member) -> *C_GtkWidget;
    // pub fn gtk_radio_button_new_with_label     (GSList *group, const gchar *label) -> *C_GtkWidget;
    // pub fn tk_radio_button_new_with_label_from_widget (GtkRadioButton *radio_group_member, const gchar *label) -> *C_GtkWidget;
    // pub fn gtk_radio_button_new_with_mnemonic  (GSList *group, const gchar *label) -> *C_GtkWidget;
    // pub fn gtk_radio_button_new_with_mnemonic_from_widget(GtkRadioButton *radio_group_member,  const gchar *label) -> *C_GtkWidget;
    // pub fn gtk_radio_button_set_group          (GtkRadioButton *radio_button, GSList *group) -> ();
    // pub fn gtk_radio_button_get_group          (GtkRadioButton *radio_button) -> *GSList;
    // pub fn gtk_radio_button_join_group         (GtkRadioButton *radio_button, GtkRadioButton *group_source) -> ();

    //=========================================================================
    // GtkMenuButton                                                         OK
    //=========================================================================
    pub fn gtk_menu_button_new                 () -> *C_GtkWidget;
    pub fn gtk_menu_button_set_popup           (menu_button: *C_GtkMenuButton, popup: *C_GtkWidget) -> ();
    // pub fn gtk_menu_button_get_popup           (menu_button: *C_GtkMenuButton) -> *C_GtkMenu;
    // pub fn gtk_menu_button_set_menu_model      (menu_button: *C_GtkMenuButton, menu_model: *C_GMenuModel) -> ();
    // pub fn gtk_menu_button_get_menu_model      (menu_button: *C_GtkMenuButton) -> C_GMenuModel;
    pub fn gtk_menu_button_set_direction       (menu_button: *C_GtkMenuButton, direction: GtkArrowType) -> ();
    pub fn gtk_menu_button_get_direction       (menu_button: *C_GtkMenuButton) -> GtkArrowType;
    pub fn gtk_menu_button_set_align_widget    (menu_button: *C_GtkMenuButton, align_widget: *C_GtkWidget) -> ();
    // pub fn gtk_menu_button_get_align_widget    (menu_button: *C_GtkMenuButton) -> *C_GtkWidget;

    //=========================================================================
    // GtkColorButton                                                        OK
    //=========================================================================
    pub fn gtk_color_button_new                () -> *C_GtkWidget;
    pub fn gtk_color_button_new_with_color     (color: *gdk::Color) -> *C_GtkWidget;
    pub fn gtk_color_button_new_with_rgba      (rgba: *gdk::RGBA) -> *C_GtkWidget;
    pub fn gtk_color_button_set_color          (button: *C_GtkColorButton, color: *gdk::Color) -> ();
    pub fn gtk_color_button_get_color          (button: *C_GtkColorButton, color: *gdk::Color) -> ();
    pub fn gtk_color_button_set_alpha          (button: *C_GtkColorButton, alpha: u16) -> ();
    pub fn gtk_color_button_get_alpha          (button: *C_GtkColorButton) -> u16;
    pub fn gtk_color_button_set_rgba           (button: *C_GtkColorButton, rgba: *gdk::RGBA) -> ();
    pub fn gtk_color_button_get_rgba           (button: *C_GtkColorButton, rgba: *gdk::RGBA) -> ();
    pub fn gtk_color_button_set_use_alpha      (button: *C_GtkColorButton, use_alpha: Gboolean) -> ();
    pub fn gtk_color_button_get_use_alpha      (button: *C_GtkColorButton) -> Gboolean;
    pub fn gtk_color_button_set_title          (button: *C_GtkColorButton, title: *c_char) -> ();
    pub fn gtk_color_button_get_title          (button: *C_GtkColorButton) -> *c_char;

    //=========================================================================
    // GtkLinkButton                                                         OK
    //=========================================================================
    pub fn gtk_link_button_new                 (uri: *c_char) -> *C_GtkWidget;
    pub fn gtk_link_button_new_with_label      (uri: *c_char, label: *c_char) -> *C_GtkWidget;
    pub fn gtk_link_button_get_uri             (link_button: *C_GtkLinkButton) -> *c_char;
    pub fn gtk_link_button_set_uri             (link_button: *C_GtkLinkButton, uri: *c_char) -> ();
    pub fn gtk_link_button_get_visited         (link_button: *C_GtkLinkButton) -> Gboolean;
    pub fn gtk_link_button_set_visited         (link_button: *C_GtkLinkButton, visited: Gboolean) -> ();

    //=========================================================================
    // GtkScaleButton                                                        
    //=========================================================================
    pub fn gtk_scale_button_new                (size: GtkIconSize, min: c_double, max: c_double, step: c_double, icons: **c_char) -> *C_GtkWidget;
    pub fn gtk_scale_button_set_adjustment     (button: *C_GtkScaleButton, adjustment: *C_GtkAdjustment) -> ();
    // pub fn gtk_scale_button_set_icons          (button: *C_GtkScaleButton, icons: **c_char) -> ();
    pub fn gtk_scale_button_set_value          (button: *C_GtkScaleButton, value: c_double) -> ();
    pub fn gtk_scale_button_get_adjustment     (button: *C_GtkScaleButton) -> *C_GtkAdjustment;
    pub fn gtk_scale_button_get_value          (button: *C_GtkScaleButton) -> c_double;
    // pub fn gtk_scale_button_get_popup          (button: *C_GtkScaleButton) -> *C_GtkWidget;
    // pub fn gtk_scale_button_get_plus_button    (button: *C_GtkScaleButton) -> *C_GtkWidget;
    // pub fn gtk_scale_button_get_minus_button   (button: *C_GtkScaleButton) -> *C_GtkWidget;

    //=========================================================================
    // GtkVolumeButton                                                       OK                                                        
    //=========================================================================
    pub fn gtk_volume_button_new               () -> *C_GtkWidget;

    //=========================================================================
    // GtkBox                                                                
    //=========================================================================
    pub fn gtk_box_new                         (orientation: GtkOrientation, spacing: c_int) -> *C_GtkWidget;
    pub fn gtk_box_pack_start                  (box: *C_GtkBox, child: *C_GtkWidget, expand: Gboolean, fill: Gboolean, padding: c_uint) -> ();
    pub fn gtk_box_pack_end                    (box: *C_GtkBox, child: *C_GtkWidget, expand: Gboolean, fill: Gboolean, padding: c_uint) -> ();
    pub fn gtk_box_get_homogeneous             (box: *C_GtkBox) -> Gboolean;
    pub fn gtk_box_set_homogeneous             (box: *C_GtkBox, homogeneous: Gboolean) -> ();
    pub fn gtk_box_get_spacing                 (box: *C_GtkBox) -> c_int;
    pub fn gtk_box_set_spacing                 (box: *C_GtkBox, spacing: c_int) -> ();
    pub fn gtk_box_reorder_child               (box: *C_GtkBox, child: *C_GtkWidget, position: c_int) -> ();
    pub fn gtk_box_query_child_packing         (box: *C_GtkBox, child: *C_GtkWidget, expand: *Gboolean, fill: *Gboolean, padding: *c_uint, pack_type: *GtkPackType) -> ();
    pub fn gtk_box_set_child_packing           (box: *C_GtkBox, child: *C_GtkWidget, expand: Gboolean, fill: Gboolean, padding: c_uint, pack_type: GtkPackType) -> ();
    // pub fn gtk_box_get_baseline_position       (box: *C_GtkBox) -> GtkBaselinePosition;
    // pub fn gtk_box_set_baseline_position       (box: *C_GtkBox, position: GtkBaselinePosition) -> ();

    //=========================================================================
    // GtkOrientable                                                         OK
    //=========================================================================
    pub fn gtk_orientable_get_orientation      (orientable: *C_GtkOrientable) -> GtkOrientation;
    pub fn gtk_orientable_set_orientation      (orientable: *C_GtkOrientable,  orientation: GtkOrientation) -> ();

    //=========================================================================
    // GtkButtonBox                                                          OK
    //=========================================================================
    pub fn gtk_button_box_new                  (orientation: GtkOrientation) -> *C_GtkWidget;
    pub fn gtk_button_box_get_layout           (widget: *C_GtkButtonBox) -> GtkButtonBoxStyle;
    pub fn gtk_button_box_get_child_secondary  (widget: *C_GtkButtonBox, child : *C_GtkWidget) -> Gboolean;
    pub fn gtk_button_box_get_child_non_homogeneous(Gwidget: *C_GtkButtonBox, child : *C_GtkWidget) -> Gboolean;
    pub fn gtk_button_box_set_layout           (widget: *C_GtkButtonBox, layout_style: GtkButtonBoxStyle) -> ();
    pub fn gtk_button_box_set_child_secondary  (widget: *C_GtkButtonBox, child : *C_GtkWidget, is_secondary: Gboolean) -> ();
    pub fn gtk_button_box_set_child_non_homogeneous(widget: *C_GtkButtonBox, child : *C_GtkWidget, non_homogeneous: Gboolean) -> ();

    //=========================================================================
    // GtkVersion                                                            OK
    //=========================================================================
    pub fn gtk_get_major_version               () -> c_uint;
    pub fn gtk_get_minor_version               () -> c_uint;
    pub fn gtk_get_micro_version               () -> c_uint;
    pub fn gtk_get_binary_age                  () -> c_uint;
    pub fn gtk_get_interface_age               () -> c_uint;
    pub fn gtk_check_version                   (required_major: c_uint, required_minor: c_uint, required_micro: c_uint) -> *c_char;

    //=========================================================================
    // GtkFrame                                                              OK
    //=========================================================================
    pub fn gtk_frame_new                       (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_frame_set_label                 (frame: *C_GtkFrame, label: *c_char) -> ();
    pub fn gtk_frame_set_label_widget          (frame: *C_GtkFrame, label_widget: *C_GtkWidget) -> ();
    pub fn gtk_frame_set_label_align           (frame: *C_GtkFrame, xalign: c_float, yalign: c_float) -> ();
    pub fn gtk_frame_set_shadow_type           (frame: *C_GtkFrame, st_type: GtkShadowType) -> ();
    pub fn gtk_frame_get_label                 (frame: *C_GtkFrame) -> *c_char;
    pub fn gtk_frame_get_label_align           (frame: *C_GtkFrame, xalign: *c_float, yalign: *c_float) -> ();
    // pub fn gtk_frame_get_label_widget          (frame: *C_GtkFrame) -> *C_GtkWidget;
    pub fn gtk_frame_get_shadow_type           (frame: *C_GtkFrame) -> GtkShadowType;

    //=========================================================================
    // GtkAspectFrame                                                        OK
    //=========================================================================
    pub fn gtk_aspect_frame_new                (label: *c_char, xalign: c_float, yalign: c_float, ratio: c_float, obey_child: Gboolean) -> *C_GtkWidget;
    pub fn gtk_aspect_frame_set                (aspect_frame: *C_GtkAspectFrame, xalign: c_float, yalign: c_float, ratio: c_float, obey_child: Gboolean) -> ();

    //=========================================================================
    // GtkFixed                                                              OK
    //=========================================================================
    pub fn gtk_fixed_new                       () -> *C_GtkWidget;
    pub fn gtk_fixed_put                       (fixed: *C_GtkFixed, widget: *C_GtkWidget, x: c_int, y: c_int) -> ();
    pub fn gtk_fixed_move                      (fixed: *C_GtkFixed, widget: *C_GtkWidget, x: c_int, y: c_int) -> ();

    //=========================================================================
    // GtkBin                                                                
    //=========================================================================
    // pub fn gtk_bin_get_child                   (bin: *C_GtkBin) -> *C_GtkWidget;

    //=========================================================================
    // GtkSeparator                                                          OK
    //=========================================================================
    pub fn gtk_separator_new                   (orientation: GtkOrientation) -> *C_GtkWidget;

    //=========================================================================
    // GtkAdjustment                                                         OK
    //=========================================================================
    pub fn gtk_adjustment_new                  (value: c_double, lower: c_double, upper: c_double, step_increment: c_double, page_increment: c_double, page_size: c_double) -> *C_GtkAdjustment;
    pub fn gtk_adjustment_get_value            (adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_set_value            (adjustment: *C_GtkAdjustment, value: c_double) -> ();
    pub fn gtk_adjustment_clamp_page           (adjustment: *C_GtkAdjustment, lower: c_double, upper: c_double) -> ();
    pub fn gtk_adjustment_changed              (adjustment: *C_GtkAdjustment) -> ();
    pub fn gtk_adjustment_value_changed        (adjustment: *C_GtkAdjustment) -> ();
    pub fn gtk_adjustment_configure            (adjustment: *C_GtkAdjustment, value: c_double, lower: c_double, upper: c_double, tep_increment: c_double, page_increment: c_double, page_size: c_double) -> ();
    pub fn gtk_adjustment_get_lower            (adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_page_increment   (adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_page_size        (adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_step_increment   (adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_minimum_increment(adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_get_upper            (adjustment: *C_GtkAdjustment) -> c_double;
    pub fn gtk_adjustment_set_lower            (adjustment: *C_GtkAdjustment, lower: c_double) -> ();
    pub fn gtk_adjustment_set_page_increment   (adjustment: *C_GtkAdjustment, page_increment: c_double) -> ();
    pub fn gtk_adjustment_set_page_size        (adjustment: *C_GtkAdjustment, page_size: c_double) -> ();
    pub fn gtk_adjustment_set_step_increment   (adjustment: *C_GtkAdjustment, step_increment: c_double) -> ();
    pub fn gtk_adjustment_set_upper            (adjustment: *C_GtkAdjustment, upper: c_double) -> ();

    //=========================================================================
    // GtkGrid                                                         
    //=========================================================================
    pub fn gtk_grid_new                        () -> *C_GtkWidget;
    pub fn gtk_grid_attach                     (grid: *C_GtkGrid, child: *C_GtkWidget, left: c_int, top: c_int, width: c_int, height: c_int) -> ();
    pub fn gtk_grid_attach_next_to             (grid: *C_GtkGrid, child: *C_GtkWidget, sibling: *C_GtkWidget, side: GtkPositionType, width: c_int, height: c_int) -> ();
    // pub fn gtk_grid_get_child_at               (grid: *C_GtkGrid, left: c_int, top: c_int) -> *C_GtkWidget;
    pub fn gtk_grid_insert_row                 (grid: *C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_insert_column              (grid: *C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_remove_row                 (grid: *C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_remove_column              (grid: *C_GtkGrid, position: c_int) -> ();
    pub fn gtk_grid_insert_next_to             (grid: *C_GtkGrid, sibling: *C_GtkWidget, side: GtkPositionType) -> ();
    pub fn gtk_grid_set_row_homogeneous        (grid: *C_GtkGrid, homogeneous: Gboolean) -> ();
    pub fn gtk_grid_get_row_homogeneous        (grid: *C_GtkGrid) -> Gboolean;
    pub fn gtk_grid_set_row_spacing            (grid: *C_GtkGrid, spacing: c_uint) -> ();
    pub fn gtk_grid_get_row_spacing            (grid: *C_GtkGrid) -> c_uint;
    pub fn gtk_grid_set_column_homogeneous     (grid: *C_GtkGrid, homogeneous: Gboolean) -> ();
    pub fn gtk_grid_get_column_homogeneous     (grid: *C_GtkGrid) -> Gboolean;
    pub fn gtk_grid_set_column_spacing         (grid: *C_GtkGrid, spacing: c_uint) -> ();
    pub fn gtk_grid_get_column_spacing         (grid: *C_GtkGrid) -> c_uint;
    pub fn gtk_grid_get_baseline_row           (grid: *C_GtkGrid) -> c_int;
    pub fn gtk_grid_set_baseline_row           (grid: *C_GtkGrid, row: c_int) -> ();
    // pub fn gtk_grid_get_row_baseline_position  (grid: *C_GtkGrid, row: c_int);
    // pub fn gtk_grid_set_row_baseline_position  (grid: *C_GtkGrid, row: c_int, pos: GtkBaselinePosition) -> ();

    //=========================================================================
    // GtkEntryBuffer                                                        OK
    //=========================================================================
    pub fn gtk_entry_buffer_new                (initial_chars: *c_char, n_initial_chars: c_int) -> *C_GtkEntryBuffer;
    pub fn gtk_entry_buffer_get_text           (buffer: *C_GtkEntryBuffer) -> *c_char;
    pub fn gtk_entry_buffer_set_text           (buffer: *C_GtkEntryBuffer, chars: *c_char, n_chars: c_int) -> ();
    pub fn gtk_entry_buffer_get_bytes          (buffer: *C_GtkEntryBuffer) -> c_long;
    pub fn gtk_entry_buffer_get_length         (buffer: *C_GtkEntryBuffer) -> c_uint;
    pub fn gtk_entry_buffer_get_max_length     (buffer: *C_GtkEntryBuffer) -> c_int;
    pub fn gtk_entry_buffer_set_max_length     (buffer: *C_GtkEntryBuffer, max_length: c_int) -> ();
    pub fn gtk_entry_buffer_insert_text        (buffer: *C_GtkEntryBuffer, position: c_uint, chars: *c_char, n_chars: c_int);
    pub fn gtk_entry_buffer_delete_text        (buffer: *C_GtkEntryBuffer, position: c_uint, n_char: c_uint) -> c_uint;
    pub fn gtk_entry_buffer_emit_deleted_text  (buffer: *C_GtkEntryBuffer, position: c_uint, n_chars: c_uint) -> ();
    pub fn gtk_entry_buffer_emit_inserted_text (buffer: *C_GtkEntryBuffer, position: c_uint, chars: *c_char, n_chars: c_int) -> ();

    //=========================================================================
    // GtkEntry                                                        
    //=========================================================================
    pub fn gtk_entry_new                       () -> *C_GtkWidget;
    pub fn gtk_entry_new_with_buffer           (buffer: *C_GtkEntryBuffer) -> *C_GtkWidget;
    pub fn gtk_entry_get_buffer                (entry: *C_GtkEntry) -> *C_GtkEntryBuffer;
    pub fn gtk_entry_set_buffer                (entry: *C_GtkEntry, buffer: *C_GtkEntryBuffer) -> ();
    pub fn gtk_entry_set_text                  (entry: *C_GtkEntry, text: *c_char) -> ();
    pub fn gtk_entry_get_text                  (entry: *C_GtkEntry) -> *c_char;
    pub fn gtk_entry_get_text_length           (entry: *C_GtkEntry) -> c_short;
    // pub fn gtk_entry_get_text_area             (entry: *C_GtkEntry, text_area: *GdkRectangle) -> ();
    pub fn gtk_entry_set_visibility            (entry: *C_GtkEntry, visible: Gboolean) -> ();
    pub fn gtk_entry_set_invisible_char        (entry: *C_GtkEntry, ch: c_int) -> ();
    pub fn gtk_entry_unset_invisible_char      (entry: *C_GtkEntry) -> ();
    pub fn gtk_entry_set_max_length            (entry: *C_GtkEntry, max: c_int) -> ();
    pub fn gtk_entry_get_activates_default     (entry: *C_GtkEntry) -> Gboolean;
    pub fn gtk_entry_get_has_frame             (entry: *C_GtkEntry) -> Gboolean;
    // pub fn gtk_entry_get_inner_border          (entry: *C_GtkEntry) -> *C_GtkBorder;
    pub fn gtk_entry_get_width_chars           (entry: *C_GtkEntry) -> c_int;
    pub fn gtk_entry_set_activates_default     (entry: *C_GtkEntry, setting: Gboolean) -> ();
    pub fn gtk_entry_set_has_frame             (entry: *C_GtkEntry, setting: Gboolean) -> ();
    // pub fn gtk_entry_set_inner_border          (entry: *C_GtkEntry, border: *C_GtkBorder) -> ();
    pub fn gtk_entry_set_width_chars           (entry: *C_GtkEntry, n_chars: c_int) -> ();
    pub fn gtk_entry_get_invisible_char        (entry: *C_GtkEntry) -> c_uint;
    pub fn gtk_entry_set_alignment             (entry: *C_GtkEntry, xalign: c_float) -> ();
    pub fn gtk_entry_get_alignment             (entry: *C_GtkEntry) -> *c_float;
    pub fn gtk_entry_set_placeholder_text      (entry: *C_GtkEntry, text: *c_char) -> ();
    pub fn gtk_entry_get_placeholder_text      (entry: *C_GtkEntry) -> *c_char;
    pub fn gtk_entry_set_overwrite_mode        (entry: *C_GtkEntry, overwrite: Gboolean) -> ();
    pub fn gtk_entry_get_overwrite_mode        (entry: *C_GtkEntry) -> Gboolean;
    // pub fn gtk_entry_get_layout                (entry: *C_GtkEntry) -> *PangoLayout;
    pub fn gtk_entry_get_layout_offsets        (entry: *C_GtkEntry, x: *c_int,  y: *c_int) -> ();
    pub fn gtk_entry_layout_index_to_text_index(entry: *C_GtkEntry, layout_index: c_int) -> c_int;
    pub fn gtk_entry_text_index_to_layout_index(entry: *C_GtkEntry,  text_index: c_int) -> c_int;
    // pub fn gtk_entry_set_attributes            (entry: *C_GtkEntry, attrs: *PangoAttrList) -> ();
    // pub fn gtk_entry_get_attributes            (entry: *C_GtkEntry) -> *PangoAttrList;
    pub fn gtk_entry_get_max_length            (entry: *C_GtkEntry) -> c_int;
    pub fn gtk_entry_get_visibility            (entry: *C_GtkEntry) -> Gboolean;
    // pub fn gtk_entry_set_completion            (entry: *C_GtkEntry, completion: *C_GtkEntryCompletion) -> ();
    // pub fn gtk_entry_get_completion            (entry: *C_GtkEntry) -> *GtkEntryCompletion;
    pub fn gtk_entry_set_cursor_hadjustment    (entry: *C_GtkEntry, adjustment: *C_GtkAdjustment) -> ();
    pub fn gtk_entry_get_cursor_hadjustment    (entry: *C_GtkEntry) -> *C_GtkAdjustment;
    pub fn gtk_entry_set_progress_fraction     (entry: *C_GtkEntry, fraction: c_double) -> ();
    pub fn gtk_entry_get_progress_fraction     (entry: *C_GtkEntry) -> c_double;
    pub fn gtk_entry_set_progress_pulse_step   (entry: *C_GtkEntry, fraction: c_double) -> ();
    pub fn gtk_entry_get_progress_pulse_step   (entry: *C_GtkEntry) -> c_double;
    pub fn gtk_entry_progress_pulse            (entry: *C_GtkEntry) -> ();
    // pub fn gtk_entry_im_context_filter_keypress(entry: *C_GtkEntry, event: *GdkEventKey) -> Gboolean;
    pub fn gtk_entry_reset_im_context          (entry: *C_GtkEntry) -> ();
    // pub fn gtk_entry_get_tabs                  (entry: *C_GtkEntry) -> *PangoTabArray;
    // pub fn gtk_entry_set_tabs                  (entry: *C_GtkEntry, tabs: *PangoTabArray) -> ();
    // pub fn gtk_entry_set_icon_from_pixbuf      (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, pixbuf: *GdkPixbuf) -> ();
    pub fn gtk_entry_set_icon_from_stock       (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, stock_id: *c_char) -> ();
    pub fn gtk_entry_set_icon_from_icon_name   (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, icon_name: *c_char) -> ();
    // pub fn gtk_entry_set_icon_from_gicon       (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, icon: *GIcon) -> ();
    pub fn gtk_entry_get_icon_storage_type     (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> GtkImageType;
    // pub fn gtk_entry_get_icon_pixbuf           (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> *C_GdkPixbuf;
    pub fn gtk_entry_get_icon_stock            (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> *c_char;
    pub fn gtk_entry_get_icon_name             (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> *c_char;
    // pub fn gtk_entry_get_icon_gicon            (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> *GIcon;
    pub fn gtk_entry_set_icon_activatable      (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, activatable: Gboolean) -> ();
    pub fn gtk_entry_get_icon_activatable      (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> Gboolean;
    pub fn gtk_entry_set_icon_sensitive        (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, sensitive: Gboolean) -> ();
    pub fn gtk_entry_get_icon_sensitive        (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> Gboolean;
    pub fn gtk_entry_get_icon_at_pos           (entry: *C_GtkEntry, x: c_int, y: c_int) -> c_int;
    pub fn gtk_entry_set_icon_tooltip_text     (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, tooltip: *c_char) -> ();
    pub fn gtk_entry_get_icon_tooltip_text     (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> *c_char;
    pub fn gtk_entry_set_icon_tooltip_markup   (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, tooltip: *c_char) -> ();
    pub fn gtk_entry_get_icon_tooltip_markup   (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition) -> *c_char;
    // pub fn gtk_entry_set_icon_drag_source      (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, target_list: *GtkTargetList, actions: GdkDragAction) -> ();
    pub fn gtk_entry_get_current_icon_drag_source(entry: *C_GtkEntry) -> c_int;
    // pub fn gtk_entry_get_icon_area             (entry: *C_GtkEntry, icon_pos: GtkEntryIconPosition, icon_area: *GdkRectangle) -> ();
    pub fn gtk_entry_set_input_purpose         (entry: *C_GtkEntry, purpose: GtkInputPurpose) -> ();
    pub fn gtk_entry_get_input_purpose         (entry: *C_GtkEntry) -> GtkInputPurpose;
    pub fn gtk_entry_set_input_hints           (entry: *C_GtkEntry, hints: GtkInputHints) -> ();
    pub fn gtk_entry_get_input_hints           (entry: *C_GtkEntry) -> GtkInputHints;

    //=========================================================================
    // GtkSearchEntry                                                        OK
    //=========================================================================
    pub fn gtk_search_entry_new                () -> *C_GtkWidget;

    //=========================================================================
    // GtkSwitch                                                             OK
    //=========================================================================
    pub fn gtk_switch_new                      () -> *C_GtkWidget;
    pub fn gtk_switch_set_active               (switch: *C_GtkSwitch,  is_active: Gboolean) -> ();
    pub fn gtk_switch_get_active               (switch: *C_GtkSwitch) -> Gboolean;

    //=========================================================================
    // GtkScale                                                         
    //=========================================================================
    pub fn gtk_scale_new                       (orientation: GtkOrientation, adjustment: *C_GtkAdjustment) -> *C_GtkWidget;
    pub fn gtk_scale_new_with_range            (orientation: GtkOrientation, min: c_double, max: c_double, step: c_double) -> *C_GtkWidget;
    pub fn gtk_scale_set_digits                (scale: *C_GtkScale, digits: c_int) -> ();
    pub fn gtk_scale_set_draw_value            (scale: *C_GtkScale, draw_value: Gboolean) -> ();
    pub fn gtk_scale_set_has_origin            (scale: *C_GtkScale, has_origin: Gboolean) -> ();
    pub fn gtk_scale_set_value_pos             (scale: *C_GtkScale, position: GtkPositionType) -> ();
    pub fn gtk_scale_get_digits                (scale: *C_GtkScale) -> c_int;
    pub fn gtk_scale_get_draw_value            (scale: *C_GtkScale) -> Gboolean;
    pub fn gtk_scale_get_has_origin            (scale: *C_GtkScale) -> Gboolean;
    pub fn gtk_scale_get_value_pos             (scale: *C_GtkScale) -> GtkPositionType;
    // pub fn gtk_scale_get_layout                (scale: *C_GtkScale) -> *PangoLayout;
    pub fn gtk_scale_get_layout_offsets        (scale: *C_GtkScale, x: *c_int, y: *c_int) -> ();
    pub fn gtk_scale_add_mark                  (scale: *C_GtkScale, value: c_double, position: GtkPositionType, markup: *c_char) -> ();
    pub fn gtk_scale_clear_marks               (scale: *C_GtkScale) -> ();

    //=========================================================================
    // GtkLevelBar                                                        
    //=========================================================================
    pub fn gtk_level_bar_new                   () -> *C_GtkWidget;
    pub fn gtk_level_bar_new_for_interval      (min_value: c_double, max_value: c_double) -> *C_GtkWidget;
    pub fn gtk_level_bar_set_mode              (bar: *C_GtkLevelBar, mode: GtkLevelBarMode) -> ();
    pub fn gtk_level_bar_get_mode              (bar: *C_GtkLevelBar) -> GtkLevelBarMode;
    pub fn gtk_level_bar_set_value             (bar: *C_GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_value             (bar: *C_GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_min_value         (bar: *C_GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_min_value         (bar: *C_GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_max_value         (bar: *C_GtkLevelBar, value: c_double) -> ();
    pub fn gtk_level_bar_get_max_value         (bar: *C_GtkLevelBar) -> c_double;
    pub fn gtk_level_bar_set_inverted          (bar: *C_GtkLevelBar, inverted: Gboolean) -> ();
    pub fn gtk_level_bar_get_inverted          (bar: *C_GtkLevelBar) -> Gboolean;
    pub fn gtk_level_bar_add_offset_value      (bar: *C_GtkLevelBar, name: *c_char, value: c_double) -> ();
    pub fn gtk_level_bar_remove_offset_value   (bar: *C_GtkLevelBar, name: *c_char) -> ();
    pub fn gtk_level_bar_get_offset_value      (bar: *C_GtkLevelBar, name: *c_char, value: *c_double) -> Gboolean;

    //=========================================================================
    // GtkSearchBar                                                        
    //=========================================================================
    pub fn gtk_search_bar_new                  () -> *C_GtkWidget;
    pub fn gtk_search_bar_connect_entry        (bar: *C_GtkSearchBar, entry: *C_GtkEntry) -> ();
    pub fn gtk_search_bar_get_search_mode      (bar: *C_GtkSearchBar) -> Gboolean;
    pub fn gtk_search_bar_set_search_mode      (bar: *C_GtkSearchBar, search_mode: Gboolean) -> ();
    pub fn gtk_search_bar_get_show_close_button(bar: *C_GtkSearchBar) -> Gboolean;
    pub fn gtk_search_bar_set_show_close_button(bar: *C_GtkSearchBar, visible: Gboolean) -> ();
    // pub fn gtk_search_bar_handle_event         (bar: *C_GtkSearchBar, event: *GdkEvent) -> Gboolean;

    //=========================================================================
    // GtkSpinButton                                                       
    //=========================================================================
    pub fn gtk_spin_button_configure           (spin_button: *C_GtkSpinButton, adjustment: *C_GtkAdjustment, climb_rate: c_double, digits: c_uint) -> ();
    pub fn gtk_spin_button_new                 (adjustment: *C_GtkAdjustment, climb_rate: c_double, digits: c_uint) -> *C_GtkWidget;
    pub fn gtk_spin_button_new_with_range      (min: c_double, max: c_double, step: c_double) -> *C_GtkWidget;
    pub fn gtk_spin_button_set_adjustment      (spin_button: *C_GtkSpinButton, adjustment: *C_GtkAdjustment) -> ();
    pub fn gtk_spin_button_get_adjustment      (spin_button: *C_GtkSpinButton) -> *C_GtkAdjustment;
    pub fn gtk_spin_button_set_digits          (spin_button: *C_GtkSpinButton, digits: c_uint) -> ();
    pub fn gtk_spin_button_set_increments      (spin_button: *C_GtkSpinButton, step: c_double, page: c_double) -> ();
    pub fn gtk_spin_button_set_range           (spin_button: *C_GtkSpinButton, min: c_double, max: c_double);
    pub fn gtk_spin_button_get_value_as_int    (spin_button: *C_GtkSpinButton) -> c_int;
    pub fn gtk_spin_button_set_value           (spin_button: *C_GtkSpinButton, value: c_double) -> ();
    pub fn gtk_spin_button_set_update_policy   (spin_button: *C_GtkSpinButton, policy: GtkSpinButtonUpdatePolicy) -> ();
    pub fn gtk_spin_button_set_numeric         (spin_button: *C_GtkSpinButton, gnumeric: Gboolean) -> ();
    pub fn gtk_spin_button_spin                (spin_button: *C_GtkSpinButton, direction: GtkSpinType, increment: c_double) -> ();
    pub fn gtk_spin_button_set_wrap            (spin_button: *C_GtkSpinButton, wrap: Gboolean) -> ();
    pub fn gtk_spin_button_set_snap_to_ticks   (spin_button: *C_GtkSpinButton, snap_to_ticks: Gboolean) -> ();
    pub fn gtk_spin_button_update              (spin_button: *C_GtkSpinButton) -> ();
    pub fn gtk_spin_button_get_digits          (spin_button: *C_GtkSpinButton) -> c_uint;
    pub fn gtk_spin_button_get_increments      (spin_button: *C_GtkSpinButton, step: *c_double, page: *c_double) -> ();
    pub fn gtk_spin_button_get_numeric         (spin_button: *C_GtkSpinButton) -> Gboolean;
    pub fn gtk_spin_button_get_range           (spin_button: *C_GtkSpinButton, min: *c_double, max: *c_double) -> ();
    pub fn gtk_spin_button_get_snap_to_ticks   (spin_button: *C_GtkSpinButton) -> Gboolean;
    pub fn gtk_spin_button_get_update_policy   (spin_button: *C_GtkSpinButton) -> GtkSpinButtonUpdatePolicy;
    pub fn gtk_spin_button_get_value           (spin_button: *C_GtkSpinButton) -> c_double;
    pub fn gtk_spin_button_get_wrap            (spin_button: *C_GtkSpinButton) -> Gboolean;

    //=========================================================================
    // GtkSpinner                                                            OK
    //=========================================================================
    pub fn gtk_spinner_new                     () -> *C_GtkWidget;
    pub fn gtk_spinner_start                   (spinner: *C_GtkSpinner) -> ();
    pub fn gtk_spinner_stop                    (spinner: *C_GtkSpinner) -> ();

    //=========================================================================
    // GtkImage                                                           
    //=========================================================================
    pub fn gtk_image_new_from_file             (filename: *c_char) -> *C_GtkWidget;

    //=========================================================================
    // GtkProgressBar                                                          
    //=========================================================================
    pub fn gtk_progress_bar_new                () -> *C_GtkWidget;
    pub fn gtk_progress_bar_pulse              (pbar: *C_GtkProgressBar) -> ();
    pub fn gtk_progress_bar_set_fraction       (pbar: *C_GtkProgressBar, fraction: c_double) -> ();
    pub fn gtk_progress_bar_get_fraction       (pbar: *C_GtkProgressBar) -> c_double;
    pub fn gtk_progress_bar_set_inverted       (pbar: *C_GtkProgressBar, inverted: Gboolean) -> ();
    pub fn gtk_progress_bar_get_inverted       (pbar: *C_GtkProgressBar) -> Gboolean;
    pub fn gtk_progress_bar_set_show_text      (pbar: *C_GtkProgressBar, show_text: Gboolean) -> ();
    pub fn gtk_progress_bar_get_show_text      (pbar: *C_GtkProgressBar) -> Gboolean;
    pub fn gtk_progress_bar_set_text           (pbar: *C_GtkProgressBar, text: *c_char) -> ();
    pub fn gtk_progress_bar_get_text           (pbar: *C_GtkProgressBar) -> *c_char;
    // pub fn gtk_progress_bar_set_ellipsize      (pbar: *C_GtkProgressBar, mode: PangoEllipsizeMode) -> ();
    // pub fn gtk_progress_bar_get_ellipsize      (pbar: *C_GtkProgressBar) -> PangoEllipsizeMode
    pub fn gtk_progress_bar_set_pulse_step     (pbar: *C_GtkProgressBar, fraction: c_double) -> ();
    pub fn gtk_progress_bar_get_pulse_step     (pbar: *C_GtkProgressBar) -> c_double;

    //=========================================================================
    // GtkArrow                                                              OK
    //=========================================================================
    pub fn gtk_arrow_new                       (arrow_type: GtkArrowType, shadow_type: GtkShadowType) -> *C_GtkWidget;
    pub fn gtk_arrow_set                       (arrow: *C_GtkArrow,arrow_type: GtkArrowType, shadow_type: GtkShadowType) -> ();

    //=========================================================================
    // GtkCalendar                                                
    //=========================================================================
    pub fn gtk_calendar_new                    () -> *C_GtkWidget;
    pub fn gtk_calendar_select_month           (calendar: *C_GtkCalendar, month: c_uint, year: c_uint) -> ();
    pub fn gtk_calendar_select_day             (calendar: *C_GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_mark_day               (calendar: *C_GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_unmark_day             (calendar: *C_GtkCalendar, day: c_uint) -> ();
    pub fn gtk_calendar_get_day_is_marked      (calendar: *C_GtkCalendar, day: c_uint) -> Gboolean;
    pub fn gtk_calendar_clear_marks            (calendar: *C_GtkCalendar) -> ();
    pub fn gtk_calendar_get_display_options    (calendar: *C_GtkCalendar) -> GtkCalendarDisplayOptions;
    pub fn gtk_calendar_set_display_options    (calendar: *C_GtkCalendar, flags: GtkCalendarDisplayOptions) -> ();
    pub fn gtk_calendar_get_date               (calendar: *C_GtkCalendar, year: *c_uint, month: *c_uint, day: *c_uint) -> ();
    // pub fn gtk_calendar_set_detail_func        (calendar: *C_GtkCalendar, GtkCalendarDetailFunc func, gpointer data, GDestroyNotify destroy) -> ();
    pub fn gtk_calendar_get_detail_width_chars (calendar: *C_GtkCalendar) -> c_int;
    pub fn gtk_calendar_set_detail_width_chars (calendar: *C_GtkCalendar, chars: c_int) -> ();
    pub fn gtk_calendar_get_detail_height_rows (calendar: *C_GtkCalendar) -> c_int;
    pub fn gtk_calendar_set_detail_height_rows (calendar: *C_GtkCalendar, rows: c_int) -> ();

    //=========================================================================
    // GtkAlignments                                                         OK
    //=========================================================================
    pub fn gtk_alignment_new                   (xalign: c_float, yalign: c_float, xscale: c_float, yscale: c_float) -> *C_GtkWidget;
    pub fn gtk_alignment_set                   (alignment: *C_GtkAlignment, xalign: c_float, yalign: c_float, xscale: c_float, yscale: c_float) -> ();
    pub fn gtk_alignment_get_padding           (alignment: *C_GtkAlignment, padding_top: *c_uint, padding_bottom: *c_uint, padding_left: *c_uint, padding_right: *c_uint) -> ();
    pub fn gtk_alignment_set_padding           (alignment: *C_GtkAlignment, padding_top: c_uint, padding_bottom: c_uint, padding_left: c_uint, padding_right: c_uint) -> ();

    //=========================================================================
    // GtkExpander                                                           OK
    //=========================================================================
    pub fn gtk_expander_new                    (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_expander_new_with_mnemonic      (label: *c_char) -> *C_GtkWidget;
    pub fn gtk_expander_set_expanded           (expander: *C_GtkExpander, expanded: Gboolean) -> ();
    pub fn gtk_expander_get_expanded           (expander: *C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_spacing            (expander: *C_GtkExpander, spacing: c_int) -> ();
    pub fn gtk_expander_get_spacing            (expander: *C_GtkExpander) -> c_int;
    pub fn gtk_expander_set_label              (expander: *C_GtkExpander, label: *c_char) -> ();
    pub fn gtk_expander_get_label              (expander: *C_GtkExpander) -> *c_char;
    pub fn gtk_expander_set_use_underline      (expander: *C_GtkExpander, use_underline: Gboolean)-> ();
    pub fn gtk_expander_get_use_underline      (expander: *C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_use_markup         (expander: *C_GtkExpander, use_markup: Gboolean) -> ();
    pub fn gtk_expander_get_use_markup         (expander: *C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_label_widget       (expander: *C_GtkExpander, label_widget: *C_GtkWidget) -> ();
    pub fn gtk_expander_get_label_widget       (expander: *C_GtkExpander) -> *C_GtkWidget;
    pub fn gtk_expander_set_label_fill         (expander: *C_GtkExpander, label_fill: Gboolean) -> ();
    pub fn gtk_expander_get_label_fill         (expander: *C_GtkExpander) -> Gboolean;
    pub fn gtk_expander_set_resize_toplevel    (expander: *C_GtkExpander, resize_toplevel: Gboolean) -> ();
    pub fn gtk_expander_get_resize_toplevel    (expander: *C_GtkExpander) -> Gboolean;

    //=========================================================================
    // GtkPaned                                                              OK
    //=========================================================================
    pub fn gtk_paned_new                       (orientation: GtkOrientation) -> *C_GtkWidget;
    pub fn gtk_paned_add1                      (paned: *C_GtkPaned, child: *C_GtkWidget) -> ();
    pub fn gtk_paned_add2                      (paned: *C_GtkPaned, child: *C_GtkWidget) -> ();
    pub fn gtk_paned_pack1                     (paned: *C_GtkPaned, child: *C_GtkWidget, resize: Gboolean, schrink: Gboolean) -> ();
    pub fn gtk_paned_pack2                     (paned: *C_GtkPaned, child: *C_GtkWidget, resize: Gboolean, schrink: Gboolean) -> ();
    // pub fn gtk_paned_get_child1                (paned: *C_GtkPaned) -> *C_GtkWidget;
    // pub fn gtk_paned_get_child2                (paned: *C_GtkPaned) -> *C_GtkWidget;
    pub fn gtk_paned_set_position              (paned: *C_GtkPaned, position: c_int) -> ();
    pub fn gtk_paned_get_position              (paned: *C_GtkPaned) -> c_int;
    pub fn gtk_paned_get_handle_window         (paned: *C_GtkPaned) -> *C_GtkWidget;

    //=========================================================================
    // GtkInfoBar
    //=========================================================================
    pub fn gtk_info_bar_new                    () -> *C_GtkWidget;
    // pub fn gtk_info_bar_new_with_buttons       (const gchar *first_button_text, ...) -> *C_GtkWidget;
    pub fn gtk_info_bar_add_action_widget      (info_bar: *C_GtkInfoBar, child: *C_GtkWidget, response_id: c_int);
    pub fn gtk_info_bar_add_button             (info_bar: *C_GtkInfoBar, button_text: *c_char, response_id: c_int) -> *C_GtkWidget;
    // pub fn gtk_info_bar_add_buttons            (GtkInfoBar *info_bar, const gchar *first_button_text, ...) -> ();
    pub fn gtk_info_bar_set_response_sensitive (info_bar: *C_GtkInfoBar, response_id: c_int, setting: Gboolean) -> ();
    pub fn gtk_info_bar_set_default_response   (info_bar: *C_GtkInfoBar, response_id: c_int) -> ();
    pub fn gtk_info_bar_response               (info_bar: *C_GtkInfoBar, response_id: c_int) -> ();
    pub fn gtk_info_bar_set_message_type       (info_bar: *C_GtkInfoBar, message_type: GtkMessageType) -> ();
    pub fn gtk_info_bar_get_message_type       (info_bar: *C_GtkInfoBar) -> GtkMessageType;
    // pub fn gtk_info_bar_get_action_area        (info_bar: *C_GtkInfoBar) -> *C_GtkWidget;
    // pub fn gtk_info_bar_get_content_area       (info_bar: *C_GtkInfoBar) -> *C_GtkWidget;
    pub fn gtk_info_bar_get_show_close_button  (info_bar: *C_GtkInfoBar) -> Gboolean;
    pub fn gtk_info_bar_set_show_close_button  (info_bar: *C_GtkInfoBar, setting: Gboolean) -> ();

    //=========================================================================
    // Glu fixe code
    //=========================================================================
    pub fn signal_connect(g_object: *C_GtkWidget, signal: *c_char, func: Option<fn()>);
    pub fn signal_connect_2params(g_object: *C_GtkWidget, signal: *c_char, func: Option<extern "C" fn(*C_GtkWidget, *c_void)>, params: *c_void);

    //=========================================================================
    // GTK Casts functions
    //=========================================================================
    pub fn cast_GtkWindow(widget: *C_GtkWidget) -> *C_GtkWindow;
    pub fn cast_GtkBin(widget: *C_GtkWidget) -> *C_GtkBin;
    pub fn cast_GtkButton(widget: *C_GtkWidget) -> *C_GtkButton;
    pub fn cast_GtkContainer(widget: *C_GtkWidget) -> *C_GtkContainer;
    pub fn cast_GtkFrame(widget: *C_GtkWidget) -> *C_GtkFrame;
    pub fn cast_GtkLabel(widget: *C_GtkWidget) -> *C_GtkLabel;
    pub fn cast_GtkMisc(widget: *C_GtkWidget) -> *C_GtkMisc;
    pub fn cast_GtkOrientable(widget: *C_GtkWidget) -> *C_GtkOrientable;
    pub fn cast_GtkBox(widget: *C_GtkWidget) -> *C_GtkBox;
    pub fn cast_GtkFixed(widget: *C_GtkWidget) -> *C_GtkFixed;
    pub fn cast_GtkButtonBox(widget: *C_GtkWidget) -> *C_GtkButtonBox;
    pub fn cast_GtkAspectFrame(widget: *C_GtkWidget) -> *C_GtkAspectFrame;
    pub fn cast_GtkFontButton(widget: *C_GtkWidget) -> *C_GtkFontButton;
    pub fn cast_GtkToggleButton(widget: *C_GtkWidget) -> *C_GtkToggleButton;
    pub fn cast_GtkCheckButton(widget: *C_GtkWidget) -> *C_GtkCheckButton;
    pub fn cast_GtkMenuButton(widget: *C_GtkWidget) -> *C_GtkMenuButton;
    pub fn cast_GtkColorButton(widget: *C_GtkWidget) -> *C_GtkColorButton;
    pub fn cast_GtkLinkButton(widget: *C_GtkWidget) -> *C_GtkLinkButton;
    pub fn cast_GtkScaleButton(widget: *C_GtkWidget) -> *C_GtkScaleButton;
    pub fn cast_GtkGrid(widget: *C_GtkWidget) -> *C_GtkGrid;
    pub fn cast_GtkEntry(widget: *C_GtkWidget) -> *C_GtkEntry;
    pub fn cast_GtkSwitch(widget: *C_GtkWidget) -> *C_GtkSwitch;
    pub fn cast_GtkScale(widget: *C_GtkWidget) -> *C_GtkScale;
    pub fn cast_GtkLevelBar(widget: *C_GtkWidget) -> *C_GtkLevelBar;
    pub fn cast_GtkSearchBar(widget: *C_GtkWidget) -> *C_GtkSearchBar;
    pub fn cast_GtkSpinButton(widget: *C_GtkWidget) -> *C_GtkSpinButton;
    pub fn cast_GtkSpinner(widget: *C_GtkWidget) -> *C_GtkSpinner;
    pub fn cast_GtkProgressBar(widget: *C_GtkWidget) -> *C_GtkProgressBar;
    pub fn cast_GtkArrow(widget: *C_GtkWidget) -> *C_GtkArrow;
    pub fn cast_GtkCalendar(widget: *C_GtkWidget) -> *C_GtkCalendar;
    pub fn cast_GtkAlignment(widget: *C_GtkWidget) -> *C_GtkAlignment;
    pub fn cast_GtkExpander(widget: *C_GtkWidget) -> *C_GtkExpander;
    pub fn cast_GtkPaned(widget: *C_GtkWidget) -> *C_GtkPaned;
    pub fn cast_GtkInfoBar(widget: *C_GtkWidget) -> *C_GtkInfoBar;
}


