// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_int;
use pango;
use glib::translate::*;
use ffi;
use glib::{to_bool, to_gboolean, Type};
use gdk;
use gdk_ffi;
use glib;

pub trait WidgetTrait: ::FFIWidget + ::GObjectTrait {
    fn show_all(&self) -> () {
        unsafe {
            ffi::gtk_widget_show_all(self.unwrap_widget());
        }
    }

    fn show_now(&self) {
        unsafe { ffi::gtk_widget_show_now(self.unwrap_widget()) }
    }

    fn hide(&self) {
        unsafe { ffi::gtk_widget_hide(self.unwrap_widget()) }
    }

    fn map(&self) {
        unsafe { ffi::gtk_widget_map(self.unwrap_widget()) }
    }

    fn unmap(&self) {
        unsafe { ffi::gtk_widget_unmap(self.unwrap_widget()) }
    }

    fn realize(&self) {
        unsafe { ffi::gtk_widget_realize(self.unwrap_widget()) }
    }

    fn unrealize(&self) {
        unsafe { ffi::gtk_widget_unrealize(self.unwrap_widget()) }
    }

    fn queue_draw(&self) {
        unsafe { ffi::gtk_widget_queue_draw(self.unwrap_widget()) }
    }

    fn queue_resize(&self) {
        unsafe { ffi::gtk_widget_queue_resize(self.unwrap_widget()) }
    }

    fn queue_resize_no_redraw(&self) {
        unsafe { ffi::gtk_widget_queue_resize_no_redraw(self.unwrap_widget()) }
    }

