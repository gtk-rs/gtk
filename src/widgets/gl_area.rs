// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget for custom drawing with OpenGL

use cast::GTK_GL_AREA;
use ffi;
use glib::{to_bool, to_gboolean};
use glib::translate::from_glib_none;

///
/// GtkGLArea sets up its own GdkGLContext for the window it creates, and creates a custom GL framebuffer that the widget will do GL rendering onto. It also ensures that this framebuffer is the default GL rendering target when rendering.
///
/// In order to draw, you have to connect to the "render" signal, or subclass GtkGLArea and override the GtkGLAreaClass.render() virtual function.
///
/// The GtkGLArea widget ensures that the GdkGLContext is associated with the widget's drawing area, and it is kept updated when the size and position of the drawing area changes.
/// Drawing with GtkGLArea
///
/// The simplest way to draw using OpenGL commands in a GtkGLArea is to create a widget instance and connect to the "render" signal:
///
/// ```Rust,ignore
/// // create a GtkGLArea instance
/// GtkWidget *gl_area = gtk_gl_area_new ();
///
/// // connect to the "render" signal
/// g_signal_connect (gl_area, "render", G_CALLBACK (render), NULL);
/// ```
///
/// The render() function will be called when the GtkGLArea is ready for you to draw its content:
///
/// ```Rust,ignore
/// fn render (area: &gtk::GLArea, context: &gdk::GLContext) -> bool {
///     // inside this function it's safe to use GL; the given
///     // #GdkGLContext has been made current to the drawable
///     // surface used by the #GtkGLArea and the viewport has
///     // already been set to be the size of the allocation
/// 
///     // we can start by clearing the buffer
///     glClearColor(0, 0, 0, 0);
///     glClear(GL_COLOR_BUFFER_BIT);
///
///     // draw your object
///     draw_an_object();
///
///     // we completed our drawing; the draw commands will be
///     // flushed at the end of the signal emission chain, and
///     // the buffers will be drawn on the window
///     true
/// }
/// ```
///
/// If you need to initialize OpenGL state, e.g. buffer objects or shaders, you should use the "realize" signal; you can use the "unrealize" signal to clean up. Since the GdkGLContext creation and initialization may fail, you will need to check for errors, using gtk_gl_area_get_error(). An example of how to safely initialize the GL state is:
///
/// ```Rust,ignore
/// fn on_realize (area: &GLarea) {
///     // We need to make the context current if we want to
///     // call GL API
///     area.make_current();
///
///     // If there were errors during the initialization or
///     // when trying to make the context current, this
///     // function will return a #GError for you to catch
///     if area.get_error.is_some() {
///         return;
///     }
///
///     // You can also use gtk_gl_area_set_error() in order
///     // to show eventual initialization errors on the
///     // GtkGLArea widget itself
///     let internal_error = NULL;
///     match init_buffer_objects() {
///         Some(e) => {
///             area.set_error(e);
///             return;
///         }
///         None => {}
///     }
///
///     match init_shaders() {
///         Some(e) => {
///             area.set_error(e);
///             return;
///         }
///         None => {}
///     }
/// }
/// ```
///
/// If you need to change the options for creating the GdkGLContext you should use the "create-context" signal.
struct_Widget!(GLArea);

