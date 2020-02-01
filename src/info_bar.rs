// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{from_glib_none, ToGlibPtr};
use glib::{Cast, IsA};
use gtk_sys;
use Box;
use InfoBar;
use Widget;

pub trait InfoBarExtManual: 'static {
    fn get_content_area(&self) -> Box;
}

impl<T: IsA<InfoBar>> InfoBarExtManual for T {
    fn get_content_area(&self) -> Box {
        unsafe {
            let widget: Widget = from_glib_none(gtk_sys::gtk_info_bar_get_content_area(
                self.as_ref().to_glib_none().0,
            ));
            widget
                .downcast()
                .expect("gtk_info_bar_get_content_area returns not GtkBox")
        }
    }
}
