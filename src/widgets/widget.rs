// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types::{self, Type};
use gdk;
use gdk_ffi;
use pango;
use ffi;

use object::{Object, Upcast};
use {
    Align,
    DirectionType,
    Orientation,
    SizeRequestMode,
    StateFlags,
    TextDirection,
};

pub type Widget = Object<ffi::GtkWidget>;

impl types::StaticType for Widget {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_widget_get_type()) }
    }
}

unsafe impl Upcast<::Buildable> for Widget { }

pub trait WidgetExt {
    fn show_all(&self);
    fn show_now(&self);
    fn hide(&self);
    fn map(&self);
    fn unmap(&self);
    fn realize(&self);
    fn unrealize(&self);
    fn queue_draw(&self);
    fn queue_resize(&self);
    fn queue_resize_no_redraw(&self);
    #[cfg(gtk_3_10)]
    fn get_scale_factor(&self) -> i32;
    fn activate(&self) -> bool;
    fn reparent<T: Upcast<Widget>>(&self, new_parent: &T);
    fn is_focus(&self) -> bool;
    fn grab_focus(&self);
    fn grab_default(&self);
    fn set_name(&self, name: &str);
    fn get_name(&self) -> Option<String>;
    fn set_sensitive(&self, sensitive: bool);
    fn set_parent<T: Upcast<Widget>>(&self, parent: &T);
    fn set_parent_window(&self, parent: &gdk::window::Window);
    fn get_toplevel(&self) -> Widget;
    fn get_ancestor(&self, widget_type: Type) -> Option<Widget>;
    fn is_ancestor<T: Upcast<Widget>>(&self, ancestor: &T) -> bool;
    fn hide_on_delete(&self) -> bool;
    fn set_direction(&self, dir: TextDirection);
    fn get_direction(&self) -> TextDirection;
    fn set_default_direction(dir: TextDirection);
    fn get_default_direction() -> TextDirection;
    fn in_destruction(&self) -> bool;
    fn unparent(&self);
    fn translate_coordinates<T: Upcast<Widget>>(&self, dest_widget: &T, src_x: i32, src_y: i32)
        -> Option<(i32, i32)>;
    fn override_background_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA);
    fn override_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA);
    fn override_symbolic_color(&self, name: &str, color: &gdk_ffi::GdkRGBA);
    fn override_cursor(&self, cursor: &gdk_ffi::GdkRGBA, secondary_cursor: &gdk_ffi::GdkRGBA);
    fn override_font(&self, font: &pango::FontDescription);
    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32);
    fn set_app_paintable(&self, app_paintable: bool);
    fn set_double_buffered(&self, double_buffered: bool);
    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool);
    fn mnemonic_activate(&self, group_cycling: bool) -> bool;
    fn child_focus(&self, direction: DirectionType) -> bool;
    fn get_child_visible(&self) -> bool;
    fn get_parent(&self) -> Option<Widget>;
    fn has_screen(&self) -> bool;
    fn get_screen(&self) -> gdk::Screen;
    fn get_size_request(&self) -> (i32, i32);
    fn set_child_visible(&self, is_visible: bool);
    fn set_size_request(&self, width: i32, height: i32);
    fn set_no_show_all(&self, no_show_all: bool);
    fn get_no_show_all(&self) -> bool;
    fn list_mnemonic_labels(&self) -> Vec<Widget>;
    fn add_mnemonic_label<T: Upcast<Widget>>(&self, label: &T);
    fn remove_mnemonic_label<T: Upcast<Widget>>(&self, label: &T);
    fn is_composited(&self) -> bool;
    fn error_bell(&self);
    fn keynav_failed(&self, direction: DirectionType) -> bool;
    fn get_tooltip_markup(&self) -> Option<String>;
    fn set_tooltip_markup(&self, markup: &str);
    fn get_tooltip_text(&self) -> Option<String>;
    fn set_tooltip_text(&self, text: &str);
    fn get_has_tooltip(&self) -> bool;
    fn set_has_tooltip(&self, has_tooltip: bool);
    fn trigger_tooltip_query(&self);
    #[cfg(gtk_3_10)]
    fn get_allocated_baseline(&self) -> i32;
    fn get_app_paintable(&self) -> bool;
    fn get_can_default(&self) -> bool;
    fn set_can_default(&self, can_default: bool);
    fn get_can_focus(&self) -> bool;
    fn set_can_focus(&self, can_focus: bool);
    fn get_double_buffered(&self) -> bool;
    fn get_window(&self) -> Option<gdk::Window>;
    fn get_has_window(&self) -> bool;
    fn set_has_window(&self, has_window: bool);
    fn get_sensitive(&self) -> bool;
    fn is_sensitive(&self) -> bool;
    fn get_visible(&self) -> bool;
    #[cfg(gtk_3_8)]
    fn is_visible(&self) -> bool;
    fn set_visible(&self, visible: bool);
    fn set_state_flags(&self, flags: StateFlags, clear: bool);
    fn unset_state_flags(&self, flags: StateFlags);
    fn get_state_flags(&self) -> StateFlags;
    fn has_default(&self) -> bool;
    fn has_focus(&self) -> bool;
    fn has_visible_focus(&self) -> bool;
    fn has_grab(&self) -> bool;
    fn is_drawable(&self) -> bool;
    fn is_toplevel(&self) -> bool;
    fn set_receives_default(&self, receives_default: bool);
    fn get_receives_default(&self) -> bool;
    fn set_support_multidevice(&self, support_multidevice: bool);
    fn get_support_multidevice(&self) -> bool;
    fn set_realized(&self, realized: bool);
    fn get_realized(&self) -> bool;
    fn set_mapped(&self, mapped: bool);
    fn get_mapped(&self) -> bool;
    fn get_modifier_mask(&self, intent: gdk::ModifierIntent) -> gdk::ModifierType;
    #[cfg(gtk_3_8)]
    fn set_opacity(&self, opacity: f64);
    #[cfg(gtk_3_8)]
    fn get_opacity(&self) -> f64;
    fn set_margin_top(&self, margin: i32) -> ();
    fn set_margin_bottom(&self, margin: i32) -> ();
    fn get_margin_top(&self) -> i32;
    fn get_margin_bottom(&self) -> i32;
    fn get_allocated_width(&self) -> i32;
    fn get_allocated_height(&self) -> i32;
    fn reset_style(&self);
    fn get_preferred_height(&self) -> (i32, i32);
    fn get_preferred_width(&self) -> (i32, i32);
    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32);
    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32);
    #[cfg(gtk_3_10)]
    fn get_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32);
    fn get_request_mode(&self) -> SizeRequestMode;
    fn get_halign(&self) -> Align;
    fn set_halign(&self, align: Align);
    fn get_valign(&self) -> Align;
    #[cfg(gtk_3_10)]
    fn get_valign_with_baseline(&self) -> Align;
    fn set_valign(&self, align: Align);
    #[cfg(gtk_3_12)]
    fn get_margin_start(&self) -> i32;
    #[cfg(gtk_3_12)]
    fn set_margin_start(&self, margin: i32);
    #[cfg(gtk_3_12)]
    fn get_margin_end(&self) -> i32;
    #[cfg(gtk_3_12)]
    fn set_margin_end(&self, margin: i32);
    fn get_hexpand(&self) -> bool;
    fn set_hexpand(&self, expand: bool);
    fn get_hexpand_set(&self) -> bool;
    fn set_hexpand_set(&self, expand: bool);
    fn get_vexpand(&self) -> bool;
    fn set_vexpand(&self, expand: bool);
    fn get_vexpand_set(&self) -> bool;
    fn set_vexpand_set(&self, expand: bool);
    fn queue_compute_expand(&self);
    fn compute_expand(&self, orientation: Orientation) -> bool;
    #[cfg(gtk_3_10)]
    fn init_template(&self);
    fn thaw_child_notify(&self);
    fn freeze_child_notify(&self);
    fn child_notify(&self, child_property: &str);
    fn destroy(&self);
}

