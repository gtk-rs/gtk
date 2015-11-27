// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::ToGlibPtr;

struct_Widget!(MenuItem);

impl MenuItem {
    pub fn new() -> Option<MenuItem> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_menu_item_new() };
        check_pointer!(tmp_pointer, MenuItem)
    }

    pub fn new_with_label(label: &str) -> Option<MenuItem> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_menu_item_new_with_label(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<MenuItem> {
        assert_initialized_main_thread!();
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
