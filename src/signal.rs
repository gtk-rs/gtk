// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//use std::boxed::into_raw;
use std::mem::transmute;

use glib::signal::connect;
use glib::translate::*;
use glib::{FFIGObject, ParamSpec};

use glib_ffi::gboolean;
use ffi::{GtkAdjustment, GtkTreeSelection, GtkTreeViewColumn};
use gdk::{
    EventAny,
    EventButton,
    EventConfigure,
    EventCrossing,
    EventExpose,
    EventFocus,
    EventGrabBroken,
    EventKey,
    EventMotion,
    EventProperty,
    EventProximity,
    EventScroll,
    EventWindowState,
    Screen,
};
use cairo::{Context, RectangleInt};

use {
    Adjustment,
    Button,
    Calendar,
    ComboBox,
    DeleteType,
    Dialog,
    DirectionType,
    Entry,
    MovementStep,
    Range,
    ScrollType,
    SpinButton,
    StateFlags,
    StatusIcon,
    TextDirection,
    ToolButton,
    Tooltip,
    TreeIter,
    TreePath,
    TreeSelection,
    TreeView,
    TreeViewColumn,
    Widget,
    WidgetHelpType,
};

/// Whether to propagate the signal to other handlers
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

impl ToGlib for Inhibit {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

// libstd stability workaround
unsafe fn into_raw<T>(b: Box<T>) -> *mut T { transmute(b) }

pub trait WidgetSignals {
    fn connect_notify<F: Fn(Widget, &ParamSpec) + 'static>(&self, f: F) -> u64;
    fn connect_accel_closures_changed<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_button_press_event<F: Fn(Widget, &EventButton) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_button_release_event<F: Fn(Widget, &EventButton) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_can_activate_accel<F: Fn(Widget, u64) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_child_notify<F: Fn(Widget, &ParamSpec) + 'static>(&self, f: F) -> u64;
    fn connect_composited_changed<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_configure_event<F: Fn(Widget, &EventConfigure) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_damage_event<F: Fn(Widget, &EventExpose) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_delete_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_destroy<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_destroy_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_direction_changed<F: Fn(Widget, TextDirection) + 'static>(&self, f: F) -> u64;
    fn connect_draw<F: Fn(Widget, Context) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_enter_notify_event<F: Fn(Widget, &EventCrossing) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_event_after<F: Fn(Widget, &EventAny) + 'static>(&self, f: F) -> u64;
    fn connect_focus<F: Fn(Widget, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_focus_in_event<F: Fn(Widget, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_focus_out_event<F: Fn(Widget, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_grab_broken_event<F: Fn(Widget, &EventGrabBroken) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_grab_focus<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_grab_notify<F: Fn(Widget, bool) + 'static>(&self, f: F) -> u64;
    fn connect_hide<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_key_press_event<F: Fn(Widget, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_key_release_event<F: Fn(Widget, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_keynav_failed<F: Fn(Widget, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_leave_notify_event<F: Fn(Widget, &EventCrossing) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_map<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_map_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_mnemonic_activate<F: Fn(Widget, bool) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_motion_notify_event<F: Fn(Widget, &EventMotion) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_move_focus<F: Fn(Widget, DirectionType) + 'static>(&self, f: F) -> u64;
    fn connect_popup_menu<F: Fn(Widget) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_property_notify_event<F: Fn(Widget, &EventProperty) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_proximity_in_event<F: Fn(Widget, &EventProximity) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_proximity_out_event<F: Fn(Widget, &EventProximity) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_query_tooltip<F: Fn(Widget, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_realize<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_screen_changed<F: Fn(Widget, Screen) + 'static>(&self, f: F) -> u64;
    fn connect_scroll_event<F: Fn(Widget, &EventScroll) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_show<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_show_help<F: Fn(Widget, WidgetHelpType) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_size_allocate<F: Fn(Widget, &RectangleInt) + 'static>(&self, f: F) -> u64;
    fn connect_state_flags_changed<F: Fn(Widget, StateFlags) + 'static>(&self, f: F) -> u64;
    fn connect_style_updated<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_touch_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_unmap<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_unmap_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_unrealize<F: Fn(Widget) + 'static>(&self, f: F) -> u64;
    fn connect_window_state_event<F: Fn(Widget, &EventWindowState) -> Inhibit + 'static>(&self, f: F) -> u64;
}

mod widget {
    use super::into_raw;
    use std::mem::transmute;
    use libc::{c_int, c_uint};
    use glib::{ParamSpec};
    use glib::signal::connect;
    use glib::translate::*;
    use gdk::{
        EventAny, EventButton, EventConfigure, EventCrossing, EventExpose, EventFocus,
        EventGrabBroken, EventKey, EventMotion, EventProperty, EventProximity, EventScroll,
        EventWindowState, Screen,
    };
    use cairo_ffi::cairo_t;
    use cairo::{Context, RectangleInt};
    use traits::{FFIWidget, WidgetTrait};
    use gdk_ffi::GdkScreen;
    use glib_ffi::gboolean;
    use ffi::{GtkWidget, GtkTooltip};
    use {Widget, DirectionType, StateFlags, TextDirection, Tooltip, WidgetHelpType};
    use super::Inhibit;

    impl<T: FFIWidget + WidgetTrait> super::WidgetSignals for T {
        // this is a GObject signal actually
        fn connect_notify<F: Fn(Widget, &ParamSpec) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &ParamSpec) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "notify",
                    transmute(notify_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_accel_closures_changed<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "accel-closures-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_button_press_event<F: Fn(Widget, &EventButton) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventButton) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "button-press-event",
                    transmute(event_button_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_button_release_event<F: Fn(Widget, &EventButton) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventButton) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "button-release-event",
                    transmute(event_button_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_can_activate_accel<F: Fn(Widget, u64) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, u64) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "can-activate-accel",
                    transmute(accel_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_child_notify<F: Fn(Widget, &ParamSpec) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &ParamSpec) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "child-notify",
                    transmute(notify_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_composited_changed<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "composited-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_configure_event<F: Fn(Widget, &EventConfigure) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventConfigure) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "configure-event",
                    transmute(event_configure_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_damage_event<F: Fn(Widget, &EventExpose) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventExpose) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "damage-event",
                    transmute(event_expose_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_delete_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "delete-event",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_destroy<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "destroy",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_destroy_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "destroy-event",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_direction_changed<F: Fn(Widget, TextDirection) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, TextDirection) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "direction-changed",
                    transmute(text_direction_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_draw<F: Fn(Widget, Context) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, Context) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "draw",
                    transmute(draw_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_enter_notify_event<F: Fn(Widget, &EventCrossing) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventCrossing) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "enter-notify-event",
                    transmute(event_crossing_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "event",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_event_after<F: Fn(Widget, &EventAny) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "event-after",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_focus<F: Fn(Widget, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, DirectionType) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "focus",
                    transmute(direction_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_focus_in_event<F: Fn(Widget, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventFocus) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "focus-in-event",
                    transmute(event_focus_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_focus_out_event<F: Fn(Widget, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventFocus) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "focus-out-event",
                    transmute(event_focus_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_grab_broken_event<F: Fn(Widget, &EventGrabBroken) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventGrabBroken) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "grab-broken-event",
                    transmute(event_grab_broken_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_grab_focus<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "grab-focus",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_grab_notify<F: Fn(Widget, bool) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, bool) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "grab-notify",
                    transmute(grab_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_hide<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "hide",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_keynav_failed<F: Fn(Widget, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, DirectionType) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "keynav-failed",
                    transmute(direction_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_key_press_event<F: Fn(Widget, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventKey) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "key-press-event",
                    transmute(event_key_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_key_release_event<F: Fn(Widget, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventKey) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "key-release-event",
                    transmute(event_key_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_leave_notify_event<F: Fn(Widget, &EventCrossing) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventCrossing) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "leave-notify-event",
                    transmute(event_crossing_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_map<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "map",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_map_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "map-event",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_mnemonic_activate<F: Fn(Widget, bool) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, bool) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "mnemonic-activate",
                    transmute(mnemonic_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_move_focus<F: Fn(Widget, DirectionType) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, DirectionType) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "move-focus",
                    transmute(direction_void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_motion_notify_event<F: Fn(Widget, &EventMotion) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventMotion) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "motion-notify-event",
                    transmute(event_motion_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_property_notify_event<F: Fn(Widget, &EventProperty) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventProperty) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "property-notify-event",
                    transmute(event_property_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_proximity_in_event<F: Fn(Widget, &EventProximity) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventProximity) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "proximity-in-event",
                    transmute(event_proximity_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_proximity_out_event<F: Fn(Widget, &EventProximity) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventProximity) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "proximity-out-event",
                    transmute(event_proximity_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_popup_menu<F: Fn(Widget) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "popup-menu",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_query_tooltip<F: Fn(Widget, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, i32, i32, bool, Tooltip) -> bool + 'static>> =
                    Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "query-tooltip",
                    transmute(query_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_realize<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "realize",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_screen_changed<F: Fn(Widget, Screen) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, Screen) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "screen-changed",
                    transmute(screen_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_scroll_event<F: Fn(Widget, &EventScroll) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventScroll) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "scroll-event",
                    transmute(event_scroll_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_show<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "show",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_show_help<F: Fn(Widget, WidgetHelpType) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, WidgetHelpType) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "show-help",
                    transmute(help_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_size_allocate<F: Fn(Widget, &RectangleInt) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &RectangleInt) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "size-allocate",
                    transmute(rectangle_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_state_flags_changed<F: Fn(Widget, StateFlags) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, StateFlags) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "state-flags-changed",
                    transmute(state_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_style_updated<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "style-updated",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_touch_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "touch-event",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_unmap<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "unmap",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_unmap_event<F: Fn(Widget, &EventAny) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "unmap-event",
                    transmute(event_any_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_unrealize<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "unrealize",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_window_state_event<F: Fn(Widget, &EventWindowState) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventWindowState) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "window-state-event",
                    transmute(event_window_state_trampoline), into_raw(f) as *mut _)
            }
        }

    }

    extern "C" fn void_trampoline(this: *mut GtkWidget, f: &Box<Fn(Widget) + 'static>) {
        f(FFIWidget::wrap_widget(this));
    }

    extern "C" fn bool_trampoline(this: *mut GtkWidget, f: &Box<Fn(Widget) -> bool + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this)).to_glib()
    }

    extern "C" fn accel_trampoline(this: *mut GtkWidget, signal_id: c_uint,
            f: &Box<Fn(Widget, u64) -> bool + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this), signal_id as u64).to_glib()
    }

    extern "C" fn draw_trampoline(this: *mut GtkWidget, cr: *mut cairo_t,
            f: &Box<Fn(Widget, Context) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), from_glib_none(cr)).to_glib() }
    }

    extern "C" fn event_any_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventAny) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_button_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventButton) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_configure_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventConfigure) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_crossing_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventCrossing) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_expose_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventExpose) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_focus_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventFocus) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_grab_broken_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventGrabBroken) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_key_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventKey) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_motion_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventMotion) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_property_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventProperty) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_proximity_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventProximity) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_scroll_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventScroll) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_window_state_trampoline(this: *mut GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventWindowState) -> Inhibit + 'static>) -> gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn direction_trampoline(this: *mut GtkWidget, direction: DirectionType,
            f: &Box<Fn(Widget, DirectionType) -> Inhibit + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this), direction).to_glib()
    }

    extern "C" fn direction_void_trampoline(this: *mut GtkWidget, direction: DirectionType,
            f: &Box<Fn(Widget, DirectionType) + 'static>) {
        f(FFIWidget::wrap_widget(this), direction);
    }

    extern "C" fn grab_trampoline(this: *mut GtkWidget, was_grabbed: gboolean,
            f: &Box<Fn(Widget, bool) + 'static>) {
        f(FFIWidget::wrap_widget(this), from_glib(was_grabbed));
    }

    extern "C" fn help_trampoline(this: *mut GtkWidget, help_type: WidgetHelpType,
            f: &Box<Fn(Widget, WidgetHelpType) -> bool + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this), help_type).to_glib()
    }

    extern "C" fn mnemonic_trampoline(this: *mut GtkWidget, arg1: gboolean,
            f: &Box<Fn(Widget, bool) -> Inhibit + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this), from_glib(arg1)).to_glib()
    }

    extern "C" fn notify_trampoline(this: *mut GtkWidget, pspec: *mut ParamSpec,
            f: &Box<Fn(Widget, &ParamSpec) + 'static>) {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(pspec)); }
    }

    extern "C" fn query_trampoline(this: *mut GtkWidget, x: c_int, y: c_int, keyboard: gboolean,
            _tooltip: *mut GtkTooltip, f: &Box<Fn(Widget, i32, i32, bool, Tooltip) -> bool + 'static>)
            -> gboolean {
        f(FFIWidget::wrap_widget(this), x, y, from_glib(keyboard), Tooltip).to_glib()
    }

    extern "C" fn rectangle_trampoline(this: *mut GtkWidget, allocation: *mut RectangleInt,
            f: &Box<Fn(Widget, &RectangleInt) + 'static>) {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(allocation)); }
    }

    extern "C" fn state_trampoline(this: *mut GtkWidget, flags: StateFlags,
            f: &Box<Fn(Widget, StateFlags) + 'static>) {
        f(FFIWidget::wrap_widget(this), flags);
    }

    extern "C" fn screen_trampoline(this: *mut GtkWidget, screen: *mut GdkScreen,
            f: &Box<Fn(Widget, Screen) + 'static>) {
        unsafe { f(FFIWidget::wrap_widget(this), Screen::from_glib_none(screen)); }
    }

    extern "C" fn text_direction_trampoline(this: *mut GtkWidget, previous: TextDirection,
            f: &Box<Fn(Widget, TextDirection) + 'static>) {
        f(FFIWidget::wrap_widget(this), previous);
    }

}

