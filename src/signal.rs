// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::mem::transmute;

pub use glib::signal::Inhibit;
use glib::SourceId;
use glib::translate::*;

use glib_ffi::{self, gboolean, gpointer};

use {
    Continue,
    ScrollType,
    SpinButton,
};

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
pub fn idle_add<F>(func: F) -> SourceId
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(glib_ffi::g_idle_add_full(glib_ffi::G_PRIORITY_DEFAULT_IDLE, Some(trampoline),
            into_raw(func), Some(destroy_closure)))
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
pub fn timeout_add<F>(interval: u32, func: F) -> SourceId
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(
            glib_ffi::g_timeout_add_full(glib_ffi::G_PRIORITY_DEFAULT, interval, Some(trampoline),
                 into_raw(func), Some(destroy_closure)))
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
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> SourceId
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(glib_ffi::g_timeout_add_seconds_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            Some(trampoline), into_raw(func), Some(destroy_closure)))
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

pub trait SpinButtonSignals {
    fn connect_change_value<F>(&self, change_value_func: F) -> u64
        where F: Fn(&SpinButton, ScrollType) + 'static;
    fn connect_input<F>(&self, input_func: F) -> u64
        where F: Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static;
    fn connect_output<F>(&self, output_func: F) -> u64
        where F: Fn(&SpinButton) -> Inhibit + 'static;
    fn connect_value_changed<F>(&self, value_changed_func: F) -> u64
        where F: Fn(&SpinButton) + 'static;
    fn connect_wrapped<F>(&self, wrapped_func: F) -> u64
        where F: Fn(&SpinButton) + 'static;
}

mod spin_button {
    use Inhibit;
    use SpinButton;
    use ScrollType;
    use ffi::{GTK_INPUT_ERROR, GtkSpinButton};
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::{GTRUE, GFALSE};
    use libc::{c_int, c_double};
    use std::boxed::Box as Box_;
    use std::mem::transmute;
    use glib_ffi::gboolean;

    impl ::SpinButtonSignals for SpinButton {
        fn connect_change_value<F>(&self, change_value_func: F) -> u64
        where F: Fn(&SpinButton, ScrollType) + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton, ScrollType) + 'static>> =
                    Box::new(Box::new(change_value_func));
                connect(self.to_glib_none().0, "change_value",
                        transmute(change_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_input<F>(&self, f: F) -> u64
        where F: Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static {
            unsafe {
                let f: Box_<Box_<Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static>> = Box_::new(Box_::new(f));
                connect(self.to_glib_none().0, "input",
                        transmute(input_trampoline as usize), Box_::into_raw(f) as *mut _)
            }
        }

        fn connect_output<F>(&self, output_func: F) -> u64
        where F: Fn(&SpinButton) -> Inhibit + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton) -> Inhibit + 'static>> =
                    Box::new(Box::new(output_func));
                connect(self.to_glib_none().0, "output",
                        transmute(output_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_value_changed<F>(&self, value_changed_func: F) -> u64
        where F: Fn(&SpinButton) + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton) + 'static>> =
                    Box::new(Box::new(value_changed_func));
                connect(self.to_glib_none().0, "value-changed",
                        transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_wrapped<F>(&self, wrapped_func: F) -> u64
        where F: Fn(&SpinButton) + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton) + 'static>> =
                    Box::new(Box::new(wrapped_func));
                connect(self.to_glib_none().0, "wrapped",
                        transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn change_trampoline(this: *mut GtkSpinButton,
                                           scroll: ScrollType,
                                           f: &Box<Fn(&SpinButton, ScrollType) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), scroll)
    }

    unsafe extern "C" fn input_trampoline(this: *mut GtkSpinButton,
                                          new_value: *mut c_double,
                                          f: &Box_<Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static>)
                                          -> c_int {
        callback_guard!();
        match f(&from_glib_none(this)) {
            Some(Ok(v)) => {
                *new_value = v;
                GTRUE
            }
            Some(Err(_)) => GTK_INPUT_ERROR,
            None => GFALSE,
        }
    }

    unsafe extern "C" fn output_trampoline(this: *mut GtkSpinButton,
                                           f: &Box<Fn(&SpinButton) -> Inhibit + 'static>)
                                           -> gboolean {
        callback_guard!();
        f(&from_glib_none(this)).to_glib()
    }

    unsafe extern "C" fn trampoline(this: *mut GtkSpinButton,
                                    f: &Box<Fn(&SpinButton) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this))
    }
}
