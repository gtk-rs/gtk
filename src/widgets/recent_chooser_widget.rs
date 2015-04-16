// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkRecentChooserWidget — Displays recently used files

use cast::GTK_RECENT_MANAGER;
use ffi;
use FFIWidget;
use RecentManager;

struct_Widget!(RecentChooserWidget);

impl RecentChooserWidget {
    pub fn new() -> Option<RecentChooserWidget> {
        let tmp_pointer = unsafe { ffi::gtk_recent_chooser_widget_new() };
        check_pointer!(tmp_pointer, RecentChooserWidget)
    }

    pub fn new_for_manager(manager: &RecentManager) -> Option<RecentChooserWidget> {
        let tmp_pointer = unsafe { ffi::gtk_recent_chooser_widget_new_for_manager(GTK_RECENT_MANAGER(manager.unwrap_widget())) };
        check_pointer!(tmp_pointer, RecentChooserWidget)
    }
}

impl_drop!(RecentChooserWidget);
impl_TraitWidget!(RecentChooserWidget);

impl ::ContainerTrait for RecentChooserWidget {}
impl ::OrientableTrait for RecentChooserWidget {}
impl ::RecentChooserTrait for RecentChooserWidget {}
impl ::BoxTrait for RecentChooserWidget {}

impl_widget_events!(RecentChooserWidget);
