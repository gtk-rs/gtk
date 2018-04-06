// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use StateType;
use TextDirection;
use ffi;
use gdk_pixbuf;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconSource(Boxed<ffi::GtkIconSource>);

    match fn {
        copy => |ptr| ffi::gtk_icon_source_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_icon_source_free(ptr),
        get_type => || ffi::gtk_icon_source_get_type(),
    }
}

impl IconSource {
    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn new() -> IconSource {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_source_new())
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_direction(&self) -> TextDirection {
        unsafe {
            from_glib(ffi::gtk_icon_source_get_direction(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_direction_wildcarded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_source_get_direction_wildcarded(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::gtk_icon_source_get_filename(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_icon_source_get_icon_name(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_icon_source_get_pixbuf(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_source_get_size(self.to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_size_wildcarded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_source_get_size_wildcarded(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_state(&self) -> StateType {
        unsafe {
            from_glib(ffi::gtk_icon_source_get_state(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn get_state_wildcarded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_source_get_state_wildcarded(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_direction(&mut self, direction: TextDirection) {
        unsafe {
            ffi::gtk_icon_source_set_direction(self.to_glib_none_mut().0, direction.to_glib());
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_direction_wildcarded(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_icon_source_set_direction_wildcarded(self.to_glib_none_mut().0, setting.to_glib());
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_filename<P: AsRef<std::path::Path>>(&mut self, filename: P) {
        unsafe {
            ffi::gtk_icon_source_set_filename(self.to_glib_none_mut().0, filename.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_icon_name<'a, P: Into<Option<&'a str>>>(&mut self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_icon_source_set_icon_name(self.to_glib_none_mut().0, icon_name.0);
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_pixbuf(&mut self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_icon_source_set_pixbuf(self.to_glib_none_mut().0, pixbuf.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_size(&mut self, size: i32) {
        unsafe {
            ffi::gtk_icon_source_set_size(self.to_glib_none_mut().0, size);
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_size_wildcarded(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_icon_source_set_size_wildcarded(self.to_glib_none_mut().0, setting.to_glib());
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_state(&mut self, state: StateType) {
        unsafe {
            ffi::gtk_icon_source_set_state(self.to_glib_none_mut().0, state.to_glib());
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn set_state_wildcarded(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_icon_source_set_state_wildcarded(self.to_glib_none_mut().0, setting.to_glib());
        }
    }
}

#[cfg_attr(feature = "v3_10", deprecated)]
impl Default for IconSource {
    fn default() -> Self {
        Self::new()
    }
}
