// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A frame that constrains its child to a particular aspect ratio

use libc::c_float;

use glib::translate::ToGlibPtr;
use cast::GTK_ASPECTFRAME;
use ffi;
use glib::to_gboolean;

/// AspectFrame — A frame that constrains its child to a particular aspect ratio
struct_Widget!(AspectFrame);

impl AspectFrame {
    pub fn new(label: Option<&str>, x_align: f32, y_align: f32, ratio: f32, obey_child: bool) -> Option<AspectFrame> {
        let tmp_pointer = unsafe {
            ffi::gtk_aspect_frame_new(label.to_glib_none().0,
                                      x_align as c_float, y_align as c_float,
                                      ratio as c_float, to_gboolean(obey_child))
        };
        check_pointer!(tmp_pointer, AspectFrame)
    }

    pub fn set(&self,
               x_align: f32,
               y_align: f32,
               ratio: f32,
               obey_child: bool) -> () {

        unsafe {
            ffi::gtk_aspect_frame_set(GTK_ASPECTFRAME(self.pointer), x_align as c_float, y_align as c_float, ratio as c_float, to_gboolean(obey_child));
        }
    }
}

impl_drop!(AspectFrame);
impl_TraitWidget!(AspectFrame);

impl ::FrameTrait for AspectFrame {}
impl ::ContainerTrait for AspectFrame {}