pub trait EntrySignals {
    fn connect_activate<F: Fn(Entry) + 'static>(&self, f: F) -> u64;
    fn connect_backspace<F: Fn(Entry) + 'static>(&self, f: F) -> u64;
    fn connect_copy_clipboard<F: Fn(Entry) + 'static>(&self, f: F) -> u64;
    fn connect_cut_clipboard<F: Fn(Entry) + 'static>(&self, f: F) -> u64;
    fn connect_paste_clipboard<F: Fn(Entry) + 'static>(&self, f: F) -> u64;
    fn connect_toggle_overwrite<F: Fn(Entry) + 'static>(&self, f: F) -> u64;
    fn connect_delete_from_cursor<F: Fn(Entry, DeleteType, i32) + 'static>(&self, f: F) -> u64;
    fn connect_move_cursor<F: Fn(Entry, MovementStep, i32, bool) + 'static>(&self, f: F) -> u64;
    fn connect_insert_at_cursor<F: Fn(Entry, &str) + 'static>(&self, f: F) -> u64;
    fn connect_preedit_changed<F: Fn(Entry, &str) + 'static>(&self, f: F) -> u64;
}

mod entry {
    use super::into_raw;
    use std::mem::transmute;
    use std::str;
    use std::ffi::CStr;
    use glib::signal::connect;
    use libc::c_char;
    use traits::{FFIWidget, EntryTrait};
    use ffi::GtkEntry;
    use {Entry, DeleteType, MovementStep};

