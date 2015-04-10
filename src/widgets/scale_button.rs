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

impl_widget_events!(ScaleButton);
impl_button_events!(ScaleButton);
