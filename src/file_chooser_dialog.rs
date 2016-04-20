// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib::object::{Downcast, IsA};
use std::ptr;
use FileChooserAction;
use FileChooserDialog;
use Widget;
use Window;

impl FileChooserDialog {
    pub fn new<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>, action: FileChooserAction)
            -> FileChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_file_chooser_dialog_new(title.to_glib_none().0, parent.to_glib_none().0,
                    action.to_glib(), ptr::null_mut()))
                .downcast_unchecked()
        }
    }
}
