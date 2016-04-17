// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::mem::transmute;

use glib::translate::*;

use glib_ffi::{self, gboolean, gpointer};
use gdk::{
    Event,
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
    ModifierType,
    Screen,
};
use cairo::{Context, RectangleInt};

use {
    CellEditable,
    CellRenderer,
    Continue,
    DirectionType,
    Rectangle,
    ScrollType,
    StateFlags,
    TextDirection,
    TreeIter,
    TreeModel,
    TreePath,
    TreeViewColumn,
    WidgetHelpType,
};

pub struct Tooltip;

/// Whether to propagate the signal to the default handler.
///
/// Don't inhibit default handlers without a reason, they're usually helpful.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

#[doc(hidden)]
impl ToGlib for Inhibit {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

// idle_add and timeout_add fixed to the main thread

unsafe extern "C" fn trampoline(func: gpointer) -> gboolean {
    callback_guard!();
    let func: &RefCell<Box<FnMut() -> Continue + 'static>> = transmute(func);
    (&mut *func.borrow_mut())().to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    callback_guard!();
    Box::<RefCell<Box<FnMut() -> Continue + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() -> Continue + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut() -> Continue + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// Similar to `glib::idle_add` but only callable from the main thread and
/// doesn't require `Send`.
pub fn idle_add<F>(func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        glib_ffi::g_idle_add_full(glib_ffi::G_PRIORITY_DEFAULT_IDLE, Some(trampoline),
            into_raw(func), Some(destroy_closure))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// Similar to `glib::timeout_add` but only callable from the main thread and
/// doesn't require `Send`.
pub fn timeout_add<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        glib_ffi::g_timeout_add_full(glib_ffi::G_PRIORITY_DEFAULT, interval, Some(trampoline),
            into_raw(func), Some(destroy_closure))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// Similar to `glib::timeout_add_seconds` but only callable from the main thread and
/// doesn't require `Send`.
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        glib_ffi::g_timeout_add_seconds_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            Some(trampoline), into_raw(func), Some(destroy_closure))
    }
}

