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

use gtk::ffi;
use gtk::traits;
use gtk::cast::GTK_CELL_RENDERER;
use gtk;

pub trait CellRenderer: traits::Widget {
    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_renderer_stop_editing(GTK_CELL_RENDERER(self.get_widget()), match canceled {
                true => 1,
                false => 0
            })
        }
    }

    fn get_fixed_size(&self, width: &mut i32, height: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_fixed_size(GTK_CELL_RENDERER(self.get_widget()), width, height) }
    }

    fn set_fixed_size(&self, width: i32, height: i32) {
        unsafe { ffi::gtk_cell_renderer_set_fixed_size(GTK_CELL_RENDERER(self.get_widget()), width, height) }
    }

    fn get_visible(&self) -> bool {
        match unsafe { ffi::gtk_cell_renderer_get_visible(GTK_CELL_RENDERER(self.get_widget())) } {
            0 => false,
            _ => true
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_visible(GTK_CELL_RENDERER(self.get_widget()), match visible {
                true => 1,
                false => 0
            })
        }
    }

    fn get_sensitive(&self) -> bool {
        match unsafe { ffi::gtk_cell_renderer_get_sensitive(GTK_CELL_RENDERER(self.get_widget())) } {
            0 => false,
            _ => true
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_sensitive(GTK_CELL_RENDERER(self.get_widget()), match sensitive {
                true => 1,
                false => 0
            })
        }
    }

    fn get_alignment(&self, xalign: &mut f32, yalign: &mut f32) {
        unsafe { ffi::gtk_cell_renderer_get_alignment(GTK_CELL_RENDERER(self.get_widget()), xalign, yalign) }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe { ffi::gtk_cell_renderer_set_alignment(GTK_CELL_RENDERER(self.get_widget()), xalign, yalign) }
    }

    fn get_padding(&self, xpad: &mut i32, ypad: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_padding(GTK_CELL_RENDERER(self.get_widget()), xpad, ypad) }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe { ffi::gtk_cell_renderer_set_padding(GTK_CELL_RENDERER(self.get_widget()), xpad, ypad) }
    }

    fn get_state(&self, widget: &traits::Widget, cell_state: gtk::CellRendererState) -> gtk::StateFlags {
        unsafe { ffi::gtk_cell_renderer_get_state(GTK_CELL_RENDERER(self.get_widget()), widget.get_widget(), cell_state) }
    }

    fn is_activable(&self) -> bool {
        match unsafe { ffi::gtk_cell_renderer_is_activatable(GTK_CELL_RENDERER(self.get_widget())) } {
            0 => false,
            _ => true
        }
    }
    
    fn get_preferred_height(&self, widget: &traits::Widget, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_height(GTK_CELL_RENDERER(self.get_widget()), widget.get_widget(), minimum_size,
            natural_size) }
    }

    fn get_preferred_height_for_width(&self, widget: &traits::Widget, width: i32, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_height_for_width(GTK_CELL_RENDERER(self.get_widget()), widget.get_widget(), width,
            minimum_size, natural_size) }
    }

    fn get_preferred_width(&self, widget: &traits::Widget, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_width(GTK_CELL_RENDERER(self.get_widget()), widget.get_widget(), minimum_size,
            natural_size) }
    }

    fn get_preferred_width_for_height(&self, widget: &traits::Widget, height: i32, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_preferred_width_for_height(GTK_CELL_RENDERER(self.get_widget()), widget.get_widget(), height,
            minimum_size, natural_size) }
    }

    fn get_request_mode(&self) -> gtk::SizeRequestMode {
        unsafe { ffi::gtk_cell_renderer_get_request_mode(GTK_CELL_RENDERER(self.get_widget())) }
    }
}