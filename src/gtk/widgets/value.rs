// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! Generic values — A polymorphic type that can hold values of any other type

use gtk;
use gtk::ffi;

trait GValuePrivate {
    fn get(gvalue: &GValue) -> Self;
    fn set(&self, gvalue: &GValue);
}

pub trait GValuePublic: GValuePrivate {}

// Possible improvment : store a function pointer inside the struct and make the struct templated
pub struct GValue {
    pointer: *mut ffi::C_GValue
}

impl GValue {
    pub fn new() -> Option<GValue> {
        let tmp_pointer = unsafe { ffi::create_gvalue() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(GValue {
                pointer: tmp_pointer
            })
        }
    }

    pub fn init(&self, _type: gtk::GType) {
        unsafe { ffi::g_value_init(self.pointer, ffi::get_gtype(_type)) }
    }

    pub fn reset(&self) {
        unsafe { ffi::g_value_reset(self.pointer) }
    }

    pub fn unset(&self) {
        unsafe { ffi::g_value_unset(self.pointer) }
    }

    // to free !
    pub fn strdup_value_contents(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_strdup_value_contents(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { ::std::string::raw::from_buf(tmp_pointer as *const u8) })
        }
    }

    fn set_boolean(&self, v_boolean: bool) {
        unsafe { ffi::g_value_set_boolean(self.pointer, ffi::to_gboolean(v_boolean)) }
    }

    fn get_boolean(&self) -> bool {
        unsafe { ffi::to_bool(ffi::g_value_get_boolean(self.pointer)) }
    }

    fn set_schar(&self, v_char: i8) {
        unsafe { ffi::g_value_set_schar(self.pointer, v_char) }
    }

    fn get_schar(&self) -> i8 {
        unsafe { ffi::g_value_get_schar(self.pointer) }
    }

    fn set_uchar(&self, v_uchar: u8) {
        unsafe { ffi::g_value_set_uchar(self.pointer, v_uchar) }
    }

    fn get_uchar(&self) -> u8 {
        unsafe { ffi::g_value_get_uchar(self.pointer) }
    }

    fn set_int(&self, v_int: i32) {
        unsafe { ffi::g_value_set_int(self.pointer, v_int) }
    }

    fn get_int(&self) -> i32 {
        unsafe { ffi::g_value_get_int(self.pointer) }
    }

    fn set_uint(&self, v_uint: u32) {
        unsafe { ffi::g_value_set_uint(self.pointer, v_uint) }
    }

    fn get_uint(&self) -> u32 {
        unsafe { ffi::g_value_get_uint(self.pointer) }
    }

    pub fn set_long(&self, v_long: i64) {
        unsafe { ffi::g_value_set_long(self.pointer, v_long) }
    }

    pub fn get_long(&self) -> i64 {
        unsafe { ffi::g_value_get_long(self.pointer) }
    }

    pub fn set_ulong(&self, v_ulong: u64) {
        unsafe { ffi::g_value_set_ulong(self.pointer, v_ulong) }
    }

    pub fn get_ulong(&self) -> u64 {
        unsafe { ffi::g_value_get_ulong(self.pointer) }
    }

    fn set_int64(&self, v_int64: i64) {
        unsafe { ffi::g_value_set_int64(self.pointer, v_int64) }
    }

    fn get_int64(&self) -> i64 {
        unsafe { ffi::g_value_get_int64(self.pointer) }
    }

    fn set_uint64(&self, v_uint64: u64) {
        unsafe { ffi::g_value_set_uint64(self.pointer, v_uint64) }
    }

    fn get_uint64(&self) -> u64 {
        unsafe { ffi::g_value_get_uint64(self.pointer) }
    }

    fn set_float(&self, v_float: f32) {
        unsafe { ffi::g_value_set_float(self.pointer, v_float) }
    }

    fn get_float(&self) -> f32 {
        unsafe { ffi::g_value_get_float(self.pointer) }
    }

    fn set_double(&self, v_double: f64) {
        unsafe { ffi::g_value_set_double(self.pointer, v_double) }
    }

    fn get_double(&self) -> f64 {
        unsafe { ffi::g_value_get_double(self.pointer) }
    }

    // FIXME shouldn't be like that
    pub fn set_enum(&self, v_enum: gtk::GType) {
        unsafe { ffi::g_value_set_enum(self.pointer, v_enum) }
    }

    // FIXME shouldn't be like that
    pub fn get_enum(&self) -> gtk::GType {
        unsafe { ffi::g_value_get_enum(self.pointer) }
    }

    // FIXME shouldn't be like that
    pub fn set_flags(&self, v_flags: gtk::GType) {
        unsafe { ffi::g_value_set_flags(self.pointer, v_flags) }
    }

    // FIXME shouldn't be like that
    pub fn get_flags(&self) -> gtk::GType {
        unsafe { ffi::g_value_get_flags(self.pointer) }
    }

    fn set_string(&self, v_string: &str) {
        unsafe {
            v_string.with_c_str(|c_str| {
                ffi::g_value_set_string(self.pointer, c_str)
            })
        }
    }

    /// Set the contents of a G_TYPE_STRING GValue to v_string . The string is assumed to be static, and is thus not duplicated
    /// when setting the GValue.
    pub fn set_static_string(&self, v_string: &str) {
        unsafe {
            v_string.with_c_str(|c_str| {
                ffi::g_value_set_static_string(self.pointer, c_str)
            })
        }
    }

    /*pub fn take_string(&self, v_string: &str) {
        unsafe {
            v_string.with_c_str(|c_str| {
                ffi::g_value_take_string(self.pointer, c_str)
            })
        }
    }*/

    fn get_string(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_value_get_string(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { ::std::string::raw::from_buf(tmp_pointer as *const u8) })
        }
    }

    // to free !!
    pub fn dup_string(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_value_dup_string(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { ::std::string::raw::from_buf(tmp_pointer as *const u8) })
        }
    }

    pub fn set_boxed<T>(&self, v_box: &T) {
        unsafe { ffi::g_value_set_boxed(self.pointer, ::std::mem::transmute(v_box)) }
    }

    /// Set the contents of a G_TYPE_BOXED derived GValue to v_boxed . The boxed value is assumed to be static, and is thus not duplicated
    /// when setting the GValue.
    pub fn set_static_boxed<T>(&self, v_box: &T) {
        unsafe { ffi::g_value_set_static_boxed(self.pointer, ::std::mem::transmute(v_box)) }
    }

    /*pub fn take_boxed<T>(&self, v_box: &T) {
        unsafe { ffi::g_value_take_boxed(self.pointer, ::std::mem::transmute(v_box)) }
    }*/

    pub fn get_boxed<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_get_boxed(self.pointer)) }
    }

    /*pub fn dup_boxed<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_dup_boxed(self.pointer)) }
    }*/

    pub fn set_pointer<T>(&self, v_pointer: &T) {
        unsafe { ffi::g_value_set_pointer(self.pointer, ::std::mem::transmute(v_pointer)) }
    }

    pub fn get_pointer<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_get_pointer(self.pointer)) }
    }

    pub fn set_object<T>(&self, v_object: &T) {
        unsafe { ffi::g_value_set_object(self.pointer, ::std::mem::transmute(v_object)) }
    }

    /// Sets the contents of a G_TYPE_OBJECT derived GValue to v_object and takes over the ownership of the callers reference to
    /// v_object ; the caller doesn't have to unref it any more (i.e. the reference count of the object is not increased).
    pub fn take_object<T>(&self, v_object: &T) {
        unsafe { ffi::g_value_take_object(self.pointer, ::std::mem::transmute(v_object)) }
    }

    pub fn get_object<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_get_object(self.pointer)) }
    }

    /*pub fn dup_object<'r, T>(&'r self) -> &'r T {
        unsafe { ::std::mem::transmute(ffi::g_value_dup_object(self.pointer)) }
    }*/

    // FIXME shouldn't be like that
    fn set_gtype(&self, v_gtype: gtk::GType) {
        unsafe { ffi::g_value_set_gtype(self.pointer, v_gtype) }
    }

    // FIXME shouldn't be like that
    fn get_gtype(&self) -> gtk::GType {
        unsafe { ffi::g_value_get_gtype(self.pointer) }
    }

    pub fn set<T: GValuePublic>(&self, val: &T) {
        val.set(self);
    }

    pub fn get<T: GValuePublic>(&self) -> T {
        GValuePrivate::get(self)
    }

    pub fn compatible(src_type: gtk::GType, dest_type: gtk::GType) -> bool {
        unsafe { ffi::to_bool(ffi::g_value_type_compatible(src_type, dest_type)) }
    }

    pub fn transformable(src_type: gtk::GType, dest_type: gtk::GType) -> bool {
        unsafe { ffi::to_bool(ffi::g_value_type_transformable(src_type, dest_type)) }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GValue {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_value: *mut ffi::C_GValue) -> GValue {
        GValue {
            pointer: c_value
        }
    }
}

