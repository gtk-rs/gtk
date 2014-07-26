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

use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk::traits;
use gtk::cast::{GTK_WINDOW, GTK_APP_CHOOSER_DIALOG};
use gtk;
use std::string;

struct_Widget!(AppChooserDialog)

impl AppChooserDialog {
    pub fn new_for_content_type(parent: Option<gtk::Window>, flags: gtk::DialogFlags, content_type: &str) -> Option<AppChooserDialog> {
        let tmp_pointer = unsafe {
            content_type.with_c_str(|c_str|{
                ffi::gtk_app_chooser_dialog_new_for_content_type(match parent {
                    Some(ref p) => GTK_WINDOW(p.get_widget()),
                    None => ::std::ptr::mut_null()
                }, flags, c_str)
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn widget<T: traits::Widget>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_dialog_get_widget(GTK_APP_CHOOSER_DIALOG(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn set_heading(&self, heading: &str) -> () {
        unsafe {
            heading.with_c_str(|c_str| {
                ffi::gtk_app_chooser_dialog_set_heading(GTK_APP_CHOOSER_DIALOG(self.get_widget()), c_str)
            })
        }
    }

    pub fn get_heading(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_dialog_get_heading(GTK_APP_CHOOSER_DIALOG(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { string::raw::from_buf(tmp_pointer as *const u8) })
        }
    }
}

impl_drop!(AppChooserDialog)
impl_TraitWidget!(AppChooserDialog)

impl traits::Container for AppChooserDialog {}
impl traits::Bin for AppChooserDialog {}
impl traits::Window for AppChooserDialog {}
impl traits::Dialog for AppChooserDialog {}
impl traits::AppChooser for AppChooserDialog {}