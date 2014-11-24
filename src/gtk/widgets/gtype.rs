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

//! A numerical value which represents the unique identifier of a registered type.

// https://developer.gnome.org/gobject/unstable/gobject-Type-Information.html#GType
pub mod g_type {
    use gtk::{mod, ffi};

    pub fn name(_type: gtk::GType) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_type_name(_type) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { String::from_raw_buf(tmp_pointer as *const u8) })
        }
    }

    pub fn from_name(name: &str) -> gtk::GType {
        unsafe {
            name.with_c_str(|c_str| {
                ffi::g_type_from_name(c_str)
            })
        }
    }

    pub fn parent(_type: gtk::GType) -> gtk::GType {
        unsafe { ffi::g_type_parent(_type) }
    }

    pub fn depth(_type: gtk::GType) -> u32 {
        unsafe { ffi::g_type_depth(_type) }
    }

    pub fn next_base(leaf_type: gtk::GType, root_type: gtk::GType) -> gtk::GType {
        unsafe { ffi::g_type_next_base(leaf_type, root_type) }
    }

    pub fn is_a(_type: gtk::GType, is_a_type: gtk::GType) -> bool {
        unsafe { ffi::to_bool(ffi::g_type_is_a(_type, is_a_type)) }
    }

    pub fn children(_type: gtk::GType) -> Vec<gtk::GType> {
        let mut n_children = 0u32;
        let tmp_vec = unsafe { ffi::g_type_children(_type, &mut n_children) };

        if n_children == 0u32 || tmp_vec.is_null() {
            Vec::new()
        } else {
            unsafe { Vec::from_raw_buf(tmp_vec as *const gtk::GType, n_children as uint) }
        }
    }

    pub fn interfaces(_type: gtk::GType) -> Vec<gtk::GType> {
        let mut n_interfaces = 0u32;
        let tmp_vec = unsafe { ffi::g_type_interfaces(_type, &mut n_interfaces) };

        if n_interfaces == 0u32 || tmp_vec.is_null() {
            Vec::new()
        } else {
            unsafe { Vec::from_raw_buf(tmp_vec as *const gtk::GType, n_interfaces as uint) }
        }
    }

    pub fn interface_prerequisites(interface_type: gtk::GType) -> Vec<gtk::GType> {
        let mut n_prerequisites = 0u32;
        let tmp_vec = unsafe { ffi::g_type_interface_prerequisites(interface_type, &mut n_prerequisites) };

        if n_prerequisites == 0u32 || tmp_vec.is_null() {
            Vec::new()
        } else {
            unsafe { Vec::from_raw_buf(tmp_vec as *const gtk::GType, n_prerequisites as uint) }
        }
    }

    pub fn interface_add_prerequisite(interface_type: gtk::GType, prerequisite_type: gtk::GType) {
        unsafe { ffi::g_type_interface_add_prerequisite(interface_type, prerequisite_type) }
    }

    pub fn fundamental_next() -> gtk::GType {
        unsafe { ffi::g_type_fundamental_next() }
    }

    pub fn fundamental(type_id: gtk::GType) -> gtk::GType {
        unsafe { ffi::g_type_fundamental(type_id) }
    }

    pub fn ensure(_type: gtk::GType) {
        unsafe { ffi::g_type_ensure(_type) }
    }

    pub fn get_type_registration_serial() -> u32 {
        unsafe { ffi::g_type_get_type_registration_serial() }
    }
}