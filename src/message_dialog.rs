// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_char;
use ffi;
use glib::translate::*;
use glib::object::{Downcast, IsA};
use std::ptr;
use ButtonsType;
use DialogFlags;
use MessageDialog;
use MessageType;
use Widget;
use Window;

impl MessageDialog {
    pub fn new<T: IsA<Window>>(parent: Option<&T>, flags: DialogFlags, type_: MessageType,
            buttons: ButtonsType, message: &str) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(
                ffi::gtk_message_dialog_new(parent.to_glib_none().0, flags, type_, buttons,
                    b"%s\0".as_ptr() as *const c_char, message.0, ptr::null_mut::<*const c_char>()))
                .downcast_unchecked()
        }
    }
}
