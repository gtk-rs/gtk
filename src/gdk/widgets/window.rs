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

//! Windows — Onscreen display areas in the target window system

use gdk::{self, ffi};
use libc::{c_int};
use std::ffi::CString;
use std::ptr;
use glib::translate::{StackBox, ToGlib, ToGlibPtr, ToTemp, from_glib};

/// Attributes to use for a newly-created window.
pub struct WindowAttr {
    /// title of the window (for toplevel windows)
    pub title: Option<String>,
    /// event mask (see [gdk_window_set_events()](https://developer.gnome.org/gdk3/3.14/gdk3-Windows.html#gdk-window-set-events))
    pub event_mask: i32,
    /// X coordinate relative to parent window
    pub x: Option<i32>,
    /// Y coordinate relative to parent window
    pub y: Option<i32>,
    /// width of window
    pub width: i32,
    /// height of window
    pub height: i32,
    /// GDK_INPUT_OUTPUT (normal window) or GDK_INPUT_ONLY (invisible window that receives events)
    pub wclass: gdk::WindowWindowClass,
    /// GdkVisual for window
    pub visual: Option<gdk::Visual>,
    /// type of window
    pub window_type: gdk::WindowType,
    /// cursor for the window
    pub cursor: Option<gdk::Cursor>,
    /// TRUE to bypass the window manager
    pub override_redirect: bool,
    /// a hint of the function of the window
    pub type_hint: Option<gdk::WindowTypeHint>,
}

impl WindowAttr {
    fn get_mask(&self) -> i32 {
        let mut mask = 0;
        if self.title.is_some() { mask |= ffi::GDK_WA_TITLE; }
        if self.x.is_some() { mask |= ffi::GDK_WA_X; }
        if self.y.is_some() { mask |= ffi::GDK_WA_Y; }
        if self.cursor.is_some() { mask |= ffi::GDK_WA_CURSOR; }
        if self.visual.is_some() { mask |= ffi::GDK_WA_VISUAL; }
        if self.override_redirect { mask |= ffi::GDK_WA_NOREDIR; }
        if self.type_hint.is_some() { mask |= ffi::GDK_WA_TYPE_HINT; }
        mask
    }
}

type WindowAttrBox = StackBox<ffi::C_GdkWindowAttr, Option<CString>>;

impl <'a> ToTemp for &'a WindowAttr {
    type Temp = WindowAttrBox;

    fn to_temp_for_borrow(self) -> WindowAttrBox {
        let mut title = self.title.to_temp_for_borrow();

        let attrs = ffi::C_GdkWindowAttr {
            title: title.to_glib(),
            event_mask: self.event_mask,
            x: self.x.unwrap_or(0),
            y: self.y.unwrap_or(0),
            width: self.width,
            height: self.height,
            wclass: self.wclass,
            visual: match self.visual { Some(s) => s.unwrap_pointer(), None => ptr::null_mut() },
            window_type: self.window_type,
            cursor: match self.cursor { Some(s) => s.unwrap_pointer(), None => ptr::null_mut() },
            wmclass_name: ptr::null(),
            wmclass_class: ptr::null(),
            override_redirect: self.override_redirect.to_glib(),
            type_hint: self.type_hint.unwrap_or(gdk::WindowTypeHint::Normal),
        };

        StackBox(attrs, title)
    }
}

#[repr(C)]
pub struct Window {
    pointer: *mut ffi::C_GdkWindow
}

