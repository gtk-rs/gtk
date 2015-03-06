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

use std::ptr;
use glib::translate::{ToGlibPtr, ToTmp};
use gtk::{self, ffi};
use gtk::FFIWidget;
use gtk::ResponseType;
use gtk::cast::{GTK_WINDOW, GTK_RECENT_MANAGER};

struct_Widget!(RecentChooserDialog);

impl RecentChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<RecentChooserDialog> {
        let mut tmp_title = title.to_tmp_for_borrow();
        let mut tmp_ok = "Ok".to_tmp_for_borrow();
        let mut tmp_cancel = "Cancel".to_tmp_for_borrow();
        let parent = match parent {
            Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
            None => ptr::null_mut()
        };

        let tmp_pointer = unsafe {
            ffi::gtk_recent_chooser_dialog_new(
                tmp_title.to_glib_ptr(), parent,
                tmp_ok.to_glib_ptr(), ResponseType::Ok,
                tmp_cancel.to_glib_ptr(), ResponseType::Cancel,
                ptr::null::<()>())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn new_for_manager(title: &str, parent: Option<gtk::Window>, manager: &gtk::RecentManager) -> Option<RecentChooserDialog> {
        let mut tmp_title = title.to_tmp_for_borrow();
        let mut tmp_ok = "Ok".to_tmp_for_borrow();
        let mut tmp_cancel = "Cancel".to_tmp_for_borrow();
        let parent = match parent {
            Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
            None => ptr::null_mut()
        };

        let tmp_pointer = unsafe {
            ffi::gtk_recent_chooser_dialog_new_for_manager(
                tmp_title.to_glib_ptr(), parent,
                GTK_RECENT_MANAGER(manager.unwrap_widget()),
                tmp_ok.to_glib_ptr(), ResponseType::Ok,
                tmp_cancel.to_glib_ptr(), ResponseType::Cancel,
                ptr::null::<()>())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(RecentChooserDialog);
impl_TraitWidget!(RecentChooserDialog);

impl gtk::ContainerTrait for RecentChooserDialog {}
impl gtk::BinTrait for RecentChooserDialog {}
impl gtk::WindowTrait for RecentChooserDialog {}
impl gtk::DialogTrait for RecentChooserDialog {}
impl gtk::RecentChooserTrait for RecentChooserDialog {}

impl_widget_events!(RecentChooserDialog);
