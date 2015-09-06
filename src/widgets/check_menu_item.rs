// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! The widget used for item in menus

use ffi;
use glib::translate::ToGlibPtr;

/// CheckMenuItem — The widget used for item in menus
struct_Widget!(CheckMenuItem);

impl CheckMenuItem {
    pub fn new() -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe { ffi::gtk_check_menu_item_new() };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }

    pub fn new_with_label(label: &str) -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_menu_item_new_with_label(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_menu_item_new_with_mnemonic(
                    mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }
}

impl_drop!(CheckMenuItem);
impl_TraitWidget!(CheckMenuItem);

impl ::ContainerTrait for CheckMenuItem {}
impl ::BinTrait for CheckMenuItem {}
impl ::MenuItemTrait for CheckMenuItem {}
impl ::CheckMenuItemTrait for CheckMenuItem {}