impl Window {
    pub fn new(parent: Option<&Window>, attributes: &WindowAttr) -> Option<Window> {
        let t_parent = match parent {
            Some(s) => s.unwrap_pointer(),
            None => ::std::ptr::null_mut()
        };
        let mut attrs = attributes.to_temp_for_borrow();
        let tmp = unsafe { ffi::gdk_window_new(t_parent, attrs.to_glib(), attributes.get_mask()) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_window_type(&self) -> gdk::WindowType {
        unsafe { ffi::gdk_window_get_window_type(self.pointer) }
    }

    pub fn get_display(&self) -> Option<gdk::Display> {
        let tmp = unsafe { ffi::gdk_window_get_display(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Display::wrap_pointer(tmp))
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        let tmp = unsafe { ffi::gdk_window_get_screen(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Screen::wrap_pointer(tmp))
        }
    }

    pub fn get_visual(&self) -> Option<gdk::Visual> {
        let tmp = unsafe { ffi::gdk_window_get_visual(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Visual::wrap_pointer(tmp))
        }
    }

    pub fn show(&self) {
        unsafe { ffi::gdk_window_show(self.pointer) }
    }

    pub fn show_unraised(&self) {
        unsafe { ffi::gdk_window_show_unraised(self.pointer) }
    }

    pub fn hide(&self) {
        unsafe { ffi::gdk_window_hide(self.pointer) }
    }

    pub fn is_destroyed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_destroyed(self.pointer)) }
    }