impl Drop for GValue {
    fn drop(&mut self) {
        if self.pointer.is_not_null() {
            unsafe { ::libc::funcs::c95::stdlib::free(self.pointer as *mut ::libc::types::common::c95::c_void) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}

impl GValuePrivate for i32 {
    fn get(gvalue: &GValue) -> i32 {
        gvalue.get_int()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_int(*self)
    }
}

impl GValuePrivate for u32 {
    fn get(gvalue: &GValue) -> u32 {
        gvalue.get_uint()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_uint(*self)
    }
}

impl GValuePrivate for i64 {
    fn get(gvalue: &GValue) -> i64 {
        gvalue.get_int64()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_int64(*self)
    }
}

impl GValuePrivate for u64 {
    fn get(gvalue: &GValue) -> u64 {
        gvalue.get_uint64()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_uint64(*self)
    }
}

impl GValuePrivate for bool {
    fn get(gvalue: &GValue) -> bool {
        gvalue.get_boolean()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_boolean(*self)
    }
}

impl GValuePrivate for i8 {
    fn get(gvalue: &GValue) -> i8 {
        gvalue.get_schar()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_schar(*self)
    }
}

impl GValuePrivate for u8 {
    fn get(gvalue: &GValue) -> u8 {
        gvalue.get_uchar()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_uchar(*self)
    }
}

impl GValuePrivate for f32 {
    fn get(gvalue: &GValue) -> f32 {
        gvalue.get_float()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_float(*self)
    }
}

impl GValuePrivate for f64 {
    fn get(gvalue: &GValue) -> f64 {
        gvalue.get_double()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_double(*self)
    }
}

impl GValuePrivate for gtk::GType {
    fn get(gvalue: &GValue) -> gtk::GType {
        gvalue.get_gtype()
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_gtype(*self)
    }
}

impl GValuePrivate for String {
    fn get(gvalue: &GValue) -> String {
        match gvalue.get_string() {
            Some(s) => s,
            None => String::new()
        }
    }

    fn set(&self, gvalue: &GValue) {
        gvalue.set_string(self.as_slice())
    }
}

impl GValuePublic for i32 {}
impl GValuePublic for u32 {}
impl GValuePublic for i64 {}
impl GValuePublic for u64 {}
impl GValuePublic for i8 {}
impl GValuePublic for u8 {}
impl GValuePublic for gtk::GType {}
impl GValuePublic for String {}
impl GValuePublic for f32 {}
impl GValuePublic for f64 {}
impl GValuePublic for bool {}