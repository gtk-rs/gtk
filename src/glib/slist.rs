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

// FIXME: @jeremyletang implements the new index traits when it's available

use std::mem;
use libc::c_void;

use glib::ffi;
use glib::GlibContainer;

pub struct SList<T> {
    pointer: *mut ffi::C_GSList
}

pub struct SElem<'a, T> {
    pointer: *mut ffi::C_GSList
}

impl<T> SList<T> {
    pub fn new() -> SList<T> {
        SList {
            pointer: ::std::ptr::mut_null()
        }
    }

    pub fn from_vec(values: Vec<T>) -> SList<T> {
        FromIterator::from_iter(values.move_iter())
    }

    pub fn from_slice<T: Clone>(values: &[T]) -> SList<T> {
        let v: Vec<T> = values.iter().map(|x| (*x).clone()).collect();
        FromIterator::from_iter(v.move_iter())
    }

    pub fn append(&mut self, data: T) {
        unsafe {
            self.pointer = ffi::g_slist_append(self.pointer, mem::transmute(box data));
        }
    }

    pub fn prepend(&mut self, data: T) {
        unsafe {
            self.pointer = ffi::g_slist_prepend(self.pointer, mem::transmute(box data));
        }
    }

    pub fn nth(&self, n: u32) -> &T {
        unsafe {
            mem::transmute::<*mut c_void, &T>(ffi::g_slist_nth_data(self.pointer, n))
        }
    }

    pub fn last(&self) -> &T {
        let elem = unsafe { ffi::g_slist_last(self.pointer) };
        unsafe { mem::transmute::<*mut c_void, &T>((*elem).data)}
    }

    pub fn insert(&mut self, data: T, position: i32) {
        unsafe {
            self.pointer = ffi::g_slist_insert(self.pointer, mem::transmute(box data), position);
        }
    }

    pub fn concat(&mut self, list: SList<T>) {
        unsafe {
            ffi::g_slist_concat(self.pointer, list.unwrap());
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            self.pointer = ffi::g_slist_reverse(self.pointer);
        }
    }

    pub fn iter(&self) -> SElem<T> {
        SElem {
            pointer: self.pointer
        }
    }
}

impl<T> Mutable for SList<T> {
    fn clear(&mut self) {
        unsafe {
            ffi::g_slist_free(self.pointer)
        }
    }
}

impl<'a, T> Index<uint, &'a T> for SList<T> {
    fn index(&self, _rhs: &uint) -> &'a T {
        self.nth(*_rhs as u32)
    }
}

impl<'a, T> Iterator<&'a T> for SElem<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        if self.pointer.is_null() {
            None
        } else {
            let ret = unsafe { mem::transmute::<*mut c_void, &T>((*self.pointer).data)};
            unsafe { self.pointer = (*self.pointer).next; }
            Some(ret)
        }
    }
}

impl<T> Collection for SList<T> {
    fn len(&self) -> uint {
        unsafe { ffi::g_slist_length(self.pointer) as uint }
    }
}

impl<T> FromIterator<T> for SList<T> {
    fn from_iter<It: Iterator<T>>(it: It) -> SList<T> {
        let mut new_list = SList::new();
        new_list.extend(it);
        new_list
    }
}

impl<T> Extendable<T> for SList<T> {
    fn extend<It: Iterator<T>>(&mut self, mut it: It) {
        for elem in it {
            self.append(elem);
        }
    }
}

impl<T> Clone for SList<T> {
    fn clone(&self) -> SList<T> {
        unsafe {
            GlibContainer::wrap(ffi::g_slist_copy(self.pointer))
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for SList<T> {
    fn drop(&mut self) {
        unsafe { ffi::g_slist_free(self.pointer); }
    }
}

impl<T> GlibContainer<*mut ffi::C_GSList> for SList<T> {
    fn wrap(pointer: *mut ffi::C_GSList) -> SList<T> {
        SList {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut ffi::C_GSList {
        self.pointer
    }
}
