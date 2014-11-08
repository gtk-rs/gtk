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

#![no_implicit_prelude]

//! A container box
use libc::c_int;
use std::prelude::{Option, Some, None, RawPtr};

use gtk::Orientation;
use gtk::{mod, ffi};
use std::clone::Clone;
use std::ops::Drop;

/// Box — A container box
struct_Widget!(Box)

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Option<Box> {
        let tmp_pointer = unsafe { ffi::gtk_box_new(orientation, spacing as c_int) };
        check_pointer!(tmp_pointer, Box)
    }
}

impl_drop!(Box)
impl_TraitWidget!(Box)

impl gtk::ContainerTrait for Box {}
impl gtk::BoxTrait for Box {}
impl gtk::OrientableTrait for Box {}

impl_widget_events!(Box)
