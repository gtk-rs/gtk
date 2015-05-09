// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use glib::translate::ToGlibPtr;
use ffi;
use FFIWidget;
use DialogButtons;
use cast::{GTK_WINDOW, GTK_RECENT_MANAGER};

struct_Widget!(RecentChooserDialog);

impl RecentChooserDialog {
    pub fn new<T: DialogButtons>(title: &str, parent: Option<&::Window>,
                                 buttons: T) -> RecentChooserDialog {
        let parent = match parent {
            Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
            None => ptr::null_mut()
        };

        unsafe {
            ::FFIWidget::wrap_widget(
                buttons.invoke2(
                    ffi::gtk_recent_chooser_dialog_new,
                    title.borrow_to_glib().0,
                    parent))
        }
    }

    pub fn new_for_manager<T: DialogButtons>(title: &str, parent: Option<&::Window>,
                                             manager: &::RecentManager, buttons: T) -> RecentChooserDialog {
        let parent = match parent {
            Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
            None => ptr::null_mut()
        };

        unsafe {
            ::FFIWidget::wrap_widget(
                buttons.invoke3(
                    ffi::gtk_recent_chooser_dialog_new_for_manager,
                    title.borrow_to_glib().0,
                    parent,
                    GTK_RECENT_MANAGER(manager.unwrap_widget())))
        }
    }
}

impl_drop!(RecentChooserDialog);
impl_TraitWidget!(RecentChooserDialog);

impl ::ContainerTrait for RecentChooserDialog {}
impl ::BinTrait for RecentChooserDialog {}
impl ::WindowTrait for RecentChooserDialog {}
impl ::DialogTrait for RecentChooserDialog {}
impl ::RecentChooserTrait for RecentChooserDialog {}
