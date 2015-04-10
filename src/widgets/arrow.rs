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

//! Displays an arrow

use {ShadowType, ArrowType};
use cast::GTK_ARROW;
use ffi;

/// Arrow — Displays an arrow
struct_Widget!(Arrow);

impl Arrow {
    pub fn new(arrow_type: ArrowType, shadow_type: ShadowType) -> Option<Arrow> {
        let tmp_pointer = unsafe { ffi::gtk_arrow_new(arrow_type, shadow_type) };
        check_pointer!(tmp_pointer, Arrow)
    }

    pub fn set(&mut self, arrow_type: ArrowType, shadow_type: ShadowType) -> () {
        unsafe {
            ffi::gtk_arrow_set(GTK_ARROW(self.pointer), arrow_type, shadow_type);
        }
    }
}

impl_drop!(Arrow);
impl_TraitWidget!(Arrow);

impl ::MiscTrait for Arrow {}

impl_widget_events!(Arrow);
