// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! The widget used for item in menus

use ffi;
use glib::translate::ToGlibPtr;

/// MenuItem — The widget used for item in menus
struct_Widget!(MenuItem);

impl MenuItem {
    pub fn new() -> Option<MenuItem> {
        let tmp_pointer = unsafe { ffi::gtk_menu_item_new() };
        check_pointer!(tmp_pointer, MenuItem)
    }

    pub fn new_with_label(label: &str) -> Option<MenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_menu_item_new_with_label(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<MenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_menu_item_new_with_mnemonic(mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }
}

impl_drop!(MenuItem);
impl_TraitWidget!(MenuItem);

impl ::ContainerTrait for MenuItem {}
impl ::BinTrait for MenuItem {}
impl ::MenuItemTrait for MenuItem {}