    impl<T: FFIWidget + EntryTrait> super::EntrySignals for T {
        fn connect_activate<F: Fn(Entry) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "activate",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_backspace<F: Fn(Entry) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "backspace",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_copy_clipboard<F: Fn(Entry) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "copy_clipboard",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_cut_clipboard<F: Fn(Entry) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "cut_clipboard",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_paste_clipboard<F: Fn(Entry) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "paste_clipboard",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_toggle_overwrite<F: Fn(Entry) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "toggle_overwrite",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_delete_from_cursor<F: Fn(Entry, DeleteType, i32) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry, DeleteType, i32) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "delete_from_cursor",
                    transmute(delete_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_move_cursor<F: Fn(Entry, MovementStep, i32, bool) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry, MovementStep, i32, bool) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "move_cursor",
                    transmute(move_cursor_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_insert_at_cursor<F: Fn(Entry, &str) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry, &str) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "insert_at_cursor",
                    transmute(string_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_preedit_changed<F: Fn(Entry, &str) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Entry, &str) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "preedit_changed",
                    transmute(string_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkEntry, f: &Box<Fn(Entry) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn delete_trampoline(this: *mut GtkEntry, delete_type: DeleteType, count: i32,
                                    f: &Box<Fn(Entry, DeleteType, i32) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), delete_type, count);
    }

    extern "C" fn move_cursor_trampoline(this: *mut GtkEntry, step: MovementStep, count: i32,
                                         extend_selection: bool,
                                         f: &Box<Fn(Entry, MovementStep, i32, bool) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), step, count, extend_selection);
    }

    extern "C" fn string_trampoline(this: *mut GtkEntry, c_str: *const c_char,
                                    f: &Box<Fn(Entry, &str) + 'static>) {
        let buf = unsafe { CStr::from_ptr(c_str).to_bytes() };
        let string = str::from_utf8(buf).unwrap();
        f(FFIWidget::wrap_widget(this as *mut _), string);
    }
}

