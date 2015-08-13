// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_int, c_double};
use glib::translate::{from_glib_none, ToGlibPtr};

use ffi;
use glib::{to_bool, to_gboolean};
use Justification;
use cast::GTK_LABEL;

pub trait LabelTrait: ::WidgetTrait {
    fn set_label(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_label_set_label(GTK_LABEL(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn set_text(&self, text: &str) -> () {
        unsafe {
	    ffi::gtk_label_set_text(GTK_LABEL(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn set_justify(&self, jtype: Justification) -> () {
        unsafe {
            ffi::gtk_label_set_justify(GTK_LABEL(self.unwrap_widget()), jtype);
        }
    }

    fn set_markup(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_label_set_markup(GTK_LABEL(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn set_markup_with_mnemonic(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_label_set_markup_with_mnemonic(GTK_LABEL(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn set_pattern(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_label_set_pattern(GTK_LABEL(self.unwrap_widget()), text.to_glib_none().0)
        }
    }

    fn set_text_with_mnemonic(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_label_set_text_with_mnemonic(GTK_LABEL(self.unwrap_widget()), text.to_glib_none().0);
        }
    }

    fn set_width_chars(&self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_label_set_width_chars(GTK_LABEL(self.unwrap_widget()), n_chars as c_int);
        }
    }

    fn set_max_width_chars(&self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_label_set_max_width_chars(GTK_LABEL(self.unwrap_widget()), n_chars as c_int);
        }
    }

    fn set_line_wrap(&self, wrap: bool) -> () {
        unsafe { ffi::gtk_label_set_line_wrap(GTK_LABEL(self.unwrap_widget()), to_gboolean(wrap)); }
    }

    fn get_line_wrap(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_line_wrap(GTK_LABEL(self.unwrap_widget()))) }
    }

    #[cfg(gtk_3_10)]
    fn set_lines(&self, lines: i32) -> () {
        unsafe {
            ffi::gtk_label_set_lines(GTK_LABEL(self.unwrap_widget()), lines as c_int);
        }
    }

    #[cfg(gtk_3_10)]
    fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(GTK_LABEL(self.unwrap_widget())) as c_int
        }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            ffi::gtk_label_get_layout_offsets(GTK_LABEL(self.unwrap_widget()), &mut x, &mut y);
        }
        (x, y)
    }

    fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(GTK_LABEL(self.unwrap_widget())) as u32
        }
    }

    fn set_selectable(&self, selectable: bool) -> () {
        unsafe { ffi::gtk_label_set_selectable(GTK_LABEL(self.unwrap_widget()), to_gboolean(selectable)); }
    }

    fn get_selectable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_selectable(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_use_markup(&self, use_markup: bool) -> () {
        unsafe { ffi::gtk_label_set_use_markup(GTK_LABEL(self.unwrap_widget()), to_gboolean(use_markup)); }
    }

    fn get_use_markup(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_use_markup(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_use_underline(&self, use_underline: bool) -> () {
        unsafe { ffi::gtk_label_set_use_underline(GTK_LABEL(self.unwrap_widget()), to_gboolean(use_underline)); }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_use_underline(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_single_line_mode(&self, single_line_mode: bool) -> () {
        unsafe { ffi::gtk_label_set_single_line_mode(GTK_LABEL(self.unwrap_widget()), to_gboolean(single_line_mode)); }
    }

    fn get_single_line_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_single_line_mode(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_track_visited_links(&self, track_visited_links: bool) -> () {
        unsafe { ffi::gtk_label_set_track_visited_links(GTK_LABEL(self.unwrap_widget()), to_gboolean(track_visited_links)); }
    }

    fn get_track_visited_links(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_track_visited_links(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_text(GTK_LABEL(self.unwrap_widget())))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_label(GTK_LABEL(self.unwrap_widget())))
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_current_uri(GTK_LABEL(self.unwrap_widget())))
        }
    }

    fn select_region(&self, start_offset: i32, end_offset: i32) -> () {
        unsafe {
            ffi::gtk_label_select_region(GTK_LABEL(self.unwrap_widget()), start_offset as c_int, end_offset as c_int);
        }
    }

    fn get_justify(&self) -> Justification {
        unsafe {
            ffi::gtk_label_get_justify(GTK_LABEL(self.unwrap_widget()))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(GTK_LABEL(self.unwrap_widget())) as i32
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(GTK_LABEL(self.unwrap_widget())) as i32
        }
    }

    fn get_selection_bounds(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            ffi::gtk_label_get_selection_bounds(GTK_LABEL(self.unwrap_widget()), &mut x, &mut y);
        }
        (x, y)
    }

    fn set_angle(&self, angle: f64) -> () {
        unsafe {
            ffi::gtk_label_set_angle(GTK_LABEL(self.unwrap_widget()), angle as c_double);
        }
    }

    fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(GTK_LABEL(self.unwrap_widget())) as f64
        }
    }
}