    pub fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_visible(self.pointer)) }
    }

    pub fn is_viewable(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_viewable(self.pointer)) }
    }

    pub fn is_input_only(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_input_only(self.pointer)) }
    }

    pub fn is_shaped(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_is_shaped(self.pointer)) }
    }

    pub fn get_state(&self) -> gdk::WindowState {
        unsafe { ffi::gdk_window_get_state(self.pointer) }
    }

    pub fn withdraw(&self) {
        unsafe { ffi::gdk_window_withdraw(self.pointer) }
    }

    pub fn iconify(&self) {
        unsafe { ffi::gdk_window_iconify(self.pointer) }
    }

    pub fn deiconify(&self) {
        unsafe { ffi::gdk_window_deiconify(self.pointer) }
    }

    pub fn stick(&self) {
        unsafe { ffi::gdk_window_stick(self.pointer) }
    }

    pub fn unstick(&self) {
        unsafe { ffi::gdk_window_unstick(self.pointer) }
    }

    pub fn maximize(&self) {
        unsafe { ffi::gdk_window_maximize(self.pointer) }
    }

    pub fn unmaximize(&self) {
        unsafe { ffi::gdk_window_unmaximize(self.pointer) }
    }

    pub fn fullscreen(&self) {
        unsafe { ffi::gdk_window_fullscreen(self.pointer) }
    }

    pub fn unfullscreen(&self) {
        unsafe { ffi::gdk_window_unfullscreen(self.pointer) }
    }

    pub fn get_fullscreen_mode(&self) -> gdk::FullscreenMode {
        unsafe { ffi::gdk_window_get_fullscreen_mode(self.pointer) }
    }

    pub fn set_fullscreen_mode(&self, mode: gdk::FullscreenMode) {
        unsafe { ffi::gdk_window_set_fullscreen_mode(self.pointer, mode) }
    }

    pub fn set_keep_above(&self, setting: bool) {
        unsafe { ffi::gdk_window_set_keep_above(self.pointer, setting.to_glib()) }
    }

    pub fn set_keep_below(&self, setting: bool) {
        unsafe { ffi::gdk_window_set_keep_below(self.pointer, setting.to_glib()) }
    }

    pub fn set_opacity(&self, opacity: f64) {
        unsafe { ffi::gdk_window_set_opacity(self.pointer, opacity) }
    }

    pub fn set_composited(&self, composited: bool) {
        unsafe { ffi::gdk_window_set_composited(self.pointer, composited.to_glib()) }
    }

    pub fn get_composited(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_composited(self.pointer)) }
    }

    pub fn _move(&self, x: i32, y: i32) {
        unsafe { ffi::gdk_window_move(self.pointer, x as c_int, y as c_int) }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe { ffi::gdk_window_resize(self.pointer, width as c_int, height as c_int) }
    }

    pub fn move_resize(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::gdk_window_move_resize(self.pointer, x as c_int, y as c_int, width as c_int, height as c_int) }
    }

    pub fn scroll(&self, dx: i32, dy: i32) {
        unsafe { ffi::gdk_window_scroll(self.pointer, dx as c_int, dy as c_int) }
    }

    pub fn has_native(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_has_native(self.pointer)) }
    }

    pub fn ensure_native(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_ensure_native(self.pointer)) }
    }

    pub fn reparent(&self, new_parent: &Window, x: i32, y: i32) {
        unsafe { ffi::gdk_window_reparent(self.pointer, new_parent.pointer, x as c_int, y as c_int) }
    }

    pub fn raise(&self) {
        unsafe { ffi::gdk_window_raise(self.pointer) }
    }

    pub fn lower(&self) {
        unsafe { ffi::gdk_window_lower(self.pointer) }
    }

    pub fn restack(&self, sibling: &Window, above: bool) {
        unsafe { ffi::gdk_window_restack(self.pointer, sibling.pointer, above.to_glib()) }
    }

    pub fn focus(&self, timestamp: u32) {
        unsafe { ffi::gdk_window_focus(self.pointer, timestamp) }
    }

    pub fn register_dnd(&self) {
        unsafe { ffi::gdk_window_register_dnd(self.pointer) }
    }

    pub fn begin_resize_drag(&self, edge: gdk::WindowEdge, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe { ffi::gdk_window_begin_resize_drag(self.pointer, edge, button as c_int, root_x as c_int, root_y as c_int, timestamp) }
    }

    pub fn begin_resize_drag_for_device(&self, edge: gdk::WindowEdge, device: &gdk::Device, button: i32, root_x: i32, root_y: i32,
        timestamp: u32) {
        unsafe { ffi::gdk_window_begin_resize_drag_for_device(self.pointer, edge, device.unwrap_pointer(), button as c_int,
            root_x as c_int, root_y as c_int, timestamp) }
    }

    pub fn begin_move_drag(&self, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe { ffi::gdk_window_begin_move_drag(self.pointer, button as c_int, root_x as c_int, root_y as c_int, timestamp) }
    }

    pub fn begin_move_drag_for_device(&self, device: &gdk::Device, button: i32, root_x: i32, root_y: i32, timestamp: u32) {
        unsafe { ffi::gdk_window_begin_move_drag_for_device(self.pointer, device.unwrap_pointer(), button as c_int, root_x as c_int,
            root_y as c_int, timestamp) }
    }

    /* FIXME : I think the Event struct is missing, not just a trait is needed:
    https://developer.gnome.org/gdk3/3.14/gdk3-Event-Structures.html#GdkEvent

    // Since 3.14
    pub fn show_window_menu(&self, event: &gdk::Event) {
        unsafe { ffi::gdk_window_show_window_menu(self.pointer, event.unwrap_pointer()) }
    }*/

    pub fn constrain_size(&self, flags: gdk::WindowHints, width: i32, height: i32, new_width: &mut i32, new_height: &mut i32) {
        unsafe { ffi::gdk_window_constrain_size(self.pointer, flags, width as c_int, height as c_int, new_width as *mut c_int,
            new_height as *mut c_int) }
    }

    pub fn beep(&self) {
        unsafe { ffi::gdk_window_beep(self.pointer) }
    }

    // Since 3.10
    pub fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_window_get_scale_factor(self.pointer) }
    }

    pub fn begin_paint_rect(&self, rect: &ffi::C_GdkRectangle) {
        unsafe { ffi::gdk_window_begin_paint_rect(self.pointer, rect) }
    }

    pub fn end_paint(&self) {
        unsafe { ffi::gdk_window_end_paint(self.pointer) }
    }

    pub fn invalidate_rect(&self, rect: &ffi::C_GdkRectangle, invalidate_children: bool) {
        unsafe { ffi::gdk_window_invalidate_rect(self.pointer, rect, invalidate_children.to_glib()) }
    }

    pub fn freeze_updates(&self) {
        unsafe { ffi::gdk_window_freeze_updates(self.pointer) }
    }

    pub fn thaw_updates(&self) {
        unsafe { ffi::gdk_window_thaw_updates(self.pointer) }
    }

    pub fn process_all_updates() {
        unsafe { ffi::gdk_window_process_all_updates() }
    }

    pub fn process_updates(&self, update_children: bool) {
        unsafe { ffi::gdk_window_process_updates(self.pointer, update_children.to_glib()) }
    }

    pub fn set_debug_updates(setting: bool) {
        unsafe { ffi::gdk_window_set_debug_updates(setting.to_glib()) }
    }

    pub fn get_frame_clock(&self) -> Option<gdk::FrameClock> {
        let tmp = unsafe { ffi::gdk_window_get_frame_clock(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::FrameClock::wrap_pointer(tmp))
        }
    }

    pub fn set_user_data<T>(&self, user_data: &mut T) {
        unsafe { ffi::gdk_window_set_user_data(self.pointer, ::std::mem::transmute(user_data)) }
    }

    pub fn set_override_redirect(&self, override_redirect: bool) {
        unsafe { ffi::gdk_window_set_override_redirect(self.pointer, override_redirect.to_glib()) }
    }

    pub fn set_accept_focus(&self, accept_focus: bool) {
        unsafe { ffi::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
    }

    pub fn get_accept_focus(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_accept_focus(self.pointer)) }
    }

    pub fn set_focus_on_map(&self, focus_on_map: bool) {
        unsafe { ffi::gdk_window_set_focus_on_map(self.pointer, focus_on_map.to_glib()) }
    }

    pub fn get_focus_on_map(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_focus_on_map(self.pointer)) }
    }

    pub fn set_child_shapes(&self) {
        unsafe { ffi::gdk_window_set_child_shapes(self.pointer) }
    }

    pub fn merge_child_shapes(&self) {
        unsafe { ffi::gdk_window_merge_child_shapes(self.pointer) }
    }

    pub fn set_child_input_shapes(&self) {
        unsafe { ffi::gdk_window_set_child_input_shapes(self.pointer) }
    }

    pub fn merge_child_input_shapes(&self) {
        unsafe { ffi::gdk_window_merge_child_input_shapes(self.pointer) }
    }

    pub fn set_static_gravities(&self, use_static: bool) -> bool {
        unsafe { from_glib(ffi::gdk_window_set_static_gravities(self.pointer, use_static.to_glib())) }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            let mut title = title.to_temp_for_borrow();
            ffi::gdk_window_set_title(self.pointer, title.to_glib())
        }
    }

    pub fn set_background_rgba(&self, rgba: &ffi::C_GdkRGBA) {
        unsafe { ffi::gdk_window_set_background_rgba(self.pointer, rgba) }
    }

    pub fn set_cursor(&self, cursor: &gdk::Cursor) {
        unsafe { ffi::gdk_window_set_cursor(self.pointer, cursor.unwrap_pointer()) }
    }

    pub fn get_cursor(&self) -> Option<gdk::Cursor> {
        let tmp = unsafe { ffi::gdk_window_get_cursor(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Cursor::wrap_pointer(tmp))
        }
    }

    pub fn get_user_data<'a, T>(&'a self) -> &'a mut T {
        let mut pointer = ::std::ptr::null_mut();

        unsafe {
            ffi::gdk_window_get_user_data(self.pointer, &mut pointer);
            ::std::mem::transmute(pointer)
        }
    }

    pub fn get_geometry(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) {
        unsafe { ffi::gdk_window_get_geometry(self.pointer, x as *mut c_int, y as *mut c_int, width as *mut c_int,
            height as *mut c_int) }
    }

    pub fn set_geometry_hints(&self, geometry: &ffi::C_GdkGeometry, geom_mask: gdk::WindowHints) {
        unsafe { ffi::gdk_window_set_geometry_hints(self.pointer, geometry, geom_mask) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::gdk_window_get_width(self.pointer) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::gdk_window_get_height(self.pointer) }
    }

    pub fn set_modal_hint(&self, modal: bool) {
        unsafe { ffi::gdk_window_set_modal_hint(self.pointer, modal.to_glib()) }
    }

    pub fn get_modal_hint(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_modal_hint(self.pointer)) }
    }

    pub fn set_type_hint(&self, hint: gdk::WindowTypeHint) {
        unsafe { ffi::gdk_window_set_type_hint(self.pointer, hint) }
    }

    pub fn get_type_hint(&self) -> gdk::WindowTypeHint {
        unsafe { ffi::gdk_window_get_type_hint(self.pointer) }
    }

    pub fn set_shadow_width(&self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe { ffi::gdk_window_set_shadow_width(self.pointer, left as c_int, right as c_int, top as c_int,
            bottom as c_int) }
    }

    pub fn set_skip_taskbar_hint(&self, skips_taskbar: bool) {
        unsafe { ffi::gdk_window_set_skip_taskbar_hint(self.pointer, skips_taskbar.to_glib()) }
    }

    pub fn set_skip_pager_hint(&self, skips_pager: bool) {
        unsafe { ffi::gdk_window_set_skip_pager_hint(self.pointer, skips_pager.to_glib()) }
    }

    pub fn set_urgency_hint(&self, urgent: bool) {
        unsafe { ffi::gdk_window_set_urgency_hint(self.pointer, urgent.to_glib()) }
    }

    pub fn get_position(&self, x: &mut i32, y: &mut i32) {
        unsafe { ffi::gdk_window_get_position(self.pointer, x as *mut c_int, y as *mut c_int) }
    }

    pub fn get_root_origin(&self, x: &mut i32, y: &mut i32) {
        unsafe { ffi::gdk_window_get_root_origin(self.pointer, x as *mut c_int, y as *mut c_int) }
    }

    pub fn get_frame_extents(&self, rect: &mut ffi::C_GdkRectangle) {
        unsafe { ffi::gdk_window_get_frame_extents(self.pointer, rect) }
    }

    pub fn get_origin(&self, x: &mut i32, y: &mut i32) {
        unsafe { ffi::gdk_window_get_origin(self.pointer, x as *mut c_int, y as *mut c_int) }
    }

    pub fn get_root_coords(&self, x: i32, y: i32, root_x: &mut i32, root_y: &mut i32) {
        unsafe { ffi::gdk_window_get_root_coords(self.pointer, x as c_int, y as c_int, root_x as *mut c_int, root_y as *mut c_int) }
    }

    pub fn get_device_position(&self, device: &gdk::Device, x: &mut i32, y: &mut i32,
        mask: &mut gdk::ModifierType) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_device_position(self.pointer, device.unwrap_pointer(), x as *mut c_int, y as *mut c_int,
            mask) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_device_position_double(&self, device: &gdk::Device, x: &mut f64, y: &mut f64,
        mask: &mut gdk::ModifierType) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_device_position_double(self.pointer, device.unwrap_pointer(), x, y, mask) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_parent(&self) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_parent(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_toplevel(&self) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_toplevel(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_events(&self) -> gdk::EventMask {
        unsafe { ffi::gdk_window_get_events(self.pointer) }
    }

    pub fn set_events(&self, event_mask: gdk::EventMask) {
        unsafe { ffi::gdk_window_set_events(self.pointer, event_mask) }
    }

    pub fn set_icon_name(&self, name: &str) {
        unsafe {
            let mut name = name.to_temp_for_borrow();
            ffi::gdk_window_set_icon_name(self.pointer, name.to_glib())
        }
    }

    pub fn set_transient_for(&self, parent: &Window) {
        unsafe { ffi::gdk_window_set_transient_for(self.pointer, parent.pointer) }
    }

    pub fn set_role(&self, role: &str) {
        unsafe {
            let mut role = role.to_temp_for_borrow();
            ffi::gdk_window_set_role(self.pointer, role.to_glib())
        }
    }

    pub fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            let mut startup_id = startup_id.to_temp_for_borrow();
            ffi::gdk_window_set_startup_id(self.pointer, startup_id.to_glib())
        }
    }

    pub fn set_group(&self, leader: &Window) {
        unsafe { ffi::gdk_window_set_group(self.pointer, leader.pointer) }
    }

    pub fn get_group(&self) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_group(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn set_decorations(&self, decorations: gdk::WMDecoration) {
        unsafe { ffi::gdk_window_set_decorations(self.pointer, decorations) }
    }

    pub fn get_decorations(&self, decorations: &mut gdk::WMDecoration) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_decorations(self.pointer, decorations)) }
    }

    pub fn set_functions(&self, functions: gdk::WMFunction) {
        unsafe { ffi::gdk_window_set_functions(self.pointer, functions) }
    }

    pub fn get_default_root_window() -> Option<Window> {
        let tmp = unsafe { ffi::gdk_get_default_root_window() };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_support_multidevice(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_support_multidevice(self.pointer)) }
    }

    pub fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe { ffi::gdk_window_set_support_multidevice(self.pointer, support_multidevice.to_glib()) }
    }

    pub fn get_device_cursor(&self, device: &gdk::Device) -> Option<gdk::Cursor> {
        let tmp = unsafe { ffi::gdk_window_get_device_cursor(self.pointer, device.unwrap_pointer()) };

        if tmp.is_null() {
            None
        } else {
            Some(gdk::Cursor::wrap_pointer(tmp))
        }
    }

    pub fn set_device_cursor(&self, device: &gdk::Device, cursor: &gdk::Cursor) {
        unsafe { ffi::gdk_window_set_device_cursor(self.pointer, device.unwrap_pointer(), cursor.unwrap_pointer()) }
    }

    pub fn get_device_events(&self, device: &gdk::Device) -> gdk::EventMask {
        unsafe { ffi::gdk_window_get_device_events(self.pointer, device.unwrap_pointer()) }
    }

    pub fn set_device_events(&self, device: &gdk::Device, event_mask: gdk::EventMask) {
        unsafe { ffi::gdk_window_set_device_events(self.pointer, device.unwrap_pointer(), event_mask) }
    }

    pub fn get_source_events(&self, source: gdk::InputSource) -> gdk::EventMask {
        unsafe { ffi::gdk_window_get_source_events(self.pointer, source) }
    }

    pub fn set_source_events(&self, source: gdk::InputSource, event_mask: gdk::EventMask) {
        unsafe { ffi::gdk_window_set_source_events(self.pointer, source, event_mask) }
    }

    pub fn get_event_compression(&self) -> bool {
        unsafe { from_glib(ffi::gdk_window_get_event_compression(self.pointer)) }
    }

    pub fn set_event_compression(&self, event_compression: bool) {
        unsafe { ffi::gdk_window_set_event_compression(self.pointer, event_compression.to_glib()) }
    }

    pub fn offscreen_window_set_embedder(&self, embedder: &Window) {
        unsafe { ffi::gdk_offscreen_window_set_embedder(self.pointer, embedder.pointer) }
    }

    pub fn offscreen_window_get_embedder(&self) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_offscreen_window_get_embedder(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn geometry_changed(&self) {
        unsafe { ffi::gdk_window_geometry_changed(self.pointer) }
    }

    pub fn coords_from_parent(&self, parent_x: f64, parent_y: f64, x: &mut f64, y: &mut f64) {
        unsafe { ffi::gdk_window_coords_from_parent(self.pointer, parent_x, parent_y, x, y) }
    }

    pub fn coords_to_parent(&self, x: f64, y: f64, parent_x: &mut f64, parent_y: &mut f64) {
        unsafe { ffi::gdk_window_coords_to_parent(self.pointer, x, y, parent_x, parent_y) }
    }

    pub fn get_effective_parent(&self) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_effective_parent(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_effective_toplevel(&self) -> Option<Window> {
        let tmp = unsafe { ffi::gdk_window_get_effective_toplevel(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            Some(Window {
                pointer: tmp
            })
        }
    }

    pub fn get_drag_protocol(&self, target: &mut Window) -> gdk::DragProtocol {
        unsafe { ffi::gdk_window_get_drag_protocol(self.pointer, &mut target.pointer) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            unsafe { ffi::gdk_window_destroy(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}

impl_GObjectFunctions!(Window, C_GdkWindow);
