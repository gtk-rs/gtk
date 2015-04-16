// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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
