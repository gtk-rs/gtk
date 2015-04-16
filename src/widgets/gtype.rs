// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A numerical value which represents the unique identifier of a registered type.

// https://developer.gnome.org/gobject/unstable/gobject-Type-Information.html#GType
pub mod g_type {
    use ffi;
    use std::ffi::CString;
    use std::slice;
    use glib::translate::{FromGlibPtr, ToGlibPtr};
    use glib_ffi::{self};

    pub fn name(_type: glib_ffi::GType) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::g_type_name(_type))
        }
    }

    pub fn from_name(name: &str) -> glib_ffi::GType {
        unsafe {
            ffi::g_type_from_name(name.borrow_to_glib().0)
        }
    }

    pub fn parent(_type: glib_ffi::GType) -> glib_ffi::GType {
        unsafe { ffi::g_type_parent(_type) }
    }

    pub fn depth(_type: glib_ffi::GType) -> u32 {
        unsafe { ffi::g_type_depth(_type) }
    }

    pub fn next_base(leaf_type: glib_ffi::GType, root_type: glib_ffi::GType) -> glib_ffi::GType {
        unsafe { ffi::g_type_next_base(leaf_type, root_type) }
    }

    pub fn is_a(_type: glib_ffi::GType, is_a_type: glib_ffi::GType) -> bool {
        unsafe { to_bool(ffi::g_type_is_a(_type, is_a_type)) }
    }

    pub fn children(_type: glib_ffi::GType) -> Vec<glib_ffi::GType> {
        let mut n_children = 0u32;
        let tmp_vec = unsafe { ffi::g_type_children(_type, &mut n_children) };

        if n_children == 0u32 || tmp_vec.is_null() {
            Vec::new()
        } else {
            unsafe {
                Vec::from(
                    slice::from_raw_parts(
                        tmp_vec as *const glib_ffi::GType,
                        n_children as usize))
            }
        }
    }

    pub fn interfaces(_type: glib_ffi::GType) -> Vec<glib_ffi::GType> {
        let mut n_interfaces = 0u32;
        let tmp_vec = unsafe { ffi::g_type_interfaces(_type, &mut n_interfaces) };

        if n_interfaces == 0u32 || tmp_vec.is_null() {
            Vec::new()
        } else {
            unsafe {
                Vec::from(
                    slice::from_raw_parts(
                        tmp_vec as *const glib_ffi::GType,
                        n_interfaces as usize))
            }
        }
    }

    pub fn interface_prerequisites(interface_type: glib_ffi::GType) -> Vec<glib_ffi::GType> {
        let mut n_prerequisites = 0u32;
        let tmp_vec = unsafe { ffi::g_type_interface_prerequisites(interface_type, &mut n_prerequisites) };

        if n_prerequisites == 0u32 || tmp_vec.is_null() {
            Vec::new()
        } else {
            unsafe {
                Vec::from(
                    slice::from_raw_parts(
                        tmp_vec as *const glib_ffi::GType, n_prerequisites as usize))
            }
        }
    }

    pub fn interface_add_prerequisite(interface_type: glib_ffi::GType, prerequisite_type: glib_ffi::GType) {
        unsafe { ffi::g_type_interface_add_prerequisite(interface_type, prerequisite_type) }
    }

    pub fn fundamental_next() -> glib_ffi::GType {
        unsafe { ffi::g_type_fundamental_next() }
    }

    pub fn fundamental(type_id: glib_ffi::GType) -> glib_ffi::GType {
        unsafe { ffi::g_type_fundamental(type_id) }
    }

    pub fn ensure(_type: glib_ffi::GType) {
        unsafe { ffi::g_type_ensure(_type) }
    }

    pub fn get_type_registration_serial() -> u32 {
        unsafe { ffi::g_type_get_type_registration_serial() }
    }
}
