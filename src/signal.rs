// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk::Rectangle;
pub use glib::signal::Inhibit;
use glib::signal::SignalHandlerId;

use {ScrollType, Widget};

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
    use glib::IsA;
    use gtk_sys::GtkEditable;
    use libc::{c_char, c_int, c_uchar};
    use std::ffi::CStr;
    use std::mem::transmute;
    use std::slice;
    use std::str;
    use Editable;

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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        trampoline::<Self, F> as *const (),
                    )),
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        delete_trampoline::<Self, F> as *const (),
                    )),
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        insert_trampoline::<Self, F> as *const (),
                    )),
                    Box::into_raw(f),
                )
            }
        }
    }

    unsafe extern "C" fn trampoline<T, F: Fn(&T) + 'static>(this: *mut GtkEditable, f: &F)
    where
        T: IsA<Editable>,
    {
        f(&Editable::from_glib_borrow(this).unsafe_cast_ref());
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
            &Editable::from_glib_borrow(this).unsafe_cast_ref(),
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
            &Editable::from_glib_borrow(this).unsafe_cast_ref(),
            string,
            // To cast a mutable pointer into a mutable reference.
            &mut *position,
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
    use gtk_sys::{GtkScrollType, GtkSpinButton, GTK_INPUT_ERROR};
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        change_trampoline::<Self, F> as *const (),
                    )),
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        input_trampoline::<Self, F> as *const (),
                    )),
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        output_trampoline::<Self, F> as *const (),
                    )),
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        trampoline::<Self, F> as *const (),
                    )),
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        trampoline::<Self, F> as *const (),
                    )),
                    Box::into_raw(f),
                )
            }
        }
    }

    unsafe extern "C" fn change_trampoline<T, F: Fn(&T, ScrollType) + 'static>(
        this: *mut GtkSpinButton,
        scroll: GtkScrollType,
        f: &F,
    ) where
        T: IsA<SpinButton>,
    {
        f(
            &SpinButton::from_glib_borrow(this).unsafe_cast_ref(),
            from_glib(scroll),
        )
    }

    unsafe extern "C" fn input_trampoline<T, F: Fn(&T) -> Option<Result<f64, ()>> + 'static>(
        this: *mut GtkSpinButton,
        new_value: *mut c_double,
        f: &F,
    ) -> c_int
    where
        T: IsA<SpinButton>,
    {
        match f(&SpinButton::from_glib_borrow(this).unsafe_cast_ref()) {
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
        f(&SpinButton::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
    }

    unsafe extern "C" fn trampoline<T, F: Fn(&T) + 'static>(this: *mut GtkSpinButton, f: &F)
    where
        T: IsA<SpinButton>,
    {
        f(&SpinButton::from_glib_borrow(this).unsafe_cast_ref())
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
    use glib::IsA;
    use glib_sys::{gboolean, gpointer};
    use gtk_sys::{GtkOverlay, GtkWidget};
    use std::mem::transmute;
    use std::ptr;
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
                    Some(transmute::<_, unsafe extern "C" fn()>(
                        get_child_position_trampoline::<Self, F> as *const (),
                    )),
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
            &Overlay::from_glib_borrow(this).unsafe_cast_ref(),
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