    #[cfg(gtk_3_10)]
    fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_scale_factor(self.unwrap_widget()) }
    }

    fn activate(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_activate(self.unwrap_widget())) }
    }

    fn reparent<T: WidgetTrait>(&self, new_parent: &T) {
        unsafe { ffi::gtk_widget_reparent(self.unwrap_widget(), new_parent.unwrap_widget()) }
    }

    fn is_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_focus(self.unwrap_widget())) }
    }

    fn grab_focus(&self) {
        unsafe { ffi::gtk_widget_grab_focus(self.unwrap_widget()) }
    }

    fn grab_default(&self) {
        unsafe { ffi::gtk_widget_grab_default(self.unwrap_widget()) }
    }

    fn set_name(&self, name: &str) {
        unsafe { ffi::gtk_widget_set_name(self.unwrap_widget(), name.to_glib_none().0) }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_widget_get_name(self.unwrap_widget()))
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe { ffi::gtk_widget_set_sensitive(self.unwrap_widget(), to_gboolean(sensitive)) }
    }

    fn set_parent<T: WidgetTrait>(&self, parent: &T) {
        unsafe { ffi::gtk_widget_set_parent(self.unwrap_widget(), parent.unwrap_widget()) }
    }

    /*fn set_parent_window(&self, parent: &Widget) {
        unsafe { gtk_widget_set_parent_window(self.unwrap_widget(), parent.unwrap_widget()) }
    }*/

    fn get_toplevel(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_toplevel(self.unwrap_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp))
        }
    }

    fn get_ancestor(&self, widget_type: Type) -> Option<Self> {
        let tmp = unsafe {
            ffi::gtk_widget_get_ancestor(self.unwrap_widget(), widget_type.to_glib())
        };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp))
        }
    }

    fn is_ancestor<T: WidgetTrait>(&self, ancestor: &T) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_ancestor(self.unwrap_widget(), ancestor.unwrap_widget())) }
    }

    fn hide_on_delete(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_hide_on_delete(self.unwrap_widget())) }
    }

    fn set_direction(&self, dir: ::TextDirection) {
        unsafe { ffi::gtk_widget_set_direction(self.unwrap_widget(), dir) }
    }

    fn get_direction(&self) -> ::TextDirection {
        unsafe { ffi::gtk_widget_get_direction(self.unwrap_widget()) }
    }

    fn set_default_direction(dir: ::TextDirection) {
        assert_initialized_main_thread!();
        unsafe { ffi::gtk_widget_set_default_direction(dir) }
    }

    fn get_default_direction() -> ::TextDirection {
        assert_initialized_main_thread!();
        unsafe { ffi::gtk_widget_get_default_direction() }
    }

    fn in_destruction(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_in_destruction(self.unwrap_widget())) }
    }

    fn unparent(&self) {
        unsafe { ffi::gtk_widget_unparent(self.unwrap_widget()) }
    }

    fn translate_coordinates<T: WidgetTrait>(&self, dest_widget: &T, src_x: i32, src_y: i32) -> Option<(i32, i32)> {
        let mut dest_x = 0i32;
        let mut dest_y = 0i32;

        let r = to_bool(unsafe { ffi::gtk_widget_translate_coordinates(self.unwrap_widget(), dest_widget.unwrap_widget(), src_x, src_y, &mut dest_x, &mut dest_y) });
        if r {
            Some((dest_x, dest_y))
        }
        else {
            None
        }
    }

    fn override_background_color(&self, state: ::StateFlags, color: &gdk_ffi::GdkRGBA) {
        unsafe { ffi::gtk_widget_override_background_color(self.unwrap_widget(), state, color) }
    }

    fn override_color(&self, state: ::StateFlags, color: &gdk_ffi::GdkRGBA) {
        unsafe { ffi::gtk_widget_override_color(self.unwrap_widget(), state, color) }
    }

    fn override_symbolic_color(&self, name: &str, color: &gdk_ffi::GdkRGBA) {
        unsafe { ffi::gtk_widget_override_symbolic_color(self.unwrap_widget(), name.to_glib_none().0, color); }
    }

    fn override_cursor(&self, cursor: &gdk_ffi::GdkRGBA, secondary_cursor: &gdk_ffi::GdkRGBA) {
        unsafe { ffi::gtk_widget_override_cursor(self.unwrap_widget(), cursor, secondary_cursor) }
    }

    fn override_font(&self, font: &pango::FontDescription) {
        unsafe { ffi::gtk_widget_override_font(self.unwrap_widget(), font.to_glib_none().0) }
    }

    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_queue_draw_area(self.unwrap_widget(), x, y, width, height) }
    }

    fn set_app_paintable(&self, app_paintable: bool) {
        unsafe { ffi::gtk_widget_set_app_paintable(self.unwrap_widget(), to_gboolean(app_paintable)) }
    }

    fn set_double_buffered(&self, double_buffered: bool) {
        unsafe { ffi::gtk_widget_set_double_buffered(self.unwrap_widget(), to_gboolean(double_buffered)) }
    }

    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool) {
        unsafe { ffi::gtk_widget_set_redraw_on_allocate(self.unwrap_widget(), to_gboolean(redraw_on_allocate)) }
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        unsafe { to_bool(ffi::gtk_widget_mnemonic_activate(self.unwrap_widget(), to_gboolean(group_cycling))) }
    }

    /*fn send_expose(&self, event: &mut gdk::Event) -> i32 {
        unsafe { ffi::gtk_widget_send_expose(self.unwrap_widget(), event) }
    }

    fn send_focus_change(&self, event: &mut gdk::Event) -> bool {
        unsafe { to_bool(ffi::gtk_widget_send_expose(self.unwrap_widget(), event)) }
    }*/

    fn child_focus(&self, direction: ::DirectionType) -> bool {
        unsafe { to_bool(ffi::gtk_widget_child_focus(self.unwrap_widget(), direction)) }
    }

    fn get_child_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_child_visible(self.unwrap_widget())) }
    }

    fn get_parent(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_parent(self.unwrap_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp))
        }
    }

    fn has_screen(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_screen(self.unwrap_widget())) }
    }

    fn get_screen(&self) -> gdk::Screen {
        unsafe { from_glib_none(ffi::gtk_widget_get_screen(self.unwrap_widget())) }
    }

    fn get_size_request(&self) -> (i32, i32) {
        let mut width = 0i32;
        let mut height = 0i32;

        unsafe { ffi::gtk_widget_get_size_request(self.unwrap_widget(), &mut width, &mut height) };
        (width, height)
    }

    fn set_child_visible(&self, is_visible: bool) {
        unsafe { ffi::gtk_widget_set_child_visible(self.unwrap_widget(), to_gboolean(is_visible)) }
    }

    fn set_size_request(&self, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_set_size_request(self.unwrap_widget(), width, height) }
    }

    fn set_no_show_all(&self, no_show_all: bool) {
        unsafe { ffi::gtk_widget_set_no_show_all(self.unwrap_widget(), to_gboolean(no_show_all)) }
    }

    fn get_no_show_all(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_no_show_all(self.unwrap_widget())) }
    }

    fn list_mnemonic_labels(&self) -> glib::List<Box<Self>> {
        let tmp = unsafe { ffi::gtk_widget_list_mnemonic_labels(self.unwrap_widget()) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::GtkWidget> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<Self>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(::FFIWidget::wrap_widget(*it)));
            }
            tmp_vec
        }
    }

    fn add_mnemonic_label<T: WidgetTrait>(&self, label: &T) {
        unsafe { ffi::gtk_widget_add_mnemonic_label(self.unwrap_widget(), label.unwrap_widget()) }
    }

    fn remove_mnemonic_label<T: WidgetTrait>(&self, label: &T) {
        unsafe { ffi::gtk_widget_remove_mnemonic_label(self.unwrap_widget(), label.unwrap_widget()) }
    }

    fn is_composited(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_composited(self.unwrap_widget())) }
    }

    fn error_bell(&self) {
        unsafe { ffi::gtk_widget_error_bell(self.unwrap_widget()) }
    }

    fn keynav_failed(&self, direction: ::DirectionType) -> bool {
        unsafe { to_bool(ffi::gtk_widget_keynav_failed(self.unwrap_widget(), direction)) }
    }

    fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_widget_get_tooltip_markup(self.unwrap_widget()))
        }
    }

    fn set_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_widget_set_tooltip_markup(
                self.unwrap_widget(),
                markup.to_glib_none().0);
        }
    }


    fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_widget_get_tooltip_text(self.unwrap_widget()))
        }
    }

    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_widget_set_tooltip_text(
                self.unwrap_widget(),
                text.to_glib_none().0);
        }
    }

    fn get_has_tooltip(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_has_tooltip(self.unwrap_widget())) }
    }

    fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe { ffi::gtk_widget_set_has_tooltip(self.unwrap_widget(), to_gboolean(has_tooltip)) }
    }

    fn trigger_tooltip_query(&self) {
        unsafe { ffi::gtk_widget_trigger_tooltip_query(self.unwrap_widget()) }
    }

    #[cfg(gtk_3_10)]
    fn get_allocated_baseline(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_allocated_baseline(self.unwrap_widget()) }
    }

    fn get_app_paintable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_app_paintable(self.unwrap_widget())) }
    }

    fn get_can_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_can_default(self.unwrap_widget())) }
    }

    fn set_can_default(&self, can_default: bool) {
        unsafe { ffi::gtk_widget_set_can_default(self.unwrap_widget(), to_gboolean(can_default)) }
    }

    fn get_can_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_can_focus(self.unwrap_widget())) }
    }

    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::gtk_widget_set_can_focus(self.unwrap_widget(), to_gboolean(can_focus)) }
    }

    fn get_double_buffered(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_double_buffered(self.unwrap_widget())) }
    }

    fn get_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_widget_get_window(self.unwrap_widget()))
        }
    }

    fn get_has_window(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_has_window(self.unwrap_widget())) }
    }

    fn set_has_window(&self, has_window: bool) {
        unsafe { ffi::gtk_widget_set_has_window(self.unwrap_widget(), to_gboolean(has_window)) }
    }

    fn get_sensitive(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_sensitive(self.unwrap_widget())) }
    }

    fn is_sensitive(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_sensitive(self.unwrap_widget())) }
    }

    fn get_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_visible(self.unwrap_widget())) }
    }

    #[cfg(gtk_3_8)]
    fn is_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_visible(self.unwrap_widget())) }
    }

    fn set_visible(&self, visible: bool) {
        unsafe { ffi::gtk_widget_set_visible(self.unwrap_widget(), to_gboolean(visible)) }
    }

    fn set_state_flags(&self, flags: ::StateFlags, clear: bool) {
        unsafe { ffi::gtk_widget_set_state_flags(self.unwrap_widget(), flags, to_gboolean(clear)) }
    }

    fn unset_state_flags(&self, flags: ::StateFlags) {
        unsafe { ffi::gtk_widget_unset_state_flags(self.unwrap_widget(), flags) }
    }

    fn get_state_flags(&self) -> ::StateFlags {
        unsafe { ffi::gtk_widget_get_state_flags(self.unwrap_widget()) }
    }

    fn has_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_default(self.unwrap_widget())) }
    }

    fn has_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_focus(self.unwrap_widget())) }
    }

    fn has_visible_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_visible_focus(self.unwrap_widget())) }
    }

    fn has_grab(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_grab(self.unwrap_widget())) }
    }

    fn is_drawable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_drawable(self.unwrap_widget())) }
    }

    fn is_toplevel(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_toplevel(self.unwrap_widget())) }
    }

    fn set_receives_default(&self, receives_default: bool) {
        unsafe { ffi::gtk_widget_set_receives_default(self.unwrap_widget(), to_gboolean(receives_default)) }
    }

    fn get_receives_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_receives_default(self.unwrap_widget())) }
    }

    fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe { ffi::gtk_widget_set_support_multidevice(self.unwrap_widget(), to_gboolean(support_multidevice)) }
    }

    fn get_support_multidevice(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_support_multidevice(self.unwrap_widget())) }
    }

    fn set_realized(&self, realized: bool) {
        unsafe { ffi::gtk_widget_set_realized(self.unwrap_widget(), to_gboolean(realized)) }
    }

    fn get_realized(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_realized(self.unwrap_widget())) }
    }

    fn set_mapped(&self, mapped: bool) {
        unsafe { ffi::gtk_widget_set_mapped(self.unwrap_widget(), to_gboolean(mapped)) }
    }

    fn get_mapped(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_mapped(self.unwrap_widget())) }
    }

    fn get_modifier_mask(&self, intent: gdk::ModifierIntent) -> gdk::ModifierType {
        unsafe { ffi::gtk_widget_get_modifier_mask(self.unwrap_widget(), intent) }
    }

    #[cfg(gtk_3_8)]
    fn set_opacity(&self, opacity: f64) {
        unsafe { ffi::gtk_widget_set_opacity(self.unwrap_widget(), opacity) }
    }

    #[cfg(gtk_3_8)]
    fn get_opacity(&self) -> f64 {
        unsafe { ffi::gtk_widget_get_opacity(self.unwrap_widget()) }
    }

    fn set_margin_top(&self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_top(self.unwrap_widget(), margin as c_int)
        }
    }

    fn set_margin_bottom(&self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_bottom(self.unwrap_widget(), margin as c_int)
        }
    }

    fn get_margin_top(&self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_top(self.unwrap_widget()) as i32
        }
    }

    fn get_margin_bottom(&self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_bottom(self.unwrap_widget()) as i32
        }
    }

    fn get_allocated_width(&self) -> i32 {
        unsafe{
            ffi::gtk_widget_get_allocated_width(self.unwrap_widget()) as i32
        }
    }

    fn get_allocated_height(&self) -> i32 {
        unsafe{
            ffi::gtk_widget_get_allocated_height(self.unwrap_widget()) as i32
        }
    }

    fn reset_style(&self) {
        unsafe { ffi::gtk_widget_reset_style(self.unwrap_widget()) }
    }

    fn get_preferred_height(&self) -> (i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_height(self.unwrap_widget(), &mut minimum_height, &mut natural_height); }
        (minimum_height, natural_height)
    }

    fn get_preferred_width(&self) -> (i32, i32) {
        let mut minimum_width = 0i32;
        let mut natural_width = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_width(self.unwrap_widget(), &mut minimum_width, &mut natural_width); }
        (minimum_width, natural_width)
    }

    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_height_for_width(self.unwrap_widget(), width, &mut minimum_height, &mut natural_height) };
        (minimum_height, natural_height)
    }

    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        let mut minimum_width = 0i32;
        let mut natural_width = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_width_for_height(self.unwrap_widget(), height, &mut minimum_width, &mut natural_width) };
        (minimum_width, natural_width)
    }

    #[cfg(gtk_3_10)]
    fn get_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;
        let mut minimum_baseline = 0i32;
        let mut natural_baseline = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_height_and_baseline_for_width(self.unwrap_widget(), width, &mut minimum_height,
            &mut natural_height, &mut minimum_baseline, &mut natural_baseline) };

        (minimum_height, natural_height, minimum_baseline, natural_baseline)
    }

    fn get_request_mode(&self) -> ::SizeRequestMode {
        unsafe { ffi::gtk_widget_get_request_mode(self.unwrap_widget()) }
    }

    fn get_halign(&self) -> ::Align {
        unsafe { ffi::gtk_widget_get_halign(self.unwrap_widget()) }
    }

    fn set_halign(&self, align: ::Align) {
        unsafe { ffi::gtk_widget_set_halign(self.unwrap_widget(), align) }
    }

    fn get_valign(&self) -> ::Align {
        unsafe { ffi::gtk_widget_get_valign(self.unwrap_widget()) }
    }

    #[cfg(gtk_3_10)]
    fn get_valign_with_baseline(&self) -> ::Align {
        unsafe { ffi::gtk_widget_get_valign_with_baseline(self.unwrap_widget()) }
    }

    fn set_valign(&self, align: ::Align) {
        unsafe { ffi::gtk_widget_set_valign(self.unwrap_widget(), align) }
    }

    #[cfg(gtk_3_12)]
    fn get_margin_start(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_margin_start(self.unwrap_widget()) }
    }

    #[cfg(gtk_3_12)]
    fn set_margin_start(&self, margin: i32) {
        unsafe { ffi::gtk_widget_set_margin_start(self.unwrap_widget(), margin) }
    }

    #[cfg(gtk_3_12)]
    fn get_margin_end(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_margin_end(self.unwrap_widget()) }
    }

    #[cfg(gtk_3_12)]
    fn set_margin_end(&self, margin: i32) {
        unsafe { ffi::gtk_widget_set_margin_end(self.unwrap_widget(), margin) }
    }

    fn get_hexpand(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_hexpand(self.unwrap_widget())) }
    }

    fn set_hexpand(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_hexpand(self.unwrap_widget(), to_gboolean(expand)) }
    }

    fn get_hexpand_set(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_hexpand_set(self.unwrap_widget())) }
    }

    fn set_hexpand_set(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_hexpand_set(self.unwrap_widget(), to_gboolean(expand)) }
    }

    fn get_vexpand(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_vexpand(self.unwrap_widget())) }
    }

    fn set_vexpand(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_vexpand(self.unwrap_widget(), to_gboolean(expand)) }
    }

    fn get_vexpand_set(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_vexpand_set(self.unwrap_widget())) }
    }

    fn set_vexpand_set(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_vexpand_set(self.unwrap_widget(), to_gboolean(expand)) }
    }

    fn get_style_context(&self) -> ::StyleContext {
        unsafe {
            ::StyleContext { pointer: ffi::gtk_widget_get_style_context(self.unwrap_widget()) }
        }
    }

    fn queue_compute_expand(&self) {
        unsafe { ffi::gtk_widget_queue_compute_expand(self.unwrap_widget()) }
    }

    fn compute_expand(&self, orientation: ::Orientation) -> bool {
        unsafe { to_bool(ffi::gtk_widget_compute_expand(self.unwrap_widget(), orientation)) }
    }

    #[cfg(gtk_3_10)]
    fn init_template(&self) {
        unsafe { ffi::gtk_widget_init_template(self.unwrap_widget()) }
    }

    fn thaw_child_notify(&self) {
        unsafe { ffi::gtk_widget_thaw_child_notify(self.unwrap_widget()) }
    }

    fn freeze_child_notify(&self) {
        unsafe { ffi::gtk_widget_freeze_child_notify(self.unwrap_widget()) }
    }

    fn child_notify(&self, child_property: &str) {
        unsafe {
            ffi::gtk_widget_child_notify(self.unwrap_widget(), child_property.to_glib_none().0)
        }
    }

    fn destroy(&self) {
        unsafe { ffi::gtk_widget_destroy(self.unwrap_widget()) }
    }

    fn destroyed(&self, other: &Self) {
        let mut tmp = other.unwrap_widget();

        unsafe { ffi::gtk_widget_destroyed(self.unwrap_widget(), &mut tmp) }
    }
}