pub trait ButtonSignals {
    fn connect_activate<F: Fn(Button) + 'static>(&self, f: F) -> u64;
    fn connect_clicked<F: Fn(Button) + 'static>(&self, f: F) -> u64;
}

mod button {
    use super::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use traits::{FFIWidget, ButtonTrait};
    use ffi::GtkButton;
    use Button;

    impl<T: FFIWidget + ButtonTrait> super::ButtonSignals for T {
        fn connect_activate<F: Fn(Button) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Button) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "activate",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_clicked<F: Fn(Button) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Button) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "clicked",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkButton, f: &Box<Fn(Button) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }
}

pub trait ComboBoxSignals {
    fn connect_changed<F: Fn(ComboBox) + 'static>(&self, f: F) -> u64;
    fn connect_move_active<F: Fn(ComboBox, ScrollType) + 'static>(&self, f: F) -> u64;
    fn connect_popdown<F: Fn(ComboBox) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_popup<F: Fn(ComboBox) + 'static>(&self, f: F) -> u64;
}

mod combobox {
    use super::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::gboolean;
    use ffi::GtkComboBox;
    use traits::{FFIWidget, ComboBoxTrait};
    use {ComboBox, ScrollType};

    impl<T: FFIWidget + ComboBoxTrait> super::ComboBoxSignals for T {
        fn connect_changed<F: Fn(ComboBox) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(ComboBox) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_move_active<F: Fn(ComboBox, ScrollType) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(ComboBox, ScrollType) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "move-active",
                    transmute(move_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_popdown<F: Fn(ComboBox) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(ComboBox) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "popdown",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_popup<F: Fn(ComboBox) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(ComboBox) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "popup",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkComboBox, f: &Box<Fn(ComboBox) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn bool_trampoline(this: *mut GtkComboBox, f: &Box<Fn(ComboBox) -> bool + 'static>)
            -> gboolean {
        f(FFIWidget::wrap_widget(this as *mut _)).to_glib()
    }

    extern "C" fn move_trampoline(this: *mut GtkComboBox, scroll_type: ScrollType,
            f: &Box<Fn(ComboBox, ScrollType) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), scroll_type);
    }
}

pub trait ToolButtonSignals {
    fn connect_clicked<F: Fn(ToolButton) + 'static>(&self, f: F) -> u64;
}

mod tool_button {
    use super::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use traits::{FFIWidget, ToolButtonTrait};
    use ffi::GtkToolButton;
    use ToolButton;

