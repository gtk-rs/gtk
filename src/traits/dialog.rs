// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::c_char;
use glib::translate::*;
use cast::GTK_DIALOG;
use ffi;
use Box;

pub trait DialogButtons {
    unsafe fn invoke1<A0, R>(
        &self,
        f: unsafe extern "C" fn(A0, *const c_char, ...) -> R,
        a0: A0) -> R;
    unsafe fn invoke2<A0, A1, R>(
        &self,
        f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
        a0: A0, a1: A1) -> R;
    unsafe fn invoke3<A0, A1, A2, R>(
        &self,
        f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
        a0: A0, a1: A1, a2: A2) -> R;
}

pub mod buttons {
    use ResponseType;
    use ResponseType::*;

    pub const OK: [(&'static str, ResponseType); 1] = [("Ok", Ok)];
    pub const OK_CANCEL: [(&'static str, ResponseType); 2] =[
        ("Ok", Ok),
        ("Cancel", Cancel)
    ];
    pub const YES_NO: [(&'static str, ResponseType); 2] = [
        ("Yes", Yes),
        ("No", No)
    ];
}

macro_rules! impl_dialog_buttons {
    // Work around Rust bug #22897
    ($n:expr,) => (
        impl <'a> DialogButtons for [(&'a str, i32); 0] {
            unsafe fn invoke1<A0, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, *const c_char, ...) -> R,
                    a0: A0) -> R {
                f(a0, ptr::null())
            }

            unsafe fn invoke2<A0, A1, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
                    a0: A0, a1: A1) -> R {
                f(a0, a1, ptr::null())
            }

            unsafe fn invoke3<A0, A1, A2, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
                    a0: A0, a1: A1, a2: A2) -> R {
                f(a0, a1, a2, ptr::null())
            }
        }

        impl <'a> DialogButtons for [(&'a str, ::ResponseType); 0] {
            unsafe fn invoke1<A0, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, *const c_char, ...) -> R,
                    a0: A0) -> R {
                f(a0, ptr::null())
            }

            unsafe fn invoke2<A0, A1, R>(
                        &self,
                        f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
                        a0: A0, a1: A1) -> R {
                f(a0, a1, ptr::null())
            }

            unsafe fn invoke3<A0, A1, A2, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
                    a0: A0, a1: A1, a2: A2) -> R {
                f(a0, a1, a2, ptr::null())
            }
        }
    );

    ($nn:expr, $($n:expr,)+) => (
        impl_dialog_buttons!($($n,)+);

        impl <'a> DialogButtons for [(&'a str, i32); $nn] {
            unsafe fn invoke1<A0, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, *const c_char, ...) -> R,
                    a0: A0) -> R {
                // reverse the order
                let tmp_0 = [$(self[$n].0.to_glib_none()),+];
                let tmp_1 = [$(self[$n].1),+];
                f(a0, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke2<A0, A1, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
                    a0: A0, a1: A1) -> R {
                let tmp_0 = [$(self[$n].0.to_glib_none()),+];
                let tmp_1 = [$(self[$n].1),+];
                f(a0, a1, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke3<A0, A1, A2, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
                    a0: A0, a1: A1, a2: A2) -> R {
                let tmp_0 = [$(self[$n].0.to_glib_none()),+];
                let tmp_1 = [$(self[$n].1),+];
                f(a0, a1, a2, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }
        }

        impl <'a> DialogButtons for [(&'a str, ::ResponseType); $nn] {
            unsafe fn invoke1<A0, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, *const c_char, ...) -> R,
                    a0: A0) -> R {
                let tmp_0 = [$(self[$n].0.to_glib_none()),+];
                let tmp_1 = [$(self[$n].1 as i32),+];
                f(a0, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke2<A0, A1, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
                    a0: A0, a1: A1) -> R {
                let tmp_0 = [$(self[$n].0.to_glib_none()),+];
                let tmp_1 = [$(self[$n].1 as i32),+];
                f(a0, a1, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke3<A0, A1, A2, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
                    a0: A0, a1: A1, a2: A2) -> R {
                let tmp_0 = [$(self[$n].0.to_glib_none()),+];
                let tmp_1 = [$(self[$n].1 as i32),+];
                f(a0, a1, a2, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }
        }
    )
}

impl_dialog_buttons!(16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,);

pub trait DialogTrait: ::WidgetTrait + ::ContainerTrait + ::BinTrait + ::WindowTrait {
    fn run(&self) -> i32 {
        unsafe { ffi::gtk_dialog_run(GTK_DIALOG(self.unwrap_widget())) }
    }

    fn response(&self, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_response(GTK_DIALOG(self.unwrap_widget()), response_id) }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Option<::Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_dialog_add_button(GTK_DIALOG(self.unwrap_widget()), button_text.to_glib_none().0, response_id)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    fn add_buttons<T: DialogButtons>(&self, buttons: T) {
        unsafe {
            buttons.invoke1(
                ffi::gtk_dialog_add_buttons,
                GTK_DIALOG(self.unwrap_widget()))
        }
    }

    fn add_action_widget<T: ::WidgetTrait>(&self, child: &T, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_add_action_widget(GTK_DIALOG(self.unwrap_widget()), child.unwrap_widget(), response_id) }
    }

    fn set_default_response(&self, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_set_default_response(GTK_DIALOG(self.unwrap_widget()), response_id) }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: bool) -> () {
        unsafe { ffi::gtk_dialog_set_response_sensitive(GTK_DIALOG(self.unwrap_widget()), response_id, setting.to_glib()) }
    }

    fn get_response_for_widget<T: ::WidgetTrait>(&self, widget: &T) -> Result<i32, ::ResponseType> {
        let tmp = unsafe { ffi::gtk_dialog_get_response_for_widget(GTK_DIALOG(self.unwrap_widget()), widget.unwrap_widget()) };

        if tmp < 0 {
            Err(unsafe { ::std::mem::transmute(tmp) })
        } else {
            Ok(tmp)
        }
    }

    unsafe fn get_widget_for_reponse<T: ::WidgetTrait>(&self, response_id: i32) -> Option<T> {
        let tmp_pointer = ffi::gtk_dialog_get_widget_for_response(GTK_DIALOG(self.unwrap_widget()), response_id);

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    unsafe fn get_action_area<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = ffi::gtk_dialog_get_action_area(GTK_DIALOG(self.unwrap_widget()));

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    fn get_content_area(&self) -> Box {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_content_area(GTK_DIALOG(self.unwrap_widget())) };

        check_pointer!(tmp_pointer, Box).unwrap()
    }

    #[cfg(gtk_3_12)]
    unsafe fn get_header_bar<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = ffi::gtk_dialog_get_header_bar(GTK_DIALOG(self.unwrap_widget()));

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}
