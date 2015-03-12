// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use std::ptr;
use libc::c_char;
use glib::translate::ToGlibPtr;
use gtk::cast::GTK_DIALOG;
use gtk::ffi;
use gtk;

/// Pseudo-variadic array of buttons
///
/// It's implemented for fixed-sized arrays `[(&str, i32); N]`
/// and `[(&str, gtk::ResponseType); N]` (`N <= 16`) to allow passing variable
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
    use gtk::ResponseType;
    use gtk::ResponseType::*;

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

        impl <'a> DialogButtons for [(&'a str, gtk::ResponseType); 0] {
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
                let tmp_0 = [$(self[$n].0.borrow_to_glib()),+];
                let tmp_1 = [$(self[$n].1),+];
                f(a0, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke2<A0, A1, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
                    a0: A0, a1: A1) -> R {
                let tmp_0 = [$(self[$n].0.borrow_to_glib()),+];
                let tmp_1 = [$(self[$n].1),+];
                f(a0, a1, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke3<A0, A1, A2, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
                    a0: A0, a1: A1, a2: A2) -> R {
                let tmp_0 = [$(self[$n].0.borrow_to_glib()),+];
                let tmp_1 = [$(self[$n].1),+];
                f(a0, a1, a2, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }
        }

        impl <'a> DialogButtons for [(&'a str, gtk::ResponseType); $nn] {
            unsafe fn invoke1<A0, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, *const c_char, ...) -> R,
                    a0: A0) -> R {
                let tmp_0 = [$(self[$n].0.borrow_to_glib()),+];
                let tmp_1 = [$(self[$n].1 as i32),+];
                f(a0, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke2<A0, A1, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, *const c_char, ...) -> R,
                    a0: A0, a1: A1) -> R {
                let tmp_0 = [$(self[$n].0.borrow_to_glib()),+];
                let tmp_1 = [$(self[$n].1 as i32),+];
                f(a0, a1, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }

            unsafe fn invoke3<A0, A1, A2, R>(
                    &self,
                    f: unsafe extern "C" fn(A0, A1, A2, *const c_char, ...) -> R,
                    a0: A0, a1: A1, a2: A2) -> R {
                let tmp_0 = [$(self[$n].0.borrow_to_glib()),+];
                let tmp_1 = [$(self[$n].1 as i32),+];
                f(a0, a1, a2, $(tmp_0[$n].0, tmp_1[$n],)+ ptr::null::<c_char>())
            }
        }
    )
}

impl_dialog_buttons!(16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,);

pub trait DialogTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait + gtk::WindowTrait {
    fn run(&self) -> i32 {
        unsafe { ffi::gtk_dialog_run(GTK_DIALOG(self.unwrap_widget())) }
    }

    fn response(&self, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_response(GTK_DIALOG(self.unwrap_widget()), response_id) }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Option<gtk::Button> {
        let tmp_pointer = unsafe {
            ffi::gtk_dialog_add_button(GTK_DIALOG(self.unwrap_widget()), button_text.borrow_to_glib().0, response_id)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    fn add_buttons<T: DialogButtons>(&self, buttons: T) {
        unsafe {
            buttons.invoke1(
                ffi::gtk_dialog_add_buttons,
                GTK_DIALOG(self.unwrap_widget()))
        }
    }

    fn add_action_widget<T: gtk::WidgetTrait>(&self, child: &T, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_add_action_widget(GTK_DIALOG(self.unwrap_widget()), child.unwrap_widget(), response_id) }
    }

    fn set_default_response(&self, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_set_default_response(GTK_DIALOG(self.unwrap_widget()), response_id) }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: ffi::Gboolean) -> () {
        unsafe { ffi::gtk_dialog_set_response_sensitive(GTK_DIALOG(self.unwrap_widget()), response_id, setting) }
    }

    fn get_response_for_widget<T: gtk::WidgetTrait>(&self, widget: &T) -> Result<i32, gtk::ResponseType> {
        let tmp = unsafe { ffi::gtk_dialog_get_response_for_widget(GTK_DIALOG(self.unwrap_widget()), widget.unwrap_widget()) };

        if tmp < 0 {
            Err(unsafe { ::std::mem::transmute(tmp) })
        } else {
            Ok(tmp)
        }
    }

    fn get_widget_for_reponse<T: gtk::WidgetTrait>(&self, response_id: i32) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_widget_for_response(GTK_DIALOG(self.unwrap_widget()), response_id) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    fn get_action_area<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_action_area(GTK_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    fn get_content_area<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_content_area(GTK_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    #[cfg(any(feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn get_header_bar<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_header_bar(GTK_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}
