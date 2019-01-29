// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(any(feature = "v3_8", feature = "dox"))]
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;

use glib::object::{Cast, IsA};
use glib::signal::{SignalHandlerId, connect_raw};
use glib::translate::*;
use glib_ffi::gboolean;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use glib_ffi::gpointer;
use gdk::{DragAction, Event, ModifierType};
#[cfg(any(feature = "v3_8", feature = "dox"))]
use gdk::FrameClock;
use gdk;
use gdk_ffi;
use pango;
use ffi;

use {
    DestDefaults,
    Inhibit,
    Rectangle,
    TargetEntry,
    Widget,
};
#[cfg(any(feature = "v3_8", feature = "dox"))]
use::Continue;

pub trait WidgetExtManual: 'static {
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction);

    fn drag_source_set(&self, start_button_mask: ModifierType, targets: &[TargetEntry], actions: DragAction);

    fn intersect<'a, I: Into<Option<&'a mut Rectangle>>>(
        &self,
        area: &Rectangle,
        intersection: I,
    ) -> bool;

    fn override_font(&self, font: &pango::FontDescription);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn add_tick_callback<F>(&self, func: F) -> u32
    where
        F: FnMut(&Self, &FrameClock) -> Continue + 'static;

    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn add_events(&self, events: gdk::EventMask);
    fn get_events(&self) -> gdk::EventMask;
    fn set_events(&self, events: gdk::EventMask);
}

impl<O: IsA<Widget>> WidgetExtManual for O {
    fn drag_dest_set(&self, flags: DestDefaults, targets: &[TargetEntry], actions: DragAction) {
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let t: Vec<_> = stashes.iter().map(|stash| unsafe { *stash.0 }).collect();
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe { ffi::gtk_drag_dest_set(self.as_ref().to_glib_none().0,
                                        flags.to_glib(),
                                        t_ptr,
                                        t.len() as i32,
                                        actions.to_glib())};
    }

    fn drag_source_set(&self, start_button_mask: ModifierType, targets: &[TargetEntry], actions: DragAction) {
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let t: Vec<_> = stashes.iter().map(|stash| unsafe { *stash.0 }).collect();
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe { ffi::gtk_drag_source_set(self.as_ref().to_glib_none().0,
                                          start_button_mask.to_glib(),
                                          t_ptr,
                                          t.len() as i32,
                                          actions.to_glib())};
    }

    fn intersect<'a, I: Into<Option<&'a mut Rectangle>>>(
        &self,
        area: &Rectangle,
        intersection: I,
    ) -> bool {
        let mut intersection = intersection.into();
        unsafe {
            from_glib(ffi::gtk_widget_intersect(self.as_ref().to_glib_none().0, area.to_glib_none().0, intersection.to_glib_none_mut().0))
        }
    }

    fn override_font(&self, font: &pango::FontDescription) {
        unsafe {
            ffi::gtk_widget_override_font(self.as_ref().to_glib_none().0, font.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn add_tick_callback<F>(&self, func: F) -> u32
    where
        F: FnMut(&Self, &FrameClock) -> Continue + 'static,
    {
        unsafe extern "C" fn add_tick_callback_trampoline<T, F: FnMut(&T, &FrameClock) -> Continue + 'static>(
            this: *mut ffi::GtkWidget,
            frame_clock: *mut gdk_ffi::GdkFrameClock,
            func: gpointer,
        ) -> gboolean
        where T: IsA<Widget>
        {
            let func: &RefCell<F> = transmute(func);

            (&mut *func.borrow_mut())(
                &Widget::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(frame_clock)
            ).to_glib()
        }

        unsafe extern "C" fn destroy_closure<T, F: FnMut(&T, &FrameClock) -> Continue + 'static>(func: gpointer) {
            Box::<RefCell<F>>::from_raw(func as *mut _);
        }

        let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
        unsafe {
            ffi::gtk_widget_add_tick_callback(
                self.as_ref().to_glib_none().0,
                Some(add_tick_callback_trampoline::<Self, F>),
                Box::into_raw(func) as gpointer,
                Some(destroy_closure::<Self, F>),
            )
        }
    }

   fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.to_glib_none().0 as *mut _, b"map-event\0".as_ptr() as *mut _,
                Some(transmute(event_any_trampoline::<Self, F> as usize)), Box::into_raw(f))
        }
    }

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(self.to_glib_none().0 as *mut _, b"unmap-event\0".as_ptr() as *mut _,
                Some(transmute(event_any_trampoline::<Self, F> as usize)), Box::into_raw(f))
        }
    }

    fn add_events(&self, events: gdk::EventMask) {
        unsafe {
            ffi::gtk_widget_add_events(self.as_ref().to_glib_none().0, events.to_glib() as i32);
        }
    }

    fn get_events(&self) -> gdk::EventMask {
        unsafe {
            from_glib(ffi::gtk_widget_get_events(self.as_ref().to_glib_none().0) as u32)
        }
    }

    fn set_events(&self, events: gdk::EventMask) {
        unsafe {
            ffi::gtk_widget_set_events(self.as_ref().to_glib_none().0, events.to_glib() as i32);
        }
    }
}

unsafe extern "C" fn event_any_trampoline<T, F: Fn(&T, &Event) -> Inhibit + 'static>(this: *mut ffi::GtkWidget,
                                             event: *mut gdk_ffi::GdkEventAny,
                                             f: &F) -> gboolean
where T: IsA<Widget> {
    f(&Widget::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(event)).to_glib()
}
