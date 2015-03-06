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

use gtk::{self, ffi};
use gtk::FFIWidget;
use gtk::cast::{GTK_WINDOW, GTK_APP_CHOOSER_DIALOG};
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};

struct_Widget!(AppChooserDialog);

impl AppChooserDialog {
    pub fn new_for_content_type(parent: Option<gtk::Window>, flags: gtk::DialogFlags, content_type: &str) -> Option<AppChooserDialog> {
        let tmp_pointer = unsafe {
            let parent = match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => ::std::ptr::null_mut()
            };
            let mut tmp_content_type = content_type.to_tmp_for_borrow();

            ffi::gtk_app_chooser_dialog_new_for_content_type(parent, flags,
                                                             tmp_content_type.to_glib_ptr())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn widget<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_dialog_get_widget(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_heading(&self, heading: &str) -> () {
        unsafe {
            let mut tmp_heading = heading.to_tmp_for_borrow();
            ffi::gtk_app_chooser_dialog_set_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget()), tmp_heading.to_glib_ptr())
        }
    }

    pub fn get_heading(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_app_chooser_dialog_get_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())))
        }
    }
}

impl_drop!(AppChooserDialog);
impl_TraitWidget!(AppChooserDialog);

impl gtk::ContainerTrait for AppChooserDialog {}
impl gtk::BinTrait for AppChooserDialog {}
impl gtk::WindowTrait for AppChooserDialog {}
impl gtk::DialogTrait for AppChooserDialog {}
impl gtk::AppChooserTrait for AppChooserDialog {}

impl_widget_events!(AppChooserDialog);
