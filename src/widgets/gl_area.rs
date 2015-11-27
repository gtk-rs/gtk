// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_GL_AREA;
use ffi;
use glib::{to_bool, to_gboolean};
use glib::translate::from_glib_none;

struct_Widget!(GLArea);

impl GLArea {
    pub fn new() -> Option<GLArea> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_gl_area_new() };
        check_pointer!(tmp_pointer, GLArea)
    }

    pub fn get_context(&self) -> Option<::gdk::GLContext> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_context(GTK_GL_AREA(self.pointer))) }
    }

    pub fn make_current(&self) {
        unsafe { ffi::gtk_gl_area_make_current(GTK_GL_AREA(self.pointer)) }
    }

    pub fn queue_render(&self) {
        unsafe { ffi::gtk_gl_area_queue_render(GTK_GL_AREA(self.pointer)) }
    }

    pub fn attach_buffers(&self) {
        unsafe { ffi::gtk_gl_area_attach_buffers(GTK_GL_AREA(self.pointer)) }
    }

    /*pub fn set_error(&self, error: glib::Error) {
        unsafe { ffi::gtk_gl_area_set_error(GTK_GL_AREA(self.pointer), error.pointer) }
    }

    pub fn get_error(&self) -> Option<error: glib::Error> {
        unsafe { check_pointer!(ffi::gtk_gl_area_get_error(GTK_GL_AREA(self.pointer)), glib::Error) }
    }
    */

    pub fn set_has_alpha(&self, has_alpha: bool) {
        unsafe { ffi::gtk_gl_area_set_has_alpha(GTK_GL_AREA(self.pointer), to_gboolean(has_alpha)) }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_has_alpha(GTK_GL_AREA(self.pointer))) }
    }

    pub fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe { ffi::gtk_gl_area_set_has_depth_buffer(GTK_GL_AREA(self.pointer), to_gboolean(has_depth_buffer)) }
    }

    pub fn get_has_depth_buffer(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_has_depth_buffer(GTK_GL_AREA(self.pointer))) }
    }

    pub fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe { ffi::gtk_gl_area_set_has_stencil_buffer(GTK_GL_AREA(self.pointer), to_gboolean(has_stencil_buffer)) }
    }

    pub fn get_has_stencil_buffer(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_has_stencil_buffer(GTK_GL_AREA(self.pointer))) }
    }

    pub fn set_auto_render(&self, auto_render: bool) {
        unsafe { ffi::gtk_gl_area_set_auto_render(GTK_GL_AREA(self.pointer), to_gboolean(auto_render)) }
    }

    pub fn get_auto_render(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_auto_render(GTK_GL_AREA(self.pointer))) }
    }

    pub fn get_required_version(&self) -> (i32, i32) {
        let mut major = 0;
        let mut minor = 0;

        unsafe {
            ffi::gtk_gl_area_get_required_version(GTK_GL_AREA(self.pointer), &mut major, &mut minor)
        };
        (major, minor)
    }

    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe { ffi::gtk_gl_area_set_required_version(GTK_GL_AREA(self.pointer), major, minor) }
    }
}

impl_drop!(GLArea);
impl_TraitWidget!(GLArea);
