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

use ffi;
use FFIWidget;
use cast::{GTK_WINDOW, GTK_APP_CHOOSER_DIALOG};
use glib::translate::{FromGlibPtr, ToGlibPtr};

struct_Widget!(AppChooserDialog);

impl AppChooserDialog {
    pub fn new_for_content_type(parent: Option<::Window>, flags: ::DialogFlags, content_type: &str) -> Option<AppChooserDialog> {
        let tmp_pointer = unsafe {
            let parent = match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => ::std::ptr::null_mut()
            };

            ffi::gtk_app_chooser_dialog_new_for_content_type(parent, flags,
                                                             content_type.borrow_to_glib().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn widget<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_dialog_get_widget(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_heading(&self, heading: &str) -> () {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget()), heading.borrow_to_glib().0)
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

impl ::ContainerTrait for AppChooserDialog {}
impl ::BinTrait for AppChooserDialog {}
impl ::WindowTrait for AppChooserDialog {}
impl ::DialogTrait for AppChooserDialog {}
impl ::AppChooserTrait for AppChooserDialog {}

impl_widget_events!(AppChooserDialog);
