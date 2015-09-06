// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use cast::GTK_CELL_RENDERER;

pub trait CellRendererTrait: ::WidgetTrait {
    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_renderer_stop_editing(GTK_CELL_RENDERER(self.unwrap_widget()), match canceled {
                true => 1,
                false => 0
            })
        }
    }

    fn get_fixed_size(&self, width: &mut i32, height: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_fixed_size(GTK_CELL_RENDERER(self.unwrap_widget()), width, height) }
    }

    fn set_fixed_size(&self, width: i32, height: i32) {
        unsafe { ffi::gtk_cell_renderer_set_fixed_size(GTK_CELL_RENDERER(self.unwrap_widget()), width, height) }
    }

    fn get_visible(&self) -> bool {
        match unsafe { ffi::gtk_cell_renderer_get_visible(GTK_CELL_RENDERER(self.unwrap_widget())) } {
            0 => false,
            _ => true
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_visible(GTK_CELL_RENDERER(self.unwrap_widget()), match visible {
                true => 1,
                false => 0
            })
        }
    }

    fn get_sensitive(&self) -> bool {
        match unsafe { ffi::gtk_cell_renderer_get_sensitive(GTK_CELL_RENDERER(self.unwrap_widget())) } {
            0 => false,
            _ => true
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_sensitive(GTK_CELL_RENDERER(self.unwrap_widget()), match sensitive {
                true => 1,
                false => 0
            })
        }
    }

    fn get_alignment(&self, xalign: &mut f32, yalign: &mut f32) {
        unsafe { ffi::gtk_cell_renderer_get_alignment(GTK_CELL_RENDERER(self.unwrap_widget()), xalign, yalign) }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe { ffi::gtk_cell_renderer_set_alignment(GTK_CELL_RENDERER(self.unwrap_widget()), xalign, yalign) }
    }

    fn get_padding(&self, xpad: &mut i32, ypad: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_padding(GTK_CELL_RENDERER(self.unwrap_widget()), xpad, ypad) }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe { ffi::gtk_cell_renderer_set_padding(GTK_CELL_RENDERER(self.unwrap_widget()), xpad, ypad) }
    }

    fn get_state<T: ::WidgetTrait>(&self, widget: &T, cell_state: ::CellRendererState) -> ::StateFlags {
        unsafe { ffi::gtk_cell_renderer_get_state(GTK_CELL_RENDERER(self.unwrap_widget()), widget.unwrap_widget(), cell_state) }
    }

    fn is_activable(&self) -> bool {
        match unsafe { ffi::gtk_cell_renderer_is_activatable(GTK_CELL_RENDERER(self.unwrap_widget())) } {
            0 => false,
            _ => true
        }
    }

    fn get_preferred_height<T: ::WidgetTrait>(&self, widget: &T, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_height(GTK_CELL_RENDERER(self.unwrap_widget()), widget.unwrap_widget(), minimum_size,
            natural_size) }
    }

    fn get_preferred_height_for_width<T: ::WidgetTrait>(&self, widget: &T, width: i32, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_height_for_width(GTK_CELL_RENDERER(self.unwrap_widget()), widget.unwrap_widget(), width,
            minimum_size, natural_size) }
    }

    fn get_preferred_width<T: ::WidgetTrait>(&self, widget: &T, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_width(GTK_CELL_RENDERER(self.unwrap_widget()), widget.unwrap_widget(), minimum_size,
            natural_size) }
    }

    fn get_preferred_width_for_height<T: ::WidgetTrait>(&self, widget: &T, height: i32, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_width_for_height(GTK_CELL_RENDERER(self.unwrap_widget()), widget.unwrap_widget(), height,
            minimum_size, natural_size) }
    }

    fn get_request_mode(&self) -> ::SizeRequestMode {
        unsafe { ffi::gtk_cell_renderer_get_request_mode(GTK_CELL_RENDERER(self.unwrap_widget())) }
    }
}
