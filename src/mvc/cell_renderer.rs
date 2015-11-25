// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

use glib::object::{Downcast, Upcast};
use widgets::widget::Widget;

use {
    CellRendererState,
    SizeRequestMode,
    StateFlags,
};

///////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct CellRenderer(Object<ffi::GtkCellRenderer>);

    match fn {
        get_type => || ffi::gtk_cell_renderer_get_type(),
    }
}

pub trait CellRendererExt {
    fn stop_editing(&self, cancelled: bool);
    fn get_fixed_size(&self, width: &mut i32, height: &mut i32);
    fn set_fixed_size(&self, width: i32, height: i32);
    fn get_visible(&self) -> bool;
    fn set_visible(&self, visible: bool);
    fn get_sensitive(&self) -> bool;
    fn set_sensitive(&self, sensitive: bool);
    fn get_alignment(&self, xalign: &mut f32, yalign: &mut f32);
    fn set_alignment(&self, xalign: f32, yalign: f32);
    fn get_padding(&self, xpad: &mut i32, ypad: &mut i32);
    fn set_padding(&self, xpad: i32, ypad: i32);
    fn get_state<T: Upcast<Widget>>(&self, widget: &T, cell_state: CellRendererState) -> StateFlags;
    fn is_activable(&self) -> bool;
    fn get_preferred_height<T: Upcast<Widget>>(&self, widget: &T, minimum_size: &mut i32,
        natural_size: &mut i32);
    fn get_preferred_height_for_width<T: Upcast<Widget>>(&self, widget: &T, width: i32,
        minimum_size: &mut i32, natural_size: &mut i32);
    fn get_preferred_width<T: Upcast<Widget>>(&self, widget: &T, minimum_size: &mut i32,
        natural_size: &mut i32);
    fn get_preferred_width_for_height<T: Upcast<Widget>>(&self, widget: &T, height: i32,
        minimum_size: &mut i32, natural_size: &mut i32);
    fn get_request_mode(&self) -> SizeRequestMode;
}

impl<O: Upcast<CellRenderer>> CellRendererExt for O {
    fn stop_editing(&self, cancelled: bool) {
        unsafe {
            ffi::gtk_cell_renderer_stop_editing(self.upcast().to_glib_none().0, cancelled.to_glib())
        }
    }

    fn get_fixed_size(&self, width: &mut i32, height: &mut i32) {
        unsafe {
            ffi::gtk_cell_renderer_get_fixed_size(self.upcast().to_glib_none().0, width, height)
        }
    }

    fn set_fixed_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_cell_renderer_set_fixed_size(self.upcast().to_glib_none().0, width, height)
        }
    }

    fn get_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_renderer_get_visible(self.upcast().to_glib_none().0)) }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_visible(self.upcast().to_glib_none().0, visible.to_glib())
        }
    }

    fn get_sensitive(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_renderer_get_sensitive(self.upcast().to_glib_none().0)) }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::gtk_cell_renderer_set_sensitive(self.upcast().to_glib_none().0,
                sensitive.to_glib())
        }
    }

    fn get_alignment(&self, xalign: &mut f32, yalign: &mut f32) {
        unsafe {
            ffi::gtk_cell_renderer_get_alignment(self.upcast().to_glib_none().0, xalign, yalign)
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_cell_renderer_set_alignment(self.upcast().to_glib_none().0, xalign, yalign)
        }
    }

    fn get_padding(&self, xpad: &mut i32, ypad: &mut i32) {
        unsafe { ffi::gtk_cell_renderer_get_padding(self.upcast().to_glib_none().0, xpad, ypad) }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe { ffi::gtk_cell_renderer_set_padding(self.upcast().to_glib_none().0, xpad, ypad) }
    }

    fn get_state<T: Upcast<Widget>>(&self, widget: &T, cell_state: CellRendererState)
            -> StateFlags {
        unsafe {
            ffi::gtk_cell_renderer_get_state(self.upcast().to_glib_none().0,
                widget.upcast().to_glib_none().0, cell_state)
        }
    }

    fn is_activable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_renderer_is_activatable(self.upcast().to_glib_none().0)) }
    }

    fn get_preferred_height<T: Upcast<Widget>>(&self, widget: &T, minimum_size: &mut i32,
            natural_size: &mut i32) {
        unsafe {
            ffi::gtk_cell_renderer_get_preferred_height(self.upcast().to_glib_none().0,
                widget.upcast().to_glib_none().0, minimum_size, natural_size)
        }
    }

    fn get_preferred_height_for_width<T: Upcast<Widget>>(&self, widget: &T, width: i32,
            minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe {
            ffi::gtk_cell_renderer_get_preferred_height_for_width(self.upcast().to_glib_none().0,
                widget.upcast().to_glib_none().0, width, minimum_size, natural_size)
        }
    }

    fn get_preferred_width<T: Upcast<Widget>>(&self, widget: &T, minimum_size: &mut i32,
            natural_size: &mut i32) {
        unsafe {
            ffi::gtk_cell_renderer_get_preferred_width(self.upcast().to_glib_none().0,
                widget.upcast().to_glib_none().0, minimum_size, natural_size)
        }
    }

    fn get_preferred_width_for_height<T: Upcast<Widget>>(&self, widget: &T, height: i32,
            minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe {
            ffi::gtk_cell_renderer_get_preferred_width_for_height(self.upcast().to_glib_none().0,
                widget.upcast().to_glib_none().0, height, minimum_size, natural_size)
        }
    }

    fn get_request_mode(&self) -> SizeRequestMode {
        unsafe { ffi::gtk_cell_renderer_get_request_mode(self.upcast().to_glib_none().0) }
    }
}

///////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct CellRendererText(Object<ffi::GtkCellRendererText>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_text_get_type(),
    }
}

impl CellRendererText {
    pub fn new() -> CellRendererText {
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_text_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererTextExt {
    fn set_fixed_height_from_font(&self, number_of_rows: i32);
}

impl<O: Upcast<CellRendererText>> CellRendererTextExt for O {
    fn set_fixed_height_from_font(&self, number_of_rows: i32) {
        unsafe {
            ffi::gtk_cell_renderer_text_set_fixed_height_from_font(
                self.upcast().to_glib_none().0, number_of_rows)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

glib_wrapper! {
    pub struct CellRendererToggle(Object<ffi::GtkCellRendererToggle>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    pub fn new() -> CellRendererToggle {
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_toggle_new()).downcast_unchecked()
        }
    }

    pub fn get_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_radio(self.to_glib_none().0))
        }
    }

    pub fn set_radio(&self, radio: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(self.to_glib_none().0, radio.to_glib());
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_active(self.to_glib_none().0))
        }
    }

    pub fn set_active(&self, active: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(self.to_glib_none().0, active.to_glib());
        }
    }
}
