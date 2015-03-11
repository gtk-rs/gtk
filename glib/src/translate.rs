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

use std::iter::IntoIterator;
use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use libc::{c_void, c_char};
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

    fn to_tmp_for_borrow(&self) -> Self::Tmp;
}

impl <S: Str> ToTmp for S {
    type Tmp = CString;

    fn to_tmp_for_borrow(&self) -> CString {
        CString::new(self.as_slice()).unwrap()
    }
}

impl <S: Str> ToTmp for Option<S> {
    type Tmp = Option<CString>;

    fn to_tmp_for_borrow(&self) -> Option<CString> {
        match self {
            &Some(ref s) => Some(CString::new(s.as_slice()).unwrap()),
            &None => None,
        }
    }
}

/// Translate to an intermediate variable for passing an array
pub trait ToArray {
    type Tmp;

    fn to_array_for_borrow(self) -> Self::Tmp;
}

impl <'a, T, I> ToArray for I
where T: ToTmp, <T as ToTmp>::Tmp: ToGlibPtr, I: IntoIterator<Item = &'a T> {
    type Tmp = PtrArray<T>;

    fn to_array_for_borrow(self) -> PtrArray<T> {
        let mut tmp_vec: Vec<_> =
            self.into_iter().map(|v| v.to_tmp_for_borrow()).collect();
        let mut ptr_vec: Vec<_> =
            tmp_vec.iter_mut().map(|v| v.to_glib_ptr()).collect();
        unsafe {
            let zero = mem::zeroed();
            ptr_vec.push(zero);
        }
        PtrArray(ptr_vec, tmp_vec)
    }
}

/// Temporary storage for passing array of pointers
pub struct PtrArray<T> (Vec<<<T as ToTmp>::Tmp as ToGlibPtr>::GlibType>,
                      Vec<<T as ToTmp>::Tmp>)
    where T: ToTmp, <T as ToTmp>::Tmp: ToGlibPtr;

impl <T> PtrArray<T>
    where T: ToTmp, <T as ToTmp>::Tmp: ToGlibPtr {
    pub fn len(&self) -> usize {
        self.1.len()
    }
}

