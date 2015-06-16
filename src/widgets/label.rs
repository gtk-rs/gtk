// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use Justification;

/// A widget that displays a small to medium amount of text.
pub type Label = Object<ffi::GtkLabel>;

impl Label {
    pub fn new(text: &str) -> Label {
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new(text.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(text: &str) -> Label {
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new_with_mnemonic(text.to_glib_none().0))
                .downcast_unchecked()
        }
    }
}

impl types::StaticType for Label {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_label_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Label { }
unsafe impl Upcast<super::misc::Misc> for Label { }
unsafe impl Upcast<::builder::Buildable> for Label { }

pub trait LabelExt {
    fn set_label(&self, text: &str);
    fn set_text(&self, text: &str);
    fn set_justify(&self, jtype: Justification);
    fn set_markup(&self, text: &str);
    fn set_markup_with_mnemonic(&self, text: &str);
    fn set_pattern(&self, text: &str);
    fn set_text_with_mnemonic(&self, text: &str);
    fn set_width_chars(&self, n_chars: i32);
    fn set_max_width_chars(&self, n_chars: i32);
    fn set_line_wrap(&self, wrap: bool);
    fn get_line_wrap(&self) -> bool;
    #[cfg(feature = "gtk_3_10")]
    fn set_lines(&self, lines: i32);
    #[cfg(feature = "gtk_3_10")]
    fn get_lines(&self) -> i32;
    fn get_layout_offsets(&self) -> (i32, i32);
    fn get_mnemonic_keyval(&self) -> u32;
    fn set_selectable(&self, selectable: bool);
    fn get_selectable(&self) -> bool;
    fn set_use_markup(&self, use_markup: bool);
    fn get_use_markup(&self) -> bool;
    fn set_use_underline(&self, use_underline: bool);
    fn get_use_underline(&self) -> bool;
    fn set_single_line_mode(&self, single_line_mode: bool);
    fn get_single_line_mode(&self) -> bool;
    fn set_track_visited_links(&self, track_visited_links: bool);
    fn get_track_visited_links(&self) -> bool;
    fn get_text(&self) -> Option<String>;
    fn get_label(&self) -> Option<String>;
    fn get_current_uri(&self) -> Option<String>;
    fn select_region(&self, start_offset: i32, end_offset: i32);
    fn get_justify(&self) -> Justification;
    fn get_width_chars(&self) -> i32;
    fn get_max_width_chars(&self) -> i32;
    fn get_selection_bounds(&self) -> (i32, i32);
    fn set_angle(&self, angle: f64);
    fn get_angle(&self) -> f64;
}

impl<O: Upcast<Label>> LabelExt for O {
    fn set_label(&self, text: &str) {
        unsafe { ffi::gtk_label_set_label(self.upcast().to_glib_none().0, text.to_glib_none().0) }
    }

    fn set_text(&self, text: &str) {
        unsafe { ffi::gtk_label_set_text(self.upcast().to_glib_none().0, text.to_glib_none().0) }
    }

    fn set_justify(&self, jtype: Justification) {
        unsafe { ffi::gtk_label_set_justify(self.upcast().to_glib_none().0, jtype); }
    }

    fn set_markup(&self, text: &str) {
        unsafe { ffi::gtk_label_set_markup(self.upcast().to_glib_none().0, text.to_glib_none().0) }
    }

    fn set_markup_with_mnemonic(&self, text: &str) {
        unsafe {
            ffi::gtk_label_set_markup_with_mnemonic(self.upcast().to_glib_none().0,
                text.to_glib_none().0)
        }
    }

    fn set_pattern(&self, text: &str) {
        unsafe { ffi::gtk_label_set_pattern(self.upcast().to_glib_none().0, text.to_glib_none().0) }
    }

