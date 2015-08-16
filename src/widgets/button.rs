// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;
use {PositionType, ReliefStyle};

pub type Button = Object<ffi::GtkButton>;

impl Button {
    pub fn new() -> Button {
        unsafe { Widget::from_glib_none(ffi::gtk_button_new()).downcast_unchecked() }
    }

    pub fn new_with_label(label: &str) -> Button {
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_label(label.to_glib_none().0))
                .downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Button {
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_mnemonic(mnemonic.to_glib_none().0))
                .downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_icon_name(icon_name: &str, size: i32) -> Button {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_button_new_from_icon_name(icon_name.to_glib_none().0, size))
                .downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> Button {
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_from_stock(stock_id.to_glib_none().0))
                .downcast_unchecked()
        }
    }
}

impl types::StaticType for Button {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_button_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Button { }
unsafe impl Upcast<super::container::Container> for Button { }
unsafe impl Upcast<super::bin::Bin> for Button { }
unsafe impl Upcast<::builder::Buildable> for Button { }

pub trait ButtonExt {
    fn pressed(&self);
    fn released(&self);
    fn clicked(&self);
    fn enter(&self);
    fn leave(&self);
    fn set_relief(&self, new_style: ReliefStyle);
    fn get_relief(&self) -> ReliefStyle;
    fn get_label(&self) -> Option<String>;
    fn set_label(&self, label: &str);
    fn get_use_stock(&self) -> bool;
    fn set_use_stock(&self, use_stock: bool);
    fn get_use_underline(&self) -> bool;
    fn set_use_underline(&self, use_underline: bool);
    fn set_focus_on_click(&self, focus_on_click: bool);
    fn get_focus_on_click(&self) -> bool;
    fn set_alignment(&self, x_align: f32, y_align: f32);
    fn get_alignment(&self) -> (f32, f32);
    fn set_image<T: Upcast<Widget>>(&self, image: &T);
    fn set_image_position(&self, position: PositionType);
    fn get_image_position(&self) -> PositionType;
    fn set_always_show_image(&self, always_show: bool);
    fn get_always_show_image(&self) -> bool;
}

impl<O: Upcast<Button>> ButtonExt for O {
    fn pressed(&self) {
        unsafe {
            ffi::gtk_button_pressed(self.upcast().to_glib_none().0);
        }
    }

    fn released(&self) {
        unsafe {
            ffi::gtk_button_released(self.upcast().to_glib_none().0);
        }
    }

    fn clicked(&self) {
        unsafe {
            ffi::gtk_button_clicked(self.upcast().to_glib_none().0);
        }
    }

    fn enter(&self) {
        unsafe {
            ffi::gtk_button_enter(self.upcast().to_glib_none().0);
        }
    }

    fn leave(&self) {
        unsafe {
            ffi::gtk_button_leave(self.upcast().to_glib_none().0);
        }
    }

    fn set_relief(&self, new_style: ReliefStyle) {
        unsafe {
            ffi::gtk_button_set_relief(self.upcast().to_glib_none().0, new_style);
        }
    }

    fn get_relief(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_button_get_relief(self.upcast().to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_label(self.upcast().to_glib_none().0))
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_button_set_label(self.upcast().to_glib_none().0, label.to_glib_none().0)
        }
    }

    fn get_use_stock(&self) -> bool {
        unsafe { from_glib(ffi::gtk_button_get_use_stock(self.upcast().to_glib_none().0)) }
    }

    fn set_use_stock(&self, use_stock: bool) {
        unsafe {
            ffi::gtk_button_set_use_stock(self.upcast().to_glib_none().0, use_stock.to_glib());
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { from_glib(ffi::gtk_button_get_use_underline(self.upcast().to_glib_none().0)) }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_button_set_use_underline(
                self.upcast().to_glib_none().0, use_underline.to_glib());
        }
    }

    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_button_set_focus_on_click(
                self.upcast().to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn get_focus_on_click(&self) -> bool {
        unsafe { from_glib(ffi::gtk_button_get_focus_on_click(self.upcast().to_glib_none().0)) }
    }

    fn set_alignment(&self, x_align: f32, y_align: f32) {
        unsafe {
            ffi::gtk_button_set_alignment(self.upcast().to_glib_none().0, x_align, y_align)
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let mut x_align = 0.0;
        let mut y_align = 0.0;
        unsafe {
            ffi::gtk_button_get_alignment(
                self.upcast().to_glib_none().0, &mut x_align, &mut y_align);
        }
        (x_align, y_align)
    }

    fn set_image<T: Upcast<Widget>>(&self, image: &T) {
        unsafe {
            ffi::gtk_button_set_image(
                self.upcast().to_glib_none().0, image.upcast().to_glib_none().0);
        }
    }

    fn set_image_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_button_set_image_position(self.upcast().to_glib_none().0, position);
        }
    }

    fn get_image_position(&self) -> PositionType {
        unsafe {
            ffi::gtk_button_get_image_position(self.upcast().to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    fn set_always_show_image(&self, always_show: bool) {
        unsafe {
            ffi::gtk_button_set_always_show_image(
                self.upcast().to_glib_none().0, always_show.to_glib());
        }
    }

    #[cfg(gtk_3_6)]
    fn get_always_show_image(&self) -> bool {
        unsafe { from_glib(ffi::gtk_button_get_always_show_image(self.upcast().to_glib_none().0)) }
    }
}
