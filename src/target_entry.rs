use glib;
use glib::translate::*;
use gtk_sys;
use libc::c_char;
use std::ffi::CStr;
use std::mem;
use TargetFlags;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TargetEntry {
    target: String,
    flags: TargetFlags,
    info: u32,
}

impl TargetEntry {
    pub fn new(target: &str, flags: TargetFlags, info: u32) -> TargetEntry {
        assert_initialized_main_thread!();
        TargetEntry {
            target: target.to_owned(),
            flags,
            info,
        }
    }

    pub fn get_target(&self) -> &str {
        &self.target
    }

    pub fn get_flags(&self) -> TargetFlags {
        self.flags
    }

    pub fn get_info(&self) -> u32 {
        self.info
    }
}

#[doc(hidden)]
impl Uninitialized for TargetEntry {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gtk_sys::GtkTargetEntry> for TargetEntry {
    type Storage = (Box<gtk_sys::GtkTargetEntry>, Stash<'a, *mut c_char, String>);

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gtk_sys::GtkTargetEntry, Self> {
        let target = self.target.to_glib_none();

        let target_entry = Box::new(gtk_sys::GtkTargetEntry {
            target: target.0,
            flags: self.flags.bits(),
            info: self.info,
        });
        Stash(&*target_entry, (target_entry, target))
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gtk_sys::GtkTargetEntry> for TargetEntry {
    type Storage = (Box<gtk_sys::GtkTargetEntry>, Stash<'a, *mut c_char, String>);

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gtk_sys::GtkTargetEntry, Self> {
        let target = self.target.to_glib_none();

        let mut target_entry = Box::new(gtk_sys::GtkTargetEntry {
            target: target.0,
            flags: self.flags.bits(),
            info: self.info,
        });
        StashMut(&mut *target_entry, (target_entry, target))
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const gtk_sys::GtkTargetEntry> for TargetEntry {
    unsafe fn from_glib_none(ptr: *const gtk_sys::GtkTargetEntry) -> Self {
        TargetEntry {
            target: CStr::from_ptr((*ptr).target).to_string_lossy().into_owned(),
            flags: TargetFlags::from_bits((*ptr).flags).unwrap(),
            info: (*ptr).info,
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut gtk_sys::GtkTargetEntry> for TargetEntry {
    unsafe fn from_glib_none(ptr: *mut gtk_sys::GtkTargetEntry) -> Self {
        TargetEntry {
            target: CStr::from_ptr((*ptr).target).to_string_lossy().into_owned(),
            flags: TargetFlags::from_bits((*ptr).flags).unwrap(),
            info: (*ptr).info,
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gtk_sys::GtkTargetEntry> for TargetEntry {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gtk_sys::GtkTargetEntry) -> Self {
        let target_entry = TargetEntry {
            target: CStr::from_ptr((*ptr).target).to_string_lossy().into_owned(),
            flags: TargetFlags::from_bits((*ptr).flags).unwrap(),
            info: (*ptr).info,
        };
        gtk_sys::gtk_target_entry_free(ptr);
        target_entry
    }
}

impl glib::StaticType for TargetEntry {
    fn static_type() -> glib::types::Type {
        skip_assert_initialized!();
        unsafe { from_glib(gtk_sys::gtk_target_entry_get_type()) }
    }
}