pub trait WidgetSignals {
    fn connect_accel_closures_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_button_press_event<F: Fn(&Self, &EventButton) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_button_release_event<F: Fn(&Self, &EventButton) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_can_activate_accel<F: Fn(&Self, u64) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_composited_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_configure_event<F: Fn(&Self, &EventConfigure) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_damage_event<F: Fn(&Self, &EventExpose) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_delete_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_destroy_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_direction_changed<F: Fn(&Self, TextDirection) + 'static>(&self, f: F) -> u64;
    fn connect_draw<F: Fn(&Self, &Context) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_enter_notify_event<F: Fn(&Self, &EventCrossing) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_event_after<F: Fn(&Self, &Event) + 'static>(&self, f: F) -> u64;
    fn connect_focus<F: Fn(&Self, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_focus_in_event<F: Fn(&Self, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_focus_out_event<F: Fn(&Self, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_grab_broken_event<F: Fn(&Self, &EventGrabBroken) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_grab_focus<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_grab_notify<F: Fn(&Self, bool) + 'static>(&self, f: F) -> u64;
    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_key_press_event<F: Fn(&Self, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_key_release_event<F: Fn(&Self, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_keynav_failed<F: Fn(&Self, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_leave_notify_event<F: Fn(&Self, &EventCrossing) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_map<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_mnemonic_activate<F: Fn(&Self, bool) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_motion_notify_event<F: Fn(&Self, &EventMotion) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_move_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64;
    fn connect_popup_menu<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_property_notify_event<F: Fn(&Self, &EventProperty) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_proximity_in_event<F: Fn(&Self, &EventProximity) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_proximity_out_event<F: Fn(&Self, &EventProximity) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_realize<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_screen_changed<F: Fn(&Self, &Screen) + 'static>(&self, f: F) -> u64;
    fn connect_scroll_event<F: Fn(&Self, &EventScroll) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_show_help<F: Fn(&Self, WidgetHelpType) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_size_allocate<F: Fn(&Self, &RectangleInt) + 'static>(&self, f: F) -> u64;
    fn connect_state_flags_changed<F: Fn(&Self, StateFlags) + 'static>(&self, f: F) -> u64;
    fn connect_style_updated<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_touch_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_unmap<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_unrealize<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_window_state_event<F: Fn(&Self, &EventWindowState) -> Inhibit + 'static>(&self, f: F) -> u64;
}

mod widget {
    use std::mem::transmute;
    use libc::{c_int, c_uint};
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use gdk::{
        Event, EventButton, EventConfigure, EventCrossing, EventExpose, EventFocus,
        EventGrabBroken, EventKey, EventMotion, EventProperty, EventProximity, EventScroll,
        EventWindowState, Screen,
    };
    use cairo_ffi::cairo_t;
    use cairo::{Context, RectangleInt};
    use gdk_ffi::{
        GdkEventAny, GdkEventButton, GdkEventConfigure, GdkEventCrossing, GdkEventExpose,
        GdkEventFocus, GdkEventGrabBroken, GdkEventKey, GdkEventMotion, GdkEventProperty,
        GdkEventProximity, GdkEventScroll, GdkEventWindowState, GdkScreen,
    };
    use glib_ffi::gboolean;
    use ffi::{GtkWidget, GtkTooltip};
    use {Widget, DirectionType, StateFlags, TextDirection, WidgetHelpType};
    use super::Tooltip;
    use super::Inhibit;
    use {Object, IsA};

    impl<T: IsA<Widget> + IsA<Object>> super::WidgetSignals for T {
        fn connect_accel_closures_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "accel-closures-changed",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_button_press_event<F: Fn(&Self, &EventButton) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventButton) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "button-press-event",
                    transmute(event_button_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_button_release_event<F: Fn(&Self, &EventButton) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventButton) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "button-release-event",
                    transmute(event_button_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_can_activate_accel<F: Fn(&Self, u64) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, u64) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "can-activate-accel",
                    transmute(accel_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_composited_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "composited-changed",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_configure_event<F: Fn(&Self, &EventConfigure) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventConfigure) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "configure-event",
                    transmute(event_configure_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_damage_event<F: Fn(&Self, &EventExpose) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventExpose) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "damage-event",
                    transmute(event_expose_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_delete_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "delete-event",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "destroy",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_destroy_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "destroy-event",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_direction_changed<F: Fn(&Self, TextDirection) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, TextDirection) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "direction-changed",
                    transmute(text_direction_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_draw<F: Fn(&Self, &Context) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Context) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "draw",
                    transmute(draw_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_enter_notify_event<F: Fn(&Self, &EventCrossing) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventCrossing) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "enter-notify-event",
                    transmute(event_crossing_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "event",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_event_after<F: Fn(&Self, &Event) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "event-after",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_focus<F: Fn(&Self, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, DirectionType) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "focus",
                    transmute(direction_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_focus_in_event<F: Fn(&Self, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventFocus) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "focus-in-event",
                    transmute(event_focus_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_focus_out_event<F: Fn(&Self, &EventFocus) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventFocus) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "focus-out-event",
                    transmute(event_focus_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_grab_broken_event<F: Fn(&Self, &EventGrabBroken) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventGrabBroken) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "grab-broken-event",
                    transmute(event_grab_broken_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_grab_focus<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "grab-focus",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_grab_notify<F: Fn(&Self, bool) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, bool) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "grab-notify",
                    transmute(grab_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "hide",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_keynav_failed<F: Fn(&Self, DirectionType) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, DirectionType) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "keynav-failed",
                    transmute(direction_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_key_press_event<F: Fn(&Self, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventKey) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "key-press-event",
                    transmute(event_key_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_key_release_event<F: Fn(&Self, &EventKey) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventKey) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "key-release-event",
                    transmute(event_key_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_leave_notify_event<F: Fn(&Self, &EventCrossing) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventCrossing) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "leave-notify-event",
                    transmute(event_crossing_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_map<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "map",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "map-event",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_mnemonic_activate<F: Fn(&Self, bool) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, bool) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "mnemonic-activate",
                    transmute(mnemonic_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_move_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, DirectionType) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "move-focus",
                    transmute(direction_void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_motion_notify_event<F: Fn(&Self, &EventMotion) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventMotion) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "motion-notify-event",
                    transmute(event_motion_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_property_notify_event<F: Fn(&Self, &EventProperty) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventProperty) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "property-notify-event",
                    transmute(event_property_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_proximity_in_event<F: Fn(&Self, &EventProximity) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventProximity) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "proximity-in-event",
                    transmute(event_proximity_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_proximity_out_event<F: Fn(&Self, &EventProximity) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventProximity) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "proximity-out-event",
                    transmute(event_proximity_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_popup_menu<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "popup-menu",
                    transmute(bool_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, i32, i32, bool, Tooltip) -> bool + 'static>> =
                    Box::new(Box::new(f));
                connect(self.to_glib_none().0, "query-tooltip",
                    transmute(query_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_realize<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "realize",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_screen_changed<F: Fn(&Self, &Screen) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Screen) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "screen-changed",
                    transmute(screen_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_scroll_event<F: Fn(&Self, &EventScroll) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventScroll) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "scroll-event",
                    transmute(event_scroll_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "show",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_show_help<F: Fn(&Self, WidgetHelpType) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, WidgetHelpType) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "show-help",
                    transmute(help_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_size_allocate<F: Fn(&Self, &RectangleInt) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &RectangleInt) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "size-allocate",
                    transmute(rectangle_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_state_flags_changed<F: Fn(&Self, StateFlags) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, StateFlags) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "state-flags-changed",
                    transmute(state_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_style_updated<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "style-updated",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_touch_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "touch-event",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_unmap<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "unmap",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "unmap-event",
                    transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_unrealize<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "unrealize",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_window_state_event<F: Fn(&Self, &EventWindowState) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventWindowState) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "window-state-event",
                    transmute(event_window_state_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

    }

    unsafe extern "C" fn void_trampoline<T>(this: *mut GtkWidget, f: &Box<Fn(&T) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked());
    }

    unsafe extern "C" fn bool_trampoline<T>(this: *mut GtkWidget, f: &Box<Fn(&T) -> bool + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked()).to_glib()
    }

    unsafe extern "C" fn accel_trampoline<T>(this: *mut GtkWidget, signal_id: c_uint,
            f: &Box<Fn(&T, u64) -> bool + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), signal_id as u64).to_glib()
    }

    unsafe extern "C" fn draw_trampoline<T>(this: *mut GtkWidget, cr: *mut cairo_t,
            f: &Box<Fn(&T, &Context) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(cr)).to_glib()
    }

    unsafe extern "C" fn event_any_trampoline<T>(this: *mut GtkWidget, event: *mut GdkEventAny,
            f: &Box<Fn(&T, &Event) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_button_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventButton,
            f: &Box<Fn(&T, &EventButton) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_configure_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventConfigure,
            f: &Box<Fn(&T, &EventConfigure) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_crossing_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventCrossing,
            f: &Box<Fn(&T, &EventCrossing) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_expose_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventExpose,
            f: &Box<Fn(&T, &EventExpose) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_focus_trampoline<T>(this: *mut GtkWidget, event: *mut GdkEventFocus,
            f: &Box<Fn(&T, &EventFocus) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_grab_broken_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventGrabBroken,
            f: &Box<Fn(&T, &EventGrabBroken) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_key_trampoline<T>(this: *mut GtkWidget, event: *mut GdkEventKey,
            f: &Box<Fn(&T, &EventKey) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_motion_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventMotion,
            f: &Box<Fn(&T, &EventMotion) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_property_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventProperty,
            f: &Box<Fn(&T, &EventProperty) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_proximity_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventProximity,
            f: &Box<Fn(&T, &EventProximity) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_scroll_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventScroll,
            f: &Box<Fn(&T, &EventScroll) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn event_window_state_trampoline<T>(this: *mut GtkWidget,
            event: *mut GdkEventWindowState,
            f: &Box<Fn(&T, &EventWindowState) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
    }

    unsafe extern "C" fn direction_trampoline<T>(this: *mut GtkWidget, direction: DirectionType,
            f: &Box<Fn(&T, DirectionType) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), direction).to_glib()
    }

    unsafe extern "C" fn direction_void_trampoline<T>(this: *mut GtkWidget, direction: DirectionType,
            f: &Box<Fn(&T, DirectionType) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), direction);
    }

    unsafe extern "C" fn grab_trampoline<T>(this: *mut GtkWidget, was_grabbed: gboolean,
            f: &Box<Fn(&T, bool) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), from_glib(was_grabbed));
    }

    unsafe extern "C" fn help_trampoline<T>(this: *mut GtkWidget, help_type: WidgetHelpType,
            f: &Box<Fn(&T, WidgetHelpType) -> bool + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), help_type).to_glib()
    }

    unsafe extern "C" fn mnemonic_trampoline<T>(this: *mut GtkWidget, arg1: gboolean,
            f: &Box<Fn(&T, bool) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), from_glib(arg1)).to_glib()
    }

    unsafe extern "C" fn query_trampoline<T>(this: *mut GtkWidget, x: c_int, y: c_int,
        keyboard: gboolean, _tooltip: *mut GtkTooltip,
        f: &Box<Fn(&T, i32, i32, bool, Tooltip) -> bool + 'static>) -> gboolean
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), x, y, from_glib(keyboard), Tooltip).to_glib()
    }

    unsafe extern "C" fn rectangle_trampoline<T>(this: *mut GtkWidget, allocation: *mut RectangleInt,
            f: &Box<Fn(&T, &RectangleInt) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), transmute(allocation));
    }

    unsafe extern "C" fn state_trampoline<T>(this: *mut GtkWidget, flags: StateFlags,
            f: &Box<Fn(&T, StateFlags) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), flags);
    }

    unsafe extern "C" fn screen_trampoline<T>(this: *mut GtkWidget, screen: *mut GdkScreen,
            f: &Box<Fn(&T, &Screen) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(screen));
    }

    unsafe extern "C" fn text_direction_trampoline<T>(this: *mut GtkWidget, previous: TextDirection,
            f: &Box<Fn(&T, TextDirection) + 'static>)
    where T: IsA<Widget> {
        callback_guard!();
        f(&Widget::from_glib_none(this).downcast_unchecked(), previous);
    }
}

