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

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib_ffi;

pub mod enums;

use libc::{c_int, c_char, c_double, c_void, c_uint, c_uchar, c_ulong};
use glib_ffi::Gboolean;

#[repr(C)]
#[derive(Copy)]
pub struct C_GdkWindow;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDisplay;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDisplayManager;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkScreen;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkVisual;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkEvent;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkRectangle { // FIXME should be just an alias to cairo_rectangle_int_t
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int
}
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkFrameClock;
/// The Color structure is used to describe a color, similar to the XColor struct used in the X11 drawing API.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct C_GdkColor {
    /// For allocated colors, the pixel value used to draw this color on the screen. Not used anymore.
    pub pixel:  u32,
    /// The red component of the color. This is a value between 0 and 65535, with 65535 indicating full intensity
    pub red:    u16,
    /// The green component of the color
    pub green:  u16,
    /// The blue component of the color
    pub blue:   u16
}
/// The GdkRGBA structure is used to represent a (possibly translucent) color, in a way that is compatible with cairos notion of color.
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkRGBA {
    /// The intensity of the red channel from 0.0 to 1.0 inclusive
    pub red: f64,
    /// The intensity of the green channel from 0.0 to 1.0 inclusive
    pub green: f64,
    /// The intensity of the blue channel from 0.0 to 1.0 inclusive
    pub blue: f64,
    /// The opacity of the color from 0.0 for completely translucent to 1.0 for opaque
    pub alpha: f64
}
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkCursor;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkGeometry {
    /// minimum width of window (or -1 to use requisition, with GtkWindow only)
    pub min_width: c_int,
    /// minimum height of window (or -1 to use requisition, with GtkWindow only)
    pub min_height: c_int,
    /// maximum width of window (or -1 to use requisition, with GtkWindow only)
    pub max_width: c_int,
    /// maximum height of window (or -1 to use requisition, with GtkWindow only)
    pub max_height: c_int,
    /// allowed window widths are base_width + width_inc * N where N is any integer (-1 allowed with GtkWindow)
    pub base_width: c_int,
    /// allowed window widths are base_height + height_inc * N where N is any integer (-1 allowed with GtkWindow)
    pub base_height: c_int,
    /// width resize increment
    pub width_inc: c_int,
    /// height resize increment
    pub height_inc: c_int,
    /// minimum width/height ratio
    pub min_aspect: f64,
    /// maximum width/height ratio
    pub max_aspect: f64,
    /// window gravity, see gtk_window_set_gravity()
    pub win_gravity: enums::Gravity,
}
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDevice;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkTimeCoord;
pub type C_GdkAtom = *mut c_void;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDeviceManager;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkAppLaunchContext;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkPixbuf;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkFrameTimings;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkWindowAttr {
    pub title: *const c_char,
    pub event_mask: c_int,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub wclass: enums::WindowWindowClass,
    pub visual: *mut C_GdkVisual,
    pub window_type: enums::WindowType,
    pub cursor: *mut C_GdkCursor,
    pub wmclass_name: *const c_char,
    pub wmclass_class: *const c_char,
    pub override_redirect: Gboolean,
    pub type_hint: enums::WindowTypeHint
}
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDragContext;

// GdkWindowAttributesTypes
/// Honor the title field
pub const GDK_WA_TITLE: i32 = 1 << 1;
/// Honor the X coordinate field
pub const GDK_WA_X: i32 = 1 << 2;
/// Honor the Y coordinate field
pub const GDK_WA_Y: i32 = 1 << 3;
/// Honor the cursor field
pub const GDK_WA_CURSOR: i32 = 1 << 4;
/// Honor the visual field
pub const GDK_WA_VISUAL: i32 = 1 << 5;
// Deprecated
//const GDK_WA_WMCLASS: i32 = 1 << 6;
/// Honor the override_redirect field
pub const GDK_WA_NOREDIR: i32 = 1 << 7;
/// Honor the type_hint field
pub const GDK_WA_TYPE_HINT: i32 = 1 << 8;

