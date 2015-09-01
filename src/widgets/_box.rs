// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![no_implicit_prelude]

//! A container box
use libc::c_int;
//use std::prelude::{Option, Some, None, RawPtr};
use std::option::Option;

use Orientation;
use ffi;
use std::clone::Clone;
use std::ops::Drop;

/// Box — A container box
struct_Widget!(Box);

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Option<Box> {
        let tmp_pointer = unsafe { ffi::gtk_box_new(orientation, spacing as c_int) };
        check_pointer!(tmp_pointer, Box)
    }
}

impl_drop!(Box);
impl_TraitWidget!(Box);

impl ::ContainerTrait for Box {}
impl ::BoxTrait for Box {}
impl ::OrientableTrait for Box {}
