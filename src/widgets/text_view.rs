// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTextView — Widget that displays a GtkTextBuffer

use ffi;
use TextBuffer;
use FFIWidget;
use cast::{GTK_TEXT_VIEW, GTK_TEXT_BUFFER};
use glib::{to_bool, to_gboolean};
use glib::translate::{ToGlibPtr, ToGlibPtrMut};
use TextIter;

struct_Widget!(TextView);

impl TextView {
    pub fn new() -> Option<TextView> {
        let tmp_pointer = unsafe { ffi::gtk_text_view_new() };
        check_pointer!(tmp_pointer, TextView)
    }

    pub fn new_with_buffer(buffer: TextBuffer) -> Option<TextView> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_view_new_with_buffer(GTK_TEXT_BUFFER(buffer.unwrap_widget()))
        };
        check_pointer!(tmp_pointer, TextView)
    }

    pub fn set_buffer(&self, buffer: TextBuffer) -> () {
        unsafe {
            ffi::gtk_text_view_set_buffer(GTK_TEXT_VIEW(self.unwrap_widget()), GTK_TEXT_BUFFER(buffer.unwrap_widget()));
        }
    }

    pub fn get_buffer(&self) -> Option<TextBuffer> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_view_get_buffer(GTK_TEXT_VIEW(self.unwrap_widget()))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::GtkWidget))
        }
    }

    pub fn scroll_to_mark(&self, mark: &::TextMark, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) {
        unsafe { ffi::gtk_text_view_scroll_to_mark(GTK_TEXT_VIEW(self.unwrap_widget()), mark.unwrap_pointer(), within_margin,
            to_gboolean(use_align), xalign, yalign) }
    }

    pub fn scroll_to_iter(&self, iter: &mut TextIter, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_scroll_to_iter(GTK_TEXT_VIEW(self.unwrap_widget()), iter.to_glib_none_mut().0, within_margin,
            to_gboolean(use_align), xalign, yalign)) }
    }

    pub fn scroll_mark_onscreen(&self, mark: &::TextMark) {
        unsafe { ffi::gtk_text_view_scroll_mark_onscreen(GTK_TEXT_VIEW(self.unwrap_widget()), mark.unwrap_pointer()) }
    }

    pub fn move_mark_onscreen(&self, mark: &::TextMark) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_move_mark_onscreen(GTK_TEXT_VIEW(self.unwrap_widget()), mark.unwrap_pointer())) }
    }

    pub fn place_cursor_onscreen(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_place_cursor_onscreen(GTK_TEXT_VIEW(self.unwrap_widget()))) }
    }

    pub fn get_line_at_y(&self, y: i32) -> (TextIter, i32) {
        unsafe {
            let mut target_iter = TextIter::uninitialized();
            let mut line_top = 0;
            ffi::gtk_text_view_get_line_at_y(GTK_TEXT_VIEW(self.unwrap_widget()),
                target_iter.to_glib_none_mut().0, y, &mut line_top);
            (target_iter, line_top)
        }
    }

    pub fn get_iter_at_location(&self, x: i32, y: i32) -> TextIter {
        unsafe {
            let mut target_iter = TextIter::uninitialized();
            ffi::gtk_text_view_get_iter_at_location(GTK_TEXT_VIEW(self.unwrap_widget()),
                target_iter.to_glib_none_mut().0, x, y);
            target_iter
        }
    }

    pub fn buffer_to_window_coords(&self, win: ::TextWindowType, buffer_x: i32, buffer_y: i32, window_x: *mut i32, window_y: &mut i32) {
        unsafe { ffi::gtk_text_view_buffer_to_window_coords(GTK_TEXT_VIEW(self.unwrap_widget()), win, buffer_x as ::libc::c_int,
            buffer_y as ::libc::c_int, window_x, window_y) }
    }

    pub fn window_to_buffer_coords(&self, win: ::TextWindowType, window_x: i32, window_y: i32, buffer_x: *mut i32, buffer_y: &mut i32) {
        unsafe { ffi::gtk_text_view_window_to_buffer_coords(GTK_TEXT_VIEW(self.unwrap_widget()), win, window_x as ::libc::c_int,
            window_y as ::libc::c_int, buffer_x, buffer_y) }
    }

    pub fn set_border_window_size(&self, _type: ::TextWindowType, size: i32) {
        unsafe { ffi::gtk_text_view_set_border_window_size(GTK_TEXT_VIEW(self.unwrap_widget()), _type, size as ::libc::c_int) }
    }

    pub fn get_border_window_size(&self, _type: ::TextWindowType) -> i32 {
        unsafe { ffi::gtk_text_view_get_border_window_size(GTK_TEXT_VIEW(self.unwrap_widget()), _type) }
    }

    pub fn forward_display_line(&self, iter: &mut TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_forward_display_line(GTK_TEXT_VIEW(self.unwrap_widget()), iter.to_glib_none_mut().0)) }
    }

    pub fn backward_display_line(&self, iter: &mut TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_forward_display_line(GTK_TEXT_VIEW(self.unwrap_widget()), iter.to_glib_none_mut().0)) }
    }

    pub fn forward_display_line_end(&self, iter: &mut TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_forward_display_line_end(GTK_TEXT_VIEW(self.unwrap_widget()), iter.to_glib_none_mut().0)) }
    }

    pub fn backward_display_line_start(&self, iter: &mut TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_backward_display_line_start(GTK_TEXT_VIEW(self.unwrap_widget()), iter.to_glib_none_mut().0)) }
    }

    pub fn starts_display_line(&self, iter: &TextIter) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_starts_display_line(GTK_TEXT_VIEW(self.unwrap_widget()),
            iter.to_glib_none().0)) }
    }

    pub fn move_visually(&self, iter: &mut TextIter, count: i32) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_move_visually(GTK_TEXT_VIEW(self.unwrap_widget()), iter.to_glib_none_mut().0,
            count as ::libc::c_int)) }
    }

    pub fn add_child_at_anchor<T: ::WidgetTrait>(&self, child: &T, anchor: &::TextChildAnchor) {
        unsafe { ffi::gtk_text_view_add_child_at_anchor(GTK_TEXT_VIEW(self.unwrap_widget()), child.unwrap_widget(), anchor.unwrap_pointer()) }
    }

    pub fn add_child_in_window<T: ::WidgetTrait>(&self, child: &T, which_window: ::TextWindowType, xpos: i32, ypos: i32) {
        unsafe { ffi::gtk_text_view_add_child_in_window(GTK_TEXT_VIEW(self.unwrap_widget()), child.unwrap_widget(), which_window,
            xpos as ::libc::c_int, ypos as ::libc::c_int) }
    }

    pub fn move_child<T: ::WidgetTrait>(&self, child: &T, xpos: i32, ypos: i32) {
        unsafe { ffi::gtk_text_view_move_child(GTK_TEXT_VIEW(self.unwrap_widget()), child.unwrap_widget(), xpos as ::libc::c_int,
            ypos as ::libc::c_int) }
    }

    pub fn set_wrap_mode(&self, wrap_mode: ::WrapMode) {
        unsafe { ffi::gtk_text_view_set_wrap_mode(GTK_TEXT_VIEW(self.unwrap_widget()), wrap_mode) }
    }

    pub fn get_wrap_mode(&self) -> ::WrapMode {
        unsafe { ffi::gtk_text_view_get_wrap_mode(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_editable(&self, setting: bool) {
        unsafe { ffi::gtk_text_view_set_editable(GTK_TEXT_VIEW(self.unwrap_widget()), to_gboolean(setting)) }
    }

    pub fn get_editable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_get_editable(GTK_TEXT_VIEW(self.unwrap_widget()))) }
    }

    pub fn set_cursor_visible(&self, setting: bool) {
        unsafe { ffi::gtk_text_view_set_cursor_visible(GTK_TEXT_VIEW(self.unwrap_widget()), to_gboolean(setting)) }
    }

    pub fn get_cursor_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_get_cursor_visible(GTK_TEXT_VIEW(self.unwrap_widget()))) }
    }

    pub fn set_overwrite(&self, overwrite: bool) {
        unsafe { ffi::gtk_text_view_set_overwrite(GTK_TEXT_VIEW(self.unwrap_widget()), to_gboolean(overwrite)) }
    }

    pub fn get_overwrite(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_get_overwrite(GTK_TEXT_VIEW(self.unwrap_widget()))) }
    }

    pub fn set_pixels_above_lines(&self, pixels_above_lines: i32) {
        unsafe { ffi::gtk_text_view_set_pixels_above_lines(GTK_TEXT_VIEW(self.unwrap_widget()), pixels_above_lines as ::libc::c_int) }
    }

    pub fn get_pixels_above_lines(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_pixels_above_lines(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_pixels_below_lines(&self, pixels_below_lines: i32) {
        unsafe { ffi::gtk_text_view_set_pixels_below_lines(GTK_TEXT_VIEW(self.unwrap_widget()), pixels_below_lines as ::libc::c_int) }
    }

    pub fn get_pixels_below_lines(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_pixels_below_lines(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_pixels_inside_wrap(&self, pixels_inside_wrap: i32) {
        unsafe { ffi::gtk_text_view_set_pixels_inside_wrap(GTK_TEXT_VIEW(self.unwrap_widget()), pixels_inside_wrap as ::libc::c_int) }
    }

    pub fn get_pixels_inside_wrap(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_pixels_inside_wrap(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_justification(&self, justification: ::Justification) {
        unsafe { ffi::gtk_text_view_set_justification(GTK_TEXT_VIEW(self.unwrap_widget()), justification) }
    }

    pub fn get_justification(&self) -> ::Justification {
        unsafe { ffi::gtk_text_view_get_justification(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_left_margin(&self, left_margin: i32) {
        unsafe { ffi::gtk_text_view_set_left_margin(GTK_TEXT_VIEW(self.unwrap_widget()), left_margin as ::libc::c_int) }
    }

    pub fn get_left_margin(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_left_margin(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_right_margin(&self, right_margin: i32) {
        unsafe { ffi::gtk_text_view_set_right_margin(GTK_TEXT_VIEW(self.unwrap_widget()), right_margin as ::libc::c_int) }
    }

    pub fn get_right_margin(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_right_margin(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_indent(&self, indent: i32) {
        unsafe { ffi::gtk_text_view_set_indent(GTK_TEXT_VIEW(self.unwrap_widget()), indent as ::libc::c_int) }
    }

    pub fn get_indent(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_indent(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_accepts_tab(&self, accepts_tab: bool) {
        unsafe { ffi::gtk_text_view_set_accepts_tab(GTK_TEXT_VIEW(self.unwrap_widget()), to_gboolean(accepts_tab)) }
    }

    pub fn get_accepts_tab(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_view_get_accepts_tab(GTK_TEXT_VIEW(self.unwrap_widget()))) }
    }

    pub fn get_default_attributes(&self) -> Option<::TextAttributes> {
        let tmp_pointer = unsafe { ffi::gtk_text_view_get_default_attributes(GTK_TEXT_VIEW(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::TextAttributes::wrap_pointer(tmp_pointer))
        }
    }

    pub fn reset_im_context(&self) {
        unsafe { ffi::gtk_text_view_reset_im_context(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_input_purpose(&self, purpose: ::InputPurpose) {
        unsafe { ffi::gtk_text_view_set_input_purpose(GTK_TEXT_VIEW(self.unwrap_widget()), purpose) }
    }

    pub fn get_input_purpose(&self) -> ::InputPurpose {
        unsafe { ffi::gtk_text_view_get_input_purpose(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }

    pub fn set_input_hints(&self, hints: ::InputHints) {
        unsafe { ffi::gtk_text_view_set_input_hints(GTK_TEXT_VIEW(self.unwrap_widget()), hints) }
    }

    pub fn get_input_hints(&self) -> ::InputHints {
        unsafe { ffi::gtk_text_view_get_input_hints(GTK_TEXT_VIEW(self.unwrap_widget())) }
    }
}

impl_drop!(TextView);
impl_TraitWidget!(TextView);

impl ::ScrollableTrait for TextView {}