    impl<T: FFIWidget + ToolButtonTrait> super::ToolButtonSignals for T {
        fn connect_clicked<F: Fn(ToolButton) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(ToolButton) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "clicked",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkToolButton, f: &Box<Fn(ToolButton) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }
}

pub trait SpinButtonSignals {
    fn connect_value_changed<F: Fn(SpinButton) + 'static>(&self, f: F) -> u64;
    fn connect_wrapped<F: Fn(SpinButton) + 'static>(&self, f: F) -> u64;
}

mod spin_button {
    use super::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use traits::FFIWidget;
    use ffi::GtkSpinButton;
    use SpinButton;

    impl super::SpinButtonSignals for SpinButton {
        fn connect_value_changed<F: Fn(SpinButton) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(SpinButton) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "value-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_wrapped<F: Fn(SpinButton) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(SpinButton) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "clicked",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkSpinButton, f: &Box<Fn(SpinButton) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }
}

pub trait DialogSignals {
    fn connect_close<F: Fn(Dialog) + 'static>(&self, f: F) -> u64;
    fn connect_response<F: Fn(Dialog, i32) + 'static>(&self, f: F) -> u64;
}

mod dialog {
    use super::into_raw;
    use std::mem::transmute;
    use libc::c_int;
    use glib::signal::connect;
    use traits::{FFIWidget, DialogTrait};
    use ffi::GtkDialog;
    use Dialog;

    impl<T: FFIWidget + DialogTrait> super::DialogSignals for T {
        fn connect_close<F: Fn(Dialog) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Dialog) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "close",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_response<F: Fn(Dialog, i32) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Dialog, i32) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "response",
                    transmute(int_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkDialog, f: &Box<Fn(Dialog) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn int_trampoline(this: *mut GtkDialog, response: c_int,
            f: &Box<Fn(Dialog, i32) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), response);
    }
}

pub trait TreeViewSignals {
    fn connect_columns_changed<F: Fn(TreeView) + 'static>(&self, f: F) -> u64;
    fn connect_cursor_changed<F: Fn(TreeView) + 'static>(&self, f: F) -> u64;
    fn connect_expand_collapse_cursor_row<F: Fn(TreeView, bool, bool, bool) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_row_activated<F: Fn(TreeView, TreePath, TreeViewColumn) + 'static>(&self, f: F) -> u64;
    fn connect_row_collapsed<F: Fn(TreeView, &mut TreeIter, TreePath) + 'static>(&self, f: F) -> u64;
    fn connect_row_expanded<F: Fn(TreeView, &mut TreeIter, TreePath) + 'static>(&self, f: F) -> u64;
    fn connect_select_all<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_select_cursor_parent<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_select_cursor_row<F: Fn(TreeView, bool) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_start_interactive_search<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_test_collapse_row<F: Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_test_expand_row<F: Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_toggle_cursor_row<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_unselect_all<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64;
}

