// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub mod about;
//pub mod app_chooser;
//pub mod color_chooser;
//pub mod file_chooser;
//pub mod font_chooser;
pub mod message;
//pub mod page_setup_unix;
//pub mod recent_chooser;

use std::ptr;
use libc::c_char;

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use widgets;
use widgets::button::Button;
#[cfg(gtk_3_12)]
use widgets::header_bar::HeaderBar;
use widgets::widget::Widget;
use window;
use {ResponseType};

/// Pseudo-variadic array of buttons
///
/// It's implemented for fixed-sized arrays `[(&str, i32); N]`
/// and `[(&str, ::ResponseType); N]` (`N <= 16`) to allow passing variable
/// numbers of buttons to some dialog methods.
///
/// ```ignore
/// Dialog::with_buttons(title, parent, flags,
///                      [("Ok", ResponseType::Accept), ("Cancel", ResponseType::Cancel)]);
/// ```
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

/// Predefined popular button combinations
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

        impl <'a> DialogButtons for [(&'a str, ResponseType); 0] {
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

        impl <'a> DialogButtons for [(&'a str, ResponseType); $nn] {
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

pub type Dialog = Object<ffi::GtkDialog>;

impl types::StaticType for Dialog {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_dialog_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Dialog { }
unsafe impl Upcast<widgets::container::Container> for Dialog { }
unsafe impl Upcast<widgets::bin::Bin> for Dialog { }
unsafe impl Upcast<window::Window> for Dialog { }
unsafe impl Upcast<::builder::Buildable> for Dialog { }

pub trait DialogExt {
    fn run(&self) -> i32;
    fn response(&self, response_id: i32);
    fn add_button(&self, button_text: &str, response_id: i32) -> Button;
    fn add_buttons<T: DialogButtons>(&self, buttons: T);
    fn add_action_widget<T: Upcast<Widget>>(&self, child: &T, response_id: i32);
    fn set_default_response(&self, response_id: i32);
    fn set_response_sensitive(&self, response_id: i32, setting: bool);
    fn get_response_for_widget<T: Upcast<Widget>>(&self, widget: &T) -> Result<i32, ResponseType>;
    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget>;
    fn get_action_area(&self) -> Widget;
    fn get_content_area(&self) -> widgets::box_::Box;
    #[cfg(gtk_3_12)]
    fn get_header_bar(&self) -> Option<HeaderBar>;
}

impl<O: Upcast<Dialog>> DialogExt for O {
    fn run(&self) -> i32 {
        unsafe { ffi::gtk_dialog_run(self.upcast().to_glib_none().0) }
    }

    fn response(&self, response_id: i32) {
        unsafe { ffi::gtk_dialog_response(self.upcast().to_glib_none().0, response_id) }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Button {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_dialog_add_button(self.upcast().to_glib_none().0,
                    button_text.to_glib_none().0, response_id))
                .downcast_unchecked()
        }
    }

    fn add_buttons<T: DialogButtons>(&self, buttons: T) {
        unsafe {
            buttons.invoke1(
                ffi::gtk_dialog_add_buttons,
                self.upcast().to_glib_none().0)
        }
    }

    fn add_action_widget<T: Upcast<Widget>>(&self, child: &T, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_add_action_widget(
                self.upcast().to_glib_none().0, child.upcast().to_glib_none().0, response_id)
        }
    }

    fn set_default_response(&self, response_id: i32) {
        unsafe { ffi::gtk_dialog_set_default_response(self.upcast().to_glib_none().0, response_id) }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: bool) {
        unsafe {
            ffi::gtk_dialog_set_response_sensitive(
                self.upcast().to_glib_none().0, response_id, setting.to_glib())
        }

    }

    fn get_response_for_widget<T: Upcast<Widget>>(&self, widget: &T) -> Result<i32, ResponseType> {
        let tmp = unsafe {
            ffi::gtk_dialog_get_response_for_widget(self.upcast().to_glib_none().0,
                widget.upcast().to_glib_none().0)
        };

        if tmp < 0 {
            Err(unsafe { ::std::mem::transmute(tmp) })
        } else {
            Ok(tmp)
        }
    }

    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(
                ffi::gtk_dialog_get_widget_for_response(
                    self.upcast().to_glib_none().0, response_id))
        }
    }

    fn get_action_area(&self) -> Widget {
        unsafe { from_glib_none(ffi::gtk_dialog_get_action_area(self.upcast().to_glib_none().0)) }
    }

    fn get_content_area(&self) -> widgets::box_::Box {
        unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_get_content_area(self.upcast().to_glib_none().0))
                .downcast_unchecked()
        }
    }

    #[cfg(gtk_3_12)]
    fn get_header_bar(&self) -> Option<HeaderBar> {
        unsafe {
            Option::<Widget>::from_glib_none(
                ffi::gtk_dialog_get_header_bar(self.upcast().to_glib_none().0))
                .map(Downcast::downcast_unchecked)
        }
    }
}
