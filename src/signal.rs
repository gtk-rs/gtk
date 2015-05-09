// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//use std::boxed::into_raw;
use std::mem::transmute;

use glib::signal::connect;
use glib::translate::*;
use glib::{FFIGObject, ParamSpec};

use ffi::{Gboolean, C_GtkAdjustment, C_GtkTreeSelection, C_GtkTreeViewColumn};
use gdk::{
    EventAny, EventButton, EventConfigure, EventCrossing, EventExpose, EventFocus, EventGrabBroken,
    EventKey, EventMotion, EventProperty, EventProximity, EventScroll, EventWindowState,
    Rectangle, Screen,
};
use cairo::Context;

use {
    Adjustment, Button, Dialog, DirectionType, Range, ScrollType, SpinButton, StateFlags,
    TextDirection, ToolButton, Tooltip, TreeIter, TreePath, TreeSelection, TreeView,
    TreeViewColumn, Widget, WidgetHelpType,
};

/// Whether to propagate the signal to other handlers
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

impl ToGlib for Inhibit {
    type GlibType = Gboolean;

    #[inline]
    fn to_glib(&self) -> Gboolean {
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
    fn connect_size_allocate<F: Fn(Widget, &Rectangle) + 'static>(&self, f: F) -> u64;
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
        EventWindowState, Rectangle, Screen,
    };
    use cairo_ffi::cairo_t;
    use cairo::Context;
    use traits::{FFIWidget, WidgetTrait};
    use gdk_ffi::C_GdkScreen;
    use ffi::{Gboolean, C_GtkWidget, C_GtkTooltip};
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