pub trait CellRendererToggleSignals {
    fn connect_toggled<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> u64;
}

mod cell_renderer_toggle {
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use libc::c_char;
    use ffi::{GtkCellRendererToggle, gtk_tree_path_new_from_string};
    use {CellRendererToggle, TreePath};

    impl super::CellRendererToggleSignals for CellRendererToggle {
        fn connect_toggled<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "toggled",
                    transmute(string_path_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn string_path_trampoline(this: *mut GtkCellRendererToggle, c_str_path: *const c_char,
            f: &Box<Fn(&CellRendererToggle, &TreePath) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(c_str_path));
        f(&from_glib_none(this), &path);
    }
}

pub trait TreeViewSignals {
    fn connect_columns_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_cursor_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_expand_collapse_cursor_row<F: Fn(&Self, bool, bool, bool) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_row_activated<F: Fn(&Self, &TreePath, &TreeViewColumn) + 'static>(&self, f: F) -> u64;
    fn connect_row_collapsed<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64;
    fn connect_row_expanded<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64;
    fn connect_select_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_select_cursor_parent<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_select_cursor_row<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_start_interactive_search<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_test_collapse_row<F: Fn(&Self, &TreeIter, &TreePath) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_test_expand_row<F: Fn(&Self, &TreeIter, &TreePath) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_toggle_cursor_row<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_unselect_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
}

mod tree_view {
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::gboolean;
    use ffi::{GtkTreeIter, GtkTreePath, GtkTreeView, GtkTreeViewColumn};
    use {TreeIter, TreePath, TreeView, TreeViewColumn};