impl GLArea {
    /// Creates a new GLArea widget.
    pub fn new() -> Option<GLArea> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_gl_area_new() };
        check_pointer!(tmp_pointer, GLArea)
    }

    /// Retrieves the gdk::GLContext used by area.
    pub fn get_context(&self) -> Option<::gdk::GLContext> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_context(GTK_GL_AREA(self.pointer))) }
    }

    /// Ensures that the gdk::GLContext used by area is associated with the gtk::GLArea.
    ///
    /// This function is automatically called before emitting the "render" signal, and doesn't
    /// normally need to be called by application code.
    pub fn make_current(&self) {
        unsafe { ffi::gtk_gl_area_make_current(GTK_GL_AREA(self.pointer)) }
    }

    /// Marks the currently rendered data (if any) as invalid, and queues a redraw of the widget,
    /// ensuring that the "render" signal is emitted during the draw.
    ///
    /// This is only needed when the GLArea::set_auto_render() has been called with a false value.
    /// The default behaviour is to emit "render" on each draw.
    pub fn queue_render(&self) {
        unsafe { ffi::gtk_gl_area_queue_render(GTK_GL_AREA(self.pointer)) }
    }

    /// Ensures that the area framebuffer object is made the current draw and read target, and that
    /// all the required buffers for the area are created and bound to the frambuffer.
    ///
    /// This function is automatically called before emitting the "render" signal, and doesn't
    /// normally need to be called by application code.
    pub fn attach_buffers(&self) {
        unsafe { ffi::gtk_gl_area_attach_buffers(GTK_GL_AREA(self.pointer)) }
    }

    /// Sets an error on the area which will be shown instead of the GL rendering. This is useful in
    /// the "create-context" signal if GL context creation fails.
    /*pub fn set_error(&self, error: glib::Error) {
        unsafe { ffi::gtk_gl_area_set_error(GTK_GL_AREA(self.pointer), error.pointer) }
    }

    /// Gets the current error set on the area.
    pub fn get_error(&self) -> Option<error: glib::Error> {
        unsafe { check_pointer!(ffi::gtk_gl_area_get_error(GTK_GL_AREA(self.pointer)), glib::Error) }
    }
    */

    /// If has_alpha is true the buffer allocated by the widget will have an alpha channel component,
    /// and when rendering to the window the result will be composited over whatever is below the widget.
    ///
    /// If has_alpha is false there will be no alpha channel, and the buffer will fully replace
    /// anything below the widget.
    pub fn set_has_alpha(&self, has_alpha: bool) {
        unsafe { ffi::gtk_gl_area_set_has_alpha(GTK_GL_AREA(self.pointer), to_gboolean(has_alpha)) }
    }

    /// Returns whether the area has an alpha component.
    pub fn get_has_alpha(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_has_alpha(GTK_GL_AREA(self.pointer))) }
    }

    /// If has_depth_buffer is true the widget will allocate and enable a depth buffer for the target
    /// framebuffer. Otherwise there will be none.
    pub fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe { ffi::gtk_gl_area_set_has_depth_buffer(GTK_GL_AREA(self.pointer), to_gboolean(has_depth_buffer)) }
    }

    /// Returns whether the area has a depth buffer.
    pub fn get_has_depth_buffer(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_has_depth_buffer(GTK_GL_AREA(self.pointer))) }
    }

    /// If has_stencil_buffer is true the widget will allocate and enable a stencil buffer for the
    /// target framebuffer. Otherwise there will be none.
    pub fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe { ffi::gtk_gl_area_set_has_stencil_buffer(GTK_GL_AREA(self.pointer), to_gboolean(has_stencil_buffer)) }
    }

    /// Returns whether the area has a stencil buffer.
    pub fn get_has_stencil_buffer(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_has_stencil_buffer(GTK_GL_AREA(self.pointer))) }
    }

    /// If auto_render is true the “render” signal will be emitted every time the widget draws. This
    /// is the default and is useful if drawing the widget is faster.
    ///
    /// If auto_render is false the data from previous rendering is kept around and will be used for
    /// drawing the widget the next time, unless the window is resized. In order to force a rendering
    /// GLArea::queue_render() must be called. This mode is useful when the scene changes seldomly,
    /// but takes a long time to redraw.
    pub fn set_auto_render(&self, auto_render: bool) {
        unsafe { ffi::gtk_gl_area_set_auto_render(GTK_GL_AREA(self.pointer), to_gboolean(auto_render)) }
    }

    /// Returns whether the area is in auto render mode or not.
    pub fn get_auto_render(&self) -> bool {
        unsafe { to_bool(ffi::gtk_gl_area_get_auto_render(GTK_GL_AREA(self.pointer))) }
    }

    /// Retrieves the required version of OpenGL set using GLArea::set_required_version().
    pub fn get_required_version(&self) -> (i32, i32) {
        let mut major = 0;
        let mut minor = 0;

        unsafe {
            ffi::gtk_gl_area_get_required_version(GTK_GL_AREA(self.pointer), &mut major, &mut minor)
        };
        (major, minor)
    }

    /// Sets the required version of OpenGL to be used when creating the context for the widget.
    ///
    /// This function must be called before the area has been realized.
    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe { ffi::gtk_gl_area_set_required_version(GTK_GL_AREA(self.pointer), major, minor) }
    }
}

impl_drop!(GLArea);
impl_TraitWidget!(GLArea);