impl <T> ToGlibPtr for PtrArray<T>
    where T: ToTmp, <T as ToTmp>::Tmp: ToGlibPtr {
    type GlibType = *mut <<T as ToTmp>::Tmp as ToGlibPtr>::GlibType;

    fn to_glib_ptr(&mut self) -> *mut <<T as ToTmp>::Tmp as ToGlibPtr>::GlibType {
        self.0.as_mut_ptr()
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

    /// Borrow the reference
    unsafe fn borrow(ptr: Self::GlibType) -> Self;

    /// Take ownership of the reference
    unsafe fn take(ptr: Self::GlibType) -> Self;

    /// Take ownership of the floating reference
    unsafe fn sink(_ptr: Self::GlibType) -> Self {
        panic!("Invalid operation for this type");
    }
}

/// Translate from a pointer type guaranteed to never be `NULL`
pub trait FromGlibPtrNotNull: Sized {
    type GlibType: PtrExt + Copy;

    /// Borrow the reference
    unsafe fn borrow(ptr: Self::GlibType) -> Self;

    /// Take ownership of the reference
    unsafe fn take(ptr: Self::GlibType) -> Self;

    /// Take ownership of the floating reference
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
        let res = FromGlibPtrNotNull::borrow(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

/// Translate from a container of pointers
pub trait FromGlibPtrContainer<GT: PtrExt + Copy>: Sized {
    /// Borrow the references
    unsafe fn borrow(ptr: GT) -> Self;

    /// Borrow the references with an advised expected number
    unsafe fn borrow_num(ptr: GT, _num: usize) -> Self;

    /// Take ownership of the container but borrow its contents
    unsafe fn take_outer(ptr: GT) -> Self;

    /// Take ownership of the container but borrow its contents with an advised expected number
    unsafe fn take_outer_num(ptr: GT, _num: usize) -> Self;

    /// Take ownership of the references
    unsafe fn take(ptr: GT) -> Self;

    /// Take ownership of the references with an advised expected number
    unsafe fn take_num(ptr: GT, _num: usize) -> Self;
}

unsafe fn c_array_len<P: PtrExt + Copy>(mut ptr: *const P) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let mut len = 0;
    while !(*ptr).is_null() {
        len += 1;
        ptr = ptr.offset(1);
    }
    len
}

impl <T: FromGlibPtrNotNull>
FromGlibPtrContainer<*const <T as FromGlibPtrNotNull>::GlibType>
for Vec<T> {
    unsafe fn borrow(ptr: *const <T as FromGlibPtrNotNull>::GlibType) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::borrow_num(ptr, num)
    }

    unsafe fn borrow_num(mut ptr: *const <T as FromGlibPtrNotNull>::GlibType,
                         num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(FromGlibPtrNotNull::borrow(*ptr));
            ptr = ptr.offset(1);
        }
        res
    }

    unsafe fn take_outer(ptr: *const <T as FromGlibPtrNotNull>::GlibType) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_outer_num(ptr: *const <T as FromGlibPtrNotNull>::GlibType,
                             num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::borrow_num(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn take(ptr: *const <T as FromGlibPtrNotNull>::GlibType) -> Vec<T> {
        let num = c_array_len(ptr);
        FromGlibPtrContainer::take_num(ptr, num)
    }

    unsafe fn take_num(mut ptr: *const <T as FromGlibPtrNotNull>::GlibType,
                       num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !(*ptr).is_null() {
            res.push(FromGlibPtrNotNull::take(*ptr));
            ptr = ptr.offset(1);
        }
        ffi::g_free(ptr as *mut _);
        res
    }
}

unsafe fn slist_len(mut ptr: *mut ffi::C_GSList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <T: FromGlibPtrNotNull> FromGlibPtrContainer<*mut ffi::C_GSList> for Vec<T> {
    unsafe fn borrow(ptr: *mut ffi::C_GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::borrow_num(ptr, num)
    }

    unsafe fn borrow_num(mut ptr: *mut ffi::C_GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::borrow(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn take_outer(ptr: *mut ffi::C_GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_outer_num(ptr: *mut ffi::C_GSList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::borrow_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_slist_free(ptr as *mut _);
        }
        res
    }

    unsafe fn take(ptr: *mut ffi::C_GSList) -> Vec<T> {
        let num = slist_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_num(mut ptr: *mut ffi::C_GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::take(item_ptr));
            }
            ptr = (*ptr).next;
        }
        if !orig_ptr.is_null() {
            ffi::g_slist_free(orig_ptr as *mut _);
        }
        res
    }
}

unsafe fn list_len(mut ptr: *mut ffi::C_GList) -> usize {
    let mut len = 0;
    while !ptr.is_null() {
        ptr = (*ptr).next;
        len += 1;
    }
    len
}

impl <T: FromGlibPtrNotNull> FromGlibPtrContainer<*mut ffi::C_GList> for Vec<T> {
    unsafe fn borrow(ptr: *mut ffi::C_GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::borrow_num(ptr, num)
    }

    unsafe fn borrow_num(mut ptr: *mut ffi::C_GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::borrow(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn take_outer(ptr: *mut ffi::C_GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_outer_num(ptr: *mut ffi::C_GList, num: usize) -> Vec<T> {
        let res = FromGlibPtrContainer::borrow_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_list_free(ptr as *mut _);
        }
        res
    }

    unsafe fn take(ptr: *mut ffi::C_GList) -> Vec<T> {
        let num = list_len(ptr);
        FromGlibPtrContainer::take_outer_num(ptr, num)
    }

    unsafe fn take_num(mut ptr: *mut ffi::C_GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new()
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        while !ptr.is_null() {
            let mut item_ptr: <T as FromGlibPtrNotNull>::GlibType = mem::uninitialized();
            // item_ptr is a pointer but the compiler doesn't know
            let hack: *mut *mut c_void = mem::transmute(&mut item_ptr);
            *hack = (*ptr).data;
            if !item_ptr.is_null() {
                res.push(FromGlibPtrNotNull::take(item_ptr));
            }
            ptr = (*ptr).next;
        }
        if !orig_ptr.is_null() {
            ffi::g_list_free(orig_ptr as *mut _);
        }
        res
    }
}
