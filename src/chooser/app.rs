// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use glib::AppInfo;
use ffi;

use object::{Object, Upcast};
use widgets::widget::Widget;

pub type AppChooser = Object<ffi::GtkAppChooser>;

impl types::StaticType for AppChooser {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_app_chooser_get_type()) }
    }
}

unsafe impl Upcast<Widget> for AppChooser { }

pub trait AppChooserExt {
    fn get_app_info(&self) -> Option<AppInfo>;
    fn get_content_type(&self) -> Option<String>;
    fn refresh(&self);
}

impl<O: Upcast<AppChooser>> AppChooserExt for O {
    fn get_app_info(&self) -> Option<AppInfo> {
        unsafe { from_glib_full(ffi::gtk_app_chooser_get_app_info(self.upcast().to_glib_none().0)) }
    }

    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_app_chooser_get_content_type(self.upcast().to_glib_none().0))
        }
    }

    fn refresh(&self) {
        unsafe { ffi::gtk_app_chooser_refresh(self.upcast().to_glib_none().0) }
    }
}
