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

use std::c_str::ToCStr;
use gtk::cast::{GTK_FONT_CHOOSER};
use gtk::{self, ffi};
use gtk::ffi::FFIWidget;
use libc::c_char;

pub trait FontChooserTrait: gtk::WidgetTrait {
    fn get_font_size(&self) -> i32 {
        unsafe { ffi::gtk_font_chooser_get_font_size(GTK_FONT_CHOOSER(self.get_widget())) }
    }

    fn get_font(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_font_chooser_get_font(GTK_FONT_CHOOSER(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(tmp as *const u8) })
        }
    }

    fn set_font(&self, font_name: &str) {
        unsafe {
            font_name.with_c_str(|c_str| {
                ffi::gtk_font_chooser_set_font(GTK_FONT_CHOOSER(self.get_widget()), c_str as *mut c_char)
            })
        }
    }

    fn get_preview_text(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_font_chooser_get_preview_text(GTK_FONT_CHOOSER(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(tmp as *const u8) })
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            text.with_c_str(|c_str| {
                ffi::gtk_font_chooser_set_preview_text(GTK_FONT_CHOOSER(self.get_widget()), c_str)
            })
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        match unsafe { ffi::gtk_font_chooser_get_show_preview_entry(GTK_FONT_CHOOSER(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe { ffi::gtk_font_chooser_set_show_preview_entry(GTK_FONT_CHOOSER(self.get_widget()), match show_preview_entry {
            true => ffi::GTRUE,
            false => ffi::GFALSE
        }) }
    }
}