        fn connect_size_allocate<F: Fn(Widget, &Rectangle) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &Rectangle) + 'static>> = Box::new(Box::new(f));
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

    extern "C" fn void_trampoline(this: *mut C_GtkWidget, f: &Box<Fn(Widget) + 'static>) {
        f(FFIWidget::wrap_widget(this));
    }

    extern "C" fn bool_trampoline(this: *mut C_GtkWidget, f: &Box<Fn(Widget) -> bool + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this)).to_glib()
    }

    extern "C" fn accel_trampoline(this: *mut C_GtkWidget, signal_id: c_uint,
            f: &Box<Fn(Widget, u64) -> bool + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this), signal_id as u64).to_glib()
    }

    extern "C" fn draw_trampoline(this: *mut C_GtkWidget, cr: *mut cairo_t,
            f: &Box<Fn(Widget, Context) -> Inhibit + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this), Context::wrap(cr)).to_glib()
    }

    extern "C" fn event_any_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventAny) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_button_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventButton) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_configure_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventConfigure) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_crossing_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventCrossing) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_expose_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventExpose) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_focus_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventFocus) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_grab_broken_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventGrabBroken) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_key_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventKey) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_motion_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventMotion) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_property_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventProperty) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_proximity_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventProximity) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_scroll_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventScroll) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn event_window_state_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventWindowState) -> Inhibit + 'static>) -> Gboolean {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(event)).to_glib() }
    }

    extern "C" fn direction_trampoline(this: *mut C_GtkWidget, direction: DirectionType,
            f: &Box<Fn(Widget, DirectionType) -> Inhibit + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this), direction).to_glib()
    }

    extern "C" fn direction_void_trampoline(this: *mut C_GtkWidget, direction: DirectionType,
            f: &Box<Fn(Widget, DirectionType) + 'static>) {
        f(FFIWidget::wrap_widget(this), direction);
    }

    extern "C" fn grab_trampoline(this: *mut C_GtkWidget, was_grabbed: Gboolean,
            f: &Box<Fn(Widget, bool) + 'static>) {
        f(FFIWidget::wrap_widget(this), from_glib(was_grabbed));
    }

    extern "C" fn help_trampoline(this: *mut C_GtkWidget, help_type: WidgetHelpType,
            f: &Box<Fn(Widget, WidgetHelpType) -> bool + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this), help_type).to_glib()
    }

    extern "C" fn mnemonic_trampoline(this: *mut C_GtkWidget, arg1: Gboolean,
            f: &Box<Fn(Widget, bool) -> Inhibit + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this), from_glib(arg1)).to_glib()
    }

    extern "C" fn notify_trampoline(this: *mut C_GtkWidget, pspec: *mut ParamSpec,
            f: &Box<Fn(Widget, &ParamSpec) + 'static>) {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(pspec)); }
    }

    extern "C" fn query_trampoline(this: *mut C_GtkWidget, x: c_int, y: c_int, keyboard: Gboolean,
            _tooltip: *mut C_GtkTooltip, f: &Box<Fn(Widget, i32, i32, bool, Tooltip) -> bool + 'static>)
            -> Gboolean {
        f(FFIWidget::wrap_widget(this), x, y, from_glib(keyboard), Tooltip).to_glib()
    }

    extern "C" fn rectangle_trampoline(this: *mut C_GtkWidget, allocation: *mut Rectangle,
            f: &Box<Fn(Widget, &Rectangle) + 'static>) {
        unsafe { f(FFIWidget::wrap_widget(this), transmute(allocation)); }
    }

    extern "C" fn state_trampoline(this: *mut C_GtkWidget, flags: StateFlags,
            f: &Box<Fn(Widget, StateFlags) + 'static>) {
        f(FFIWidget::wrap_widget(this), flags);
    }

    extern "C" fn screen_trampoline(this: *mut C_GtkWidget, screen: *mut C_GdkScreen,
            f: &Box<Fn(Widget, Screen) + 'static>) {
        f(FFIWidget::wrap_widget(this), Screen::wrap_pointer(screen));
    }

    extern "C" fn text_direction_trampoline(this: *mut C_GtkWidget, previous: TextDirection,
            f: &Box<Fn(Widget, TextDirection) + 'static>) {
        f(FFIWidget::wrap_widget(this), previous);
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
    use ffi::C_GtkButton;
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

    extern "C" fn void_trampoline(this: *mut C_GtkButton, f: &Box<Fn(Button) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
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
    use ffi::C_GtkToolButton;
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

    extern "C" fn void_trampoline(this: *mut C_GtkToolButton, f: &Box<Fn(ToolButton) + 'static>) {
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
    use ffi::C_GtkSpinButton;
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

    extern "C" fn void_trampoline(this: *mut C_GtkSpinButton, f: &Box<Fn(SpinButton) + 'static>) {
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
    use ffi::C_GtkDialog;
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

    extern "C" fn void_trampoline(this: *mut C_GtkDialog, f: &Box<Fn(Dialog) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn int_trampoline(this: *mut C_GtkDialog, response: c_int,
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
    use ffi::{Gboolean, C_GtkTreeIter, C_GtkTreePath, C_GtkTreeView, C_GtkTreeViewColumn};
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

    extern "C" fn void_trampoline(this: *mut C_GtkTreeView, f: &Box<Fn(TreeView) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn bool_trampoline(this: *mut C_GtkTreeView, f: &Box<Fn(TreeView) -> bool + 'static>)
            -> Gboolean {
        f(FFIWidget::wrap_widget(this as *mut _)).to_glib()
    }

    extern "C" fn bool_bool_trampoline(this: *mut C_GtkTreeView, arg1: Gboolean,
            f: &Box<Fn(TreeView, bool) -> bool + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), from_glib(arg1)).to_glib()
    }

    extern "C" fn bool3_bool_trampoline(this: *mut C_GtkTreeView, arg1: Gboolean, arg2: Gboolean,
            arg3: Gboolean, f: &Box<Fn(TreeView, bool, bool, bool) -> bool + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), from_glib(arg1), from_glib(arg2),
            from_glib(arg3)).to_glib()
    }

    extern "C" fn path_column_trampoline(this: *mut C_GtkTreeView, path: *mut C_GtkTreePath,
            column: *mut C_GtkTreeViewColumn, f: &Box<Fn(TreeView, TreePath, TreeViewColumn) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), TreePath::wrap_pointer(path),
            TreeViewColumn::wrap_pointer(column));
    }

    extern "C" fn iter_path_trampoline(this: *mut C_GtkTreeView, iter: *mut C_GtkTreeIter,
            path: *mut C_GtkTreePath, f: &Box<Fn(TreeView, &mut TreeIter, TreePath) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), &mut TreeIter::wrap_pointer(iter),
            TreePath::wrap_pointer(path));
    }

    extern "C" fn iter_path_bool_trampoline(this: *mut C_GtkTreeView, iter: *mut C_GtkTreeIter,
            path: *mut C_GtkTreePath, f: &Box<Fn(TreeView, &mut TreeIter, TreePath) -> bool + 'static>)
            -> Gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), &mut TreeIter::wrap_pointer(iter),
            TreePath::wrap_pointer(path)).to_glib()
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
    use ffi::{Gboolean, C_GtkRange};
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

    extern "C" fn void_trampoline(this: *mut C_GtkRange, f: &Box<Fn(Range) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }

    extern "C" fn adjust_trampoline(this: *mut C_GtkRange, value: c_double,
            f: &Box<Fn(Range, f64) + 'static>) {
        f(FFIWidget::wrap_widget(this as *mut _), value);
    }

    extern "C" fn change_trampoline(this: *mut C_GtkRange, scroll: ScrollType, value: c_double,
            f: &Box<Fn(Range, ScrollType, f64) -> Inhibit + 'static>) -> Gboolean {
        f(FFIWidget::wrap_widget(this as *mut _), scroll, value).to_glib()
    }

    extern "C" fn move_trampoline(this: *mut C_GtkRange, step: ScrollType,
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

extern "C" fn adjustment_trampoline(this: *mut C_GtkAdjustment, f: &Box<Fn(Adjustment) + 'static>) {
    f(Adjustment::wrap_pointer(this))
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

extern "C" fn tree_selection_trampoline(this: *mut C_GtkTreeSelection,
        f: &Box<Fn(TreeSelection) + 'static>) {
    f(TreeSelection::wrap_object(this as *mut _))
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

extern "C" fn tree_view_column_trampoline(this: *mut C_GtkTreeViewColumn,
        f: &Box<Fn(TreeViewColumn) + 'static>) {
    f(TreeViewColumn::wrap_pointer(this))
}
