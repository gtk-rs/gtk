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

use libc::c_void;
use std::mem;
use std::ops::Index;
use std::iter::FromIterator;

use glib::ffi;
use glib::GlibContainer;

pub struct List<T> {
    pointer: *mut ffi::C_GList
}

pub struct Elem<'a, T> {
    pointer: *mut ffi::C_GList
}

pub struct RevElem<'a, T> {
    pointer: *mut ffi::C_GList
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            pointer: ::std::ptr::null_mut()
        }
    }

    pub fn from_vec(values: Vec<T>) -> List<T> {
        FromIterator::from_iter(values.into_iter())
    }

    pub fn from_slice<T: Clone>(values: &[T]) -> List<T> {
        let v: Vec<T> = values.iter().map(|x| (*x).clone()).collect();
        FromIterator::from_iter(v.into_iter())
    }

    pub fn append(&mut self, data: T) {
        unsafe {
            self.pointer = ffi::g_list_append(self.pointer, mem::transmute(Box::new(data)));
        }
    }

    pub fn prepend(&mut self, data: T) {
        unsafe {
            self.pointer = ffi::g_list_prepend(self.pointer, mem::transmute(Box::new(data)));
        }
    }

    pub fn nth(&self, n: u32) -> &T {
        unsafe {
            mem::transmute::<*mut c_void, &T>(ffi::g_list_nth_data(self.pointer, n))
        }
    }

    pub fn last(&self) -> &T {
        let elem = unsafe { ffi::g_list_last(self.pointer) };
        unsafe { mem::transmute::<*mut c_void, &T>((*elem).data)}
    }

    pub fn first(&self) -> &T {
        let elem = unsafe { ffi::g_list_first(self.pointer) };
        unsafe { mem::transmute::<*mut c_void, &T>((*elem).data)}
    }

    pub fn insert(&mut self, data: T, position: i32) {
        unsafe {
            self.pointer = ffi::g_list_insert(self.pointer, mem::transmute(Box::new(data)), position);
        }
    }

    pub fn concat(&mut self, list: List<T>) {
        unsafe {
            ffi::g_list_concat(self.pointer, list.unwrap());
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            self.pointer = ffi::g_list_reverse(self.pointer);
        }
    }

    pub fn iter(&self) -> Elem<T> {
        Elem {
            pointer: unsafe { ffi::g_list_first(self.pointer) }
        }
    }

    pub fn rev_iter(&self) -> RevElem<T> {
        RevElem {
            pointer: unsafe { ffi::g_list_last(self.pointer) }
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::g_list_length(self.pointer) as usize }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::g_list_free(self.pointer)
        }
    }

    pub fn extend<It: Iterator<Item=T>>(&mut self, mut it: It) {
        for elem in it {
            self.append(elem);
        }
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index<'a>(&'a self, _rhs: &usize) -> &'a T {
        self.nth(*_rhs as u32)
    }
}

impl<'a, T> Iterator for Elem<'a, T> {
    type Item = &'a T;

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

impl<'a, T> Iterator for RevElem<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pointer.is_null() {
            None
        } else {
            let ret = unsafe { mem::transmute::<*mut c_void, &T>((*self.pointer).data)};
            unsafe { self.pointer = (*self.pointer).prev; }
            Some(ret)
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<It: Iterator<Item=T>>(it: It) -> List<T> {
        let mut new_list = List::new();
        new_list.extend(it);
        new_list
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> List<T> {
        unsafe {
            GlibContainer::wrap(ffi::g_list_copy(self.pointer))
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unsafe { ffi::g_list_free(self.pointer); }
    }
}

impl<T> GlibContainer<*mut ffi::C_GList> for List<T> {
    fn wrap(pointer: *mut ffi::C_GList) -> List<T> {
        List {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *mut ffi::C_GList {
        self.pointer
    }
}
