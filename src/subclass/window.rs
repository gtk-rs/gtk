use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use super::bin::BinImpl;
use BinClass;
use Widget;
use Window;
use WindowClass;

pub trait WindowImpl: WindowImplExt + BinImpl + 'static {
    fn set_focus(&self, window: &Window, focus: Option<&Widget>) {
        self.parent_set_focus(window, focus)
    }

    fn activate_focus(&self, window: &Window) {
        self.parent_activate_focus(window)
    }

    fn activate_default(&self, window: &Window) {
        self.parent_activate_default(window)
    }

    fn keys_changed(&self, window: &Window) {
        self.parent_keys_changed(window)
    }

    fn enable_debugging(&self, window: &Window, toggle: bool) -> bool {
        self.parent_enable_debugging(window, toggle)
    }
}

pub trait WindowImplExt {
    fn parent_set_focus(&self, window: &Window, focus: Option<&Widget>);
    fn parent_activate_focus(&self, window: &Window);
    fn parent_activate_default(&self, window: &Window);
    fn parent_keys_changed(&self, window: &Window);
    fn parent_enable_debugging(&self, window: &Window, toggle: bool) -> bool;
}

impl<T: WindowImpl + ObjectImpl> WindowImplExt for T {
    fn parent_set_focus(&self, window: &Window, focus: Option<&Widget>) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).set_focus {
                f(window.to_glib_none().0, focus.to_glib_none().0)
            }
        }
    }

    fn parent_activate_focus(&self, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).activate_focus {
                f(window.to_glib_none().0)
            }
        }
    }

    fn parent_activate_default(&self, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).activate_default {
                f(window.to_glib_none().0)
            }
        }
    }

    fn parent_keys_changed(&self, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).keys_changed {
                f(window.to_glib_none().0)
            }
        }
    }

    fn parent_enable_debugging(&self, window: &Window, toggle: bool) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            if let Some(f) = (*parent_class).enable_debugging {
                from_glib(f(window.to_glib_none().0, toggle.to_glib()))
            } else {
                false
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + WindowImpl> IsSubclassable<T> for WindowClass {
    fn override_vfuncs(&mut self) {
        <BinClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkWindowClass);
            klass.set_focus = Some(window_set_focus::<T>);
            klass.activate_focus = Some(window_activate_focus::<T>);
            klass.activate_default = Some(window_activate_default::<T>);
            klass.keys_changed = Some(window_keys_changed::<T>);
            klass.enable_debugging = Some(window_enable_debugging::<T>);
        }
    }
}

unsafe extern "C" fn window_set_focus<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWindow,
    widgetptr: *mut gtk_sys::GtkWidget,
) where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);
    let widget: Option<Widget> = from_glib_borrow(widgetptr);

    imp.set_focus(&wrap, widget.as_ref())
}

unsafe extern "C" fn window_activate_focus<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWindow)
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.activate_focus(&wrap)
}

unsafe extern "C" fn window_activate_default<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWindow)
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.activate_default(&wrap)
}

unsafe extern "C" fn window_keys_changed<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWindow)
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.keys_changed(&wrap)
}

unsafe extern "C" fn window_enable_debugging<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWindow,
    toggleptr: glib_sys::gboolean,
) -> glib_sys::gboolean
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);
    let toggle: bool = from_glib(toggleptr);

    imp.enable_debugging(&wrap, toggle).to_glib()
}