mod tree_view {
    use super::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use traits::FFIWidget;
    use glib_ffi::gboolean;
    use ffi::{GtkTreeIter, GtkTreePath, GtkTreeView, GtkTreeViewColumn};
    use {TreeIter, TreePath, TreeView, TreeViewColumn};

    impl super::TreeViewSignals for TreeView {
        fn connect_columns_changed<F: Fn(TreeView) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "columns-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_cursor_changed<F: Fn(TreeView) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "cursor-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_expand_collapse_cursor_row<F: Fn(TreeView, bool, bool, bool) -> bool + 'static>(&self,
                f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, bool, bool, bool) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "expand-collapse-cursor-row",
                    transmute(bool3_bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_row_activated<F: Fn(TreeView, TreePath, TreeViewColumn) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, TreePath, TreeViewColumn) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "row-activated",
                    transmute(path_column_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_row_collapsed<F: Fn(TreeView, &mut TreeIter, TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, &mut TreeIter, TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "row-collapsed",
                    transmute(iter_path_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_row_expanded<F: Fn(TreeView, &mut TreeIter, TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, &mut TreeIter, TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "row-expanded",
                    transmute(iter_path_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_select_all<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "select-all",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_select_cursor_parent<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "select-cursor-parent",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_select_cursor_row<F: Fn(TreeView, bool) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, bool) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "select-cursor-row",
                    transmute(bool_bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_start_interactive_search<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "start-interactive-search",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_test_collapse_row<F: Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>> =
                    Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "test-collapse-row",
                    transmute(iter_path_bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_test_expand_row<F: Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>> =
                    Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "test-expand-row",
                    transmute(iter_path_bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_toggle_cursor_row<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "toggle-cursor-row",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_unselect_all<F: Fn(TreeView) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(TreeView) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "unselect-all",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkTreeView, f: &Box<Fn(TreeView) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn bool_trampoline(this: *mut GtkTreeView, f: &Box<Fn(TreeView) -> bool + 'static>)
            -> gboolean {
        f(FFIWidget::wrap_widget(this as *mut _)).to_glib()
    }

    extern "C" fn bool_bool_trampoline(this: *mut GtkTreeView, arg1: gboolean,
            f: &Box<Fn(TreeView, bool) -> bool + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), from_glib(arg1)).to_glib()
    }

    extern "C" fn bool3_bool_trampoline(this: *mut GtkTreeView, arg1: gboolean, arg2: gboolean,
            arg3: gboolean, f: &Box<Fn(TreeView, bool, bool, bool) -> bool + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), from_glib(arg1), from_glib(arg2),
            from_glib(arg3)).to_glib()
    }

    extern "C" fn path_column_trampoline(this: *mut GtkTreeView, path: *mut GtkTreePath,
            column: *mut GtkTreeViewColumn, f: &Box<Fn(TreeView, TreePath, TreeViewColumn) + 'static>) {
        unsafe {
            f(FFIWidget::wrap_widget(this as *mut _), TreePath::wrap_pointer(path),
                TreeViewColumn::wrap_pointer(column));
        }
    }

    extern "C" fn iter_path_trampoline(this: *mut GtkTreeView, iter: *mut GtkTreeIter,
            path: *mut GtkTreePath, f: &Box<Fn(TreeView, &mut TreeIter, TreePath) + 'static>) {
        unsafe {
            f(FFIWidget::wrap_widget(this as *mut _), &mut from_glib_borrow(iter),
                TreePath::wrap_pointer(path));
        }
    }

    extern "C" fn iter_path_bool_trampoline(this: *mut GtkTreeView, iter: *mut GtkTreeIter,
            path: *mut GtkTreePath,
            f: &Box<Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>)
            -> gboolean {
        unsafe {
            f(FFIWidget::wrap_widget(this as *mut _), &mut from_glib_borrow(iter),
                TreePath::wrap_pointer(path)).to_glib()
        }
    }
}

pub trait RangeSignals {
    fn connect_adjust_bounds<F: Fn(Range, f64) + 'static>(&self, f: F) -> u64;
    fn connect_change_value<F: Fn(Range, ScrollType, f64) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_move_slider<F: Fn(Range, ScrollType) + 'static>(&self, f: F) -> u64;
    fn connect_value_changed<F: Fn(Range) + 'static>(&self, f: F) -> u64;
}

mod range {
    use super::into_raw;
    use std::mem::transmute;
    use libc::c_double;
    use glib::signal::connect;
    use glib::translate::*;
    use traits::{FFIWidget, RangeTrait};
    use glib_ffi::gboolean;
    use ffi::{GtkRange};
    use {Range, ScrollType};
    use super::Inhibit;

    impl<T: FFIWidget + RangeTrait> super::RangeSignals for T {
        fn connect_adjust_bounds<F: Fn(Range, f64) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Range, f64) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "adjust-bounds",
                    transmute(adjust_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_change_value<F: Fn(Range, ScrollType, f64) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Range, ScrollType, f64) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "change-value",
                    transmute(change_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_move_slider<F: Fn(Range, ScrollType) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Range, ScrollType) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "move-slider",
                    transmute(move_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_value_changed<F: Fn(Range) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Range) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "value-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

    }

    extern "C" fn void_trampoline(this: *mut GtkRange, f: &Box<Fn(Range) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn adjust_trampoline(this: *mut GtkRange, value: c_double,
            f: &Box<Fn(Range, f64) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), value);
    }

    extern "C" fn change_trampoline(this: *mut GtkRange, scroll: ScrollType, value: c_double,
            f: &Box<Fn(Range, ScrollType, f64) -> Inhibit + 'static>) -> gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), scroll, value).to_glib()
    }

    extern "C" fn move_trampoline(this: *mut GtkRange, step: ScrollType,
            f: &Box<Fn(Range, ScrollType) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), step);
    }
}

