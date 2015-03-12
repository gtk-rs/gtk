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

use glib::translate::{FromGlibPtr, ToGlibPtr};
use gtk::cast::{GTK_FONT_CHOOSER};
use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::FFIWidget;
use libc::c_char;

pub trait FontChooserTrait: gtk::WidgetTrait {
    fn get_font_size(&self) -> i32 {
        unsafe { ffi::gtk_font_chooser_get_font_size(GTK_FONT_CHOOSER(self.unwrap_widget())) }
    }

    fn get_font(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_font_chooser_get_font(GTK_FONT_CHOOSER(self.unwrap_widget())) as *const c_char)
        }
    }

    fn set_font(&self, font_name: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(GTK_FONT_CHOOSER(self.unwrap_widget()), font_name.borrow_to_glib().0)
        }
    }

    fn get_preview_text(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_font_chooser_get_preview_text(GTK_FONT_CHOOSER(self.unwrap_widget()))
                    as *const c_char)
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(GTK_FONT_CHOOSER(self.unwrap_widget()), text.borrow_to_glib().0)
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_chooser_get_show_preview_entry(GTK_FONT_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe { ffi::gtk_font_chooser_set_show_preview_entry(GTK_FONT_CHOOSER(self.unwrap_widget()),
                                                              to_gboolean(show_preview_entry));
        }
    }
}