    impl super::TreeViewSignals for TreeView {
        fn connect_columns_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "columns-changed",
                    transmute(void_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_cursor_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "cursor-changed",
                    transmute(void_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_expand_collapse_cursor_row<F: Fn(&Self, bool, bool, bool) -> bool + 'static>(&self,
                f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, bool, bool, bool) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "expand-collapse-cursor-row",
                    transmute(bool3_bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_row_activated<F: Fn(&Self, &TreePath, &TreeViewColumn) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreePath, &TreeViewColumn) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "row-activated",
                    transmute(path_column_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_row_collapsed<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "row-collapsed",
                    transmute(iter_path_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_row_expanded<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "row-expanded",
                    transmute(iter_path_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_select_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "select-all",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_select_cursor_parent<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "select-cursor-parent",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_select_cursor_row<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, bool) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "select-cursor-row",
                    transmute(bool_bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_start_interactive_search<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "start-interactive-search",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_test_collapse_row<F: Fn(&Self, &TreeIter, &TreePath) -> bool + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) -> bool + 'static>> =
                    Box::new(Box::new(f));
                connect(self.to_glib_none().0, "test-collapse-row",
                    transmute(iter_path_bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_test_expand_row<F: Fn(&Self, &TreeIter, &TreePath) -> bool + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) -> bool + 'static>> =
                    Box::new(Box::new(f));
                connect(self.to_glib_none().0, "test-expand-row",
                    transmute(iter_path_bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_toggle_cursor_row<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "toggle-cursor-row",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_unselect_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "unselect-all",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn void_trampoline(this: *mut GtkTreeView, f: &Box<Fn(&TreeView) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this));
    }

    unsafe extern "C" fn bool_trampoline(this: *mut GtkTreeView, f: &Box<Fn(&TreeView) -> bool + 'static>)
            -> gboolean {
        callback_guard!();
        f(&from_glib_none(this)).to_glib()
    }

    unsafe extern "C" fn bool_bool_trampoline(this: *mut GtkTreeView, arg1: gboolean,
            f: &Box<Fn(&TreeView, bool) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), from_glib(arg1)).to_glib()
    }

    unsafe extern "C" fn bool3_bool_trampoline(this: *mut GtkTreeView, arg1: gboolean, arg2: gboolean,
            arg3: gboolean, f: &Box<Fn(&TreeView, bool, bool, bool) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), from_glib(arg1), from_glib(arg2),
            from_glib(arg3)).to_glib()
    }

    unsafe extern "C" fn path_column_trampoline(this: *mut GtkTreeView, path: *mut GtkTreePath,
            column: *mut GtkTreeViewColumn,
            f: &Box<Fn(&TreeView, &TreePath, &TreeViewColumn) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), &from_glib_borrow(path), &from_glib_none(column));
    }

    unsafe extern "C" fn iter_path_trampoline(this: *mut GtkTreeView, iter: *mut GtkTreeIter,
            path: *mut GtkTreePath, f: &Box<Fn(&TreeView, &TreeIter, &TreePath) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path));
    }

    unsafe extern "C" fn iter_path_bool_trampoline(this: *mut GtkTreeView, iter: *mut GtkTreeIter,
            path: *mut GtkTreePath,
            f: &Box<Fn(&TreeView, &TreeIter, &TreePath) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path)).to_glib()
    }
}

