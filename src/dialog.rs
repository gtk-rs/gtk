// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use std::ptr;
use Box;
use Dialog;
use DialogFlags;
use Downcast;
use Upcast;
use Widget;
use Window;
use auto::traits::DialogExt as Auto;

impl Dialog {
    pub fn new_with_buttons<T: Upcast<Window> = Window>(title: Option<&str>, parent: Option<&T>,
            flags: DialogFlags, buttons: &[(&str, i32)]) -> Dialog {
        assert_initialized_main_thread!();
        let ret: Dialog = unsafe {
            Widget::from_glib_none(
                ffi::gtk_dialog_new_with_buttons(title.to_glib_none().0, parent.to_glib_none().0,
                    flags, ptr::null_mut()))
                .downcast_unchecked()
        };

        ret.add_buttons(buttons);
        ret
    }

    }

pub trait DialogExt {
    // -- manual --
    fn add_buttons(&self, buttons: &[(&str, i32)]);
    // -- auto --
    fn add_action_widget<T: Upcast<Widget>>(&self, child: &T, response_id: i32);
    fn add_button(&self, button_text: &str, response_id: i32) -> Widget;
    fn get_action_area(&self) -> Widget;
    fn get_content_area(&self) -> Box;
    #[cfg(gtk_3_12)]
    fn get_header_bar(&self) -> Option<Widget>;
    fn get_response_for_widget<T: Upcast<Widget>>(&self, widget: &T) -> i32;
    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget>;
    fn response(&self, response_id: i32);
    fn run(&self) -> i32;
    fn set_default_response(&self, response_id: i32);
    fn set_response_sensitive(&self, response_id: i32, setting: bool);
}

impl<O: Upcast<Dialog>> DialogExt for O {
    fn add_buttons(&self, buttons: &[(&str, i32)]) {
        for &(text, id) in buttons {
            Auto::add_button(self, text, id);
        }
    }

    #[inline]
    fn add_action_widget<T: Upcast<Widget>>(&self, child: &T, response_id: i32) {
        Auto::add_action_widget(self, child, response_id)
    }

    #[inline]
    fn add_button(&self, button_text: &str, response_id: i32) -> Widget {
        Auto::add_button(self, button_text, response_id)
    }

    #[inline]
    fn get_action_area(&self) -> Widget {
        Auto::get_action_area(self)
    }

    #[inline]
    fn get_content_area(&self) -> Box {
        Auto::get_content_area(self)
    }

    #[cfg(gtk_3_12)]
    #[inline]
    fn get_header_bar(&self) -> Option<Widget> {
        Auto::get_header_bar(self)
    }

    #[inline]
    fn get_response_for_widget<T: Upcast<Widget>>(&self, widget: &T) -> i32 {
        Auto::get_response_for_widget(self, widget)
    }

    #[inline]
    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget> {
        Auto::get_widget_for_response(self, response_id)
    }

    #[inline]
    fn response(&self, response_id: i32) {
        Auto::response(self, response_id)
    }

    #[inline]
    fn run(&self) -> i32 {
        Auto::run(self)
    }

    #[inline]
    fn set_default_response(&self, response_id: i32) {
        Auto::set_default_response(self, response_id)
    }

    #[inline]
    fn set_response_sensitive(&self, response_id: i32, setting: bool) {
        Auto::set_response_sensitive(self, response_id, setting)
    }

}
