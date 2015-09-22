// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use ShadowType;

///////////////////////////////////////////////////////////////////////////////

pub type Frame = Object<ffi::GtkFrame>;

/// A `Bin` with a decorative frame and optional label.
impl Frame {
    pub fn new(label: Option<&str>) -> Frame {
        unsafe {
            Widget::from_glib_none(ffi::gtk_frame_new(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl types::StaticType for Frame {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_frame_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Frame { }
unsafe impl Upcast<::Container> for Frame { }
unsafe impl Upcast<::Bin> for Frame { }
unsafe impl Upcast<::Buildable> for Frame { }

pub trait FrameExt {
    fn set_label(&self, label: Option<&str>);
    fn set_label_widget<T: Upcast<Widget>>(&self, label_widget: &T);
    fn set_label_align(&self, x_align: f32, y_align: f32);
    fn get_label_align(&self) -> (f32, f32);
    fn set_shadow_type(&self, st_type: ShadowType);
    fn get_label(&self) -> Option<String>;
    fn gtk_frame_get_shadow_type(&self) -> ShadowType;
}

impl<O: Upcast<Frame>> FrameExt for O {
    fn set_label(&self, label: Option<&str>) {
        unsafe { ffi::gtk_frame_set_label(self.upcast().to_glib_none().0, label.to_glib_none().0); }
    }

    fn set_label_widget<T: Upcast<Widget>>(&self, label_widget: &T) {
        unsafe {
            ffi::gtk_frame_set_label_widget(self.upcast().to_glib_none().0,
                label_widget.upcast().to_glib_none().0);
        }
    }

    fn set_label_align(&self, x_align: f32, y_align: f32) {
        unsafe { ffi::gtk_frame_set_label_align(self.upcast().to_glib_none().0, x_align, y_align); }
    }

    fn get_label_align(&self) -> (f32, f32) {
        let mut x_align = 0.;
        let mut y_align = 0.;
        unsafe {
            ffi::gtk_frame_get_label_align(self.upcast().to_glib_none().0, &mut x_align,
                &mut y_align);
        }
        (x_align, y_align)
    }

    fn set_shadow_type(&self, st_type: ShadowType) {
        unsafe { ffi::gtk_frame_set_shadow_type(self.upcast().to_glib_none().0, st_type); }
    }

    fn get_label(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_frame_get_label(self.upcast().to_glib_none().0)) }
    }

    fn gtk_frame_get_shadow_type(&self) -> ShadowType {
        unsafe { ffi::gtk_frame_get_shadow_type(self.upcast().to_glib_none().0) }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A frame that constrains its child to a particular aspect ratio.
pub type AspectFrame = Object<ffi::GtkAspectFrame>;

impl AspectFrame {
    pub fn new(label: Option<&str>, x_align: f32, y_align: f32, ratio: f32, obey_child: bool)
            -> AspectFrame {
        unsafe {
            Widget::from_glib_none(
                ffi::gtk_aspect_frame_new(label.to_glib_none().0, x_align, y_align, ratio,
                    obey_child.to_glib()))
                .downcast_unchecked()
        }
    }

    pub fn set(&self, x_align: f32, y_align: f32, ratio: f32, obey_child: bool) {
        unsafe {
            ffi::gtk_aspect_frame_set(self.to_glib_none().0, x_align, y_align, ratio,
                obey_child.to_glib());
        }
    }
}

impl types::StaticType for AspectFrame {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_aspect_frame_get_type()) }
    }
}

unsafe impl Upcast<Widget> for AspectFrame { }
unsafe impl Upcast<::Container> for AspectFrame { }
unsafe impl Upcast<::Bin> for AspectFrame { }
unsafe impl Upcast<Frame> for AspectFrame { }
unsafe impl Upcast<::Buildable> for AspectFrame { }
