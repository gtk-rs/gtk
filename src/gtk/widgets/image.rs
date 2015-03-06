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

//! A widget displaying an image

use gtk::{self, ffi};
use gtk::cast::GTK_IMAGE;
use gtk::FFIWidget;
use glib::translate::{ToGlibPtr, ToTmp};

/// Image — A widget displaying an image
struct_Widget!(Image);

impl Image {
    pub fn new_from_file(filename: &str) -> Option<Image> {
        let tmp_pointer = unsafe {
            let mut tmp_filename = filename.to_tmp_for_borrow();
            ffi::gtk_image_new_from_file(tmp_filename.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, Image)
    }

    pub fn new_from_icon_name(icon_name: &str, size: gtk::IconSize) -> Option<Image> {
        let tmp_pointer = unsafe {
            let mut tmp_icon_name = icon_name.to_tmp_for_borrow();
            ffi::gtk_image_new_from_icon_name(tmp_icon_name.to_glib_ptr(), size)
        };
        check_pointer!(tmp_pointer, Image)
    }

    pub fn set_from_file(&self, filename: &str) {
        unsafe {
            let mut tmp_filename = filename.to_tmp_for_borrow();
            ffi::gtk_image_set_from_file(GTK_IMAGE(self.unwrap_widget()),
                                         tmp_filename.to_glib_ptr());
        };
    }

    pub fn set_from_icon_name(&self, icon_name: &str, size: gtk::IconSize) {
        unsafe {
            let mut tmp_icon_name = icon_name.to_tmp_for_borrow();
            ffi::gtk_image_set_from_icon_name(GTK_IMAGE(self.unwrap_widget()),
                                              tmp_icon_name.to_glib_ptr(), size)
        };
    }
}

impl_drop!(Image);
impl_TraitWidget!(Image);

impl gtk::MiscTrait for Image {}

impl_widget_events!(Image);