pub trait RangeSignals {
    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> u64;
    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> Inhibit + 'static>(&self, f: F) -> u64;
    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> u64;
    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

mod range {
    use std::mem::transmute;
    use libc::c_double;
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::gboolean;
    use ffi::{GtkRange};
    use {Object, Range, ScrollType, IsA};
    use super::Inhibit;

    impl<T: IsA<Range> + IsA<Object>> super::RangeSignals for T {
        fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, f64) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "adjust-bounds",
                    transmute(adjust_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, ScrollType, f64) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "change-value",
                    transmute(change_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, ScrollType) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "move-slider",
                    transmute(move_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "value-changed",
                    transmute(void_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

    }

    unsafe extern "C" fn void_trampoline<T>(this: *mut GtkRange, f: &Box<Fn(&T) + 'static>)
    where T: IsA<Range> {
        callback_guard!();
        f(&Range::from_glib_none(this).downcast_unchecked());
    }

    unsafe extern "C" fn adjust_trampoline<T>(this: *mut GtkRange, value: c_double,
        f: &Box<Fn(&T, f64) + 'static>)
    where T: IsA<Range> {
        callback_guard!();
        f(&Range::from_glib_none(this).downcast_unchecked(), value);
    }

    unsafe extern "C" fn change_trampoline<T>(this: *mut GtkRange, scroll: ScrollType,
        value: c_double, f: &Box<Fn(&T, ScrollType, f64) -> Inhibit + 'static>) -> gboolean
    where T: IsA<Range> {
        callback_guard!();
        f(&Range::from_glib_none(this).downcast_unchecked(), scroll, value).to_glib()
    }

    unsafe extern "C" fn move_trampoline<T>(this: *mut GtkRange, step: ScrollType,
            f: &Box<Fn(&T, ScrollType) + 'static>)
    where T: IsA<Range> {
        callback_guard!();
        f(&Range::from_glib_none(this).downcast_unchecked(), step);
    }
}

#[cfg(feature = "v3_16")]
mod gl_area {
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use gdk;
    use gdk_ffi;
    use ffi::GtkGLArea;
    use super::Inhibit;
    use GLArea;

    impl GLArea {
        pub fn connect_create_context<F: Fn(&Self) -> gdk::GLContext + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> gdk::GLContext + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0,"create-context",
                    transmute(gl_context_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        pub fn connect_render<F: Fn(&Self, gdk::GLContext) -> Inhibit + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, gdk::GLContext) -> Inhibit + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0,"render",
                    transmute(gl_area_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        pub fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, i32, i32) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0,"resize",
                    transmute(gl_area_trampoline_res), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn gl_context_trampoline(this: *mut GtkGLArea,
            f: &Box<Fn(&GLArea) -> gdk::GLContext + 'static>) -> *mut gdk_ffi::GdkGLContext {
        callback_guard!();
        f(&from_glib_none(this)).to_glib_full()
    }

    unsafe extern "C" fn gl_area_trampoline(this: *mut GtkGLArea, context: *mut gdk_ffi::GdkGLContext,
            f: &Box<Fn(&GLArea, gdk::GLContext) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), from_glib_none(context))
    }

    unsafe extern "C" fn gl_area_trampoline_res(this: *mut GtkGLArea, width: i32, height: i32,
            f: &Box<Fn(&GLArea, i32, i32) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), width, height)
    }
}

pub trait StatusIconSignals {
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_button_press_event<F: Fn(&Self, &EventButton) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_button_release_event<F: Fn(&Self, &EventButton) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64;
    fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_scroll_event<F: Fn(&Self, &EventScroll) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> u64;
}

mod status_icon {
    use StatusIcon;
    use libc::{c_int, c_uint};
    use std::mem::transmute;
    use ffi::{GtkStatusIcon, GtkTooltip};
    use gdk::{EventButton, EventScroll};
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::gboolean;
    use super::Tooltip;

    impl super::StatusIconSignals for StatusIcon {
        fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "activate",
                    transmute(void_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_button_press_event<F: Fn(&Self, &EventButton) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventButton) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "button-press-event",
                    transmute(event_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_button_release_event<F: Fn(&Self, &EventButton) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventButton) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "button-release-event",
                    transmute(event_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, u32, u32) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "popup-menu",
                    transmute(popup_menu_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, Tooltip) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, i32, i32, bool, Tooltip) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "query-tooltip",
                    transmute(query_tooltip_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_scroll_event<F: Fn(&Self, &EventScroll) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &EventScroll) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "scroll-event",
                    transmute(event_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, i32) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "size-changed",
                    transmute(size_changed_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn void_trampoline(this: *mut GtkStatusIcon, f: &Box<Fn(&StatusIcon) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this));
    }

    unsafe extern "C" fn event_trampoline(this: *mut GtkStatusIcon, event: *mut EventButton,
            f: &Box<Fn(&StatusIcon, &EventScroll) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), transmute(event)).to_glib()
    }

