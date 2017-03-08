// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem::transmute;

use glib::object::{Downcast, IsA};
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gboolean;
use gdk::{Event, RGBA};
use gdk_ffi;
use pango;
use ffi;

use {
    Inhibit,
    Object,
    Rectangle,
    StateFlags,
    Widget,
};

pub trait WidgetExtManual {
    fn intersect(&self, area: &Rectangle, intersection: Option<&mut Rectangle>) -> bool;

    fn override_background_color(&self, state: StateFlags, color: &RGBA);

    fn override_color(&self, state: StateFlags, color: &RGBA);

    fn override_cursor(&self, cursor: &RGBA, secondary_cursor: &RGBA);

    fn override_font(&self, font: &pango::FontDescription);

    fn override_symbolic_color(&self, name: &str, color: &RGBA);

    fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Widget> + IsA<Object>> WidgetExtManual for O {
    fn intersect(&self, area: &Rectangle, mut intersection: Option<&mut Rectangle>) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_intersect(self.to_glib_none().0, area.to_glib_none().0, intersection.to_glib_none_mut().0))
        }
    }

    fn override_background_color(&self, state: StateFlags, color: &RGBA) {
        unsafe {
            ffi::gtk_widget_override_background_color(self.to_glib_none().0,
                                                      state.to_glib(),
                                                      color.to_glib_none().0);
        }
    }

    fn override_color(&self, state: StateFlags, color: &RGBA) {
        unsafe {
            ffi::gtk_widget_override_color(self.to_glib_none().0,
                                           state.to_glib(),
                                           color.to_glib_none().0);
        }
    }

    fn override_symbolic_color(&self, name: &str, color: &RGBA) {
        unsafe {
            ffi::gtk_widget_override_symbolic_color(self.to_glib_none().0,
                                                    name.to_glib_none().0,
                                                    color.to_glib_none().0);
        }
    }

    fn override_cursor(&self, cursor: &RGBA, secondary_cursor: &RGBA) {
        unsafe {
            ffi::gtk_widget_override_cursor(self.to_glib_none().0,
                                            cursor.to_glib_none().0,
                                            secondary_cursor.to_glib_none().0);
        }
    }

    fn override_font(&self, font: &pango::FontDescription) {
        unsafe {
            ffi::gtk_widget_override_font(self.to_glib_none().0, font.to_glib_none().0)
        }
    }

   fn connect_map_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
            connect(self.to_glib_none().0, "map-event",
                transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
        }
    }

    fn connect_unmap_event<F: Fn(&Self, &Event) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(&Self, &Event) -> Inhibit + 'static>> = Box::new(Box::new(f));
            connect(self.to_glib_none().0, "unmap-event",
                transmute(event_any_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn event_any_trampoline<T>(this: *mut ffi::GtkWidget, event: *mut gdk_ffi::GdkEventAny,
        f: &Box<Fn(&T, &Event) -> Inhibit + 'static>) -> gboolean
where T: IsA<Widget> {
    callback_guard!();
    f(&Widget::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
}