impl<O: Upcast<Widget>> WidgetExt for O {
    fn show_all(&self) {
        unsafe { ffi::gtk_widget_show_all(self.upcast().to_glib_none().0) }
    }

    fn show_now(&self) {
        unsafe { ffi::gtk_widget_show_now(self.upcast().to_glib_none().0) }
    }

    fn hide(&self) {
        unsafe { ffi::gtk_widget_hide(self.upcast().to_glib_none().0) }
    }

    fn map(&self) {
        unsafe { ffi::gtk_widget_map(self.upcast().to_glib_none().0) }
    }

    fn unmap(&self) {
        unsafe { ffi::gtk_widget_unmap(self.upcast().to_glib_none().0) }
    }

    fn realize(&self) {
        unsafe { ffi::gtk_widget_realize(self.upcast().to_glib_none().0) }
    }

    fn unrealize(&self) {
        unsafe { ffi::gtk_widget_unrealize(self.upcast().to_glib_none().0) }
    }

    fn queue_draw(&self) {
        unsafe { ffi::gtk_widget_queue_draw(self.upcast().to_glib_none().0) }
    }

    fn queue_resize(&self) {
        unsafe { ffi::gtk_widget_queue_resize(self.upcast().to_glib_none().0) }
    }

    fn queue_resize_no_redraw(&self) {
        unsafe { ffi::gtk_widget_queue_resize_no_redraw(self.upcast().to_glib_none().0) }
    }

