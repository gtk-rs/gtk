// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gdk;
use gdk_pixbuf;
use gdk_ffi;
use glib;
use pango;
use ffi;

use {
    AccelFlags,
    AccelGroup,
    Align,
    Allocation,
    DirectionType,
    Orientation,
    Rectangle,
    Requisition,
    SizeRequestMode,
    StateFlags,
    StyleContext,
    TextDirection,
    Widget,
    Window,
};

use auto::traits::WidgetExt as Auto;

pub trait WidgetExt {
    // -- manual --

    fn intersect(&self, area: &Rectangle, intersection: Option<&mut Rectangle>) -> bool;

    fn override_background_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA);

    fn override_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA);

    fn override_cursor(&self, cursor: &gdk_ffi::GdkRGBA, secondary_cursor: &gdk_ffi::GdkRGBA);

    fn override_font(&self, font: &pango::FontDescription);

    fn override_symbolic_color(&self, name: &str, color: &gdk_ffi::GdkRGBA);
    // -- auto --

    fn activate(&self) -> bool;

    fn add_accelerator(&self, accel_signal: &str, accel_group: &AccelGroup, accel_key: u32, accel_mods: gdk::ModifierType, accel_flags: AccelFlags);

    fn add_events(&self, events: i32);

    fn add_mnemonic_label<T: IsA<Widget>>(&self, label: &T);

    fn can_activate_accel(&self, signal_id: u32) -> bool;

    fn child_focus(&self, direction: DirectionType) -> bool;

    fn child_notify(&self, child_property: &str);

    fn compute_expand(&self, orientation: Orientation) -> bool;

    fn destroy(&self);

    fn drag_check_threshold(&self, start_x: i32, start_y: i32, current_x: i32, current_y: i32) -> bool;

    fn drag_dest_add_image_targets(&self);

    fn drag_dest_add_text_targets(&self);

    fn drag_dest_add_uri_targets(&self);

    fn drag_dest_get_track_motion(&self) -> bool;

    fn drag_dest_set_track_motion(&self, track_motion: bool);

    fn drag_dest_unset(&self);

    fn drag_highlight(&self);

    fn drag_source_add_image_targets(&self);

    fn drag_source_add_text_targets(&self);

    fn drag_source_add_uri_targets(&self);

    fn drag_source_set_icon_name(&self, icon_name: &str);

    fn drag_source_set_icon_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf);

    fn drag_source_set_icon_stock(&self, stock_id: &str);

    fn drag_source_unset(&self);

    fn drag_unhighlight(&self);

    fn error_bell(&self);

    fn freeze_child_notify(&self);

    #[cfg(feature = "v3_10")]
    fn get_allocated_baseline(&self) -> i32;

    fn get_allocated_height(&self) -> i32;

    fn get_allocated_width(&self) -> i32;

    fn get_allocation(&self) -> Allocation;

    fn get_ancestor(&self, widget_type: glib::types::Type) -> Option<Widget>;

    fn get_app_paintable(&self) -> bool;

    fn get_can_default(&self) -> bool;

    fn get_can_focus(&self) -> bool;

    fn get_child_visible(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn get_clip(&self) -> Allocation;

    fn get_composite_name(&self) -> Option<String>;

    fn get_direction(&self) -> TextDirection;

    fn get_double_buffered(&self) -> bool;

    fn get_events(&self) -> i32;

    fn get_halign(&self) -> Align;

    fn get_has_tooltip(&self) -> bool;

    fn get_has_window(&self) -> bool;

    fn get_hexpand(&self) -> bool;

    fn get_hexpand_set(&self) -> bool;

    fn get_mapped(&self) -> bool;

    fn get_margin_bottom(&self) -> i32;

    #[cfg(feature = "v3_12")]
    fn get_margin_end(&self) -> i32;

    fn get_margin_left(&self) -> i32;

    fn get_margin_right(&self) -> i32;

    #[cfg(feature = "v3_12")]
    fn get_margin_start(&self) -> i32;

    fn get_margin_top(&self) -> i32;

    fn get_modifier_mask(&self, intent: gdk::ModifierIntent) -> gdk::ModifierType;

    fn get_name(&self) -> Option<String>;

    fn get_no_show_all(&self) -> bool;

    #[cfg(feature = "v3_8")]
    fn get_opacity(&self) -> f64;

    fn get_parent(&self) -> Option<Widget>;

    fn get_parent_window(&self) -> Option<gdk::Window>;

    fn get_preferred_height(&self) -> (i32, i32);

    #[cfg(feature = "v3_10")]
    fn get_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32);

    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32);

    fn get_preferred_size(&self) -> (Requisition, Requisition);

    fn get_preferred_width(&self) -> (i32, i32);

    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32);

    fn get_realized(&self) -> bool;

    fn get_receives_default(&self) -> bool;

    fn get_request_mode(&self) -> SizeRequestMode;

    fn get_root_window(&self) -> Option<gdk::Window>;

    #[cfg(feature = "v3_10")]
    fn get_scale_factor(&self) -> i32;

    fn get_screen(&self) -> Option<gdk::Screen>;

    fn get_sensitive(&self) -> bool;

    fn get_size_request(&self) -> (i32, i32);

    fn get_style_context(&self) -> Option<StyleContext>;

    fn get_support_multidevice(&self) -> bool;

    fn get_tooltip_markup(&self) -> Option<String>;

    fn get_tooltip_text(&self) -> Option<String>;

    fn get_tooltip_window(&self) -> Option<Window>;

    fn get_toplevel(&self) -> Option<Widget>;

    fn get_valign(&self) -> Align;

    #[cfg(feature = "v3_10")]
    fn get_valign_with_baseline(&self) -> Align;

    fn get_vexpand(&self) -> bool;

    fn get_vexpand_set(&self) -> bool;

    fn get_visible(&self) -> bool;

    fn get_window(&self) -> Option<gdk::Window>;

    fn grab_add(&self);

    fn grab_default(&self);

    fn grab_focus(&self);

    fn grab_remove(&self);

    fn has_default(&self) -> bool;

    fn has_focus(&self) -> bool;

    fn has_grab(&self) -> bool;

    fn has_screen(&self) -> bool;

    fn has_visible_focus(&self) -> bool;

    fn hide(&self);

    fn hide_on_delete(&self) -> bool;

    fn in_destruction(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn init_template(&self);

    fn is_ancestor<T: IsA<Widget>>(&self, ancestor: &T) -> bool;

    fn is_composited(&self) -> bool;

    fn is_drawable(&self) -> bool;

    fn is_focus(&self) -> bool;

    fn is_sensitive(&self) -> bool;

    fn is_toplevel(&self) -> bool;

    #[cfg(feature = "v3_8")]
    fn is_visible(&self) -> bool;

    fn keynav_failed(&self, direction: DirectionType) -> bool;

    #[cfg(feature = "v3_16")]
    fn list_action_prefixes(&self) -> Vec<String>;

    fn list_mnemonic_labels(&self) -> Vec<Widget>;

    fn map(&self);

    fn mnemonic_activate(&self, group_cycling: bool) -> bool;

    fn queue_compute_expand(&self);

    fn queue_draw(&self);

    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32);

    fn queue_resize(&self);

    fn queue_resize_no_redraw(&self);

    fn realize(&self);

    #[cfg(feature = "v3_8")]
    fn register_window(&self, window: &gdk::Window);

    fn remove_accelerator(&self, accel_group: &AccelGroup, accel_key: u32, accel_mods: gdk::ModifierType) -> bool;

    fn remove_mnemonic_label<T: IsA<Widget>>(&self, label: &T);

    #[cfg(feature = "v3_8")]
    fn remove_tick_callback(&self, id: u32);

    fn render_icon_pixbuf(&self, stock_id: &str, size: i32) -> Option<gdk_pixbuf::Pixbuf>;

    fn reparent<T: IsA<Widget>>(&self, new_parent: &T);

    fn set_accel_path(&self, accel_path: Option<&str>, accel_group: Option<&AccelGroup>);

    fn set_allocation(&self, allocation: &Allocation);

    fn set_app_paintable(&self, app_paintable: bool);

    fn set_can_default(&self, can_default: bool);

    fn set_can_focus(&self, can_focus: bool);

    fn set_child_visible(&self, is_visible: bool);

    #[cfg(feature = "v3_14")]
    fn set_clip(&self, clip: &Allocation);

    fn set_composite_name(&self, name: &str);

    fn set_direction(&self, dir: TextDirection);

    fn set_double_buffered(&self, double_buffered: bool);

    fn set_events(&self, events: i32);

    fn set_halign(&self, align: Align);

    fn set_has_tooltip(&self, has_tooltip: bool);

    fn set_has_window(&self, has_window: bool);

    fn set_hexpand(&self, expand: bool);

    fn set_hexpand_set(&self, set: bool);

    fn set_mapped(&self, mapped: bool);

    fn set_margin_bottom(&self, margin: i32);

    #[cfg(feature = "v3_12")]
    fn set_margin_end(&self, margin: i32);

    fn set_margin_left(&self, margin: i32);

    fn set_margin_right(&self, margin: i32);

    #[cfg(feature = "v3_12")]
    fn set_margin_start(&self, margin: i32);

    fn set_margin_top(&self, margin: i32);

    fn set_name(&self, name: &str);

    fn set_no_show_all(&self, no_show_all: bool);

    #[cfg(feature = "v3_8")]
    fn set_opacity(&self, opacity: f64);

    fn set_parent<T: IsA<Widget>>(&self, parent: &T);

    fn set_parent_window(&self, parent_window: &gdk::Window);

    fn set_realized(&self, realized: bool);

    fn set_receives_default(&self, receives_default: bool);

    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool);

    fn set_sensitive(&self, sensitive: bool);

    fn set_size_request(&self, width: i32, height: i32);

    fn set_support_multidevice(&self, support_multidevice: bool);

    fn set_tooltip_markup(&self, markup: Option<&str>);

    fn set_tooltip_text(&self, text: Option<&str>);

    fn set_tooltip_window<T: IsA<Window>>(&self, custom_window: Option<&T>);

    fn set_valign(&self, align: Align);

    fn set_vexpand(&self, expand: bool);

    fn set_vexpand_set(&self, set: bool);

    fn set_visible(&self, visible: bool);

    fn set_window(&self, window: &gdk::Window);

    fn show(&self);

    fn show_all(&self);

    fn show_now(&self);

    fn size_allocate(&self, allocation: &mut Allocation);

    #[cfg(feature = "v3_10")]
    fn size_allocate_with_baseline(&self, allocation: &mut Allocation, baseline: i32);

    fn thaw_child_notify(&self);

    fn translate_coordinates<T: IsA<Widget>>(&self, dest_widget: &T, src_x: i32, src_y: i32) -> Option<(i32, i32)>;

    fn trigger_tooltip_query(&self);

    fn unmap(&self);

    fn unparent(&self);

    fn unrealize(&self);

    #[cfg(feature = "v3_8")]
    fn unregister_window(&self, window: &gdk::Window);
}