    fn set_text_with_mnemonic(&self, text: &str) {
        unsafe {
            ffi::gtk_label_set_text_with_mnemonic(self.upcast().to_glib_none().0,
                text.to_glib_none().0);
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe { ffi::gtk_label_set_width_chars(self.upcast().to_glib_none().0, n_chars); }
    }

    fn set_max_width_chars(&self, n_chars: i32) {
        unsafe { ffi::gtk_label_set_max_width_chars(self.upcast().to_glib_none().0, n_chars); }
    }

    fn set_line_wrap(&self, wrap: bool) {
        unsafe { ffi::gtk_label_set_line_wrap(self.upcast().to_glib_none().0, wrap.to_glib()); }
    }

    fn get_line_wrap(&self) -> bool {
        unsafe { from_glib(ffi::gtk_label_get_line_wrap(self.upcast().to_glib_none().0)) }
    }

    #[cfg(feature = "gtk_3_10")]
    fn set_lines(&self, lines: i32) {
        unsafe { ffi::gtk_label_set_lines(self.upcast().to_glib_none().0, lines); }
    }

    #[cfg(feature = "gtk_3_10")]
    fn get_lines(&self) -> i32 {
        unsafe { ffi::gtk_label_get_lines(self.upcast().to_glib_none().0) }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe { ffi::gtk_label_get_layout_offsets(self.upcast().to_glib_none().0, &x, &y); }
        (x, y)
    }

    fn get_mnemonic_keyval(&self) -> u32 {
        unsafe { ffi::gtk_label_get_mnemonic_keyval(self.upcast().to_glib_none().0) }
    }

    fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_label_set_selectable(self.upcast().to_glib_none().0, selectable.to_glib());
        }
    }

    fn get_selectable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_label_get_selectable(self.upcast().to_glib_none().0)) }
    }

    fn set_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::gtk_label_set_use_markup(self.upcast().to_glib_none().0, use_markup.to_glib());
        }
    }

    fn get_use_markup(&self) -> bool {
        unsafe { from_glib(ffi::gtk_label_get_use_markup(self.upcast().to_glib_none().0)) }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_label_set_use_underline(self.upcast().to_glib_none().0, use_underline.to_glib());
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { from_glib(ffi::gtk_label_get_use_underline(self.upcast().to_glib_none().0)) }
    }

    fn set_single_line_mode(&self, single_line_mode: bool) {
        unsafe {
            ffi::gtk_label_set_single_line_mode(self.upcast().to_glib_none().0,
                single_line_mode.to_glib());
        }
    }

    fn get_single_line_mode(&self) -> bool {
        unsafe { from_glib(ffi::gtk_label_get_single_line_mode(self.upcast().to_glib_none().0)) }
    }

    fn set_track_visited_links(&self, track_visited_links: bool) {
        unsafe {
            ffi::gtk_label_set_track_visited_links(self.upcast().to_glib_none().0,
                track_visited_links.to_glib());
        }
    }

    fn get_track_visited_links(&self) -> bool {
        unsafe { from_glib(ffi::gtk_label_get_track_visited_links(self.upcast().to_glib_none().0)) }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_text(self.upcast().to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_label(self.upcast().to_glib_none().0))
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_current_uri(self.upcast().to_glib_none().0))
        }
    }

    fn select_region(&self, start_offset: i32, end_offset: i32) {
        unsafe {
            ffi::gtk_label_select_region(self.upcast().to_glib_none().0, start_offset, end_offset);
        }
    }

    fn get_justify(&self) -> Justification {
        unsafe {
            ffi::gtk_label_get_justify(self.upcast().to_glib_none().0)
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(self.upcast().to_glib_none().0)
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(self.upcast().to_glib_none().0)
        }
    }

    fn get_selection_bounds(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe { ffi::gtk_label_get_selection_bounds(self.upcast().to_glib_none().0, &x, &y); }
        (x, y)
    }

    fn set_angle(&self, angle: f64) {
        unsafe {
            ffi::gtk_label_set_angle(self.upcast().to_glib_none().0, angle);
        }
    }

    fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(self.upcast().to_glib_none().0)
        }
    }
}
