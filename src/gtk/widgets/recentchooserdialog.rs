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

use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk::traits;
use gtk::enums::response_type;
use gtk::cast::{GTK_WINDOW, GTK_RECENT_MANAGER};
use gtk;

struct_Widget!(RecentChooserDialog)

impl RecentChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<RecentChooserDialog> {
        let tmp_pointer = unsafe { title.with_c_str(|c_str| {
            "Ok".with_c_str(|c_str2| {
                "Cancel".with_c_str(|c_str3| {
                    ffi::gtk_recent_chooser_dialog_new(c_str, match parent {
                        Some(ref p) => GTK_WINDOW(p.get_widget()),
                        None => ::std::ptr::mut_null()
                    },
                    c_str2, response_type::Ok,
                    c_str3, response_type::Cancel,
                    ::std::ptr::null::<::libc::c_void>())
                })
            })
        })};

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn new_for_manager(title: &str, parent: Option<gtk::Window>, manager: &gtk::RecentManager) -> Option<RecentChooserDialog> {
        let tmp_pointer = unsafe { title.with_c_str(|c_str| {
            "Ok".with_c_str(|c_str2| {
                "Cancel".with_c_str(|c_str3| {
                    ffi::gtk_recent_chooser_dialog_new_for_manager(c_str, match parent {
                        Some(ref p) => GTK_WINDOW(p.get_widget()),
                        None => ::std::ptr::mut_null()
                    }, GTK_RECENT_MANAGER(manager.get_widget()),
                    c_str2, response_type::Ok,
                    c_str3, response_type::Cancel,
                    ::std::ptr::null::<::libc::c_void>())
                })
            })
        })};

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }
}

impl_drop!(RecentChooserDialog)
impl_TraitWidget!(RecentChooserDialog)

impl traits::Container for RecentChooserDialog {}
impl traits::Bin for RecentChooserDialog {}
impl traits::Window for RecentChooserDialog {}
impl traits::Dialog for RecentChooserDialog {}
impl traits::RecentChooser for RecentChooserDialog {}