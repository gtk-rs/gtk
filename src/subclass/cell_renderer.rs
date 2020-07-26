use cairo_sys;
use gdk_sys;
use glib_sys;
use gtk_sys;

use libc::{c_char, c_int};
use std::mem;

use cairo;
use glib::object::IsA;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::GString;
use glib::ObjectClass;

use CellEditable;
use CellRenderer;
use CellRendererClass;
use CellRendererState;
use SizeRequestMode;
use Widget;

pub trait CellRendererImpl: CellRendererImplExt + ObjectImpl {
    fn get_request_mode(&self, renderer: &CellRenderer) -> SizeRequestMode {
        self.parent_get_request_mode(renderer)
    }

    fn get_preferred_width<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
    ) -> (i32, i32) {
        self.parent_get_preferred_width(renderer, widget)
    }

    fn get_preferred_width_for_height<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        height: i32,
    ) -> (i32, i32) {
        self.parent_get_preferred_width_for_height(renderer, widget, height)
    }

    fn get_preferred_height<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
    ) -> (i32, i32) {
        self.parent_get_preferred_height(renderer, widget)
    }

    fn get_preferred_height_for_width<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        width: i32,
    ) -> (i32, i32) {
        self.parent_get_preferred_height_for_width(renderer, widget, width)
    }

    fn get_aligned_area<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        self.parent_get_aligned_area(renderer, widget, flags, cell_area)
    }

    fn render<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        cr: &cairo::Context,
        widget: &P,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) {
        self.parent_render(renderer, cr, widget, background_area, cell_area, flags);
    }

    fn activate<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        self.parent_activate(
            renderer,
            event,
            widget,
            path,
            background_area,
            cell_area,
            flags,
        )
    }

    fn start_editing<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable> {
        self.parent_start_editing(
            renderer,
            event,
            widget,
            path,
            background_area,
            cell_area,
            flags,
        )
    }

    fn editing_canceled(&self, renderer: &CellRenderer) {
        self.parent_editing_canceled(renderer)
    }

    fn editing_started(&self, renderer: &CellRenderer, editable: &CellEditable, path: &str) {
        self.parent_editing_started(renderer, editable, path)
    }
}

pub trait CellRendererImplExt {
    fn parent_get_request_mode(&self, renderer: &CellRenderer) -> SizeRequestMode;
    fn parent_get_preferred_width<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
    ) -> (i32, i32);
    fn parent_get_preferred_width_for_height<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        height: i32,
    ) -> (i32, i32);
    fn parent_get_preferred_height<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
    ) -> (i32, i32);
    fn parent_get_preferred_height_for_width<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        width: i32,
    ) -> (i32, i32);
    fn parent_get_aligned_area<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle;
    fn parent_render<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        cr: &cairo::Context,
        widget: &P,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    );
    fn parent_activate<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;
    fn parent_start_editing<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable>;
    fn parent_editing_canceled(&self, renderer: &CellRenderer);
    fn parent_editing_started(&self, renderer: &CellRenderer, editable: &CellEditable, path: &str);
}

impl<T: CellRendererImpl> CellRendererImplExt for T {
    fn parent_get_request_mode(&self, renderer: &CellRenderer) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            let f = (*parent_class).get_request_mode.unwrap();
            from_glib(f(renderer.to_glib_none().0))
        }
    }

    fn parent_get_preferred_width<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_width.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_get_preferred_width_for_height<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_width_for_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                height,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_get_preferred_height<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_height.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_get_preferred_height_for_width<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        width: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            let f = (*parent_class).get_preferred_height_for_width.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                renderer.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_get_aligned_area<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        widget: &P,
        flags: CellRendererState,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            let mut aligned_area = gdk::Rectangle::uninitialized();
            let f = (*parent_class).get_aligned_area.unwrap();
            f(
                renderer.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                flags.to_glib(),
                cell_area.to_glib_none().0,
                aligned_area.to_glib_none_mut().0,
            );
            aligned_area
        }
    }

    fn parent_render<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        cr: &cairo::Context,
        widget: &P,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            if let Some(f) = (*parent_class).render {
                f(
                    renderer.to_glib_none().0,
                    cr.to_glib_none().0,
                    widget.as_ref().to_glib_none().0,
                    background_area.to_glib_none().0,
                    cell_area.to_glib_none().0,
                    flags.to_glib(),
                )
            }
        }
    }

    fn parent_activate<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            if let Some(f) = (*parent_class).activate {
                from_glib(f(
                    renderer.to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                    widget.as_ref().to_glib_none().0,
                    path.to_glib_none().0,
                    background_area.to_glib_none().0,
                    cell_area.to_glib_none().0,
                    flags.to_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_start_editing<P: IsA<Widget>>(
        &self,
        renderer: &CellRenderer,
        event: Option<&gdk::Event>,
        widget: &P,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            if let Some(f) = (*parent_class).start_editing {
                from_glib_none(f(
                    renderer.to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                    widget.as_ref().to_glib_none().0,
                    path.to_glib_none().0,
                    background_area.to_glib_none().0,
                    cell_area.to_glib_none().0,
                    flags.to_glib(),
                ))
            } else {
                None
            }
        }
    }

    fn parent_editing_canceled(&self, renderer: &CellRenderer) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            if let Some(f) = (*parent_class).editing_canceled {
                f(renderer.to_glib_none().0)
            }
        }
    }

    fn parent_editing_started(&self, renderer: &CellRenderer, editable: &CellEditable, path: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkCellRendererClass;
            if let Some(f) = (*parent_class).editing_started {
                f(
                    renderer.to_glib_none().0,
                    editable.to_glib_none().0,
                    path.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: CellRendererImpl> IsSubclassable<T> for CellRendererClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkCellRendererClass);
            klass.get_request_mode = Some(cell_renderer_get_request_mode::<T>);
            klass.get_preferred_width = Some(cell_renderer_get_preferred_width::<T>);
            klass.get_preferred_height_for_width =
                Some(cell_renderer_get_preferred_height_for_width::<T>);
            klass.get_preferred_height = Some(cell_renderer_get_preferred_height::<T>);
            klass.get_preferred_width_for_height =
                Some(cell_renderer_get_preferred_width_for_height::<T>);
            klass.get_aligned_area = Some(cell_renderer_get_aligned_area::<T>);
            klass.render = Some(cell_renderer_render::<T>);
            klass.activate = Some(cell_renderer_activate::<T>);
            klass.start_editing = Some(cell_renderer_start_editing::<T>);
            klass.editing_started = Some(cell_renderer_editing_started::<T>);
            klass.editing_canceled = Some(cell_renderer_editing_canceled::<T>);
        }
    }
}

