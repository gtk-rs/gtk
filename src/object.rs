// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::marker::PhantomData;
use glib::object::{Ref, Wrapper};
use glib::types::StaticType;

pub use glib::object::{Downcast, Upcast};

/// The generic type for `GObject` descendants in GTK.
pub struct Object<T>(Ref, PhantomData<T>);

impl<T> Wrapper for Object<T>
where Object<T>: StaticType {
    type GlibType = T;
    #[inline]
    unsafe fn wrap(r: Ref) -> Object<T> { Object(r, PhantomData) }
    #[inline]
    fn as_ref(&self) -> &Ref { &self.0 }
    #[inline]
    fn unwrap(self) -> Ref { self.0 }
}

impl<T> Clone for Object<T> {
    fn clone(&self) -> Object<T> {
        Object(self.0.clone(), PhantomData)
    }
}

unsafe impl<T> Upcast<::glib::object::Object> for Object<T> where Object<T>: StaticType { }
