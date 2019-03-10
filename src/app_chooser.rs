// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio::AppInfo;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use Widget;

glib_wrapper! {
    pub struct AppChooser(Interface<gtk_sys::GtkAppChooser>) @requires Widget;

    match fn {
        get_type => || gtk_sys::gtk_app_chooser_get_type(),
    }
}

pub trait AppChooserExt: 'static {
    fn get_app_info(&self) -> Option<AppInfo>;
    fn get_content_type(&self) -> Option<String>;
    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    fn get_app_info(&self) -> Option<AppInfo> {
        unsafe { from_glib_full(gtk_sys::gtk_app_chooser_get_app_info(self.as_ref().to_glib_none().0)) }
    }

    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                gtk_sys::gtk_app_chooser_get_content_type(self.as_ref().to_glib_none().0))
        }
    }

    fn refresh(&self) {
        unsafe { gtk_sys::gtk_app_chooser_refresh(self.as_ref().to_glib_none().0) }
    }
}
