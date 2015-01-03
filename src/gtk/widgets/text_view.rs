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

//! GtkTextView — Widget that displays a GtkTextBuffer

use gtk::{self, ffi};
use gtk::TextBuffer;
use gtk::ffi::FFIWidget;
use gtk::cast::{GTK_TEXT_VIEW, GTK_TEXT_BUFFER};

struct_Widget!(TextView);

impl TextView {
    pub fn new() -> Option<TextView> {
        let tmp_pointer = unsafe { ffi::gtk_text_view_new() };
        check_pointer!(tmp_pointer, TextView)
    }

    pub fn new_with_buffer(buffer: gtk::TextBuffer) -> Option<TextView> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_view_new_with_buffer(GTK_TEXT_BUFFER(buffer.get_widget()))
        };
        check_pointer!(tmp_pointer, TextView)
    }

    pub fn set_buffer(&mut self, buffer: gtk::TextBuffer) -> () {
        unsafe {
            ffi::gtk_text_view_set_buffer(GTK_TEXT_VIEW(self.get_widget()), GTK_TEXT_BUFFER(buffer.get_widget()));
        }
    }

    pub fn get_buffer(&self) -> Option<gtk::TextBuffer> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_view_get_buffer(GTK_TEXT_VIEW(self.get_widget()))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn scroll_to_mark(&self, mark: &gtk::TextMark, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) {
        unsafe { ffi::gtk_text_view_scroll_to_mark(GTK_TEXT_VIEW(self.get_widget()), mark.get_pointer(), within_margin,
            ffi::to_gboolean(use_align), xalign, yalign) }
    }

    pub fn scroll_to_iter(&self, iter: &gtk::TextIter, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_scroll_to_iter(GTK_TEXT_VIEW(self.get_widget()), iter.get_pointer(), within_margin,
            ffi::to_gboolean(use_align), xalign, yalign)) }
    }

    pub fn scroll_mark_onscreen(&self, mark: &gtk::TextMark) {
        unsafe { ffi::gtk_text_view_scroll_mark_onscreen(GTK_TEXT_VIEW(self.get_widget()), mark.get_pointer()) }
    }

    pub fn move_mark_onscreen(&self, mark: &gtk::TextMark) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_move_mark_onscreen(GTK_TEXT_VIEW(self.get_widget()), mark.get_pointer())) }
    }

    pub fn place_cursor_onscreen(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_place_cursor_onscreen(GTK_TEXT_VIEW(self.get_widget()))) }
    }

    pub fn get_line_at_y(&self, target_iter: &gtk::TextIter, y: i32, line_top: &mut i32) {
        unsafe { ffi::gtk_text_view_get_line_at_y(GTK_TEXT_VIEW(self.get_widget()), target_iter.get_pointer(), y as ::libc::c_int,
            line_top) }
    }

    pub fn get_iter_at_location(&self, target_iter: &gtk::TextIter, x: i32, y: i32) {
        unsafe { ffi::gtk_text_view_get_iter_at_location(GTK_TEXT_VIEW(self.get_widget()), target_iter.get_pointer(), x, y) }
    }

    pub fn buffer_to_window_coords(&self, win: gtk::TextWindowType, buffer_x: i32, buffer_y: i32, window_x: *mut i32, window_y: &mut i32) {
        unsafe { ffi::gtk_text_view_buffer_to_window_coords(GTK_TEXT_VIEW(self.get_widget()), win, buffer_x as ::libc::c_int,
            buffer_y as ::libc::c_int, window_x, window_y) }
    }

    pub fn window_to_buffer_coords(&self, win: gtk::TextWindowType, window_x: i32, window_y: i32, buffer_x: *mut i32, buffer_y: &mut i32) {
        unsafe { ffi::gtk_text_view_window_to_buffer_coords(GTK_TEXT_VIEW(self.get_widget()), win, window_x as ::libc::c_int,
            window_y as ::libc::c_int, buffer_x, buffer_y) }
    }

    pub fn set_border_window_size(&self, _type: gtk::TextWindowType, size: i32) {
        unsafe { ffi::gtk_text_view_set_border_window_size(GTK_TEXT_VIEW(self.get_widget()), _type, size as ::libc::c_int) }
    }

    pub fn get_border_window_size(&self, _type: gtk::TextWindowType) -> i32 {
        unsafe { ffi::gtk_text_view_get_border_window_size(GTK_TEXT_VIEW(self.get_widget()), _type) }
    }

    pub fn forward_display_line(&self, iter: &gtk::TextIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_forward_display_line(GTK_TEXT_VIEW(self.get_widget()), iter.get_pointer())) }
    }

    pub fn backward_display_line(&self, iter: &gtk::TextIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_forward_display_line(GTK_TEXT_VIEW(self.get_widget()), iter.get_pointer())) }
    }

    pub fn forward_display_line_end(&self, iter: &gtk::TextIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_forward_display_line_end(GTK_TEXT_VIEW(self.get_widget()), iter.get_pointer())) }
    }

    pub fn backward_display_line_start(&self, iter: &gtk::TextIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_backward_display_line_start(GTK_TEXT_VIEW(self.get_widget()), iter.get_pointer())) }
    }

    pub fn starts_display_line(&self, iter: &gtk::TextIter) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_starts_display_line(GTK_TEXT_VIEW(self.get_widget()),
            iter.get_pointer())) }
    }

    pub fn move_visually(&self, iter: &gtk::TextIter, count: i32) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_move_visually(GTK_TEXT_VIEW(self.get_widget()), iter.get_pointer(),
            count as ::libc::c_int)) }
    }

    pub fn add_child_at_anchor<T: gtk::WidgetTrait>(&self, child: &T, anchor: &gtk::TextChildAnchor) {
        unsafe { ffi::gtk_text_view_add_child_at_anchor(GTK_TEXT_VIEW(self.get_widget()), child.get_widget(), anchor.get_pointer()) }
    }

    pub fn add_child_in_window<T: gtk::WidgetTrait>(&self, child: &T, which_window: gtk::TextWindowType, xpos: i32, ypos: i32) {
        unsafe { ffi::gtk_text_view_add_child_in_window(GTK_TEXT_VIEW(self.get_widget()), child.get_widget(), which_window,
            xpos as ::libc::c_int, ypos as ::libc::c_int) }
    }

    pub fn move_child<T: gtk::WidgetTrait>(&self, child: &T, xpos: i32, ypos: i32) {
        unsafe { ffi::gtk_text_view_move_child(GTK_TEXT_VIEW(self.get_widget()), child.get_widget(), xpos as ::libc::c_int,
            ypos as ::libc::c_int) }
    }

    pub fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode) {
        unsafe { ffi::gtk_text_view_set_wrap_mode(GTK_TEXT_VIEW(self.get_widget()), wrap_mode) }
    }

    pub fn get_wrap_mode(&self) -> gtk::WrapMode {
        unsafe { ffi::gtk_text_view_get_wrap_mode(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_editable(&self, setting: bool) {
        unsafe { ffi::gtk_text_view_set_editable(GTK_TEXT_VIEW(self.get_widget()), ffi::to_gboolean(setting)) }
    }

    pub fn get_editable(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_get_editable(GTK_TEXT_VIEW(self.get_widget()))) }
    }

    pub fn set_cursor_visible(&self, setting: bool) {
        unsafe { ffi::gtk_text_view_set_cursor_visible(GTK_TEXT_VIEW(self.get_widget()), ffi::to_gboolean(setting)) }
    }

    pub fn get_cursor_visible(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_get_cursor_visible(GTK_TEXT_VIEW(self.get_widget()))) }
    }

    pub fn set_overwrite(&self, overwrite: bool) {
        unsafe { ffi::gtk_text_view_set_overwrite(GTK_TEXT_VIEW(self.get_widget()), ffi::to_gboolean(overwrite)) }
    }

    pub fn get_overwrite(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_get_overwrite(GTK_TEXT_VIEW(self.get_widget()))) }
    }

    pub fn set_pixels_above_lines(&self, pixels_above_lines: i32) {
        unsafe { ffi::gtk_text_view_set_pixels_above_lines(GTK_TEXT_VIEW(self.get_widget()), pixels_above_lines as ::libc::c_int) }
    }

    pub fn get_pixels_above_lines(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_pixels_above_lines(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_pixels_below_lines(&self, pixels_below_lines: i32) {
        unsafe { ffi::gtk_text_view_set_pixels_below_lines(GTK_TEXT_VIEW(self.get_widget()), pixels_below_lines as ::libc::c_int) }
    }

    pub fn get_pixels_below_lines(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_pixels_below_lines(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_pixels_inside_wrap(&self, pixels_inside_wrap: i32) {
        unsafe { ffi::gtk_text_view_set_pixels_inside_wrap(GTK_TEXT_VIEW(self.get_widget()), pixels_inside_wrap as ::libc::c_int) }
    }

    pub fn get_pixels_inside_wrap(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_pixels_inside_wrap(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_justification(&self, justification: gtk::Justification) {
        unsafe { ffi::gtk_text_view_set_justification(GTK_TEXT_VIEW(self.get_widget()), justification) }
    }

    pub fn get_justification(&self) -> gtk::Justification {
        unsafe { ffi::gtk_text_view_get_justification(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_left_margin(&self, left_margin: i32) {
        unsafe { ffi::gtk_text_view_set_left_margin(GTK_TEXT_VIEW(self.get_widget()), left_margin as ::libc::c_int) }
    }

    pub fn get_left_margin(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_left_margin(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_right_margin(&self, right_margin: i32) {
        unsafe { ffi::gtk_text_view_set_right_margin(GTK_TEXT_VIEW(self.get_widget()), right_margin as ::libc::c_int) }
    }

    pub fn get_right_margin(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_right_margin(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_indent(&self, indent: i32) {
        unsafe { ffi::gtk_text_view_set_indent(GTK_TEXT_VIEW(self.get_widget()), indent as ::libc::c_int) }
    }

    pub fn get_indent(&self) -> i32 {
        unsafe { ffi::gtk_text_view_get_indent(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_accepts_tab(&self, accepts_tab: bool) {
        unsafe { ffi::gtk_text_view_set_accepts_tab(GTK_TEXT_VIEW(self.get_widget()), ffi::to_gboolean(accepts_tab)) }
    }

    pub fn get_accepts_tab(&self) -> bool {
        unsafe { ffi::to_bool(ffi::gtk_text_view_get_accepts_tab(GTK_TEXT_VIEW(self.get_widget()))) }
    }

    pub fn get_default_attributes(&self) -> Option<gtk::TextAttributes> {
        let tmp_pointer = unsafe { ffi::gtk_text_view_get_default_attributes(GTK_TEXT_VIEW(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::TextAttributes::wrap_pointer(tmp_pointer))
        }
    }

    pub fn reset_im_context(&self) {
        unsafe { ffi::gtk_text_view_reset_im_context(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_input_purpose(&self, purpose: gtk::InputPurpose) {
        unsafe { ffi::gtk_text_view_set_input_purpose(GTK_TEXT_VIEW(self.get_widget()), purpose) }
    }

    pub fn get_input_purpose(&self) -> gtk::InputPurpose {
        unsafe { ffi::gtk_text_view_get_input_purpose(GTK_TEXT_VIEW(self.get_widget())) }
    }

    pub fn set_input_hints(&self, hints: gtk::InputHints) {
        unsafe { ffi::gtk_text_view_set_input_hints(GTK_TEXT_VIEW(self.get_widget()), hints) }
    }

    pub fn get_input_hints(&self) -> gtk::InputHints {
        unsafe { ffi::gtk_text_view_get_input_hints(GTK_TEXT_VIEW(self.get_widget())) }
    }
}

impl_drop!(TextView);
impl_TraitWidget!(TextView);

impl gtk::ScrollableTrait for TextView {}

impl_widget_events!(TextView);