    unsafe extern "C" fn popup_menu_trampoline(this: *mut GtkStatusIcon, button: c_uint,
            activate_time: c_uint, f: &Box<Fn(&StatusIcon, u32, u32) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), button, activate_time);
    }

    unsafe extern "C" fn query_tooltip_trampoline(this: *mut GtkStatusIcon, x: c_int, y: c_int,
            keyboard_mode: gboolean, _tooltip: *mut GtkTooltip,
            f: &Box<Fn(&StatusIcon, i32, i32, bool, Tooltip) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), x, y, from_glib(keyboard_mode), Tooltip).to_glib()
    }

    unsafe extern "C" fn size_changed_trampoline(this: *mut GtkStatusIcon, size: c_int,
            f: &Box<Fn(&StatusIcon, i32) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), size).to_glib()
    }
}

pub trait CellAreaSignals {
    fn connect_add_editable<F>(&self, add_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static;
    fn connect_apply_attributes<F>(&self, apply_attributes_func: F) -> u64
        where F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static;
    fn connect_focus_changed<F>(&self, focus_changed_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, TreePath) + 'static;
    fn connect_remove_editable<F>(&self, remove_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable) + 'static;
}

mod cell_area {
    use CellArea;
    use CellEditable;
    use CellRenderer;
    use TreeIter;
    use TreeModel;
    use TreePath;
    use Rectangle;
    use std::mem::transmute;
    use ffi::{GtkCellArea, GtkCellEditable, GtkCellRenderer, GtkTreeIter, GtkTreeModel};
    use ffi::gtk_tree_path_new_from_string;
    use gdk_ffi::GdkRectangle;
    use glib_ffi::gboolean;
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use IsA;
    use libc::c_char;

