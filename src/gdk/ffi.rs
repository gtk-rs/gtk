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

use libc::{c_int, c_char, c_double, c_void, c_uint};
use gtk::ffi::{Gboolean};
use gdk;

#[repr(C)]
#[derive(Copy)]
pub struct C_GdkWindow;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkWindowAttr;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDisplay;
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
pub struct C_GdkRectangle;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkFrameClock;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkRGBA;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkCursor;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkGeometry;
#[repr(C)]
#[derive(Copy)]
pub struct C_GdkDevice;
#[repr(C)]
#[deriving(Copy)]
pub struct C_GdkTimeCoord;
#[repr(C)]
#[deriving(Copy)]
pub struct C_GdkAtom;
#[repr(C)]
#[deriving(Copy)]
pub struct C_GdkDeviceManager;
#[repr(C)]
#[deriving(Copy)]
pub struct C_GdkAppLaunchContext;

extern "C" {
    //=========================================================================
    // GdkWindow                                                         NOT OK
    //=========================================================================
    pub fn gdk_window_new                (parent: *mut C_GdkWindow, attributes: *mut C_GdkWindowAttr,
        attributes_mask: c_int) -> *mut C_GdkWindow;
    pub fn gdk_window_destroy            (window: *mut C_GdkWindow);
    pub fn gdk_window_get_window_type    (window: *mut C_GdkWindow) -> gdk::WindowType;
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
    pub fn gdk_window_get_state          (window: *mut C_GdkWindow) -> gdk::WindowState;
    pub fn gdk_window_withdraw           (window: *mut C_GdkWindow);
    pub fn gdk_window_iconify            (window: *mut C_GdkWindow);
    pub fn gdk_window_deiconify          (window: *mut C_GdkWindow);
    pub fn gdk_window_stick              (window: *mut C_GdkWindow);
    pub fn gdk_window_unstick            (window: *mut C_GdkWindow);
    pub fn gdk_window_maximize           (window: *mut C_GdkWindow);
    pub fn gdk_window_unmaximize         (window: *mut C_GdkWindow);
    pub fn gdk_window_fullscreen         (window: *mut C_GdkWindow);
    pub fn gdk_window_unfullscreen       (window: *mut C_GdkWindow);
    pub fn gdk_window_get_fullscreen_mode(window: *mut C_GdkWindow) -> gdk::FullscreenMode;
    pub fn gdk_window_set_fullscreen_mode(window: *mut C_GdkWindow, mode: gdk::FullscreenMode);
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
    pub fn gdk_window_begin_resize_drag  (window: *mut C_GdkWindow, edge: gdk::WindowEdge, button: c_int, root_x: c_int, root_y: c_int,
        timestamp: u32);
    pub fn gdk_window_begin_resize_drag_for_device(window: *mut C_GdkWindow, edge: gdk::WindowEdge, device: *mut C_GdkDevice,
        button: c_int, root_x: c_int, root_y: c_int, timestamp: u32);
    pub fn gdk_window_begin_move_drag    (window: *mut C_GdkWindow, button: c_int, root_x: c_int, root_y: c_int, timestamp: u32);
    pub fn gdk_window_begin_move_drag_for_device(window: *mut C_GdkWindow, device: *mut C_GdkDevice, button: c_int, root_x: c_int,
        root_y: c_int, timestamp: u32);
    pub fn gdk_window_show_window_menu   (window: *mut C_GdkWindow, event: *mut C_GdkEvent);
    pub fn gdk_window_constrain_size     (window: *mut C_GdkWindow, flags: gdk::WindowHints, width: c_int, height: c_int,
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
    pub fn gdk_window_get_user_data      (window: *mut C_GdkWindow, data: *mut c_void);
    pub fn gdk_window_get_geometry       (window: *mut C_GdkWindow, x: *mut c_int, y: *mut c_int, width: *mut c_int, height: *mut c_int);
    pub fn gdk_window_set_geometry_hints (window: *mut C_GdkWindow, geometry: *const C_GdkGeometry, geom_mask: gdk::WindowHints);
    pub fn gdk_window_get_width          (window: *mut C_GdkWindow) -> c_int;
    pub fn gdk_window_get_height         (window: *mut C_GdkWindow) -> c_int;
    //pub fn gdk_window_set_icon_list      (window: *mut C_GdkWindow, pixbufs: *mut GList);
    pub fn gdk_window_set_modal_hint     (window: *mut C_GdkWindow, modal: Gboolean);
    pub fn gdk_window_get_modal_hint     (window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_type_hint      (window: *mut C_GdkWindow, hint: gdk::WindowTypeHint);
    pub fn gdk_window_get_type_hint      (window: *mut C_GdkWindow) -> gdk::WindowTypeHint;
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
        mask: *mut gdk::ModifierType) -> *mut C_GdkWindow;
    pub fn gdk_window_get_device_position_double(window: *mut C_GdkWindow, device: *mut C_GdkDevice, x: *mut c_double, y: *mut c_double,
        mask: *mut gdk::ModifierType) -> *mut C_GdkWindow;
    pub fn gdk_window_get_parent         (window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_get_toplevel       (window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    //pub fn gdk_window_get_children       (window: *mut C_GdkWindow) -> *mut GList;
    //pub fn gdk_window_get_children_with_user_data(window: *mut C_GdkWindow, user_data: *mut c_void) -> *mut GList;
    //pub fn gdk_window_peek_children      (window: *mut C_GdkWindow) -> *mut GList;
    pub fn gdk_window_get_events         (window: *mut C_GdkWindow) -> gdk::EventMask;
    pub fn gdk_window_set_events         (window: *mut C_GdkWindow, event_mask: gdk::EventMask);
    pub fn gdk_window_set_icon_name      (window: *mut C_GdkWindow, name: *const c_char);
    pub fn gdk_window_set_transient_for  (window: *mut C_GdkWindow, parent: *mut C_GdkWindow);
    pub fn gdk_window_set_role           (window: *mut C_GdkWindow, role: *const c_char);
    pub fn gdk_window_set_startup_id     (window: *mut C_GdkWindow, startup_id: *const c_char);
    pub fn gdk_window_set_group          (window: *mut C_GdkWindow, leader: *mut C_GdkWindow);
    pub fn gdk_window_get_group          (window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_set_decorations    (window: *mut C_GdkWindow, decorations: gdk::WMDecoration);
    pub fn gdk_window_get_decorations    (window: *mut C_GdkWindow, decorations: *mut gdk::WMDecoration) -> Gboolean;
    pub fn gdk_window_set_functions      (window: *mut C_GdkWindow, functions: gdk::WMDecoration);
    pub fn gdk_get_default_root_window   () -> *mut C_GdkWindow;
    pub fn gdk_window_get_support_multidevice(window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_support_multidevice(window: *mut C_GdkWindow, support_multidevice: Gboolean);
    pub fn gdk_window_get_device_cursor  (window: *mut C_GdkWindow, device: *mut C_GdkDevice) -> *mut C_GdkCursor;
    pub fn gdk_window_set_device_cursor  (window: *mut C_GdkWindow, device: *mut C_GdkDevice, cursor: *mut C_GdkCursor);
    pub fn gdk_window_get_device_events  (window: *mut C_GdkWindow, device: *mut C_GdkDevice) -> gdk::EventMask;
    pub fn gdk_window_set_device_events  (window: *mut C_GdkWindow, device: *mut C_GdkDevice, event_mask: gdk::EventMask);
    pub fn gdk_window_get_source_events  (window: *mut C_GdkWindow, source: gdk::InputSource) -> gdk::EventMask;
    pub fn gdk_window_set_source_events  (window: *mut C_GdkWindow, source: gdk::InputSource, event_mask: gdk::EventMask);
    pub fn gdk_window_get_event_compression(window: *mut C_GdkWindow) -> Gboolean;
    pub fn gdk_window_set_event_compression(window: *mut C_GdkWindow, event_compression: Gboolean);
    //pub fn gdk_offscreen_window_get_surface(window: *mut C_GdkWindow) -> *mut cairo_surface_t;
    pub fn gdk_offscreen_window_set_embedder(window: *mut C_GdkWindow, embedder: *mut C_GdkWindow);
    pub fn gdk_offscreen_window_get_embedder(window: *mut C_GdkWindow) -> *mut C_GdkWindow;
    pub fn gdk_window_geometry_changed   (window: *mut C_GdkWindow);
    pub fn gdk_window_coords_from_parent (window: *mut C_GdkWindow, parent_x: c_double, parent_y: c_double, x: *mut c_double,
        y: *mut c_double);
    pub fn gdk_window_coords_to_parent   (window: *mut C_GdkWindow, x: *mut c_double, y: *mut c_double, parent_x: c_double,
        parent_y: c_double);
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
    pub fn gdk_device_get_source           (device: *mut C_GdkDevice) -> gdk::InputSource;
    pub fn gdk_device_set_mode             (device: *mut C_GdkDevice, mode: gdk::InputMode);
    pub fn gdk_device_get_mode             (device: *mut C_GdkDevice) -> gdk::InputMode;
    pub fn gdk_device_set_key              (device: *mut C_GdkDevice, index_: c_uint, keyval: c_uint, modifiers: gdk::ModifierType);
    pub fn gdk_device_get_key              (device: *mut C_GdkDevice, index_: c_uint, keyval: *mut c_uint,
        modifiers: *mut gdk::ModifierType) -> Gboolean;
    pub fn gdk_device_set_axis_use         (device: *mut C_GdkDevice, index_: c_uint, use_: gdk::AxisUse);
    pub fn gdk_device_get_axis_use         (device: *mut C_GdkDevice, index_: c_uint) -> gdk::AxisUse;
    pub fn gdk_device_get_associated_device(device: *mut C_GdkDevice) -> *mut C_GdkDevice;
    //pub fn gdk_device_list_slave_devices   (device: *mut C_GdkDevice) -> *mut GList;
    pub fn gdk_device_get_device_type      (device: *mut C_GdkDevice) -> gdk::DeviceType;
    pub fn gdk_device_get_display          (device: *mut C_GdkDevice) -> *mut C_GdkDisplay;
    pub fn gdk_device_get_has_cursor       (device: *mut C_GdkDevice) -> Gboolean;
    pub fn gdk_device_get_n_axes           (device: *mut C_GdkDevice) -> c_int;
    pub fn gdk_device_get_n_keys           (device: *mut C_GdkDevice) -> c_int;
    pub fn gdk_device_warp                 (device: *mut C_GdkDevice, screen: *mut C_GdkScreen, x: c_int, y: c_int);
    pub fn gdk_device_grab                 (device: *mut C_GdkDevice, window: *mut C_GdkWindow, grab_ownership: gdk::GrabOwnership,
        owner_events: Gboolean, event_mask: gdk::EventMask, cursor: *mut C_GdkCursor, time_: u32) -> gdk::GrabStatus;
    pub fn gdk_device_ungrab               (device: *mut C_GdkDevice, time_: u32);
    pub fn gdk_device_get_state            (device: *mut C_GdkDevice, window: *mut C_GdkWindow, axes: *mut c_double,
        mask: *mut gdk::ModifierType);
    pub fn gdk_device_get_position         (device: *mut C_GdkDevice, screen: *mut *mut C_GdkScreen, x: *mut c_int, y: *mut c_int);
    pub fn gdk_device_get_position_double  (device: *mut C_GdkDevice, screen: *mut *mut C_GdkScreen, x: *mut c_double, y: *mut c_double);
    pub fn gdk_device_get_window_at_position(device: *mut C_GdkDevice, win_x: *mut c_int, win_y: *mut c_int) -> *mut C_GdkWindow;
    pub fn gdk_device_get_window_at_position_double(device: *mut C_GdkDevice, win_x: *mut c_double,
        win_y: *mut c_double) -> *mut C_GdkWindow;
    pub fn gdk_device_get_history          (device: *mut C_GdkDevice, window: *mut C_GdkWindow, start: u32, stop: u32,
        events: *mut *mut *mut C_GdkTimeCoord, n_events: *mut c_int);
    pub fn gdk_device_free_history         (events: *mut *mut C_GdkTimeCoord, n_events: c_int);
    pub fn gdk_device_get_axis             (device: *mut C_GdkDevice, axes: *mut c_double, use_: gdk::AxisUse,
        value: *mut c_double) -> Gboolean;
    //pub fn gdk_device_list_axes            (device: *mut C_GdkDevice) -> *mut GList;
    pub fn gdk_device_get_axis_value       (device: *mut C_GdkDevice, axes: *mut c_double, use_: C_GdkAtom,
        value: *mut c_double) -> Gboolean;
    pub fn gdk_device_get_last_event_window(device: *mut C_GdkDevice) -> *mut C_GdkWindow;

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
}