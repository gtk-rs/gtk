//! Translation between GLib/GLib-based FFI types and their Rust counterparts
//!
//! This module allows library bindings authors to decouple type translation
//! logic and use unified idioms at FFI boundaries. It also implements
//! translation of GLib core data types.
//!
//! `FromGlib`, `from_glib` and `ToGlib` translate simple types like `bool`.
//!
//! ```ignore
//!     pub fn set_accept_focus(&self, accept_focus: bool) {
//!         unsafe { ffi::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
//!     }
//!
//!     pub fn get_accept_focus(&self) -> bool {
//!         unsafe { from_glib(ffi::gdk_window_get_accept_focus(self.pointer)) }
//!     }
//! ```
//!
//! `FromGlibPtr`, `FromGlibPtrNotNull` and `ToGlibPtr` work on `gpointer`s
//! and support different ways of managing ownership.
//!
//! ```ignore
//!     fn get_title(&self) -> Option<String> {
//!         unsafe {
//!             let title = ffi::gtk_window_get_title(self.pointer);
//!             FromNativePtr::borrow(title)
//!         }
//!     }
//! ```
//!
//! Letting the foreign library borrow pointers from the Rust side often
//! requires having a temporary variable of an intermediate type (e.g. `CString`).
//! In such cases `ToTmp` is used. See also `StackBox`.
//!
//! ```ignore
//!     pub fn set_icon_name(&self, name: &str) {
//!         unsafe {
//!             let mut tmp_name = name.to_tmp_for_borrow();
//!             ffi::gdk_window_set_icon_name(self.pointer, tmp_name.to_glib_ptr())
//!         }
//!     }
//! ```

use std::ffi::{CString, CStr};
use std::ptr::{self, PtrExt};
use libc::{c_char};
use ffi;

/// A wrapper that sidesteps coherence checking when implementing the traits
/// for FFI types
///
/// `T` is any type that's passed by pointer.
///
/// `T2` is an optional storage for inner temporary variables of complex types.
///
/// Say you want to pass a `*mut C_GdkWindowAttr` to a function. The `StackBox`
/// will own the `C_GdkWindowAttr` and a `CString` that owns
/// `C_GdkWindowAttr::title` for the duration of its lifetime.
///
/// ```ignore
/// type WindowAttrBox = StackBox<ffi::C_GdkWindowAttr, Option<CString>>;
/// ```
///
/// The `ToTmp` implementation can then use `WindowAttrBox` as its output type
/// and `impl ToGlibPtr for WindowAttrBox` is provided by this module.
pub struct StackBox<T: Sized, T2: Sized = ()> (pub T, pub T2);

/// Translate a simple type
pub trait ToGlib {
    type GlibType;

    fn to_glib(&self) -> Self::GlibType;
}

/// Translate a pointer type
pub trait ToGlibPtr {
    type GlibType;

    fn to_glib_ptr(&mut self) -> Self::GlibType;
}

impl ToGlib for bool {
    type GlibType = ffi::Gboolean;

    fn to_glib(&self) -> ffi::Gboolean {
        if *self { ffi::GTRUE } else { ffi::GFALSE }
    }
}

impl ToGlibPtr for CString {
    type GlibType = *const c_char;

    fn to_glib_ptr(&mut self) -> *const c_char {
        self.as_ptr()
    }
}

impl ToGlibPtr for Option<CString> {
    type GlibType = *const c_char;

    fn to_glib_ptr(&mut self) -> *const c_char {
        match self {
            &mut Some(ref s) => s.as_ptr(),
            &mut None => ptr::null(),
        }
    }
}

impl <T, T2> ToGlibPtr for StackBox<T, T2> {
    type GlibType = *mut T;

    fn to_glib_ptr(&mut self) -> *mut T {
        &mut (*self).0 as  *mut _
    }
}

/// Translate to a temporary intermediate variable
pub trait ToTmp {
    type Tmp;

    fn to_tmp_for_borrow(self) -> Self::Tmp;
}

impl <'a> ToTmp for &'a str {
    type Tmp = CString;

    fn to_tmp_for_borrow(self) -> CString {
        CString::new(self).unwrap()
    }
}

impl <'a> ToTmp for &'a Option<&'a str> {
    type Tmp = Option<CString>;

    fn to_tmp_for_borrow(self) -> Option<CString> {
        match self {
            &Some(ref s) => Some(CString::new(&s[..]).unwrap()),
            &None => None,
        }
    }
}

impl <'a> ToTmp for &'a Option<String> {
    type Tmp = Option<CString>;

    fn to_tmp_for_borrow(self) -> Option<CString> {
        match self {
            &Some(ref s) => Some(CString::new(&s[..]).unwrap()),
            &None => None,
        }
    }
}

/// Translate a simple type
pub trait FromGlib: Sized {
    type GlibType: Sized;

    fn conv(val: Self::GlibType) -> Self;
}

/// Translate a simple type
pub fn from_glib<T: FromGlib>(val: <T as FromGlib>::GlibType) -> T {
    FromGlib::conv(val)
}

impl FromGlib for bool {
    type GlibType = ffi::Gboolean;

    fn conv(val: ffi::Gboolean) -> bool {
        !(val == ffi::GFALSE)
    }
}

/// Translate from a pointer type that can be `NULL`
pub trait FromGlibPtr: Sized {
    type GlibType: PtrExt + Copy;

    unsafe fn borrow(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }

    unsafe fn take(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }

    unsafe fn sink(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }
}

/// Translate from a pointer type guaranteed to never be `NULL`
pub trait FromGlibPtrNotNull: Sized {
    type GlibType: PtrExt + Copy;

    unsafe fn borrow(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }

    unsafe fn take(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }

    unsafe fn sink(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }
}

impl FromGlibPtr for Option<String> {
    type GlibType = *const c_char;

    unsafe fn borrow(ptr: *const c_char) -> Option<String> {
        if ptr.is_null() { None }
        else { Some(FromGlibPtrNotNull::borrow(ptr)) }
    }

    unsafe fn take(ptr: *const c_char) -> Option<String> {
        if ptr.is_null() { None }
        else { Some(FromGlibPtrNotNull::take(ptr)) }
    }
}

impl FromGlibPtrNotNull for String {
    type GlibType = *const c_char;

    unsafe fn borrow(ptr: *const c_char) -> Self {
        debug_assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }

    unsafe fn take(ptr: *const c_char) -> Self {
        debug_assert!(!ptr.is_null());
        let res = String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned();
        ffi::g_free(ptr as *mut _);
        res
    }
}
