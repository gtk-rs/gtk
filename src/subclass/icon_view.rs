use glib_sys;
use gtk_sys;

use libc::c_int;

use glib::subclass::prelude::*;
use glib::translate::*;

use super::container::ContainerImpl;
use ContainerClass;
use IconView;
use IconViewClass;
use MovementStep;
use TreePath;

pub trait IconViewImpl: IconViewImplExt + ContainerImpl {
    fn item_activated(&self, icon_view: &IconView, path: &TreePath) {
        self.parent_item_activated(icon_view, path)
    }
    fn selection_changed(&self, icon_view: &IconView) {
        self.parent_selection_changed(icon_view)
    }
    fn select_all(&self, icon_view: &IconView) {
        self.parent_select_all(icon_view)
    }
    fn unselect_all(&self, icon_view: &IconView) {
        self.parent_unselect_all(icon_view)
    }
    fn select_cursor_item(&self, icon_view: &IconView) {
        self.parent_select_cursor_item(icon_view)
    }
    fn toggle_cursor_item(&self, icon_view: &IconView) {
        self.parent_toggle_cursor_item(icon_view)
    }
    fn move_cursor(&self, icon_view: &IconView, step: MovementStep, count: i32) -> bool {
        self.parent_move_cursor(icon_view, step, count)
    }
    fn activate_cursor_item(&self, icon_view: &IconView) -> bool {
        self.parent_activate_cursor_item(icon_view)
    }
}

pub trait IconViewImplExt {
    fn parent_item_activated(&self, icon_view: &IconView, path: &TreePath);
    fn parent_selection_changed(&self, icon_view: &IconView);
    fn parent_select_all(&self, icon_view: &IconView);
    fn parent_unselect_all(&self, icon_view: &IconView);
    fn parent_select_cursor_item(&self, icon_view: &IconView);
    fn parent_toggle_cursor_item(&self, icon_view: &IconView);
    fn parent_move_cursor(&self, icon_view: &IconView, step: MovementStep, count: i32) -> bool;
    fn parent_activate_cursor_item(&self, icon_view: &IconView) -> bool;
}

impl<T: IconViewImpl> IconViewImplExt for T {
    fn parent_item_activated(&self, icon_view: &IconView, path: &TreePath) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).item_activated {
                f(
                    icon_view.to_glib_none().0,
                    mut_override(path.to_glib_none().0),
                )
            }
        }
    }

    fn parent_selection_changed(&self, icon_view: &IconView) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).selection_changed {
                f(icon_view.to_glib_none().0)
            }
        }
    }

    fn parent_select_all(&self, icon_view: &IconView) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).select_all {
                f(icon_view.to_glib_none().0)
            }
        }
    }

    fn parent_unselect_all(&self, icon_view: &IconView) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).unselect_all {
                f(icon_view.to_glib_none().0)
            }
        }
    }

    fn parent_select_cursor_item(&self, icon_view: &IconView) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).select_cursor_item {
                f(icon_view.to_glib_none().0)
            }
        }
    }

    fn parent_toggle_cursor_item(&self, icon_view: &IconView) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).toggle_cursor_item {
                f(icon_view.to_glib_none().0)
            }
        }
    }

    fn parent_move_cursor(&self, icon_view: &IconView, step: MovementStep, count: i32) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).move_cursor {
                from_glib(f(icon_view.to_glib_none().0, step.to_glib(), count))
            } else {
                false
            }
        }
    }

    fn parent_activate_cursor_item(&self, icon_view: &IconView) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkIconViewClass;
            if let Some(f) = (*parent_class).activate_cursor_item {
                from_glib(f(icon_view.to_glib_none().0))
            } else {
                false
            }
        }
    }
}

unsafe impl<T: IconViewImpl> IsSubclassable<T> for IconViewClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkIconViewClass);
            klass.item_activated = Some(icon_view_item_activated::<T>);
            klass.selection_changed = Some(icon_view_selection_changed::<T>);
            klass.select_all = Some(icon_view_select_all::<T>);
            klass.unselect_all = Some(icon_view_unselect_all::<T>);
            klass.select_cursor_item = Some(icon_view_select_cursor_item::<T>);
            klass.toggle_cursor_item = Some(icon_view_toggle_cursor_item::<T>);
            klass.move_cursor = Some(icon_view_move_cursor::<T>);
            klass.activate_cursor_item = Some(icon_view_activate_cursor_item::<T>);
        }
    }
}

unsafe extern "C" fn icon_view_item_activated<T: IconViewImpl>(
    ptr: *mut gtk_sys::GtkIconView,
    path: *mut gtk_sys::GtkTreePath,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);
    let path = from_glib_borrow(path);

    imp.item_activated(&wrap, &path)
}

unsafe extern "C" fn icon_view_selection_changed<T: IconViewImpl>(ptr: *mut gtk_sys::GtkIconView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.selection_changed(&wrap)
}

unsafe extern "C" fn icon_view_select_all<T: IconViewImpl>(ptr: *mut gtk_sys::GtkIconView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.select_all(&wrap)
}

unsafe extern "C" fn icon_view_unselect_all<T: IconViewImpl>(ptr: *mut gtk_sys::GtkIconView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.unselect_all(&wrap)
}

unsafe extern "C" fn icon_view_select_cursor_item<T: IconViewImpl>(ptr: *mut gtk_sys::GtkIconView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.select_cursor_item(&wrap)
}

unsafe extern "C" fn icon_view_toggle_cursor_item<T: IconViewImpl>(ptr: *mut gtk_sys::GtkIconView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.toggle_cursor_item(&wrap)
}

unsafe extern "C" fn icon_view_move_cursor<T: IconViewImpl>(
    ptr: *mut gtk_sys::GtkIconView,
    step: gtk_sys::GtkMovementStep,
    count: c_int,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.move_cursor(&wrap, from_glib(step), count).to_glib()
}

unsafe extern "C" fn icon_view_activate_cursor_item<T: IconViewImpl>(
    ptr: *mut gtk_sys::GtkIconView,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);

    imp.activate_cursor_item(&wrap).to_glib()
}
