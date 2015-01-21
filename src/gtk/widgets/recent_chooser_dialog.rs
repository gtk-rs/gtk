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

use gtk::{self, ffi};
use gtk::ffi::FFIWidget;
use gtk::enums::response_type::ResponseType;
use gtk::cast::{GTK_WINDOW, GTK_RECENT_MANAGER};
use std::ffi::CString;

struct_Widget!(RecentChooserDialog);

impl RecentChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>) -> Option<RecentChooserDialog> {
        let c_str = CString::from_slice(title.as_bytes());
        let ok_str = CString::from_slice("Ok".as_bytes());
        let cancel_str = CString::from_slice("Cancel".as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_recent_chooser_dialog_new(c_str.as_ptr(), match parent {
                Some(ref p) => GTK_WINDOW(p.get_widget()),
                None => ::std::ptr::null_mut()
            }, ok_str.as_ptr(), ResponseType::Ok,
               cancel_str.as_ptr(), ResponseType::Cancel,
               ::std::ptr::null::<::libc::c_void>())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn new_for_manager(title: &str, parent: Option<gtk::Window>, manager: &gtk::RecentManager) -> Option<RecentChooserDialog> {
        let c_str = CString::from_slice(title.as_bytes());
        let ok_str = CString::from_slice("Ok".as_bytes());
        let cancel_str = CString::from_slice("Cancel".as_bytes());

        let tmp_pointer = unsafe {
            ffi::gtk_recent_chooser_dialog_new_for_manager(c_str.as_ptr(), match parent {
                Some(ref p) => GTK_WINDOW(p.get_widget()),
                None => ::std::ptr::null_mut()
            }, GTK_RECENT_MANAGER(manager.get_widget()),
               ok_str.as_ptr(), ResponseType::Ok,
               cancel_str.as_ptr(), ResponseType::Cancel,
               ::std::ptr::null::<::libc::c_void>())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
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
