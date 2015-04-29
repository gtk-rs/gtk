// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container which overlays widgets on top of each other

use cast::{GTK_OVERLAY};
use ffi;

/// GtkOverlay — A container which overlays widgets on top of each other
struct_Widget!(Overlay);

impl Overlay {
    pub fn new() -> Option<Overlay> {
        let tmp_pointer = unsafe { ffi::gtk_overlay_new() };
        check_pointer!(tmp_pointer, Overlay)
    }

    pub fn add_overlay<T: ::WidgetTrait>(&self, widget: &T) {
        unsafe {
            ffi::gtk_overlay_add_overlay(GTK_OVERLAY(self.pointer), widget.unwrap_widget())
        }
    }
}

impl_drop!(Overlay);
impl_TraitWidget!(Overlay);

impl ::ContainerTrait for Overlay {}
impl ::BinTrait for Overlay {}
