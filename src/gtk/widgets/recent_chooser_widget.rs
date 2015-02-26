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

//! GtkRecentChooserWidget — Displays recently used files

use gtk::cast::GTK_RECENT_MANAGER;
use gtk::{self, ffi};
use gtk::FFIWidget;
use gtk::RecentManager;

struct_Widget!(RecentChooserWidget);

impl RecentChooserWidget {
    pub fn new() -> Option<RecentChooserWidget> {
        let tmp_pointer = unsafe { ffi::gtk_recent_chooser_widget_new() };
        check_pointer!(tmp_pointer, RecentChooserWidget)
    }

    pub fn new_for_manager(manager: &RecentManager) -> Option<RecentChooserWidget> {
        let tmp_pointer = unsafe { ffi::gtk_recent_chooser_widget_new_for_manager(GTK_RECENT_MANAGER(manager.get_widget())) };
        check_pointer!(tmp_pointer, RecentChooserWidget)
    }
}

impl_drop!(RecentChooserWidget);
impl_TraitWidget!(RecentChooserWidget);

impl gtk::ContainerTrait for RecentChooserWidget {}
impl gtk::OrientableTrait for RecentChooserWidget {}
impl gtk::RecentChooserTrait for RecentChooserWidget {}
impl gtk::BoxTrait for RecentChooserWidget {}

impl_widget_events!(RecentChooserWidget);
