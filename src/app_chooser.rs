// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gio::AppInfo;
use ffi;

use glib::object::IsA;
use Widget;

glib_wrapper! {
    pub struct AppChooser(Object<ffi::GtkAppChooser>): Widget;

    match fn {
        get_type => || ffi::gtk_app_chooser_get_type(),
    }
}

pub trait AppChooserExt {
    fn get_app_info(&self) -> Option<AppInfo>;
    fn get_content_type(&self) -> Option<String>;
    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    fn get_app_info(&self) -> Option<AppInfo> {
        unsafe { from_glib_full(ffi::gtk_app_chooser_get_app_info(self.to_glib_none().0)) }
    }

    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_app_chooser_get_content_type(self.to_glib_none().0))
        }
    }

    fn refresh(&self) {
        unsafe { ffi::gtk_app_chooser_refresh(self.to_glib_none().0) }
    }
}
