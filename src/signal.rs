// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk::Rectangle;
use glib;
pub use glib::signal::Inhibit;
use glib::signal::SignalHandlerId;
use glib::SourceId;

use {Continue, ScrollType, Widget};

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// Similar to `glib::idle_add` but only callable from the main thread and
/// doesn't require `Send`. It is the same as `glib::idle_add_local`.
pub fn idle_add<F>(func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    assert_initialized_main_thread!();
    glib::idle_add_local(func)
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
/// doesn't require `Send`. It is the same as `glib::timeout_add_local`.
pub fn timeout_add<F>(interval: u32, func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    assert_initialized_main_thread!();
    glib::timeout_add_local(interval, func)
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// Similar to `glib::timeout_add_seconds` but only callable from the main thread and
/// doesn't require `Send`. It is the same as `glib::timeout_add_seconds_local`.
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    assert_initialized_main_thread!();
    glib::timeout_add_seconds_local(interval, func)
}

pub trait EditableSignals: 'static {
    fn connect_changed<F>(&self, changed_func: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static;
    fn connect_delete_text<F>(&self, delete_text_func: F) -> SignalHandlerId
    where
        F: Fn(&Self, i32, i32) + 'static;
    fn connect_insert_text<F>(&self, insert_text_func: F) -> SignalHandlerId
    where
        F: Fn(&Self, &str, &mut i32) + 'static;
}

mod editable {
    use glib::object::Cast;
    use glib::signal::{connect_raw, SignalHandlerId};
    use glib::translate::*;
    use gtk_sys::GtkEditable;
    use libc::{c_char, c_int, c_uchar};
    use std::ffi::CStr;
    use std::mem::transmute;
    use std::slice;
    use std::str;
    use Editable;
    use IsA;

    impl<T: IsA<Editable>> super::EditableSignals for T {
        fn connect_changed<F>(&self, changed_func: F) -> SignalHandlerId
        where
            F: Fn(&Self) + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(changed_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"changed\0".as_ptr() as *mut _,
                    Some(transmute(trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }

        fn connect_delete_text<F>(&self, delete_text_func: F) -> SignalHandlerId
        where
            F: Fn(&Self, i32, i32) + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(delete_text_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"delete-text\0".as_ptr() as *mut _,
                    Some(transmute(delete_trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }

        fn connect_insert_text<F>(&self, insert_text_func: F) -> SignalHandlerId
        where
            F: Fn(&Self, &str, &mut i32) + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(insert_text_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"insert-text\0".as_ptr() as *mut _,
                    Some(transmute(insert_trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }
    }

    unsafe extern "C" fn trampoline<T, F: Fn(&T) + 'static>(this: *mut GtkEditable, f: &F)
    where
        T: IsA<Editable>,
    {
        f(&Editable::from_glib_borrow(this).unsafe_cast());
    }

    unsafe extern "C" fn delete_trampoline<T, F: Fn(&T, i32, i32) + 'static>(
        this: *mut GtkEditable,
        start_pos: c_int,
        end_pos: c_int,
        f: &F,
    ) where
        T: IsA<Editable>,
    {
        f(
            &Editable::from_glib_borrow(this).unsafe_cast(),
            start_pos,
            end_pos,
        );
    }

    unsafe extern "C" fn insert_trampoline<T, F: Fn(&T, &str, &mut i32) + 'static>(
        this: *mut GtkEditable,
        new_text: *mut c_char,
        new_text_length: c_int,
        position: *mut c_int,
        f: &F,
    ) where
        T: IsA<Editable>,
    {
        let buf = if new_text_length != -1 {
            slice::from_raw_parts(new_text as *mut c_uchar, new_text_length as usize)
        } else {
            CStr::from_ptr(new_text).to_bytes()
        };
        let string = str::from_utf8(buf).unwrap();
        f(
            &Editable::from_glib_borrow(this).unsafe_cast(),
            string,
            transmute(position),
        );
    }
}

pub trait SpinButtonSignals: 'static {
    fn connect_change_value<F>(&self, change_value_func: F) -> SignalHandlerId
    where
        F: Fn(&Self, ScrollType) + 'static;
    fn connect_input<F>(&self, input_func: F) -> SignalHandlerId
    where
        F: Fn(&Self) -> Option<Result<f64, ()>> + 'static;
    fn connect_output<F>(&self, output_func: F) -> SignalHandlerId
    where
        F: Fn(&Self) -> Inhibit + 'static;
    fn connect_value_changed<F>(&self, value_changed_func: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static;
    fn connect_wrapped<F>(&self, wrapped_func: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static;
}

mod spin_button {
    use glib::object::Cast;
    use glib::signal::{connect_raw, SignalHandlerId};
    use glib::translate::*;
    use glib::IsA;
    use glib_sys::gboolean;
    use glib_sys::{GFALSE, GTRUE};
    use gtk_sys::{GtkSpinButton, GTK_INPUT_ERROR};
    use libc::{c_double, c_int};
    use std::boxed::Box as Box_;
    use std::mem::transmute;
    use Inhibit;
    use ScrollType;
    use SpinButton;

    impl<T: IsA<SpinButton>> ::SpinButtonSignals for T {
        fn connect_change_value<F>(&self, change_value_func: F) -> SignalHandlerId
        where
            F: Fn(&Self, ScrollType) + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(change_value_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"change_value\0".as_ptr() as *mut _,
                    Some(transmute(change_trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }

        fn connect_input<F>(&self, f: F) -> SignalHandlerId
        where
            F: Fn(&Self) -> Option<Result<f64, ()>> + 'static,
        {
            unsafe {
                let f: Box_<F> = Box_::new(f);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"input\0".as_ptr() as *mut _,
                    Some(transmute(input_trampoline::<Self, F> as usize)),
                    Box_::into_raw(f),
                )
            }
        }

        fn connect_output<F>(&self, output_func: F) -> SignalHandlerId
        where
            F: Fn(&Self) -> Inhibit + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(output_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"output\0".as_ptr() as *mut _,
                    Some(transmute(output_trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }

        fn connect_value_changed<F>(&self, value_changed_func: F) -> SignalHandlerId
        where
            F: Fn(&Self) + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(value_changed_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"value-changed\0".as_ptr() as *mut _,
                    Some(transmute(trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }

        fn connect_wrapped<F>(&self, wrapped_func: F) -> SignalHandlerId
        where
            F: Fn(&Self) + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(wrapped_func);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"wrapped\0".as_ptr() as *mut _,
                    Some(transmute(trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }
    }

    unsafe extern "C" fn change_trampoline<T, F: Fn(&T, ScrollType) + 'static>(
        this: *mut GtkSpinButton,
        scroll: ScrollType,
        f: &F,
    ) where
        T: IsA<SpinButton>,
    {
        f(&SpinButton::from_glib_borrow(this).unsafe_cast(), scroll)
    }

    unsafe extern "C" fn input_trampoline<T, F: Fn(&T) -> Option<Result<f64, ()>> + 'static>(
        this: *mut GtkSpinButton,
        new_value: *mut c_double,
        f: &F,
    ) -> c_int
    where
        T: IsA<SpinButton>,
    {
        match f(&SpinButton::from_glib_borrow(this).unsafe_cast()) {
            Some(Ok(v)) => {
                *new_value = v;
                GTRUE
            }
            Some(Err(_)) => GTK_INPUT_ERROR,
            None => GFALSE,
        }
    }

    unsafe extern "C" fn output_trampoline<T, F: Fn(&T) -> Inhibit + 'static>(
        this: *mut GtkSpinButton,
        f: &F,
    ) -> gboolean
    where
        T: IsA<SpinButton>,
    {
        f(&SpinButton::from_glib_borrow(this).unsafe_cast()).to_glib()
    }

    unsafe extern "C" fn trampoline<T, F: Fn(&T) + 'static>(this: *mut GtkSpinButton, f: &F)
    where
        T: IsA<SpinButton>,
    {
        f(&SpinButton::from_glib_borrow(this).unsafe_cast())
    }
}

pub trait OverlaySignals: 'static {
    fn connect_get_child_position<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &Widget) -> Option<Rectangle> + 'static;
}

mod overlay {
    use gdk::Rectangle;
    use gdk_sys::GdkRectangle;
    use glib::object::Cast;
    use glib::signal::{connect_raw, SignalHandlerId};
    use glib::translate::*;
    use glib_sys::{gboolean, gpointer};
    use gtk_sys::{GtkOverlay, GtkWidget};
    use std::mem::transmute;
    use std::ptr;
    use IsA;
    use Overlay;
    use Widget;

    impl<O: IsA<Overlay>> ::OverlaySignals for O {
        fn connect_get_child_position<F>(&self, f: F) -> SignalHandlerId
        where
            F: Fn(&Self, &Widget) -> Option<Rectangle> + 'static,
        {
            unsafe {
                let f: Box<F> = Box::new(f);
                connect_raw(
                    self.to_glib_none().0 as *mut _,
                    b"get-child-position\0".as_ptr() as *mut _,
                    Some(transmute(get_child_position_trampoline::<Self, F> as usize)),
                    Box::into_raw(f),
                )
            }
        }
    }

    unsafe extern "C" fn get_child_position_trampoline<
        T,
        F: Fn(&T, &Widget) -> Option<Rectangle> + 'static,
    >(
        this: *mut GtkOverlay,
        widget: *mut GtkWidget,
        allocation: *mut GdkRectangle,
        f: gpointer,
    ) -> gboolean
    where
        T: IsA<Overlay>,
    {
        let f: &F = &*(f as *const F);
        match f(
            &Overlay::from_glib_borrow(this).unsafe_cast(),
            &from_glib_borrow(widget),
        ) {
            Some(rect) => {
                ptr::write(allocation, ptr::read(rect.to_glib_none().0));
                true
            }
            None => false,
        }
        .to_glib()
    }
}