impl Adjustment {
    pub fn connect_value_changed<F: Fn(Adjustment) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(Adjustment) + 'static>> = Box::new(Box::new(f));
            connect(self.unwrap_pointer() as *mut _, "value-changed",
                transmute(adjustment_trampoline), into_raw(f) as *mut _)
        }
    }
}

extern "C" fn adjustment_trampoline(this: *mut GtkAdjustment, f: &Box<Fn(Adjustment) + 'static>) {
    unsafe { f(Adjustment::wrap_pointer(this)) }
}

impl TreeSelection {
    pub fn connect_changed<F: Fn(TreeSelection) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(TreeSelection) + 'static>> = Box::new(Box::new(f));
            connect(self.unwrap_gobject() as *mut _, "changed",
                transmute(tree_selection_trampoline), into_raw(f) as *mut _)
        }
    }
}

extern "C" fn tree_selection_trampoline(this: *mut GtkTreeSelection,
        f: &Box<Fn(TreeSelection) + 'static>) {
    unsafe { f(TreeSelection::wrap_object(this as *mut _)) }
}

impl TreeViewColumn {
    pub fn connect_clicked<F: Fn(TreeViewColumn) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(TreeViewColumn) + 'static>> = Box::new(Box::new(f));
            connect(self.unwrap_pointer() as *mut _, "clicked",
                transmute(tree_view_column_trampoline), into_raw(f) as *mut _)
        }
    }
}

extern "C" fn tree_view_column_trampoline(this: *mut GtkTreeViewColumn,
        f: &Box<Fn(TreeViewColumn) + 'static>) {
    unsafe { f(TreeViewColumn::wrap_pointer(this)) }
}

#[cfg(gtk_3_16)]
mod gl_area {
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use gdk;
    use gdk_ffi;
    use ffi::GtkGLArea;
    use cast::GTK_WIDGET;
    use super::into_raw;
    use traits::FFIWidget;
    use super::Inhibit;
    use GLArea;

    impl GLArea {
        pub fn connect_render<F: Fn(GLArea, gdk::GLContext) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(GLArea, gdk::GLContext) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _,"render",
                    transmute(gl_area_trampoline), into_raw(f) as *mut _)
            }
        }

        pub fn connect_resize<F: Fn(GLArea, i32, i32) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(GLArea, i32, i32) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _,"resize",
                    transmute(gl_area_trampoline_res), into_raw(f) as *mut _)
            }
        }
    }

    #[cfg(gtk_3_16)]
    extern "C" fn gl_area_trampoline(this: *mut GtkGLArea, context: *mut gdk_ffi::GdkGLContext,
            f: &Box<Fn(GLArea, gdk::GLContext) + 'static>) {
        unsafe { f(GLArea::wrap_widget(GTK_WIDGET(this as *mut _)), from_glib_none(context)) }
    }

    #[cfg(gtk_3_16)]
    extern "C" fn gl_area_trampoline_res(this: *mut GtkGLArea, width: i32, height: i32,
            f: &Box<Fn(GLArea, i32, i32) + 'static>) {
        f(GLArea::wrap_widget(GTK_WIDGET(this as *mut _)), width, height)
    }
}

