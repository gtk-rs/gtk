// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(any(feature = "v3_8", feature = "dox"))]
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;

use glib::object::{Downcast, IsA};
use glib::signal::{SignalHandlerId, connect};
use glib::translate::*;
use glib_ffi::gboolean;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use glib_ffi::gpointer;
use gdk::{DragAction, Event, ModifierType};
#[cfg(any(feature = "v3_8", feature = "dox"))]
use gdk::FrameClock;
use gdk_ffi;
use pango;
use ffi;

use {
    DestDefaults,
    Inhibit,
    Object,
    Rectangle,
    TargetEntry,
    Widget,
};
#[cfg(any(feature = "v3_8", feature = "dox"))]
use::Continue;

pub trait WidgetExtManual {
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction);

    fn drag_source_set(&self, start_button_mask: ModifierType, targets: &[TargetEntry], actions: DragAction);

    fn intersect(&self, area: &Rectangle, intersection: Option<&mut Rectangle>) -> bool;

    fn override_font(&self, font: &pango::FontDescription);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn add_tick_callback<F>(&self, func: F) -> u32
    where
        F: FnMut(&Self, &FrameClock) -> Continue + 'static;

    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Widget> + IsA<Object>> WidgetExtManual for O {
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction) {
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let mut t = Vec::with_capacity(stashes.len());
        for stash in &stashes {
            unsafe {
                t.push(ffi::GtkTargetEntry {
                    target: (*stash.0).target,
                    flags: (*stash.0).flags,
                    info: (*stash.0).info,
                });
            }
        }
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe { ffi::gtk_drag_dest_set(self.to_glib_none().0,
                                        flags.to_glib(),
                                        t_ptr,
                                        t.len() as i32,
                                        actions.to_glib())};
    }

    fn drag_source_set(&self, start_button_mask: ModifierType, targets: &[TargetEntry], actions: DragAction) {
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let mut t = Vec::with_capacity(stashes.len());
        for stash in &stashes {
            unsafe {
                t.push(ffi::GtkTargetEntry {
                    target: (*stash.0).target,
                    flags: (*stash.0).flags,
                    info: (*stash.0).info,
                });
            }
        }
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe { ffi::gtk_drag_source_set(self.to_glib_none().0,
                                          start_button_mask.to_glib(),
                                          t_ptr,
                                          t.len() as i32,
                                          actions.to_glib())};
    }

    fn intersect(&self, area: &Rectangle, mut intersection: Option<&mut Rectangle>) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_intersect(self.to_glib_none().0, area.to_glib_none().0, intersection.to_glib_none_mut().0))
        }
    }

    fn override_font(&self, font: &pango::FontDescription) {
        unsafe {
            ffi::gtk_widget_override_font(self.to_glib_none().0, font.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn add_tick_callback<F>(&self, func: F) -> u32
    where
        F: FnMut(&Self, &FrameClock) -> Continue + 'static,
    {
        unsafe extern "C" fn add_tick_callback_trampoline<T>(
            this: *mut ffi::GtkWidget,
            frame_clock: *mut gdk_ffi::GdkFrameClock,
            func: gpointer,
        ) -> gboolean
        where T: IsA<Widget>
        {
            callback_guard!();
            let func: &RefCell<Box<FnMut(&T, &FrameClock) -> Continue + 'static>> = transmute(func);

            (&mut *func.borrow_mut())(
                &Widget::from_glib_borrow(this).downcast_unchecked(),
                &from_glib_borrow(frame_clock)
            ).to_glib()
        }

        unsafe extern "C" fn destroy_closure<T>(func: gpointer) {
            callback_guard!();
            Box::<RefCell<Box<FnMut(&T, &FrameClock) -> Continue + 'static>>>::from_raw(
                func as *mut _
            );
        }

        let add_tick_callback_trampoline = add_tick_callback_trampoline::<Self>;
        let func: Box<RefCell<Box<FnMut(&Self, &FrameClock) -> Continue + 'static>>> =
            Box::new(RefCell::new(Box::new(func)));
        let destroy_closure = destroy_closure::<Self>;
                
        unsafe {
            ffi::gtk_widget_add_tick_callback(
                self.to_glib_none().0,
                Some(add_tick_callback_trampoline),
                Box::into_raw(func) as gpointer,
                Some(destroy_closure),
            )
        }
    }

   fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
            connect(self.to_glib_none().0, "map-event",
                transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
        }
    }

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
            connect(self.to_glib_none().0, "unmap-event",
                transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn event_any_trampoline<T>(this: *mut ffi::GtkWidget,
                                             event: *mut gdk_ffi::GdkEventAny,
                                             f: &&(Fn(&T, &Event) -> Inhibit + 'static)) -> gboolean
where T: IsA<Widget> {
    callback_guard!();
    f(&Widget::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(event)).to_glib()
}
