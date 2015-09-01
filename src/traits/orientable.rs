// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Orientation;
use cast::GTK_ORIENTABLE;
use ffi;

pub trait OrientableTrait: ::WidgetTrait {
    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_orientable_get_orientation(GTK_ORIENTABLE(self.unwrap_widget()))
        }
    }

    fn set_orientation(&self, orientation: Orientation) -> () {
        unsafe {
            ffi::gtk_orientable_set_orientation(GTK_ORIENTABLE(self.unwrap_widget()), orientation)
        }
    }
}
