// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

    pub fn set(&self, arrow_type: ArrowType, shadow_type: ShadowType) -> () {
        unsafe {
            ffi::gtk_arrow_set(GTK_ARROW(self.pointer), arrow_type, shadow_type);
        }
    }
}

impl_drop!(Arrow);
impl_TraitWidget!(Arrow);

impl ::MiscTrait for Arrow {}

impl_widget_events!(Arrow);
