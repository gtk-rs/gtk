// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button which pops up a scale

use libc::c_double;
use std::ptr;

use ffi;
use IconSize;

/// ScaleButton — A button which pops up a scale
/*
* # Availables signals :
* * `popdown` : Action
* * `popup` : Action
* * `value-changed` : Run Last
*/
struct_Widget!(ScaleButton);

impl ScaleButton {
    // FIXME: icons -> last parameter
    pub fn new(size: IconSize, min: f64, max: f64, step: f64) -> Option<ScaleButton> {
        let tmp_pointer = unsafe { ffi::gtk_scale_button_new(size, min as c_double, max as c_double, step as c_double, ptr::null()) };
        check_pointer!(tmp_pointer, ScaleButton)
    }
}

impl_drop!(ScaleButton);
impl_TraitWidget!(ScaleButton);

impl ::ContainerTrait for ScaleButton {}
impl ::ButtonTrait for ScaleButton {}
impl ::ScaleButtonTrait for ScaleButton {}
impl ::OrientableTrait for ScaleButton {}
