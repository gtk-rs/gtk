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

use cast::GTK_APP_CHOOSER_WIDGET;
use ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::{to_bool, to_gboolean};

struct_Widget!(AppChooserWidget);

impl AppChooserWidget {
    pub fn new(content_type: &str) -> Option<AppChooserWidget> {
        let tmp_pointer = unsafe {
            ffi::gtk_app_chooser_widget_new(content_type.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, AppChooserWidget)
    }

    pub fn set_show_default(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_default(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_default(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_default(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_recommended(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_recommended(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_recommended(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_recommended(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_fallback(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_fallback(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_fallback(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_fallback(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_other(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_other(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_other(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_other(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_all(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_all(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_all(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_all(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(GTK_APP_CHOOSER_WIDGET(self.pointer), text.borrow_to_glib().0)
        }
    }

    pub fn get_default_text(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_app_chooser_widget_get_default_text(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }
}

impl_drop!(AppChooserWidget);
impl_TraitWidget!(AppChooserWidget);

impl ::ContainerTrait for AppChooserWidget {}
impl ::BoxTrait for AppChooserWidget {}

impl_widget_events!(AppChooserWidget);
