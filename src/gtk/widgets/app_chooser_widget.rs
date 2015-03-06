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

//! Application chooser widget that can be embedded in other widgets

use gtk::cast::GTK_APP_CHOOSER_WIDGET;
use gtk::{self, ffi};
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};
use glib::{to_bool, to_gboolean};

struct_Widget!(AppChooserWidget);

impl AppChooserWidget {
    pub fn new(content_type: &str) -> Option<AppChooserWidget> {
        let tmp_pointer = unsafe {
            let mut tmp_content_type = content_type.to_tmp_for_borrow();
            ffi::gtk_app_chooser_widget_new(tmp_content_type.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, AppChooserWidget)
    }

    pub fn set_show_default(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_default(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_default(&mut self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_default(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_recommended(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_recommended(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_recommended(&mut self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_recommended(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_fallback(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_fallback(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_fallback(&mut self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_fallback(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_other(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_other(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_other(&mut self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_other(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_all(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_all(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_all(&mut self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_all(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_default_text(&self, text: &str) {
        unsafe {
            let mut tmp_text = text.to_tmp_for_borrow();
            ffi::gtk_app_chooser_widget_set_default_text(GTK_APP_CHOOSER_WIDGET(self.pointer), tmp_text.to_glib_ptr())
        }
    }

    pub fn get_default_text(&mut self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_app_chooser_widget_get_default_text(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }
}

impl_drop!(AppChooserWidget);
impl_TraitWidget!(AppChooserWidget);

impl gtk::ContainerTrait for AppChooserWidget {}
impl gtk::BoxTrait for AppChooserWidget {}

impl_widget_events!(AppChooserWidget);
