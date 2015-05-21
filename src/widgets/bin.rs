// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Upcast};
use super::widget::Widget;

pub type Bin = Object<ffi::GtkBin>;

impl types::StaticType for Bin {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_bin_get_type()) }
    }
}

unsafe impl Upcast<Widget> for Bin { }
unsafe impl Upcast<super::container::Container> for Bin { }
unsafe impl Upcast<::builder::Buildable> for Bin { }

pub trait BinExt {
    fn get_child(&self) -> Option<Widget>;
}

impl<O: Upcast<Bin>> BinExt for O {
    fn get_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_bin_get_child(self.upcast().to_glib_none().0)) }
    }
}