extern "C" {
    //=========================================================================
    // General                                                           NOT OK
    //=========================================================================
    pub fn gdk_init                      (argc: *mut c_int, argv: *mut *mut *mut c_char);
    pub fn gdk_init_check                (argc: *mut c_int, argv: *mut *mut *mut c_char) -> Gboolean;
    pub fn gdk_parse_args                (argc: *mut c_int, argv: *mut *mut *mut c_char);
    pub fn gdk_get_display_arg_name      () -> *const c_char;
    pub fn gdk_notify_startup_complete   ();
    pub fn gdk_notify_startup_complete_with_id(startup_id: *const c_char);
    pub fn gdk_set_allowed_backends      (backends: *const c_char);
    pub fn gdk_get_program_class         () -> *const c_char;
    pub fn gdk_set_program_class         (program_class: *const c_char);
    pub fn gdk_flush                     ();
    pub fn gdk_screen_width              () -> c_int;
    pub fn gdk_screen_height             () -> c_int;
    pub fn gdk_screen_width_mm           () -> c_int;
    pub fn gdk_screen_height_mm          () -> c_int;
    pub fn gdk_set_double_click_time     (msec: c_uint);
    pub fn gdk_beep                      ();
    pub fn gdk_error_trap_push           ();
    pub fn gdk_error_trap_pop            ();
    pub fn gdk_error_trap_pop_ignored    ();

    //=========================================================================
    // GdkWindow                                                         NOT OK
    //=========================================================================
    pub fn gdk_window_new                (parent: *mut C_GdkWindow, attributes: *mut C_GdkWindowAttr,
        attributes_mask: c_int) -> *mut C_GdkWindow;
    pub fn gdk_window_destroy            (window: *mut C_GdkWindow);
    pub fn gdk_window_get_window_type    (window: *mut C_GdkWindow) -> enums::WindowType;
    pub fn gdk_window_get_display        (window: *mut C_GdkWindow) -> *mut C_GdkDisplay;
    pub fn gdk_window_get_screen         (window: *mut C_GdkWindow) -> *mut C_GdkScreen;
    pub fn gdk_window_get_visual         (window: *mut C_GdkWindow) -> *mut C_GdkVisual;
    pub fn gdk_window_show               (window: *mut C_GdkWindow);
    pub fn gdk_window_show_unraised      (window: *mut C_GdkWindow);
    pub fn gdk_window_hide               (window: *mut C_GdkWindow);
    pub fn gdk_window_is_destroyed       (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_is_visible         (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_is_viewable        (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_is_input_only      (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_is_shaped          (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_get_state          (window: *mut C_GdkWindow) -> enums::WindowState;
    pub fn gdk_window_withdraw           (window: *mut C_GdkWindow);
    pub fn gdk_window_iconify            (window: *mut C_GdkWindow);
    pub fn gdk_window_deiconify          (window: *mut C_GdkWindow);
    pub fn gdk_window_stick              (window: *mut C_GdkWindow);
    pub fn gdk_window_unstick            (window: *mut C_GdkWindow);
    pub fn gdk_window_maximize           (window: *mut C_GdkWindow);
    pub fn gdk_window_unmaximize         (window: *mut C_GdkWindow);
    pub fn gdk_window_fullscreen         (window: *mut C_GdkWindow);
    pub fn gdk_window_unfullscreen       (window: *mut C_GdkWindow);
    pub fn gdk_window_get_fullscreen_mode(window: *mut C_GdkWindow) -> enums::FullscreenMode;
    pub fn gdk_window_set_fullscreen_mode(window: *mut C_GdkWindow, mode: enums::FullscreenMode);
    pub fn gdk_window_set_keep_above     (window: *mut C_GdkWindow, setting: Gboolean);
    pub fn gdk_window_set_keep_below     (window: *mut C_GdkWindow, setting: Gboolean);
    pub fn gdk_window_set_opacity        (window: *mut C_GdkWindow, opacity: c_double);
    pub fn gdk_window_set_composited     (window: *mut C_GdkWindow, composited: Gboolean);
    pub fn gdk_window_get_composited     (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_move               (window: *mut C_GdkWindow, x: c_int, y: c_int);
    pub fn gdk_window_resize             (window: *mut C_GdkWindow, width: c_int, height: c_int);
    pub fn gdk_window_move_resize        (window: *mut C_GdkWindow, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn gdk_window_scroll             (window: *mut C_GdkWindow, dx: c_int, dy: c_int);
    //pub fn gdk_window_move_region        (window: *mut C_GdkWindow, region: *mut cairo_region_t, dx: c_int, dy: c_int);
    pub fn gdk_window_has_native         (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_ensure_native      (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_reparent           (window: *mut C_GdkWindow, new_parent: *mut C_GdkWindow, x: c_int, y: c_int);
    pub fn gdk_window_raise              (window: *mut C_GdkWindow);
    pub fn gdk_window_lower              (window: *mut C_GdkWindow);
    pub fn gdk_window_restack            (window: *mut C_GdkWindow, sibling: *mut C_GdkWindow, above: Gboolean);
    pub fn gdk_window_focus              (window: *mut C_GdkWindow, timestamp: u32);
    pub fn gdk_window_register_dnd       (window: *mut C_GdkWindow);
    pub fn gdk_window_begin_resize_drag  (window: *mut C_GdkWindow, edge: enums::WindowEdge, button: c_int, root_x: c_int, root_y: c_int,
        timestamp: u32);
    pub fn gdk_window_begin_resize_drag_for_device(window: *mut C_GdkWindow, edge: enums::WindowEdge, device: *mut C_GdkDevice,
        button: c_int, root_x: c_int, root_y: c_int, timestamp: u32);
    pub fn gdk_window_begin_move_drag    (window: *mut C_GdkWindow, button: c_int, root_x: c_int, root_y: c_int, timestamp: u32);
    pub fn gdk_window_begin_move_drag_for_device(window: *mut C_GdkWindow, device: *mut C_GdkDevice, button: c_int, root_x: c_int,
        root_y: c_int, timestamp: u32);
    pub fn gdk_window_show_window_menu   (window: *mut C_GdkWindow, event: *mut C_GdkEvent);
    pub fn gdk_window_constrain_size     (window: *mut C_GdkWindow, flags: enums::WindowHints, width: c_int, height: c_int,
        new_width: *mut c_int, new_height: *mut c_int);
    pub fn gdk_window_beep               (window: *mut C_GdkWindow);
    pub fn gdk_window_get_scale_factor   (window: *mut C_GdkWindow) -> c_int;
    //pub fn gdk_window_set_opaque_region  (window: *mut C_GdkWindow, region: *mut cairo_region_t);
    //pub fn gdk_window_get_clip_region    (window: *mut C_GdkWindow) -> *mut cairo_region_t;
    pub fn gdk_window_begin_paint_rect   (window: *mut C_GdkWindow, rectangle: *const C_GdkRectangle);
    //pub fn gdk_window_begin_paint_region (window: *mut C_GdkWindow, region: *const cairo_region_t);
    pub fn gdk_window_end_paint          (window: *mut C_GdkWindow);
    //pub fn gdk_window_get_visible_region (window: *mut C_GdkWindow) -> *mut cairo_region_t;
    //pub fn gdk_window_set_invalidate_handler(window: *mut C_GdkWindow, handler: GdkWindowInvalidateHandlerFunc);
    pub fn gdk_window_invalidate_rect    (window: *mut C_GdkWindow, rectangle: *const C_GdkRectangle, invalidate_children: Gboolean);
    //pub fn gdk_window_invalidate_region  (window: *mut C_GdkWindow, region: *const cairo_region_t, invalidate_children: Gboolean);
    //pub fn gdk_window_invalidate_maybe_recurse(window: *mut C_GdkWindow, region: *const cairo_region_t, child_func: GdkWindowChildFunc,
    //    user_data: *mut c_void);
    //pub fn gdk_window_get_update_area    (window: *mut C_GdkWindow) -> *mut cairo_region_t;
    pub fn gdk_window_freeze_updates     (window: *mut C_GdkWindow);
    pub fn gdk_window_thaw_updates       (window: *mut C_GdkWindow);
    pub fn gdk_window_process_all_updates();
    pub fn gdk_window_process_updates    (window: *mut C_GdkWindow, update_children: Gboolean);
    pub fn gdk_window_set_debug_updates  (setting: Gboolean);
    pub fn gdk_window_get_frame_clock    (window: *mut C_GdkWindow) -> *mut C_GdkFrameClock;
    pub fn gdk_window_set_user_data      (window: *mut C_GdkWindow, user_data: *mut c_void);
    pub fn gdk_window_set_override_redirect(window: *mut C_GdkWindow, override_redirect: Gboolean);
    pub fn gdk_window_set_accept_focus   (window: *mut C_GdkWindow, accept_focus: Gboolean);
    pub fn gdk_window_get_accept_focus   (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_focus_on_map   (window: *mut C_GdkWindow, focus_on_map: Gboolean);
    pub fn gdk_window_get_focus_on_map   (window: *mut C_GdkWindow) -> Gboolean;
    //pub fn gdk_window_add_filter         (window: *mut C_GdkWindow, function: GdkFilterFunc, data: *mut c_void);
    //pub fn gdk_window_remove_filter      (window: *mut C_GdkWindow, function: GdkFilterFunc, data: *mut c_void);
    //pub fn gdk_window_shape_combine_region(window: *mut C_GdkWindow, shape_region: *const cairo_region_t, offset_x: c_int,
    //    offset_y: c_int);
    pub fn gdk_window_set_child_shapes   (window: *mut C_GdkWindow);
    pub fn gdk_window_merge_child_shapes (window: *mut C_GdkWindow);
    //pub fn gdk_window_input_shape_combine_region(window: *mut C_GdkWindow, shape_region: *const cairo_region_t, offset_x: c_int,
    //    offset_y: c_int);
    pub fn gdk_window_set_child_input_shapes(window: *mut C_GdkWindow);
    pub fn gdk_window_merge_child_input_shapes(window: *mut C_GdkWindow);
    pub fn gdk_window_set_static_gravities(window: *mut C_GdkWindow, use_static: Gboolean) -> Gboolean;
    pub fn gdk_window_set_title          (window: *mut C_GdkWindow, title: *const c_char);
    pub fn gdk_window_set_background_rgba(window: *mut C_GdkWindow, rgba: *const C_GdkRGBA);
    //pub fn gdk_window_set_background_pattern(window: *mut C_GdkWindow, pattern: *const cairo_pattern_t);
    //pub fn gdk_window_get_background_pattern(window: *mut C_GdkWindow) -> *const cairo_pattern_t;
    pub fn gdk_window_set_cursor         (window: *mut C_GdkWindow, cursor: *mut C_GdkCursor);
    pub fn gdk_window_get_cursor         (window: *mut C_GdkWindow) -> *mut C_GdkCursor;
    pub fn gdk_window_get_user_data      (window: *mut C_GdkWindow, data: *mut *mut c_void);
    pub fn gdk_window_get_geometry       (window: *mut C_GdkWindow, x: *mut c_int, y: *mut c_int, width: *mut c_int, height: *mut c_int);
    pub fn gdk_window_set_geometry_hints (window: *mut C_GdkWindow, geometry: *const C_GdkGeometry, geom_mask: enums::WindowHints);
    pub fn gdk_window_get_width          (window: *mut C_GdkWindow) -> c_int;
    pub fn gdk_window_get_height         (window: *mut C_GdkWindow) -> c_int;
    //pub fn gdk_window_set_icon_list      (window: *mut C_GdkWindow, pixbufs: *mut GList);
    pub fn gdk_window_set_modal_hint     (window: *mut C_GdkWindow, modal: Gboolean);
    pub fn gdk_window_get_modal_hint     (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_type_hint      (window: *mut C_GdkWindow, hint: enums::WindowTypeHint);
    pub fn gdk_window_get_type_hint      (window: *mut C_GdkWindow) -> enums::WindowTypeHint;
    pub fn gdk_window_set_shadow_width   (window: *mut C_GdkWindow, left: c_int, right: c_int, top: c_int, bottom: c_int);
    pub fn gdk_window_set_skip_taskbar_hint(window: *mut C_GdkWindow, skips_taskbar: Gboolean);
    pub fn gdk_window_set_skip_pager_hint(window: *mut C_GdkWindow, skips_pager: Gboolean);
    pub fn gdk_window_set_urgency_hint   (window: *mut C_GdkWindow, urgent: Gboolean);
    pub fn gdk_window_get_position       (window: *mut C_GdkWindow, x: *mut c_int, y: *mut c_int);
    pub fn gdk_window_get_root_origin    (window: *mut C_GdkWindow, x: *mut c_int, y: *mut c_int);
    pub fn gdk_window_get_frame_extents  (window: *mut C_GdkWindow, rect: *mut C_GdkRectangle);
    pub fn gdk_window_get_origin         (window: *mut C_GdkWindow, x: *mut c_int, y: *mut c_int);
    pub fn gdk_window_get_root_coords    (window: *mut C_GdkWindow, x: c_int, y: c_int, root_x: *mut c_int, root_y: *mut c_int);
    pub fn gdk_window_get_device_position(window: *mut C_GdkWindow, device: *mut C_GdkDevice, x: *mut c_int, y: *mut c_int,
        mask: *mut enums::modifier_type::ModifierType) -> *mut C_GdkWindow;
    pub fn gdk_window_get_device_position_double(window: *mut C_GdkWindow, device: *mut C_GdkDevice, x: *mut c_double, y: *mut c_double,
        mask: *mut enums::modifier_type::ModifierType) -> *mut C_GdkWindow;
    pub fn gdk_window_get_parent         (window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_get_toplevel       (window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    //pub fn gdk_window_get_children       (window: *mut C_GdkWindow) -> *mut GList;
    //pub fn gdk_window_get_children_with_user_data(window: *mut C_GdkWindow, user_data: *mut c_void) -> *mut GList;
    //pub fn gdk_window_peek_children      (window: *mut C_GdkWindow) -> *mut GList;
    pub fn gdk_window_get_events         (window: *mut C_GdkWindow) -> enums::EventMask;
    pub fn gdk_window_set_events         (window: *mut C_GdkWindow, event_mask: enums::EventMask);
    pub fn gdk_window_set_icon_name      (window: *mut C_GdkWindow, name: *const c_char);
    pub fn gdk_window_set_transient_for  (window: *mut C_GdkWindow, parent: *mut C_GdkWindow);
    pub fn gdk_window_set_role           (window: *mut C_GdkWindow, role: *const c_char);
    pub fn gdk_window_set_startup_id     (window: *mut C_GdkWindow, startup_id: *const c_char);
    pub fn gdk_window_set_group          (window: *mut C_GdkWindow, leader: *mut C_GdkWindow);
    pub fn gdk_window_get_group          (window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_set_decorations    (window: *mut C_GdkWindow, decorations: enums::WMDecoration);
    pub fn gdk_window_get_decorations    (window: *mut C_GdkWindow, decorations: *mut enums::WMDecoration) -> Gboolean;
    pub fn gdk_window_set_functions      (window: *mut C_GdkWindow, functions: enums::WMFunction);
    pub fn gdk_get_default_root_window   () -> *mut C_GdkWindow;
    pub fn gdk_window_get_support_multidevice(window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_support_multidevice(window: *mut C_GdkWindow, support_multidevice: Gboolean);
    pub fn gdk_window_get_device_cursor  (window: *mut C_GdkWindow, device: *mut C_GdkDevice) -> *mut C_GdkCursor;
    pub fn gdk_window_set_device_cursor  (window: *mut C_GdkWindow, device: *mut C_GdkDevice, cursor: *mut C_GdkCursor);
    pub fn gdk_window_get_device_events  (window: *mut C_GdkWindow, device: *mut C_GdkDevice) -> enums::EventMask;
    pub fn gdk_window_set_device_events  (window: *mut C_GdkWindow, device: *mut C_GdkDevice, event_mask: enums::EventMask);
    pub fn gdk_window_get_source_events  (window: *mut C_GdkWindow, source: enums::InputSource) -> enums::EventMask;
    pub fn gdk_window_set_source_events  (window: *mut C_GdkWindow, source: enums::InputSource, event_mask: enums::EventMask);
    pub fn gdk_window_get_event_compression(window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_event_compression(window: *mut C_GdkWindow, event_compression: Gboolean);
    //pub fn gdk_offscreen_window_get_surface(window: *mut C_GdkWindow) -> *mut cairo_surface_t;
    pub fn gdk_offscreen_window_set_embedder(window: *mut C_GdkWindow, embedder: *mut C_GdkWindow);
    pub fn gdk_offscreen_window_get_embedder(window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_geometry_changed   (window: *mut C_GdkWindow);
    pub fn gdk_window_coords_from_parent (window: *mut C_GdkWindow, parent_x: c_double, parent_y: c_double, x: *mut c_double,
        y: *mut c_double);
    pub fn gdk_window_coords_to_parent   (window: *mut C_GdkWindow, x: c_double, y: c_double, parent_x: *mut c_double,
        parent_y: *mut c_double);
    pub fn gdk_window_get_effective_parent(window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_get_effective_toplevel(window: *mut C_GdkWindow) -> *mut C_GdkWindow;

    // Callback
    //let GdkWindowInvalidateHandlerFunc = fn(window: *mut C_GdkWindow, region: *const cairo_region_t);
    //let GdkWindowChildFunc = fn(window: *mut C_GdkWindow, user_data: *mut c_void);
    //let GdkFilterFunc = fn(xevent: *mut C_GdkXEvent, event: *mut C_GdkEvent, data: *mut c_void) -> GdkFilterReturn;

    //=========================================================================
    // GdkDevice                                                         NOT OK
    //=========================================================================
    pub fn gdk_device_get_name             (device: *mut C_GdkDevice) -> *const c_char;
    pub fn gdk_device_get_source           (device: *mut C_GdkDevice) -> enums::InputSource;
    pub fn gdk_device_set_mode             (device: *mut C_GdkDevice, mode: enums::InputMode);
    pub fn gdk_device_get_mode             (device: *mut C_GdkDevice) -> enums::InputMode;
    pub fn gdk_device_set_key              (device: *mut C_GdkDevice, index_: c_uint, keyval: c_uint, modifiers: enums::modifier_type::ModifierType);
    pub fn gdk_device_get_key              (device: *mut C_GdkDevice, index_: c_uint, keyval: *mut c_uint,
        modifiers: *mut enums::modifier_type::ModifierType) -> Gboolean;
    pub fn gdk_device_set_axis_use         (device: *mut C_GdkDevice, index_: c_uint, use_: enums::AxisUse);
    pub fn gdk_device_get_axis_use         (device: *mut C_GdkDevice, index_: c_uint) -> enums::AxisUse;
    pub fn gdk_device_get_associated_device(device: *mut C_GdkDevice) -> *mut C_GdkDevice;
    //pub fn gdk_device_list_slave_devices   (device: *mut C_GdkDevice) -> *mut GList;
    pub fn gdk_device_get_device_type      (device: *mut C_GdkDevice) -> enums::DeviceType;
    pub fn gdk_device_get_display          (device: *mut C_GdkDevice) -> *mut C_GdkDisplay;
    pub fn gdk_device_get_has_cursor       (device: *mut C_GdkDevice) -> Gboolean;
    pub fn gdk_device_get_n_axes           (device: *mut C_GdkDevice) -> c_int;
    pub fn gdk_device_get_n_keys           (device: *mut C_GdkDevice) -> c_int;
    pub fn gdk_device_warp                 (device: *mut C_GdkDevice, screen: *mut C_GdkScreen, x: c_int, y: c_int);
    pub fn gdk_device_grab                 (device: *mut C_GdkDevice, window: *mut C_GdkWindow, grab_ownership: enums::GrabOwnership,
        owner_events: Gboolean, event_mask: enums::EventMask, cursor: *mut C_GdkCursor, time_: u32) -> enums::GrabStatus;
    pub fn gdk_device_ungrab               (device: *mut C_GdkDevice, time_: u32);
    pub fn gdk_device_get_state            (device: *mut C_GdkDevice, window: *mut C_GdkWindow, axes: *mut c_double,
        mask: *mut enums::modifier_type::ModifierType);
    pub fn gdk_device_get_position         (device: *mut C_GdkDevice, screen: *mut *mut C_GdkScreen, x: *mut c_int, y: *mut c_int);
    pub fn gdk_device_get_position_double  (device: *mut C_GdkDevice, screen: *mut *mut C_GdkScreen, x: *mut c_double, y: *mut c_double);
    pub fn gdk_device_get_window_at_position(device: *mut C_GdkDevice, win_x: *mut c_int, win_y: *mut c_int) -> *mut C_GdkWindow;
    pub fn gdk_device_get_window_at_position_double(device: *mut C_GdkDevice, win_x: *mut c_double,
        win_y: *mut c_double) -> *mut C_GdkWindow;
    pub fn gdk_device_get_history          (device: *mut C_GdkDevice, window: *mut C_GdkWindow, start: u32, stop: u32,
        events: *mut *mut *mut C_GdkTimeCoord, n_events: *mut c_int);
    pub fn gdk_device_free_history         (events: *mut *mut C_GdkTimeCoord, n_events: c_int);
    pub fn gdk_device_get_axis             (device: *mut C_GdkDevice, axes: *mut c_double, use_: enums::AxisUse,
        value: *mut c_double) -> Gboolean;
    //pub fn gdk_device_list_axes            (device: *mut C_GdkDevice) -> *mut GList;
    pub fn gdk_device_get_axis_value       (device: *mut C_GdkDevice, axes: *mut c_double, use_: C_GdkAtom,
        value: *mut c_double) -> Gboolean;
    pub fn gdk_device_get_last_event_window(device: *mut C_GdkDevice) -> *mut C_GdkWindow;

    //=========================================================================
    // GdkDeviceManager                                                  NOT OK
    //=========================================================================
    pub fn gdk_disable_multidevice         ();
    pub fn gdk_device_manager_get_display  (device_manager: *mut C_GdkDeviceManager) -> *mut C_GdkDisplay;
    //pub fn gdk_device_manager_list_devices (device_manager: *mut C_GdkDeviceManager, type_: enums::DeviceType) -> *mut GList;
    pub fn gdk_device_manager_get_client_pointer(device_manager: *mut C_GdkDeviceManager) -> *mut C_GdkDevice;

    //=========================================================================
    // GdkDisplay                                                        NOT OK
    //=========================================================================
    pub fn gdk_display_open                (display_name: *const c_char) -> *mut C_GdkDisplay;
    pub fn gdk_display_get_default         () -> *mut C_GdkDisplay;
    pub fn gdk_display_get_name            (display: *mut C_GdkDisplay) -> *const c_char;
    pub fn gdk_display_get_screen          (display: *mut C_GdkDisplay, screen_num: c_int) -> *mut C_GdkScreen;
    pub fn gdk_display_get_default_screen  (display: *mut C_GdkDisplay) -> *mut C_GdkScreen;
    pub fn gdk_display_get_device_manager  (display: *mut C_GdkDisplay) -> *mut C_GdkDeviceManager;
    pub fn gdk_display_device_is_grabbed   (display: *mut C_GdkDisplay, device: *mut C_GdkDevice) -> Gboolean;
    pub fn gdk_display_beep                (display: *mut C_GdkDisplay);
    pub fn gdk_display_sync                (display: *mut C_GdkDisplay);
    pub fn gdk_display_flush               (display: *mut C_GdkDisplay);
    pub fn gdk_display_close               (display: *mut C_GdkDisplay);
    pub fn gdk_display_is_closed           (display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_get_event           (display: *mut C_GdkDisplay) -> *mut C_GdkEvent;
    pub fn gdk_display_peek_event          (display: *mut C_GdkDisplay) -> *mut C_GdkEvent;
    pub fn gdk_display_put_event           (display: *mut C_GdkDisplay, event: *const C_GdkEvent);
    pub fn gdk_display_has_pending         (display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_set_double_click_time(display: *mut C_GdkDisplay, msec: c_uint);
    pub fn gdk_display_set_double_click_distance(display: *mut C_GdkDisplay, distance: c_uint);
    pub fn gdk_display_supports_cursor_color(display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_supports_cursor_alpha(display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_get_default_cursor_size(display: *mut C_GdkDisplay) -> c_uint;
    pub fn gdk_display_get_maximal_cursor_size(display: *mut C_GdkDisplay, width: *mut c_uint, height: *mut c_uint);
    pub fn gdk_display_get_default_group   (display: *mut C_GdkDisplay) -> *mut C_GdkWindow;
    pub fn gdk_display_supports_selection_notification(display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_request_selection_notification(display: *mut C_GdkDisplay, selection: C_GdkAtom) -> Gboolean;
    pub fn gdk_display_supports_clipboard_persistence(display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_store_clipboard     (display: *mut C_GdkDisplay, clipboard_window: *mut C_GdkWindow, time_: u32,
        targets: *const C_GdkAtom, n_targets: c_int) -> Gboolean;
    pub fn gdk_display_supports_shapes     (display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_supports_input_shapes(display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_supports_composite  (display: *mut C_GdkDisplay) -> Gboolean;
    pub fn gdk_display_get_app_launch_context(display: *mut C_GdkDisplay) -> *mut C_GdkAppLaunchContext;
    pub fn gdk_display_notify_startup_complete(display: *mut C_GdkDisplay, startup_id: *const c_char);

    //=========================================================================
    // GdkDisplayManager                                                 NOT OK
    //=========================================================================
    pub fn gdk_display_manager_get                () -> *mut C_GdkDisplayManager;
    pub fn gdk_display_manager_get_default_display(manager: *mut C_GdkDisplayManager) -> *mut C_GdkDisplay;
    pub fn gdk_display_manager_set_default_display(manager: *mut C_GdkDisplayManager, display: *mut C_GdkDisplay);
    //pub fn gdk_display_manager_list_displays      (manager: *mut C_GdkDisplayManager) -> *mut GSList;
    pub fn gdk_display_manager_open_display       (manager: *mut C_GdkDisplayManager, name: *const c_char) -> *mut C_GdkDisplay;

    //=========================================================================
    // GdkScreen                                                         NOT OK
    //=========================================================================
    pub fn gdk_screen_get_default             () -> *mut C_GdkScreen;
    pub fn gdk_screen_get_system_visual       (screen: *mut C_GdkScreen) -> *mut C_GdkVisual;
    pub fn gdk_screen_get_rgba_visual         (screen: *mut C_GdkScreen) -> *mut C_GdkVisual;
    pub fn gdk_screen_is_composited           (screen: *mut C_GdkScreen) -> Gboolean;
    pub fn gdk_screen_get_root_window         (screen: *mut C_GdkScreen) -> *mut C_GdkWindow;
    pub fn gdk_screen_get_display             (screen: *mut C_GdkScreen) -> *mut C_GdkDisplay;
    pub fn gdk_screen_get_number              (screen: *mut C_GdkScreen) -> c_int;
    pub fn gdk_screen_get_width               (screen: *mut C_GdkScreen) -> c_int;
    pub fn gdk_screen_get_height              (screen: *mut C_GdkScreen) -> c_int;
    pub fn gdk_screen_get_width_mm            (screen: *mut C_GdkScreen) -> c_int;
    pub fn gdk_screen_get_height_mm           (screen: *mut C_GdkScreen) -> c_int;
    //pub fn gdk_screen_list_visuals            (screen: *mut C_GdkScreen) -> *mut GList;
    //pub fn gdk_screen_get_toplevel_windows    (screen: *mut C_GdkScreen) -> *mut GList;
    pub fn gdk_screen_make_display_name       (screen: *mut C_GdkScreen) -> *mut c_char;
    pub fn gdk_screen_get_n_monitors          (screen: *mut C_GdkScreen) -> c_int;
    pub fn gdk_screen_get_primary_monitor     (screen: *mut C_GdkScreen) -> c_int;
    pub fn gdk_screen_get_monitor_geometry    (screen: *mut C_GdkScreen, monitor_num: c_int, dest: *mut C_GdkRectangle);
    pub fn gdk_screen_get_monitor_workarea    (screen: *mut C_GdkScreen, monitor_num: c_int, dest: *mut C_GdkRectangle);
    pub fn gdk_screen_get_monitor_at_point    (screen: *mut C_GdkScreen, x: c_int, y: c_int) -> c_int;
    pub fn gdk_screen_get_monitor_at_window   (screen: *mut C_GdkScreen, window: *mut C_GdkWindow) -> c_int;
    pub fn gdk_screen_get_monitor_height_mm   (screen: *mut C_GdkScreen, monitor_num: c_int) -> c_int;
    pub fn gdk_screen_get_monitor_width_mm    (screen: *mut C_GdkScreen, monitor_num: c_int) -> c_int;
    pub fn gdk_screen_get_monitor_plug_name   (screen: *mut C_GdkScreen, monitor_num: c_int) -> *mut c_char;
    pub fn gdk_screen_get_monitor_scale_factor(screen: *mut C_GdkScreen, monitor_num: c_int) -> c_int;
    //pub fn gdk_screen_get_setting             (screen: *mut C_GdkScreen, name: *const c_char, value: *mut GValue) -> Gboolean;
    //pub fn gdk_screen_get_font_options        (screen: *mut C_GdkScreen) -> *const cairo_font_options_t;
    //pub fn gdk_screen_set_font_options        (screen: *mut C_GdkScreen, options: *const cairo_font_options_t);
    pub fn gdk_screen_get_resolution          (screen: *mut C_GdkScreen) -> c_double;
    pub fn gdk_screen_set_resolution          (screen: *mut C_GdkScreen, dpi: c_double);
    pub fn gdk_screen_get_active_window       (screen: *mut C_GdkScreen) -> *mut C_GdkWindow;
    //pub fn gdk_screen_get_window_stack        (screen: *mut C_GdkScreen) -> *mut GList;

    //=========================================================================
    // GdkVisual                                                         NOT OK
    //=========================================================================
    pub fn gdk_query_depths                   (depths: *mut *mut c_int, count: *mut c_int);
    //pub fn gdk_query_visual_types             (visual_types: *mut *mut GdkVisualType, count: *mut c_int);
    //pub fn gdk_list_visuals                   () -> *mut GList;
    pub fn gdk_visual_get_bits_per_rgb        (visual: *mut C_GdkVisual) -> c_int;
    pub fn gdk_visual_get_blue_pixel_details  (visual: *mut C_GdkVisual, mask: *mut u32, shift: *mut c_int, precision: *mut c_int);
    //pub fn gdk_visual_get_byte_order          (visual: *mut C_GdkVisual) -> GdkByteOrder;
    pub fn gdk_visual_get_colormap_size       (visual: *mut C_GdkVisual) -> c_int;
    pub fn gdk_visual_get_depth               (visual: *mut C_GdkVisual) -> c_int;
    pub fn gdk_visual_get_green_pixel_details (visual: *mut C_GdkVisual, mask: *mut u32, shift: *mut c_int, precision: *mut c_int);
    pub fn gdk_visual_get_red_pixel_details   (visual: *mut C_GdkVisual, mask: *mut u32, shift: *mut c_int, precision: *mut c_int);
    //pub fn gdk_visual_get_visual_type         (visual: *mut C_GdkVisual) -> GdkVisualType;
    pub fn gdk_visual_get_best_depth          () -> c_int;
    //pub fn gdk_visual_get_best_type           () -> GdkVisualType;
    pub fn gdk_visual_get_system              () -> *mut C_GdkVisual;
    pub fn gdk_visual_get_best                () -> *mut C_GdkVisual;
    pub fn gdk_visual_get_best_with_depth     (depth: c_int) -> *mut C_GdkVisual;
    //pub fn gdk_visual_get_best_with_type      (visual_type: GdkVisualType) -> *mut C_GdkVisual;
    //pub fn gdk_visual_get_best_with_both      (depth: c_int, visual_type: GdkVisualType) -> *mut C_GdkVisual;
    pub fn gdk_visual_get_screen              (visual: *mut C_GdkVisual) -> *mut C_GdkScreen;

    //=========================================================================
    // GdkCursor                                                         NOT OK
    //=========================================================================
    pub fn gdk_cursor_new                     (cursor_type: enums::CursorType) -> *mut C_GdkCursor;
    pub fn gdk_cursor_new_from_pixbuf         (display: *mut C_GdkDisplay, pixbuf: *mut C_GdkPixbuf, x: c_int, y: c_int) -> *mut C_GdkCursor;
    //pub fn gdk_cursor_new_from_surface        (display: *mut C_GdkDisplay, surface: *mut cairo_surface_t, x: c_double,
    //    y: c_double) -> *mut C_GdkCursor;
    pub fn gdk_cursor_new_from_name           (display: *mut C_GdkDisplay, name: *const c_char) -> *mut C_GdkCursor;
    pub fn gdk_cursor_new_for_display         (display: *mut C_GdkDisplay, cursor_type: enums::CursorType) -> *mut C_GdkCursor;
    pub fn gdk_cursor_get_display             (cursor: *mut C_GdkCursor) -> *mut C_GdkDisplay;
    pub fn gdk_cursor_get_image               (cursor: *mut C_GdkCursor) -> *mut C_GdkPixbuf;
    //pub fn gdk_cursor_get_surface             (cursor: *mut C_GdkCursor, x_hot: *mut c_double, y_hot: *mut c_double) -> *mut cairo_surface_t;
    pub fn gdk_cursor_get_cursor_type         (cursor: *mut C_GdkCursor) -> enums::CursorType;

    //=========================================================================
    // GdkPixbuf                                                         NOT OK
    //=========================================================================
    pub fn gdk_pixbuf_get_colorspace          (pixbuf: *const C_GdkPixbuf) -> enums::ColorSpace;
    pub fn gdk_pixbuf_get_n_channels          (pixbuf: *const C_GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_has_alpha           (pixbuf: *const C_GdkPixbuf) -> Gboolean;
    pub fn gdk_pixbuf_get_bits_per_sample     (pixbuf: *const C_GdkPixbuf) -> c_int;
    //pub fn gdk_pixbuf_get_pixels              (pixbuf: *const C_GdkPixbuf) -> *mut c_uchar;
    pub fn gdk_pixbuf_get_pixels_with_length  (pixbuf: *const C_GdkPixbuf, length: *mut c_uint) -> *mut c_uchar;
    pub fn gdk_pixbuf_get_width               (pixbuf: *const C_GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_height              (pixbuf: *const C_GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_rowstride           (pixbuf: *const C_GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_byte_length         (pixbuf: *const C_GdkPixbuf) -> c_ulong;
    pub fn gdk_pixbuf_get_option              (pixbuf: *const C_GdkPixbuf, key: *const c_char) -> *const c_char;

    //=========================================================================
    // GdkRectangle                                                      NOT OK
    //=========================================================================
    pub fn gdk_rectangle_intersect            (src1: *const C_GdkRectangle, src2: *const C_GdkRectangle,
        dest: *mut C_GdkRectangle) -> Gboolean;
    pub fn gdk_rectangle_union                (src1: *const C_GdkRectangle, src2: *const C_GdkRectangle, dest: *mut C_GdkRectangle);

    //=========================================================================
    // GdkRGBA                                                           NOT OK
    //=========================================================================
    //pub fn gdk_rgba_copy                      (rgba: *const C_GdkRGBA) -> *mut C_GdkRGBA;
    //pub fn gdk_rgba_free                      (rgba: *mut C_GdkRGBA);
    pub fn gdk_rgba_parse                       (rgba: *mut C_GdkRGBA, spec: *const c_char) -> Gboolean;
    pub fn gdk_rgba_equal                       (p1: *const C_GdkRGBA, p2: *const C_GdkRGBA) -> Gboolean;
    pub fn gdk_rgba_hash                        (p: *const C_GdkRGBA) -> c_uint;
    pub fn gdk_rgba_to_string                   (rgba: *const C_GdkRGBA) -> *mut c_char;

    //=========================================================================
    // GdkFrameClock                                                     NOT OK
    //=========================================================================
    pub fn gdk_frame_clock_get_frame_time       (frame_clock: *mut C_GdkFrameClock) -> i64;
    pub fn gdk_frame_clock_request_phase        (frame_clock: *mut C_GdkFrameClock, phase: enums::FrameClockPhase);
    pub fn gdk_frame_clock_begin_updating       (frame_clock: *mut C_GdkFrameClock);
    pub fn gdk_frame_clock_end_updating         (frame_clock: *mut C_GdkFrameClock);
    pub fn gdk_frame_clock_get_frame_counter    (frame_clock: *mut C_GdkFrameClock) -> i64;
    pub fn gdk_frame_clock_get_history_start    (frame_clock: *mut C_GdkFrameClock) -> i64;
    pub fn gdk_frame_clock_get_timings          (frame_clock: *mut C_GdkFrameClock, frame_counter: i64) -> *mut C_GdkFrameTimings;
    pub fn gdk_frame_clock_get_current_timings  (frame_clock: *mut C_GdkFrameClock) -> *mut C_GdkFrameTimings;
    pub fn gdk_frame_clock_get_refresh_info     (frame_clock: *mut C_GdkFrameClock, base_time: i64, refresh_interval_return: *mut i64,
        presentation_time_return: *mut i64);

    //=========================================================================
    // GdkFrameTimings                                                   NOT OK
    //=========================================================================
    // Since 3.8
    pub fn gdk_frame_timings_ref                  (timings: *mut C_GdkFrameTimings) -> *mut C_GdkFrameTimings;
    // Since 3.8
    pub fn gdk_frame_timings_unref                (timings: *mut C_GdkFrameTimings);
    // Since 3.8
    pub fn gdk_frame_timings_get_frame_counter    (timings: *mut C_GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_complete         (timings: *mut C_GdkFrameTimings) -> Gboolean;
    pub fn gdk_frame_timings_get_frame_time       (timings: *mut C_GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_presentation_time(timings: *mut C_GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_refresh_interval (timings: *mut C_GdkFrameTimings) -> i64;
    // Since 3.8
    pub fn gdk_frame_timings_get_predicted_presentation_time(timings: *mut C_GdkFrameTimings) -> i64;

    //=========================================================================
    // GdkAtom                                                           NOT OK
    //=========================================================================
    //pub fn gdk_text_property_to_utf8_list_for_display(display: *mut C_GdkDisplay, encoding: C_GdkAtom, format: c_int,
    //    text: *const c_uchar, length: c_int, list: *mut *mut *mut c_char) -> c_int;
    pub fn gdk_utf8_to_string_target               (str: *const c_char) -> *mut c_char;
    pub fn gdk_atom_intern                         (atom_name: *const c_char, only_if_exists: Gboolean) -> C_GdkAtom;
    pub fn gdk_atom_intern_static_string           (atom_name: *const c_char) -> C_GdkAtom;
    pub fn gdk_atom_name                           (atom: C_GdkAtom) -> *mut c_char;
    //pub fn gdk_property_get                        (window: *mut C_GdkWindow, atom: C_GdkAtom, type_: C_GdkAtom, offset: c_ulong,
    //    length: c_ulong, pdelete: c_int, actual_property_type: *mut C_GdkAtom, actual_format: *mut c_int, actual_length: *mut c_int,
    //    data: *mut *mut c_uchar) -> Gboolean;
    //pub fn gdk_property_change                     (window: *mut C_GdkWindow, property: C_GdkAtom, type_: C_GdkAtom, format: c_int,
    //    mode: enums::PropMode, data: *const c_uchar, nelements: c_int);
    //pub fn gdk_property_delete                     (window: *mut C_GdkWindow, property: C_GdkAtom);

    //=========================================================================
    // GdkDragContext                                                    NOT OK
    //=========================================================================
    pub fn gdk_drag_get_selection                  (context: *mut C_GdkDragContext) -> C_GdkAtom;
    pub fn gdk_drag_abort                          (context: *mut C_GdkDragContext, time_: u32);
    pub fn gdk_drop_reply                          (context: *mut C_GdkDragContext, accepted: Gboolean, time_: u32);
    pub fn gdk_drag_drop                           (context: *mut C_GdkDragContext, time_: u32);
    pub fn gdk_drag_find_window_for_screen         (context: *mut C_GdkDragContext, drag_window: *mut C_GdkWindow, screen: *mut C_GdkScreen,
        x_root: c_int, y_root: c_int, dest_window: *mut *mut C_GdkWindow, protocol: *mut enums::DragProtocol);
    //pub fn gdk_drag_begin                          (window: *mut C_GdkWindow, targets: *mut GList) -> *mut C_GdkDragContext;
    //pub fn gdk_drag_begin_for_device               (window: *mut C_GdkWindow, device: *mut C_GdkDevice,
    //    targets: *mut GList) -> *mut C_GdkDragContext;
    pub fn gdk_drag_motion                         (context: *mut C_GdkDragContext, dest_window: *mut C_GdkWindow, protocol: enums::DragProtocol,
        x_root: c_int, y_root: c_int, suggested_action: enums::DragAction, possible_actions: enums::DragAction,
        time_: u32) -> Gboolean;
    pub fn gdk_drop_finish                         (context: *mut C_GdkDragContext, success: Gboolean, time_: u32);
    pub fn gdk_drag_status                         (context: *mut C_GdkDragContext, action: enums::DragAction, time_: u32);
    pub fn gdk_drag_drop_succeeded                 (context: *mut C_GdkDragContext) -> Gboolean;
    pub fn gdk_window_get_drag_protocol            (window: *mut C_GdkWindow, target: *mut *mut C_GdkWindow) -> enums::DragProtocol;
    pub fn gdk_drag_context_get_actions            (context: *mut C_GdkDragContext) -> enums::DragAction;
    pub fn gdk_drag_context_get_suggested_action   (context: *mut C_GdkDragContext) -> enums::DragAction;
    pub fn gdk_drag_context_get_selected_action    (context: *mut C_GdkDragContext) -> enums::DragAction;
    //pub fn gdk_drag_context_list_targets           (context: *mut C_GdkDragContext) -> *mut GList;
    pub fn gdk_drag_context_get_device             (context: *mut C_GdkDragContext) -> *mut C_GdkDevice;
    pub fn gdk_drag_context_set_device             (context: *mut C_GdkDragContext, device: *mut C_GdkDevice);
    pub fn gdk_drag_context_get_source_window      (context: *mut C_GdkDragContext) -> *mut C_GdkWindow;
    pub fn gdk_drag_context_get_dest_window        (context: *mut C_GdkDragContext) -> *mut C_GdkWindow;
    pub fn gdk_drag_context_get_protocol           (context: *mut C_GdkDragContext) -> enums::DragProtocol;

    //=========================================================================
    // GdkAppLaunchContext                                               NOT OK
    //=========================================================================
    pub fn gdk_app_launch_context_set_screen       (context: *mut C_GdkAppLaunchContext, screen: *mut C_GdkScreen);
    pub fn gdk_app_launch_context_set_desktop      (context: *mut C_GdkAppLaunchContext, desktop: c_int);
    pub fn gdk_app_launch_context_set_timestamp    (context: *mut C_GdkAppLaunchContext, timestamp: u32);
    //pub fn gdk_app_launch_context_set_icon         (context: *mut C_GdkAppLaunchContext, icon: *mut C_GIcon);
    pub fn gdk_app_launch_context_set_icon_name    (context: *mut C_GdkAppLaunchContext, icon_name: *const c_char);

    //=========================================================================
    // Gdk Key Handling                                                  NOT OK
    //=========================================================================
    pub fn gdk_keyval_name                         (keyval:c_uint) -> *mut c_char;
}