impl<O: IsA<Widget>> WidgetExt for O {
    fn intersect(&self, area: &Rectangle, mut intersection: Option<&mut Rectangle>) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_intersect(self.to_glib_none().0, area.to_glib_none().0, intersection.to_glib_none_mut().0))
        }
    }

    fn override_background_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::gtk_widget_override_background_color(self.to_glib_none().0, state.to_glib(), color)
        }
    }

    fn override_color(&self, state: StateFlags, color: &gdk_ffi::GdkRGBA) {
        unsafe { ffi::gtk_widget_override_color(self.to_glib_none().0, state.to_glib(), color) }
    }

    fn override_symbolic_color(&self, name: &str, color: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::gtk_widget_override_symbolic_color(self.to_glib_none().0,
                name.to_glib_none().0, color);
        }
    }

    fn override_cursor(&self, cursor: &gdk_ffi::GdkRGBA, secondary_cursor: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::gtk_widget_override_cursor(self.to_glib_none().0, cursor,
                secondary_cursor)
        }
    }

    fn override_font(&self, font: &pango::FontDescription) {
        unsafe {
            ffi::gtk_widget_override_font(self.to_glib_none().0, font.to_glib_none().0)
        }
    }

    #[inline]
    fn activate(&self) -> bool {
        Auto::activate(self)
    }

    #[inline]
    fn add_accelerator(&self, accel_signal: &str, accel_group: &AccelGroup, accel_key: u32, accel_mods: gdk::ModifierType, accel_flags: AccelFlags) {
        Auto::add_accelerator(self, accel_signal, accel_group, accel_key, accel_mods, accel_flags)
    }

    #[inline]
    fn add_events(&self, events: i32) {
        Auto::add_events(self, events)
    }

    #[inline]
    fn add_mnemonic_label<T: IsA<Widget>>(&self, label: &T) {
        Auto::add_mnemonic_label(self, label)
    }

    #[inline]
    fn can_activate_accel(&self, signal_id: u32) -> bool {
        Auto::can_activate_accel(self, signal_id)
    }

    #[inline]
    fn child_focus(&self, direction: DirectionType) -> bool {
        Auto::child_focus(self, direction)
    }

    #[inline]
    fn child_notify(&self, child_property: &str) {
        Auto::child_notify(self, child_property)
    }

    #[inline]
    fn compute_expand(&self, orientation: Orientation) -> bool {
        Auto::compute_expand(self, orientation)
    }

    #[inline]
    fn destroy(&self) {
        Auto::destroy(self)
    }

    #[inline]
    fn drag_check_threshold(&self, start_x: i32, start_y: i32, current_x: i32, current_y: i32) -> bool {
        Auto::drag_check_threshold(self, start_x, start_y, current_x, current_y)
    }

    #[inline]
    fn drag_dest_add_image_targets(&self) {
        Auto::drag_dest_add_image_targets(self)
    }

    #[inline]
    fn drag_dest_add_text_targets(&self) {
        Auto::drag_dest_add_text_targets(self)
    }

    #[inline]
    fn drag_dest_add_uri_targets(&self) {
        Auto::drag_dest_add_uri_targets(self)
    }

    #[inline]
    fn drag_dest_get_track_motion(&self) -> bool {
        Auto::drag_dest_get_track_motion(self)
    }

    #[inline]
    fn drag_dest_set_track_motion(&self, track_motion: bool) {
        Auto::drag_dest_set_track_motion(self, track_motion)
    }

    #[inline]
    fn drag_dest_unset(&self) {
        Auto::drag_dest_unset(self)
    }

    #[inline]
    fn drag_highlight(&self) {
        Auto::drag_highlight(self)
    }

    #[inline]
    fn drag_source_add_image_targets(&self) {
        Auto::drag_source_add_image_targets(self)
    }

    #[inline]
    fn drag_source_add_text_targets(&self) {
        Auto::drag_source_add_text_targets(self)
    }

    #[inline]
    fn drag_source_add_uri_targets(&self) {
        Auto::drag_source_add_uri_targets(self)
    }

    #[inline]
    fn drag_source_set_icon_name(&self, icon_name: &str) {
        Auto::drag_source_set_icon_name(self, icon_name)
    }

    #[inline]
    fn drag_source_set_icon_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        Auto::drag_source_set_icon_pixbuf(self, pixbuf)
    }

    #[inline]
    fn drag_source_set_icon_stock(&self, stock_id: &str) {
        Auto::drag_source_set_icon_stock(self, stock_id)
    }

    #[inline]
    fn drag_source_unset(&self) {
        Auto::drag_source_unset(self)
    }

    #[inline]
    fn drag_unhighlight(&self) {
        Auto::drag_unhighlight(self)
    }

    #[inline]
    fn error_bell(&self) {
        Auto::error_bell(self)
    }

    #[inline]
    fn freeze_child_notify(&self) {
        Auto::freeze_child_notify(self)
    }

    #[cfg(feature = "v3_10")]
    #[inline]
    fn get_allocated_baseline(&self) -> i32 {
        Auto::get_allocated_baseline(self)
    }

    #[inline]
    fn get_allocated_height(&self) -> i32 {
        Auto::get_allocated_height(self)
    }

    #[inline]
    fn get_allocated_width(&self) -> i32 {
        Auto::get_allocated_width(self)
    }

    #[inline]
    fn get_allocation(&self) -> Allocation {
        Auto::get_allocation(self)
    }

    #[inline]
    fn get_ancestor(&self, widget_type: glib::types::Type) -> Option<Widget> {
        Auto::get_ancestor(self, widget_type)
    }

    #[inline]
    fn get_app_paintable(&self) -> bool {
        Auto::get_app_paintable(self)
    }

    #[inline]
    fn get_can_default(&self) -> bool {
        Auto::get_can_default(self)
    }

    #[inline]
    fn get_can_focus(&self) -> bool {
        Auto::get_can_focus(self)
    }

    #[inline]
    fn get_child_visible(&self) -> bool {
        Auto::get_child_visible(self)
    }

    #[cfg(feature = "v3_14")]
    #[inline]
    fn get_clip(&self) -> Allocation {
        Auto::get_clip(self)
    }

    #[inline]
    fn get_composite_name(&self) -> Option<String> {
        Auto::get_composite_name(self)
    }

    #[inline]
    fn get_direction(&self) -> TextDirection {
        Auto::get_direction(self)
    }

    #[inline]
    fn get_double_buffered(&self) -> bool {
        Auto::get_double_buffered(self)
    }

    #[inline]
    fn get_events(&self) -> i32 {
        Auto::get_events(self)
    }

    #[inline]
    fn get_halign(&self) -> Align {
        Auto::get_halign(self)
    }

    #[inline]
    fn get_has_tooltip(&self) -> bool {
        Auto::get_has_tooltip(self)
    }

    #[inline]
    fn get_has_window(&self) -> bool {
        Auto::get_has_window(self)
    }

    #[inline]
    fn get_hexpand(&self) -> bool {
        Auto::get_hexpand(self)
    }

    #[inline]
    fn get_hexpand_set(&self) -> bool {
        Auto::get_hexpand_set(self)
    }

    #[inline]
    fn get_mapped(&self) -> bool {
        Auto::get_mapped(self)
    }

    #[inline]
    fn get_margin_bottom(&self) -> i32 {
        Auto::get_margin_bottom(self)
    }

    #[cfg(feature = "v3_12")]
    #[inline]
    fn get_margin_end(&self) -> i32 {
        Auto::get_margin_end(self)
    }

    #[inline]
    fn get_margin_left(&self) -> i32 {
        Auto::get_margin_left(self)
    }

    #[inline]
    fn get_margin_right(&self) -> i32 {
        Auto::get_margin_right(self)
    }

    #[cfg(feature = "v3_12")]
    #[inline]
    fn get_margin_start(&self) -> i32 {
        Auto::get_margin_start(self)
    }

    #[inline]
    fn get_margin_top(&self) -> i32 {
        Auto::get_margin_top(self)
    }

    #[inline]
    fn get_modifier_mask(&self, intent: gdk::ModifierIntent) -> gdk::ModifierType {
        Auto::get_modifier_mask(self, intent)
    }

    #[inline]
    fn get_name(&self) -> Option<String> {
        Auto::get_name(self)
    }

    #[inline]
    fn get_no_show_all(&self) -> bool {
        Auto::get_no_show_all(self)
    }

    #[cfg(feature = "v3_8")]
    #[inline]
    fn get_opacity(&self) -> f64 {
        Auto::get_opacity(self)
    }

    #[inline]
    fn get_parent(&self) -> Option<Widget> {
        Auto::get_parent(self)
    }

    #[inline]
    fn get_parent_window(&self) -> Option<gdk::Window> {
        Auto::get_parent_window(self)
    }

    #[inline]
    fn get_preferred_height(&self) -> (i32, i32) {
        Auto::get_preferred_height(self)
    }

    #[cfg(feature = "v3_10")]
    #[inline]
    fn get_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32) {
        Auto::get_preferred_height_and_baseline_for_width(self, width)
    }

    #[inline]
    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        Auto::get_preferred_height_for_width(self, width)
    }

    #[inline]
    fn get_preferred_size(&self) -> (Requisition, Requisition) {
        Auto::get_preferred_size(self)
    }

    #[inline]
    fn get_preferred_width(&self) -> (i32, i32) {
        Auto::get_preferred_width(self)
    }

    #[inline]
    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        Auto::get_preferred_width_for_height(self, height)
    }

    #[inline]
    fn get_realized(&self) -> bool {
        Auto::get_realized(self)
    }

    #[inline]
    fn get_receives_default(&self) -> bool {
        Auto::get_receives_default(self)
    }

    #[inline]
    fn get_request_mode(&self) -> SizeRequestMode {
        Auto::get_request_mode(self)
    }

    #[inline]
    fn get_root_window(&self) -> Option<gdk::Window> {
        Auto::get_root_window(self)
    }

    #[cfg(feature = "v3_10")]
    #[inline]
    fn get_scale_factor(&self) -> i32 {
        Auto::get_scale_factor(self)
    }

    #[inline]
    fn get_screen(&self) -> Option<gdk::Screen> {
        Auto::get_screen(self)
    }

    #[inline]
    fn get_sensitive(&self) -> bool {
        Auto::get_sensitive(self)
    }

    #[inline]
    fn get_size_request(&self) -> (i32, i32) {
        Auto::get_size_request(self)
    }

    #[inline]
    fn get_support_multidevice(&self) -> bool {
        Auto::get_support_multidevice(self)
    }

    #[inline]
    fn get_tooltip_markup(&self) -> Option<String> {
        Auto::get_tooltip_markup(self)
    }

    #[inline]
    fn get_tooltip_text(&self) -> Option<String> {
        Auto::get_tooltip_text(self)
    }

    #[inline]
    fn get_tooltip_window(&self) -> Option<Window> {
        Auto::get_tooltip_window(self)
    }

    #[inline]
    fn get_toplevel(&self) -> Option<Widget> {
        Auto::get_toplevel(self)
    }

    #[inline]
    fn get_valign(&self) -> Align {
        Auto::get_valign(self)
    }

    #[cfg(feature = "v3_10")]
    #[inline]
    fn get_valign_with_baseline(&self) -> Align {
        Auto::get_valign_with_baseline(self)
    }

    #[inline]
    fn get_vexpand(&self) -> bool {
        Auto::get_vexpand(self)
    }

    #[inline]
    fn get_vexpand_set(&self) -> bool {
        Auto::get_vexpand_set(self)
    }

    #[inline]
    fn get_visible(&self) -> bool {
        Auto::get_visible(self)
    }

    #[inline]
    fn get_window(&self) -> Option<gdk::Window> {
        Auto::get_window(self)
    }

    #[inline]
    fn grab_add(&self) {
        Auto::grab_add(self)
    }

    #[inline]
    fn grab_default(&self) {
        Auto::grab_default(self)
    }

    #[inline]
    fn grab_focus(&self) {
        Auto::grab_focus(self)
    }

    #[inline]
    fn grab_remove(&self) {
        Auto::grab_remove(self)
    }

    #[inline]
    fn has_default(&self) -> bool {
        Auto::has_default(self)
    }

    #[inline]
    fn has_focus(&self) -> bool {
        Auto::has_focus(self)
    }

    #[inline]
    fn has_grab(&self) -> bool {
        Auto::has_grab(self)
    }

    #[inline]
    fn has_screen(&self) -> bool {
        Auto::has_screen(self)
    }

    #[inline]
    fn has_visible_focus(&self) -> bool {
        Auto::has_visible_focus(self)
    }

    #[inline]
    fn hide(&self) {
        Auto::hide(self)
    }

    #[inline]
    fn hide_on_delete(&self) -> bool {
        Auto::hide_on_delete(self)
    }

    #[inline]
    fn in_destruction(&self) -> bool {
        Auto::in_destruction(self)
    }

    #[cfg(feature = "v3_10")]
    #[inline]
    fn init_template(&self) {
        Auto::init_template(self)
    }

    #[inline]
    fn is_ancestor<T: IsA<Widget>>(&self, ancestor: &T) -> bool {
        Auto::is_ancestor(self, ancestor)
    }

    #[inline]
    fn is_composited(&self) -> bool {
        Auto::is_composited(self)
    }

    #[inline]
    fn is_drawable(&self) -> bool {
        Auto::is_drawable(self)
    }

    #[inline]
    fn is_focus(&self) -> bool {
        Auto::is_focus(self)
    }

    #[inline]
    fn is_sensitive(&self) -> bool {
        Auto::is_sensitive(self)
    }

    #[inline]
    fn is_toplevel(&self) -> bool {
        Auto::is_toplevel(self)
    }

    #[cfg(feature = "v3_8")]
    #[inline]
    fn is_visible(&self) -> bool {
        Auto::is_visible(self)
    }

    #[inline]
    fn keynav_failed(&self, direction: DirectionType) -> bool {
        Auto::keynav_failed(self, direction)
    }

    #[cfg(feature = "v3_16")]
    #[inline]
    fn list_action_prefixes(&self) -> Vec<String> {
        Auto::list_action_prefixes(self)
    }

    #[inline]
    fn list_mnemonic_labels(&self) -> Vec<Widget> {
        Auto::list_mnemonic_labels(self)
    }

    #[inline]
    fn map(&self) {
        Auto::map(self)
    }

    #[inline]
    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        Auto::mnemonic_activate(self, group_cycling)
    }

    #[inline]
    fn queue_compute_expand(&self) {
        Auto::queue_compute_expand(self)
    }

    #[inline]
    fn queue_draw(&self) {
        Auto::queue_draw(self)
    }

    #[inline]
    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32) {
        Auto::queue_draw_area(self, x, y, width, height)
    }

    #[inline]
    fn queue_resize(&self) {
        Auto::queue_resize(self)
    }

    #[inline]
    fn queue_resize_no_redraw(&self) {
        Auto::queue_resize_no_redraw(self)
    }

    #[inline]
    fn realize(&self) {
        Auto::realize(self)
    }

    #[cfg(feature = "v3_8")]
    #[inline]
    fn register_window(&self, window: &gdk::Window) {
        Auto::register_window(self, window)
    }

    #[inline]
    fn remove_accelerator(&self, accel_group: &AccelGroup, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
        Auto::remove_accelerator(self, accel_group, accel_key, accel_mods)
    }

    #[inline]
    fn remove_mnemonic_label<T: IsA<Widget>>(&self, label: &T) {
        Auto::remove_mnemonic_label(self, label)
    }

    #[cfg(feature = "v3_8")]
    #[inline]
    fn remove_tick_callback(&self, id: u32) {
        Auto::remove_tick_callback(self, id)
    }

    #[inline]
    fn render_icon_pixbuf(&self, stock_id: &str, size: i32) -> Option<gdk_pixbuf::Pixbuf> {
        Auto::render_icon_pixbuf(self, stock_id, size)
    }

    #[inline]
    fn reparent<T: IsA<Widget>>(&self, new_parent: &T) {
        Auto::reparent(self, new_parent)
    }

    #[inline]
    fn set_accel_path(&self, accel_path: Option<&str>, accel_group: Option<&AccelGroup>) {
        Auto::set_accel_path(self, accel_path, accel_group)
    }

    #[inline]
    fn set_allocation(&self, allocation: &Allocation) {
        Auto::set_allocation(self, allocation)
    }

    #[inline]
    fn set_app_paintable(&self, app_paintable: bool) {
        Auto::set_app_paintable(self, app_paintable)
    }

    #[inline]
    fn set_can_default(&self, can_default: bool) {
        Auto::set_can_default(self, can_default)
    }

    #[inline]
    fn set_can_focus(&self, can_focus: bool) {
        Auto::set_can_focus(self, can_focus)
    }

    #[inline]
    fn set_child_visible(&self, is_visible: bool) {
        Auto::set_child_visible(self, is_visible)
    }

    #[cfg(feature = "v3_14")]
    #[inline]
    fn set_clip(&self, clip: &Allocation) {
        Auto::set_clip(self, clip)
    }

    #[inline]
    fn set_composite_name(&self, name: &str) {
        Auto::set_composite_name(self, name)
    }

    #[inline]
    fn set_direction(&self, dir: TextDirection) {
        Auto::set_direction(self, dir)
    }

    #[inline]
    fn set_double_buffered(&self, double_buffered: bool) {
        Auto::set_double_buffered(self, double_buffered)
    }

    #[inline]
    fn set_events(&self, events: i32) {
        Auto::set_events(self, events)
    }

    #[inline]
    fn set_halign(&self, align: Align) {
        Auto::set_halign(self, align)
    }

    #[inline]
    fn set_has_tooltip(&self, has_tooltip: bool) {
        Auto::set_has_tooltip(self, has_tooltip)
    }

    #[inline]
    fn set_has_window(&self, has_window: bool) {
        Auto::set_has_window(self, has_window)
    }

    #[inline]
    fn set_hexpand(&self, expand: bool) {
        Auto::set_hexpand(self, expand)
    }

    #[inline]
    fn set_hexpand_set(&self, set: bool) {
        Auto::set_hexpand_set(self, set)
    }

    #[inline]
    fn set_mapped(&self, mapped: bool) {
        Auto::set_mapped(self, mapped)
    }

    #[inline]
    fn set_margin_bottom(&self, margin: i32) {
        Auto::set_margin_bottom(self, margin)
    }

    #[cfg(feature = "v3_12")]
    #[inline]
    fn set_margin_end(&self, margin: i32) {
        Auto::set_margin_end(self, margin)
    }

    #[inline]
    fn set_margin_left(&self, margin: i32) {
        Auto::set_margin_left(self, margin)
    }

    #[inline]
    fn set_margin_right(&self, margin: i32) {
        Auto::set_margin_right(self, margin)
    }

    #[cfg(feature = "v3_12")]
    #[inline]
    fn set_margin_start(&self, margin: i32) {
        Auto::set_margin_start(self, margin)
    }

    #[inline]
    fn set_margin_top(&self, margin: i32) {
        Auto::set_margin_top(self, margin)
    }

    #[inline]
    fn set_name(&self, name: &str) {
        Auto::set_name(self, name)
    }

    #[inline]
    fn set_no_show_all(&self, no_show_all: bool) {
        Auto::set_no_show_all(self, no_show_all)
    }

    #[cfg(feature = "v3_8")]
    #[inline]
    fn set_opacity(&self, opacity: f64) {
        Auto::set_opacity(self, opacity)
    }

    #[inline]
    fn set_parent<T: IsA<Widget>>(&self, parent: &T) {
        Auto::set_parent(self, parent)
    }

    #[inline]
    fn set_parent_window(&self, parent_window: &gdk::Window) {
        Auto::set_parent_window(self, parent_window)
    }

    #[inline]
    fn set_realized(&self, realized: bool) {
        Auto::set_realized(self, realized)
    }

    #[inline]
    fn set_receives_default(&self, receives_default: bool) {
        Auto::set_receives_default(self, receives_default)
    }

    #[inline]
    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool) {
        Auto::set_redraw_on_allocate(self, redraw_on_allocate)
    }

    #[inline]
    fn set_sensitive(&self, sensitive: bool) {
        Auto::set_sensitive(self, sensitive)
    }

    #[inline]
    fn set_size_request(&self, width: i32, height: i32) {
        Auto::set_size_request(self, width, height)
    }

    #[inline]
    fn get_style_context(&self) -> Option<StyleContext> {
        Auto::get_style_context(self)
    }

    #[inline]
    fn set_support_multidevice(&self, support_multidevice: bool) {
        Auto::set_support_multidevice(self, support_multidevice)
    }

    #[inline]
    fn set_tooltip_markup(&self, markup: Option<&str>) {
        Auto::set_tooltip_markup(self, markup)
    }

    #[inline]
    fn set_tooltip_text(&self, text: Option<&str>) {
        Auto::set_tooltip_text(self, text)
    }

    #[inline]
    fn set_tooltip_window<T: IsA<Window>>(&self, custom_window: Option<&T>) {
        Auto::set_tooltip_window(self, custom_window)
    }

    #[inline]
    fn set_valign(&self, align: Align) {
        Auto::set_valign(self, align)
    }

    #[inline]
    fn set_vexpand(&self, expand: bool) {
        Auto::set_vexpand(self, expand)
    }

    #[inline]
    fn set_vexpand_set(&self, set: bool) {
        Auto::set_vexpand_set(self, set)
    }

    #[inline]
    fn set_visible(&self, visible: bool) {
        Auto::set_visible(self, visible)
    }

    #[inline]
    fn set_window(&self, window: &gdk::Window) {
        Auto::set_window(self, window)
    }

    #[inline]
    fn show(&self) {
        Auto::show(self)
    }

    #[inline]
    fn show_all(&self) {
        Auto::show_all(self)
    }

    #[inline]
    fn show_now(&self) {
        Auto::show_now(self)
    }

    #[inline]
    fn size_allocate(&self, allocation: &mut Allocation) {
        Auto::size_allocate(self, allocation)
    }

    #[cfg(feature = "v3_10")]
    #[inline]
    fn size_allocate_with_baseline(&self, allocation: &mut Allocation, baseline: i32) {
        Auto::size_allocate_with_baseline(self, allocation, baseline)
    }

    #[inline]
    fn thaw_child_notify(&self) {
        Auto::thaw_child_notify(self)
    }

    #[inline]
    fn translate_coordinates<T: IsA<Widget>>(&self, dest_widget: &T, src_x: i32, src_y: i32) -> Option<(i32, i32)> {
        Auto::translate_coordinates(self, dest_widget, src_x, src_y)
    }

    #[inline]
    fn trigger_tooltip_query(&self) {
        Auto::trigger_tooltip_query(self)
    }

    #[inline]
    fn unmap(&self) {
        Auto::unmap(self)
    }

    #[inline]
    fn unparent(&self) {
        Auto::unparent(self)
    }

    #[inline]
    fn unrealize(&self) {
        Auto::unrealize(self)
    }

    #[cfg(feature = "v3_8")]
    #[inline]
    fn unregister_window(&self, window: &gdk::Window) {
        Auto::unregister_window(self, window)
    }

}
