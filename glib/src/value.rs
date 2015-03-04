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

use ffi;
use libc::{self, c_char, c_void};
use std::ffi::CString;
use super::{to_bool, to_gboolean, Type};
use translate::{ToGlib, ToGlibPtr, ToTmp, from_glib};

pub trait ValuePublic {
    fn get(gvalue: &Value) -> Self;
    fn set(&self, gvalue: &Value);
}

// Possible improvment : store a function pointer inside the struct and make the struct templated
pub struct Value {
    pointer: *mut ffi::C_GValue
}

impl Value {
    pub fn new() -> Option<Value> {
        let tmp_pointer = unsafe { ffi::create_gvalue() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(Value {
                pointer: tmp_pointer
            })
        }
    }

    pub fn init(&self, _type: Type) {
        unsafe { ffi::g_value_init(self.pointer, _type.to_glib()) }
    }

    pub fn reset(&self) {
        unsafe { ffi::g_value_reset(self.pointer) }
    }

    pub fn unset(&self) {
        unsafe { ffi::g_value_unset(self.pointer) }
    }

    // to free !
    pub fn strdup_value_contents(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::g_strdup_value_contents(self.pointer) as *const c_char)
        }
    }

    fn set_boolean(&self, v_boolean: bool) {
        unsafe { ffi::g_value_set_boolean(self.pointer, to_gboolean(v_boolean)) }
    }

    fn get_boolean(&self) -> bool {
        unsafe { to_bool(ffi::g_value_get_boolean(self.pointer)) }
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
        unsafe { ffi::g_value_set_long(self.pointer, v_long as ::libc::c_long) }
    }

    pub fn get_long(&self) -> i64 {
        unsafe { ffi::g_value_get_long(self.pointer) as i64 }
    }

    pub fn set_ulong(&self, v_ulong: u64) {
        unsafe { ffi::g_value_set_ulong(self.pointer, v_ulong as ::libc::c_ulong) }
    }

    pub fn get_ulong(&self) -> u64 {
        unsafe { ffi::g_value_get_ulong(self.pointer) as u64 }
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
    pub fn set_enum(&self, v_enum: Type) {
        unsafe { ffi::g_value_set_enum(self.pointer, v_enum.to_glib()) }
    }

    // FIXME shouldn't be like that
    pub fn get_enum(&self) -> Type {
        unsafe { from_glib(ffi::g_value_get_enum(self.pointer)) }
    }

    // FIXME shouldn't be like that
    pub fn set_flags(&self, v_flags: Type) {
        unsafe { ffi::g_value_set_flags(self.pointer, v_flags.to_glib()) }
    }

    // FIXME shouldn't be like that
    pub fn get_flags(&self) -> Type {
        unsafe { from_glib(ffi::g_value_get_flags(self.pointer)) }
    }

    fn set_string(&self, v_string: &str) {
        let c_str = CString::from_slice(v_string.as_bytes());

        unsafe {
            ffi::g_value_set_string(self.pointer, c_str.as_ptr())
        }
    }

    /// Set the contents of a G_TYPE_STRING Value to v_string . The string is assumed to be static, and is thus not duplicated
    /// when setting the Value.
    pub fn set_static_string(&self, v_string: &str) {
        unsafe {
            let mut tmp_v_string = v_string.to_tmp_for_borrow();
            ffi::g_value_set_static_string(self.pointer, tmp_v_string.to_glib_ptr())
        }
    }

    /*pub fn take_string(&self, v_string: &str) {
        unsafe {
            v_string.with_c_str(|c_str| {
                ffi::g_value_take_string(self.pointer, c_str)
        }
    }*/

    pub fn get_string(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::g_value_get_string(self.pointer))
        }
    }

    pub fn dup_string(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_value_dup_string(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            unsafe {
                let ret = Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&(tmp_pointer as *const c_char))).to_string());

                libc::funcs::c95::stdlib::free(tmp_pointer as *mut c_void);
                ret
            }
        }
    }

    pub fn set_boxed<T>(&self, v_box: &T) {
        unsafe { ffi::g_value_set_boxed(self.pointer, ::std::mem::transmute(v_box)) }
    }

    /// Set the contents of a G_TYPE_BOXED derived Value to v_boxed . The boxed value is assumed to be static, and is thus not duplicated
    /// when setting the Value.
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

    /// Sets the contents of a G_TYPE_OBJECT derived Value to v_object and takes over the ownership of the callers reference to
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
    fn set_gtype(&self, v_gtype: Type) {
        unsafe { ffi::g_value_set_gtype(self.pointer, v_gtype.to_glib()) }
    }

    // FIXME shouldn't be like that
    fn get_gtype(&self) -> Type {
        unsafe { from_glib(ffi::g_value_get_gtype(self.pointer)) }
    }

    pub fn set<T: ValuePublic>(&self, val: &T) {
        val.set(self);
    }

    pub fn get<T: ValuePublic>(&self) -> T {
        ValuePublic::get(self)
    }

    pub fn compatible(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(ffi::g_value_type_compatible(src_type.to_glib(), dest_type.to_glib())) }
    }

    pub fn transformable(src_type: Type, dest_type: Type) -> bool {
        unsafe { to_bool(ffi::g_value_type_transformable(src_type.to_glib(), dest_type.to_glib())) }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GValue {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_value: *mut ffi::C_GValue) -> Value {
        Value {
            pointer: c_value
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            unsafe { ::libc::funcs::c95::stdlib::free(self.pointer as *mut ::libc::types::common::c95::c_void) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}

impl ValuePublic for i32 {
    fn get(gvalue: &Value) -> i32 {
        gvalue.get_int()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_int(*self)
    }
}

impl ValuePublic for u32 {
    fn get(gvalue: &Value) -> u32 {
        gvalue.get_uint()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_uint(*self)
    }
}

impl ValuePublic for i64 {
    fn get(gvalue: &Value) -> i64 {
        gvalue.get_int64()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_int64(*self)
    }
}

impl ValuePublic for u64 {
    fn get(gvalue: &Value) -> u64 {
        gvalue.get_uint64()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_uint64(*self)
    }
}

impl ValuePublic for bool {
    fn get(gvalue: &Value) -> bool {
        gvalue.get_boolean()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_boolean(*self)
    }
}

impl ValuePublic for i8 {
    fn get(gvalue: &Value) -> i8 {
        gvalue.get_schar()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_schar(*self)
    }
}

impl ValuePublic for u8 {
    fn get(gvalue: &Value) -> u8 {
        gvalue.get_uchar()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_uchar(*self)
    }
}

impl ValuePublic for f32 {
    fn get(gvalue: &Value) -> f32 {
        gvalue.get_float()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_float(*self)
    }
}

impl ValuePublic for f64 {
    fn get(gvalue: &Value) -> f64 {
        gvalue.get_double()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_double(*self)
    }
}

impl ValuePublic for Type {
    fn get(gvalue: &Value) -> Type {
        gvalue.get_gtype()
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_gtype(*self)
    }
}

impl ValuePublic for String {
    fn get(gvalue: &Value) -> String {
        match gvalue.get_string() {
            Some(s) => s,
            None => String::new()
        }
    }

    fn set(&self, gvalue: &Value) {
        gvalue.set_string(self.as_slice())
    }
}
