// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkPopoverMenu — Popovers to use as menus

use ffi;
use cast::GTK_POPOVER_MENU;
use glib::translate::ToGlibPtr;

struct_Widget!(PopoverMenu);

impl PopoverMenu {
    pub fn new() -> Option<PopoverMenu> {
        let tmp_pointer = unsafe { ffi::gtk_popover_menu_new() };
        check_pointer!(tmp_pointer, PopoverMenu)
    }

    pub fn open_submenu(&self, name: &str ) {
        unsafe { ffi::gtk_popover_menu_open_submenu(GTK_POPOVER_MENU(self.pointer), name.to_glib_none().0) }
    }
}

impl_drop!(PopoverMenu);
impl_TraitWidget!(PopoverMenu);

impl ::ContainerTrait for PopoverMenu {}
impl ::BinTrait for PopoverMenu {}
