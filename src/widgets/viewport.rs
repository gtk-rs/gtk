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

//! An adapter which makes widgets scrollable

use ShadowType;
use cast::GTK_VIEWPORT;
use ffi;

/// GtkViewport — An adapter which makes widgets scrollable
struct_Widget!(Viewport);

impl Viewport {
    pub fn new(hadjustment: &::Adjustment, vadjustment: &::Adjustment) -> Option<Viewport> {
        let tmp_pointer = unsafe { ffi::gtk_viewport_new(hadjustment.unwrap_pointer(), vadjustment.unwrap_pointer()) };
        check_pointer!(tmp_pointer, Viewport)
    }

    pub fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            ffi::gtk_viewport_get_shadow_type(GTK_VIEWPORT(self.pointer))
        }
    }

    pub fn set_shadow_type(&self, ty: ShadowType) {
        unsafe {
            ffi::gtk_viewport_set_shadow_type(GTK_VIEWPORT(self.pointer), ty)
        }
    }
}

impl_drop!(Viewport);
impl_TraitWidget!(Viewport);

impl ::ContainerTrait for Viewport {}
impl ::BinTrait for Viewport {}
impl ::ScrollableTrait for Viewport {}

impl_widget_events!(Viewport);