    #[cfg(gtk_3_10)]
    fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_scale_factor(self.upcast().to_glib_none().0) }
    }

    fn activate(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_activate(self.upcast().to_glib_none().0)) }
    }

    fn reparent<T: Upcast<Widget>>(&self, new_parent: &T) {
        unsafe {
            ffi::gtk_widget_reparent(self.upcast().to_glib_none().0,
                new_parent.upcast().to_glib_none().0)
        }
    }

    fn is_focus(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_is_focus(self.upcast().to_glib_none().0)) }
    }

    fn grab_focus(&self) {
        unsafe { ffi::gtk_widget_grab_focus(self.upcast().to_glib_none().0) }
    }

    fn grab_default(&self) {
        unsafe { ffi::gtk_widget_grab_default(self.upcast().to_glib_none().0) }
    }

    fn set_name(&self, name: &str) {
        unsafe { ffi::gtk_widget_set_name(self.upcast().to_glib_none().0, name.to_glib_none().0) }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_widget_get_name(self.upcast().to_glib_none().0))
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::gtk_widget_set_sensitive(self.upcast().to_glib_none().0, sensitive.to_glib())
        }
    }

    fn set_parent<T: Upcast<Widget>>(&self, parent: &T) {
        unsafe {
            ffi::gtk_widget_set_parent(self.upcast().to_glib_none().0,
                parent.upcast().to_glib_none().0)
        }
    }

    fn set_parent_window(&self, parent: &gdk::window::Window) {
        unsafe {
            ffi::gtk_widget_set_parent_window(
                self.upcast().to_glib_none().0, parent.to_glib_none().0)
        }
    }

    fn get_toplevel(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_widget_get_toplevel(self.upcast().to_glib_none().0))
        }
    }

    fn get_ancestor(&self, widget_type: Type) -> Option<Widget> {
        unsafe {
            from_glib_none(
                ffi::gtk_widget_get_ancestor(
                    self.upcast().to_glib_none().0, widget_type.to_glib()))
        }
    }

    fn is_ancestor<T: Upcast<Widget>>(&self, ancestor: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_is_ancestor(self.upcast().to_glib_none().0,
                ancestor.upcast().to_glib_none().0))
        }
    }

    fn hide_on_delete(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_hide_on_delete(self.upcast().to_glib_none().0)) }
    }

    fn set_direction(&self, dir: TextDirection) {
        unsafe { ffi::gtk_widget_set_direction(self.upcast().to_glib_none().0, dir) }
    }

    fn get_direction(&self) -> TextDirection {
        unsafe { ffi::gtk_widget_get_direction(self.upcast().to_glib_none().0) }
    }

    fn set_default_direction(dir: TextDirection) {
        unsafe { ffi::gtk_widget_set_default_direction(dir) }
    }

    fn get_default_direction() -> TextDirection {
        unsafe { ffi::gtk_widget_get_default_direction() }
    }

    fn in_destruction(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_in_destruction(self.upcast().to_glib_none().0)) }
    }

    fn unparent(&self) {
        unsafe { ffi::gtk_widget_unparent(self.upcast().to_glib_none().0) }
    }

    fn translate_coordinates<T: Upcast<Widget>>(&self, dest_widget: &T, src_x: i32, src_y: i32)
            -> Option<(i32, i32)> {
        unsafe {
            let mut dest_x = 0;
            let mut dest_y = 0;
            if from_glib(
                ffi::gtk_widget_translate_coordinates(
                    self.upcast().to_glib_none().0, dest_widget.upcast().to_glib_none().0,
                    src_x, src_y, &mut dest_x, &mut dest_y)) {
                Some((dest_x, dest_y))
            }
            else {
                None
            }
        }
    }

    fn override_background_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::gtk_widget_override_background_color(self.upcast().to_glib_none().0, state, color)
        }
    }

    fn override_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA) {
        unsafe { ffi::gtk_widget_override_color(self.upcast().to_glib_none().0, state, color) }
    }

    fn override_symbolic_color(&self, name: &str, color: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::gtk_widget_override_symbolic_color(self.upcast().to_glib_none().0,
                name.to_glib_none().0, color);
        }
    }

    fn override_cursor(&self, cursor: &gdk_ffi::GdkRGBA, secondary_cursor: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::gtk_widget_override_cursor(self.upcast().to_glib_none().0, cursor,
                secondary_cursor)
        }
    }

    fn override_font(&self, font: &pango::FontDescription) {
        unsafe {
            ffi::gtk_widget_override_font(self.upcast().to_glib_none().0, font.to_glib_none().0)
        }
    }

    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::gtk_widget_queue_draw_area(self.upcast().to_glib_none().0, x, y, width, height)
        }
    }

    fn set_app_paintable(&self, app_paintable: bool) {
        unsafe {
            ffi::gtk_widget_set_app_paintable(
                self.upcast().to_glib_none().0, app_paintable.to_glib())
        }
    }

    fn set_double_buffered(&self, double_buffered: bool) {
        unsafe {
            ffi::gtk_widget_set_double_buffered(
                self.upcast().to_glib_none().0, double_buffered.to_glib())
        }
    }

    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool) {
        unsafe {
            ffi::gtk_widget_set_redraw_on_allocate(
                self.upcast().to_glib_none().0, redraw_on_allocate.to_glib())
        }
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_widget_mnemonic_activate(
                    self.upcast().to_glib_none().0, group_cycling.to_glib()))
        }
    }

    /*fn send_expose(&self, event: &mut gdk::Event) -> i32 {
        unsafe { ffi::gtk_widget_send_expose(self.upcast().to_glib_none().0, event) }
    }

    fn send_focus_change(&self, event: &mut gdk::Event) -> bool {
        unsafe { from_glib(ffi::gtk_widget_send_expose(self.upcast().to_glib_none().0, event)) }
    }*/

    fn child_focus(&self, direction: DirectionType) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_child_focus(self.upcast().to_glib_none().0, direction))
        }
    }

    fn get_child_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_child_visible(self.upcast().to_glib_none().0)) }
    }

    fn get_parent(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_widget_get_parent(self.upcast().to_glib_none().0))
        }
    }

    fn has_screen(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_has_screen(self.upcast().to_glib_none().0)) }
    }

    fn get_screen(&self) -> gdk::Screen {
        unsafe { from_glib_none(ffi::gtk_widget_get_screen(self.upcast().to_glib_none().0)) }
    }

    fn get_size_request(&self) -> (i32, i32) {
        let mut width = 0i32;
        let mut height = 0i32;

        unsafe {
            ffi::gtk_widget_get_size_request(
                self.upcast().to_glib_none().0, &mut width, &mut height);
        }
        (width, height)
    }

    fn set_child_visible(&self, is_visible: bool) {
        unsafe {
            ffi::gtk_widget_set_child_visible(self.upcast().to_glib_none().0, is_visible.to_glib())
        }
    }

    fn set_size_request(&self, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_set_size_request(self.upcast().to_glib_none().0, width, height) }
    }

    fn set_no_show_all(&self, no_show_all: bool) {
        unsafe {
            ffi::gtk_widget_set_no_show_all(self.upcast().to_glib_none().0, no_show_all.to_glib())
        }
    }

    fn get_no_show_all(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_no_show_all(self.upcast().to_glib_none().0)) }
    }

    fn list_mnemonic_labels(&self) -> Vec<Widget> {
        unsafe {
            Vec::from_glib_container(
                ffi::gtk_widget_list_mnemonic_labels(self.upcast().to_glib_none().0))
        }
    }

    fn add_mnemonic_label<T: Upcast<Widget>>(&self, label: &T) {
        unsafe {
            ffi::gtk_widget_add_mnemonic_label(
                self.upcast().to_glib_none().0, label.upcast().to_glib_none().0)
        }
    }

    fn remove_mnemonic_label<T: Upcast<Widget>>(&self, label: &T) {
        unsafe {
            ffi::gtk_widget_remove_mnemonic_label(
                self.upcast().to_glib_none().0, label.upcast().to_glib_none().0)
        }
    }

    fn is_composited(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_is_composited(self.upcast().to_glib_none().0)) }
    }

    fn error_bell(&self) {
        unsafe { ffi::gtk_widget_error_bell(self.upcast().to_glib_none().0) }
    }

    fn keynav_failed(&self, direction: DirectionType) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_keynav_failed(self.upcast().to_glib_none().0, direction))
        }
    }

    fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_widget_get_tooltip_markup(self.upcast().to_glib_none().0))
        }
    }

    fn set_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_widget_set_tooltip_markup(
                self.upcast().to_glib_none().0,
                markup.to_glib_none().0);
        }
    }


    fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_widget_get_tooltip_text(self.upcast().to_glib_none().0))
        }
    }

    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_widget_set_tooltip_text(
                self.upcast().to_glib_none().0,
                text.to_glib_none().0);
        }
    }

    fn get_has_tooltip(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_has_tooltip(self.upcast().to_glib_none().0)) }
    }

    fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe {
            ffi::gtk_widget_set_has_tooltip(self.upcast().to_glib_none().0, has_tooltip.to_glib())
        }
    }

    fn trigger_tooltip_query(&self) {
        unsafe { ffi::gtk_widget_trigger_tooltip_query(self.upcast().to_glib_none().0) }
    }

    #[cfg(gtk_3_10)]
    fn get_allocated_baseline(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_allocated_baseline(self.upcast().to_glib_none().0) }
    }

    fn get_app_paintable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_app_paintable(self.upcast().to_glib_none().0)) }
    }

    fn get_can_default(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_can_default(self.upcast().to_glib_none().0)) }
    }

    fn set_can_default(&self, can_default: bool) {
        unsafe {
            ffi::gtk_widget_set_can_default(self.upcast().to_glib_none().0, can_default.to_glib())
        }
    }

    fn get_can_focus(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_can_focus(self.upcast().to_glib_none().0)) }
    }

    fn set_can_focus(&self, can_focus: bool) {
        unsafe {
            ffi::gtk_widget_set_can_focus(self.upcast().to_glib_none().0, can_focus.to_glib())
        }
    }

    fn get_double_buffered(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_double_buffered(self.upcast().to_glib_none().0)) }
    }

    fn get_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_widget_get_window(self.upcast().to_glib_none().0))
        }
    }

    fn get_has_window(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_has_window(self.upcast().to_glib_none().0)) }
    }

    fn set_has_window(&self, has_window: bool) {
        unsafe {
            ffi::gtk_widget_set_has_window(self.upcast().to_glib_none().0, has_window.to_glib())
        }
    }

    fn get_sensitive(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_sensitive(self.upcast().to_glib_none().0)) }
    }

    fn is_sensitive(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_is_sensitive(self.upcast().to_glib_none().0)) }
    }

    fn get_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_visible(self.upcast().to_glib_none().0)) }
    }

    #[cfg(gtk_3_8)]
    fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_is_visible(self.upcast().to_glib_none().0)) }
    }

    fn set_visible(&self, visible: bool) {
        unsafe { ffi::gtk_widget_set_visible(self.upcast().to_glib_none().0, visible.to_glib()) }
    }

    fn set_state_flags(&self, flags: StateFlags, clear: bool) {
        unsafe {
            ffi::gtk_widget_set_state_flags(self.upcast().to_glib_none().0, flags, clear.to_glib())
        }
    }

    fn unset_state_flags(&self, flags: StateFlags) {
        unsafe { ffi::gtk_widget_unset_state_flags(self.upcast().to_glib_none().0, flags) }
    }

    fn get_state_flags(&self) -> StateFlags {
        unsafe { ffi::gtk_widget_get_state_flags(self.upcast().to_glib_none().0) }
    }

    fn has_default(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_has_default(self.upcast().to_glib_none().0)) }
    }

    fn has_focus(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_has_focus(self.upcast().to_glib_none().0)) }
    }

    fn has_visible_focus(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_has_visible_focus(self.upcast().to_glib_none().0)) }
    }

    fn has_grab(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_has_grab(self.upcast().to_glib_none().0)) }
    }

    fn is_drawable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_is_drawable(self.upcast().to_glib_none().0)) }
    }

    fn is_toplevel(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_is_toplevel(self.upcast().to_glib_none().0)) }
    }

    fn set_receives_default(&self, receives_default: bool) {
        unsafe {
            ffi::gtk_widget_set_receives_default(
                self.upcast().to_glib_none().0, receives_default.to_glib())
        }
    }

    fn get_receives_default(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_receives_default(self.upcast().to_glib_none().0)) }
    }

    fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe {
            ffi::gtk_widget_set_support_multidevice(
                self.upcast().to_glib_none().0, support_multidevice.to_glib())
        }
    }

    fn get_support_multidevice(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_get_support_multidevice(self.upcast().to_glib_none().0))
        }
    }

    fn set_realized(&self, realized: bool) {
        unsafe {
             ffi::gtk_widget_set_realized(self.upcast().to_glib_none().0, realized.to_glib())
        }
    }

    fn get_realized(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_realized(self.upcast().to_glib_none().0)) }
    }

    fn set_mapped(&self, mapped: bool) {
        unsafe { ffi::gtk_widget_set_mapped(self.upcast().to_glib_none().0, mapped.to_glib()) }
    }

    fn get_mapped(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_mapped(self.upcast().to_glib_none().0)) }
    }

    fn get_modifier_mask(&self, intent: gdk::ModifierIntent) -> gdk::ModifierType {
        unsafe { ffi::gtk_widget_get_modifier_mask(self.upcast().to_glib_none().0, intent) }
    }

    #[cfg(gtk_3_8)]
    fn set_opacity(&self, opacity: f64) {
        unsafe { ffi::gtk_widget_set_opacity(self.upcast().to_glib_none().0, opacity) }
    }

    #[cfg(gtk_3_8)]
    fn get_opacity(&self) -> f64 {
        unsafe { ffi::gtk_widget_get_opacity(self.upcast().to_glib_none().0) }
    }

    fn set_margin_top(&self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_top(self.upcast().to_glib_none().0, margin)
        }
    }

    fn set_margin_bottom(&self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_bottom(self.upcast().to_glib_none().0, margin)
        }
    }

    fn get_margin_top(&self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_top(self.upcast().to_glib_none().0) as i32
        }
    }

    fn get_margin_bottom(&self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_bottom(self.upcast().to_glib_none().0) as i32
        }
    }

    fn get_allocated_width(&self) -> i32 {
        unsafe{
            ffi::gtk_widget_get_allocated_width(self.upcast().to_glib_none().0) as i32
        }
    }

    fn get_allocated_height(&self) -> i32 {
        unsafe{
            ffi::gtk_widget_get_allocated_height(self.upcast().to_glib_none().0) as i32
        }
    }

    fn reset_style(&self) {
        unsafe { ffi::gtk_widget_reset_style(self.upcast().to_glib_none().0) }
    }

    fn get_preferred_height(&self) -> (i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;

        unsafe {
            ffi::gtk_widget_get_preferred_height(
                self.upcast().to_glib_none().0, &mut minimum_height, &mut natural_height);
        }
        (minimum_height, natural_height)
    }

    fn get_preferred_width(&self) -> (i32, i32) {
        let mut minimum_width = 0i32;
        let mut natural_width = 0i32;

        unsafe {
            ffi::gtk_widget_get_preferred_width(
                self.upcast().to_glib_none().0, &mut minimum_width, &mut natural_width);
        }
        (minimum_width, natural_width)
    }

    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;

        unsafe {
            ffi::gtk_widget_get_preferred_height_for_width(
                self.upcast().to_glib_none().0, width, &mut minimum_height, &mut natural_height);
        }
        (minimum_height, natural_height)
    }

    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        let mut minimum_width = 0i32;
        let mut natural_width = 0i32;

        unsafe {
            ffi::gtk_widget_get_preferred_width_for_height(
                self.upcast().to_glib_none().0, height, &mut minimum_width, &mut natural_width);
        }
        (minimum_width, natural_width)
    }

    #[cfg(gtk_3_10)]
    fn get_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;
        let mut minimum_baseline = 0i32;
        let mut natural_baseline = 0i32;

        unsafe {
            ffi::gtk_widget_get_preferred_height_and_baseline_for_width(
                self.upcast().to_glib_none().0, width, &mut minimum_height, &mut natural_height,
                &mut minimum_baseline, &mut natural_baseline);
        }

        (minimum_height, natural_height, minimum_baseline, natural_baseline)
    }

    fn get_request_mode(&self) -> SizeRequestMode {
        unsafe { ffi::gtk_widget_get_request_mode(self.upcast().to_glib_none().0) }
    }

    fn get_halign(&self) -> Align {
        unsafe { ffi::gtk_widget_get_halign(self.upcast().to_glib_none().0) }
    }

    fn set_halign(&self, align: Align) {
        unsafe { ffi::gtk_widget_set_halign(self.upcast().to_glib_none().0, align) }
    }

    fn get_valign(&self) -> Align {
        unsafe { ffi::gtk_widget_get_valign(self.upcast().to_glib_none().0) }
    }

    #[cfg(gtk_3_10)]
    fn get_valign_with_baseline(&self) -> Align {
        unsafe { ffi::gtk_widget_get_valign_with_baseline(self.upcast().to_glib_none().0) }
    }

    fn set_valign(&self, align: Align) {
        unsafe { ffi::gtk_widget_set_valign(self.upcast().to_glib_none().0, align) }
    }

    #[cfg(gtk_3_12)]
    fn get_margin_start(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_margin_start(self.upcast().to_glib_none().0) }
    }

    #[cfg(gtk_3_12)]
    fn set_margin_start(&self, margin: i32) {
        unsafe { ffi::gtk_widget_set_margin_start(self.upcast().to_glib_none().0, margin) }
    }

    #[cfg(gtk_3_12)]
    fn get_margin_end(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_margin_end(self.upcast().to_glib_none().0) }
    }

    #[cfg(gtk_3_12)]
    fn set_margin_end(&self, margin: i32) {
        unsafe { ffi::gtk_widget_set_margin_end(self.upcast().to_glib_none().0, margin) }
    }

    fn get_hexpand(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_hexpand(self.upcast().to_glib_none().0)) }
    }

    fn set_hexpand(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_hexpand(self.upcast().to_glib_none().0, expand.to_glib()) }
    }

    fn get_hexpand_set(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_hexpand_set(self.upcast().to_glib_none().0)) }
    }

    fn set_hexpand_set(&self, expand: bool) {
        unsafe {
            ffi::gtk_widget_set_hexpand_set(self.upcast().to_glib_none().0, expand.to_glib())
        }
    }

    fn get_vexpand(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_vexpand(self.upcast().to_glib_none().0)) }
    }

    fn set_vexpand(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_vexpand(self.upcast().to_glib_none().0, expand.to_glib()) }
    }

    fn get_vexpand_set(&self) -> bool {
        unsafe { from_glib(ffi::gtk_widget_get_vexpand_set(self.upcast().to_glib_none().0)) }
    }

    fn set_vexpand_set(&self, expand: bool) {
        unsafe {
            ffi::gtk_widget_set_vexpand_set(self.upcast().to_glib_none().0, expand.to_glib())
        }
    }

    fn queue_compute_expand(&self) {
        unsafe { ffi::gtk_widget_queue_compute_expand(self.upcast().to_glib_none().0) }
    }

    fn compute_expand(&self, orientation: Orientation) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_compute_expand(self.upcast().to_glib_none().0, orientation))
        }
    }

    #[cfg(gtk_3_10)]
    fn init_template(&self) {
        unsafe { ffi::gtk_widget_init_template(self.upcast().to_glib_none().0) }
    }

    fn thaw_child_notify(&self) {
        unsafe { ffi::gtk_widget_thaw_child_notify(self.upcast().to_glib_none().0) }
    }

    fn freeze_child_notify(&self) {
        unsafe { ffi::gtk_widget_freeze_child_notify(self.upcast().to_glib_none().0) }
    }

    fn child_notify(&self, child_property: &str) {
        unsafe {
            ffi::gtk_widget_child_notify(
                self.upcast().to_glib_none().0, child_property.to_glib_none().0)
        }
    }

    fn destroy(&self) {
        unsafe { ffi::gtk_widget_destroy(self.upcast().to_glib_none().0) }
    }
}
