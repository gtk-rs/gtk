// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT recent at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE recent or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use glib::object::{Downcast, IsA};
use std::ptr;
use RecentChooserDialog;
use RecentManager;
use Widget;
use Window;

impl RecentChooserDialog {
    pub fn new<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>) -> RecentChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_recent_chooser_dialog_new(title.to_glib_none().0, parent.to_glib_none().0,
                    ptr::null_mut()))
                .downcast_unchecked()
        }
    }

    pub fn new_for_manager<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>,
            manager: &RecentManager) -> RecentChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_recent_chooser_dialog_new_for_manager(title.to_glib_none().0,
                    parent.to_glib_none().0, manager.to_glib_none().0, ptr::null_mut()))
                .downcast_unchecked()
        }
    }
}
