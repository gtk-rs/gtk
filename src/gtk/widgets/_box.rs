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

//! A container box

use libc::c_int;

use gtk::enums::Orientation;
use gtk::ffi;
use gtk::traits;
/// Box — A container box
struct_Widget!(_Box)

impl _Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Option<_Box> {
        let tmp_pointer = unsafe { ffi::gtk_box_new(orientation, spacing as c_int) };
        check_pointer!(tmp_pointer, _Box)
    }
}

impl_TraitWidget!(_Box)

impl traits::Container for _Box {}
impl traits::_Box for _Box {}
impl traits::Orientable for _Box {}