    impl super::CellAreaSignals for CellArea {
        fn connect_add_editable<F>(&self, add_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static>> =
                    Box::new(Box::new(add_editable_func));
                connect(self.to_glib_none().0, "add-editable",
                    transmute(add_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_apply_attributes<F>(&self, apply_attributes_func: F) -> u64
        where F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>> =
                    Box::new(Box::new(apply_attributes_func));
                connect(self.to_glib_none().0, "apply-attributes",
                    transmute(apply_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_focus_changed<F>(&self, focus_changed_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellRenderer, TreePath) + 'static>> =
                    Box::new(Box::new(focus_changed_func));
                connect(self.to_glib_none().0, "focus-changed",
                    transmute(focus_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_remove_editable<F>(&self, remove_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellRenderer, &CellEditable) + 'static>> =
                    Box::new(Box::new(remove_editable_func));
                connect(self.to_glib_none().0, "remove-editable",
                    transmute(remove_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn add_trampoline<T>(this: *mut GtkCellArea,
                                           renderer: *mut GtkCellRenderer,
                                           editable: *mut GtkCellEditable,
                                           cell_area: *mut GdkRectangle,
                                           path: *const c_char,
                                           f: &Box<Fn(&T, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &CellRenderer::from_glib_none(renderer),
          &CellEditable::from_glib_none(editable),
          &Rectangle { x: (*cell_area).x, y: (*cell_area).y, width: (*cell_area).width, height: (*cell_area).height },
          path);
    }

    unsafe extern "C" fn apply_trampoline<T>(this: *mut GtkCellArea,
                                             model: *mut GtkTreeModel,
                                             iter: *mut GtkTreeIter,
                                             is_expander: gboolean,
                                             is_expanded: gboolean,
                                             f: &Box<Fn(&T, &TreeModel, &TreeIter, bool, bool) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &TreeModel::from_glib_none(model),
          &TreeIter::from_glib_borrow(iter),
          from_glib(is_expander),
          from_glib(is_expanded));
    }

    unsafe extern "C" fn focus_trampoline<T>(this: *mut GtkCellArea,
                                             renderer: *mut GtkCellRenderer,
                                             path: *const c_char,
                                             f: &Box<Fn(&T, &CellRenderer, TreePath) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &CellRenderer::from_glib_none(renderer),
          path);
    }

    unsafe extern "C" fn remove_trampoline<T>(this: *mut GtkCellArea,
                                              renderer: *mut GtkCellRenderer,
                                              editable: *mut GtkCellEditable,
                                              f: &Box<Fn(&T, &CellRenderer, &CellEditable) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &CellRenderer::from_glib_none(renderer),
          &CellEditable::from_glib_none(editable));
    }
}

pub trait CellRendererAccelSignals {
    fn connect_accel_cleared<F>(&self, accel_cleared_func: F) -> u64
        where F: Fn(&Self, TreePath) + 'static;
    fn connect_accel_edited<F>(&self, accel_edited_func: F) -> u64
        where F: Fn(&Self, TreePath, u32, ModifierType, u32) + 'static;
}

mod cell_renderer_accel {
    use CellRendererAccel;
    use TreePath;
    use std::mem::transmute;
    use ffi::GtkCellRendererAccel;
    use glib::signal::connect;
    use glib::translate::*;
    use gdk::ModifierType;
    use ffi::gtk_tree_path_new_from_string;
    use gdk_ffi::GdkModifierType;
    use libc::{c_char, c_uint};

    impl super::CellRendererAccelSignals for CellRendererAccel {
        fn connect_accel_cleared<F>(&self, accel_cleared_func: F) -> u64
        where F: Fn(&Self, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, TreePath) + 'static>> =
                    Box::new(Box::new(accel_cleared_func));
                connect(self.to_glib_none().0, "accel-cleared",
                    transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_accel_edited<F>(&self, accel_edited_func: F) -> u64
        where F: Fn(&Self, TreePath, u32, ModifierType, u32) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, TreePath, u32, ModifierType, u32) + 'static>> =
                    Box::new(Box::new(accel_edited_func));
                connect(self.to_glib_none().0, "accel-edited",
                    transmute(e_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline(this: *mut GtkCellRendererAccel,
                                    path: *const c_char,
                                    f: &Box<Fn(&CellRendererAccel, TreePath) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRendererAccel::from_glib_none(this), path);
    }

    unsafe extern "C" fn e_trampoline(this: *mut GtkCellRendererAccel,
                                      path: *const c_char,
                                      accel_key: c_uint,
                                      accel_mods: GdkModifierType,
                                      hardware_keycode: c_uint,
                                      f: &Box<Fn(&CellRendererAccel, TreePath, u32, ModifierType, u32) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRendererAccel::from_glib_none(this),
           path,
           accel_key,
           from_glib(accel_mods),
           hardware_keycode);
    }
}

pub trait CellRendererComboSignals {
    fn connect_changed<F>(&self, changed_func: F) -> u64
        where F: Fn(&Self, TreePath, &TreeIter) + 'static;
}

mod cell_renderer_combo {
    use CellRendererCombo;
    use TreeIter;
    use TreePath;
    use std::mem::transmute;
    use ffi::{GtkCellRendererCombo, GtkTreeIter};
    use glib::signal::connect;
    use glib::translate::*;
    use ffi::gtk_tree_path_new_from_string;
    use libc::c_char;

    impl super::CellRendererComboSignals for CellRendererCombo {
        fn connect_changed<F>(&self, accel_cleared_func: F) -> u64
        where F: Fn(&Self, TreePath, &TreeIter) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, TreePath, &TreeIter) + 'static>> =
                    Box::new(Box::new(accel_cleared_func));
                connect(self.to_glib_none().0, "changed",
                    transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline(this: *mut GtkCellRendererCombo,
                                    path: *const c_char,
                                    new_iter: *mut GtkTreeIter,
                                    f: &Box<Fn(&CellRendererCombo, TreePath, &TreeIter) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRendererCombo::from_glib_none(this), path, &from_glib_borrow(new_iter));
    }
}

pub trait CellRendererSignals {
    fn connect_editing_canceled<F>(&self, editing_canceled_func: F) -> u64
        where F: Fn(&Self) + 'static;
    fn connect_editing_started<F>(&self, editing_started_func: F) -> u64
        where F: Fn(&Self, &CellEditable, TreePath) + 'static;
}

mod cell_renderer {
    use CellEditable;
    use CellRenderer;
    use Object;
    use TreePath;
    use std::mem::transmute;
    use ffi::{GtkCellEditable, GtkCellRenderer};
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use IsA;
    use ffi::gtk_tree_path_new_from_string;
    use libc::c_char;

    impl<T: IsA<CellRenderer> + IsA<Object>> super::CellRendererSignals for T {
        fn connect_editing_canceled<F>(&self, editing_canceled_func: F) -> u64
        where F: Fn(&Self) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> =
                    Box::new(Box::new(editing_canceled_func));
                connect(self.to_glib_none().0, "editing-canceled",
                    transmute(trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_editing_started<F>(&self, editing_started_func: F) -> u64
        where F: Fn(&Self, &CellEditable, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellEditable, TreePath) + 'static>> =
                    Box::new(Box::new(editing_started_func));
                connect(self.to_glib_none().0, "editing-started",
                    transmute(start_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline<T>(this: *mut GtkCellRenderer,
                                       f: &Box<Fn(&T) + 'static>)
    where T: IsA<CellRenderer> {
        callback_guard!();
        f(&CellRenderer::from_glib_none(this).downcast_unchecked());
    }

    unsafe extern "C" fn start_trampoline<T>(this: *mut GtkCellRenderer,
                                             editable: *mut GtkCellEditable,
                                             path: *const c_char,
                                             f: &Box<Fn(&T, &CellEditable, TreePath) + 'static>)
    where T: IsA<CellRenderer> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRenderer::from_glib_none(this).downcast_unchecked(),
          &CellEditable::from_glib_none(editable),
          path);
    }
}

pub trait CellRendererTextSignals {
    fn connect_edited<F: Fn(&Self, &TreePath, &str) + 'static>(&self, f: F) -> u64;
}

mod cell_renderer_text {
    use Object;
    use std::mem::transmute;
    use std::str;
    use glib::signal::connect;
    use glib::translate::*;
    use libc::c_char;
    use std::ffi::CStr;
    use glib::object::Downcast;
    use IsA;
    use ffi::{GtkCellRendererText, gtk_tree_path_new_from_string};
    use {CellRendererText, TreePath};

    impl<T: IsA<CellRendererText> + IsA<Object>> super::CellRendererTextSignals for T {
        fn connect_edited<F: Fn(&Self, &TreePath, &str) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreePath, &str) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "edited",
                    transmute(trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline<T>(this: *mut GtkCellRendererText, path: *const c_char,
        new_text: *const c_char, f: &Box<Fn(&T, &TreePath, &str) + 'static>)
    where T: IsA<CellRendererText> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        let buf = CStr::from_ptr(new_text).to_bytes();
        let new_text = str::from_utf8(buf).unwrap();
        f(&CellRendererText::from_glib_none(this).downcast_unchecked(), &path, new_text);
    }
}

pub trait EditableSignals {
    fn connect_changed<F>(&self, changed_func: F) -> u64
        where F: Fn(&Self) + 'static;
    fn connect_delete_text<F>(&self, delete_text_func: F) -> u64
        where F: Fn(&Self, i32, i32) + 'static;
    fn connect_insert_text<F>(&self, insert_text_func: F) -> u64
        where F: Fn(&Self, &str, &mut i32) + 'static;
}

mod editable {
    use Editable;
    use Object;
    use std::mem::transmute;
    use ffi::GtkEditable;
    use glib::signal::connect;
    use glib::translate::*;
    use IsA;
    use libc::{c_char, c_int, c_uchar};
    use std::ffi::CStr;
    use std::str;
    use glib::object::Downcast;
    use std::slice;

    impl<T: IsA<Editable> + IsA<Object>> super::EditableSignals for T {
        fn connect_changed<F>(&self, changed_func: F) -> u64
        where F: Fn(&Self) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> =
                    Box::new(Box::new(changed_func));
                connect(self.to_glib_none().0, "changed",
                    transmute(trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_delete_text<F>(&self, delete_text_func: F) -> u64
        where F: Fn(&Self, i32, i32) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, i32, i32) + 'static>> =
                    Box::new(Box::new(delete_text_func));
                connect(self.to_glib_none().0, "delete-text",
                    transmute(delete_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_insert_text<F>(&self, insert_text_func: F) -> u64
        where F: Fn(&Self, &str, &mut i32) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &str, &mut i32) + 'static>> =
                    Box::new(Box::new(insert_text_func));
                connect(self.to_glib_none().0, "insert-text",
                    transmute(insert_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline<T>(this: *mut GtkEditable,
                                       f: &Box<Fn(&T) + 'static>)
    where T: IsA<Editable> {
        callback_guard!();
        f(&Editable::from_glib_none(this).downcast_unchecked());
    }

    unsafe extern "C" fn delete_trampoline<T>(this: *mut GtkEditable,
                                              start_pos: c_int,
                                              end_pos: c_int,
                                              f: &Box<Fn(&T, i32, i32) + 'static>)
    where T: IsA<Editable> {
        callback_guard!();
        f(&Editable::from_glib_none(this).downcast_unchecked(), start_pos, end_pos);
    }

    unsafe extern "C" fn insert_trampoline<T>(this: *mut GtkEditable,
                                              new_text: *mut c_char,
                                              new_text_length: c_int,
                                              position: *mut c_int,
                                              f: &Box<Fn(&T, &str, &mut i32) + 'static>)
    where T: IsA<Editable> {
        callback_guard!();
        let buf = if new_text_length != -1 {
            slice::from_raw_parts(new_text as *mut c_uchar,
                                  new_text_length as usize)
        } else {
            CStr::from_ptr(new_text).to_bytes()
        };
        let string = str::from_utf8(buf).unwrap();
        f(&Editable::from_glib_none(this).downcast_unchecked(),
          string,
          transmute(position));
    }
}