unsafe extern "C" fn cell_renderer_get_request_mode<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
) -> gtk_sys::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);

    imp.get_request_mode(&wrap).to_glib()
}

unsafe extern "C" fn cell_renderer_get_preferred_width<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    wdgtptr: *mut gtk_sys::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_size, nat_size) = imp.get_preferred_width(&wrap, &*widget);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn cell_renderer_get_preferred_height_for_width<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    wdgtptr: *mut gtk_sys::GtkWidget,
    width: c_int,
    min_height_ptr: *mut c_int,
    nat_height_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_height, nat_height) = imp.get_preferred_height_for_width(&wrap, &*widget, width);
    if !min_height_ptr.is_null() {
        *min_height_ptr = min_height;
    }
    if !nat_height_ptr.is_null() {
        *nat_height_ptr = nat_height;
    }
}

unsafe extern "C" fn cell_renderer_get_preferred_height<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    wdgtptr: *mut gtk_sys::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_size, nat_size) = imp.get_preferred_height(&wrap, &*widget);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn cell_renderer_get_preferred_width_for_height<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    wdgtptr: *mut gtk_sys::GtkWidget,
    height: c_int,
    min_width_ptr: *mut c_int,
    nat_width_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_width, nat_width) = imp.get_preferred_width_for_height(&wrap, &*widget, height);
    if !min_width_ptr.is_null() {
        *min_width_ptr = min_width;
    }
    if !nat_width_ptr.is_null() {
        *nat_width_ptr = nat_width;
    }
}

unsafe extern "C" fn cell_renderer_get_aligned_area<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    wdgtptr: *mut gtk_sys::GtkWidget,
    flags: gtk_sys::GtkCellRendererState,
    cellarea: *const gdk_sys::GdkRectangle,
    alignedptr: *mut gdk_sys::GdkRectangle,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let rectangle = imp.get_aligned_area(
        &wrap,
        &*widget,
        from_glib(flags),
        &from_glib_borrow(cellarea),
    );
    *alignedptr = *rectangle.to_glib_none().0;
}

unsafe extern "C" fn cell_renderer_render<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    crptr: *mut cairo_sys::cairo_t,
    wdgtptr: *mut gtk_sys::GtkWidget,
    bgptr: *const gdk_sys::GdkRectangle,
    cellptr: *const gdk_sys::GdkRectangle,
    flags: gtk_sys::GtkCellRendererState,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let cr: Borrowed<cairo::Context> = from_glib_borrow(crptr);

    imp.render(
        &wrap,
        &cr,
        &*widget,
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
    );
}

unsafe extern "C" fn cell_renderer_activate<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    evtptr: *mut gdk_sys::GdkEvent,
    wdgtptr: *mut gtk_sys::GtkWidget,
    pathptr: *const c_char,
    bgptr: *const gdk_sys::GdkRectangle,
    cellptr: *const gdk_sys::GdkRectangle,
    flags: gtk_sys::GtkCellRendererState,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let evt: Borrowed<Option<gdk::Event>> = from_glib_borrow(evtptr);

    imp.activate(
        &wrap,
        evt.as_ref().as_ref(),
        &*widget,
        &GString::from_glib_borrow(pathptr),
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
    )
    .to_glib()
}

unsafe extern "C" fn cell_renderer_start_editing<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    evtptr: *mut gdk_sys::GdkEvent,
    wdgtptr: *mut gtk_sys::GtkWidget,
    pathptr: *const c_char,
    bgptr: *const gdk_sys::GdkRectangle,
    cellptr: *const gdk_sys::GdkRectangle,
    flags: gtk_sys::GtkCellRendererState,
) -> *mut gtk_sys::GtkCellEditable {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let evt: Borrowed<Option<gdk::Event>> = from_glib_borrow(evtptr);

    imp.start_editing(
        &wrap,
        evt.as_ref().as_ref(),
        &*widget,
        &GString::from_glib_borrow(pathptr),
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
    )
    .to_glib_none()
    .0
}

unsafe extern "C" fn cell_renderer_editing_canceled<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);

    imp.editing_canceled(&wrap);
}

unsafe extern "C" fn cell_renderer_editing_started<T: CellRendererImpl>(
    ptr: *mut gtk_sys::GtkCellRenderer,
    editableptr: *mut gtk_sys::GtkCellEditable,
    pathptr: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRenderer> = from_glib_borrow(ptr);
    let editable = from_glib_borrow(editableptr);

    imp.editing_started(&wrap, &editable, &GString::from_glib_borrow(pathptr));
}