pub trait CalendarSignals {
    fn connect_day_selected<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
    fn connect_day_selected_double_click<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
    fn connect_month_changed<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
    fn connect_next_month<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
    fn connect_next_year<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
    fn connect_prev_month<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
    fn connect_prev_year<F: Fn(Calendar) + 'static>(&self, f: F) -> u64;
}

mod calendar {
    use super::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use traits::FFIWidget;
    use ffi::GtkCalendar;
    use Calendar;

    impl super::CalendarSignals for Calendar {
        fn connect_day_selected<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "day-selected",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_day_selected_double_click<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "day-selected-double-click",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_month_changed<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "month-changed",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_next_month<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "next-month",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_next_year<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "next-year",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_prev_month<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "prev-month",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_prev_year<F: Fn(Calendar) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Calendar) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "prev-year",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkCalendar, f: &Box<Fn(Calendar) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }
}

pub trait StatusIconSignals {
    fn connect_activate<F: Fn(StatusIcon) + 'static>(&self, f: F) -> u64;
    fn connect_button_press_event<F: Fn(StatusIcon, &EventButton) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_button_release_event<F: Fn(StatusIcon, &EventButton) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_popup_menu<F: Fn(StatusIcon, u32, u32) + 'static>(&self, f: F) -> u64;
    fn connect_query_tooltip<F: Fn(StatusIcon, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_scroll_event<F: Fn(StatusIcon, &EventScroll) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_size_changed<F: Fn(StatusIcon, i32) -> bool + 'static>(&self, f: F) -> u64;
}

mod status_icon {
    use super::into_raw;
    use StatusIcon;
    use libc::{c_int, c_uint};
    use std::mem::transmute;
    use ffi::{GtkStatusIcon, GtkTooltip};
    use gdk::{EventButton, EventScroll};
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::gboolean;
    use Tooltip;

    impl super::StatusIconSignals for StatusIcon {
        fn connect_activate<F: Fn(StatusIcon) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "activate",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_button_press_event<F: Fn(StatusIcon, &EventButton) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon, &EventButton) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "button-press-event",
                    transmute(event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_button_release_event<F: Fn(StatusIcon, &EventButton) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon, &EventButton) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "button-release-event",
                    transmute(event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_popup_menu<F: Fn(StatusIcon, u32, u32) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon, u32, u32) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "popup-menu",
                    transmute(popup_menu_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_query_tooltip<F: Fn(StatusIcon, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon, i32, i32, bool, Tooltip) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "query-tooltip",
                    transmute(query_tooltip_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_scroll_event<F: Fn(StatusIcon, &EventScroll) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon, &EventScroll) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "scroll-event",
                    transmute(event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_size_changed<F: Fn(StatusIcon, i32) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(StatusIcon, i32) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0 as *mut _, "size-changed",
                    transmute(size_changed_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkStatusIcon, f: &Box<Fn(StatusIcon) + 'static>) {
        unsafe {
            f(from_glib_none(this));
        }
    }

    extern "C" fn event_trampoline(this: *mut GtkStatusIcon, event: *mut EventButton, f: &Box<Fn(StatusIcon, &EventScroll) -> bool + 'static>) -> gboolean {
        unsafe {
            f(from_glib_none(this), transmute(event)).to_glib()
        }
    }

    extern "C" fn popup_menu_trampoline(this: *mut GtkStatusIcon, button: c_uint, activate_time: c_uint, f: &Box<Fn(StatusIcon, u32, u32) + 'static>) {
        unsafe {
            f(from_glib_none(this), button, activate_time);
        }
    }

    extern "C" fn query_tooltip_trampoline(this: *mut GtkStatusIcon, x: c_int, y: c_int, keyboard_mode: gboolean, _tooltip: *mut GtkTooltip, f: &Box<Fn(StatusIcon, i32, i32, bool, Tooltip) -> bool + 'static>) -> gboolean {
        unsafe {
            f(from_glib_none(this), x, y, from_glib(keyboard_mode), Tooltip).to_glib()
        }
    }

    extern "C" fn size_changed_trampoline(this: *mut GtkStatusIcon, size: c_int, f: &Box<Fn(StatusIcon, i32) -> bool + 'static>) -> gboolean {
        unsafe {
            f(from_glib_none(this), size).to_glib()
        }
    }
}
