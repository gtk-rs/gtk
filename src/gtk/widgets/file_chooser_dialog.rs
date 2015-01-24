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
use gtk::cast::{GTK_WINDOW};
use std::ffi::CString;

struct_Widget!(FileChooserDialog);

impl FileChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>, action: gtk::FileChooserAction) -> Option<FileChooserDialog> {
        let ok = "Ok";
        let cancel = "Cancel";
        let tmp_pointer = unsafe {
            let c_str = CString::from_slice(title.as_bytes());
            let c_ok = CString::from_slice(ok.as_bytes());
            let c_cancel = CString::from_slice(cancel.as_bytes());

            ffi::gtk_file_chooser_dialog_new(c_str.as_ptr(),
                match parent {
                    Some(ref p) => GTK_WINDOW(p.get_widget()),
                    None => GTK_WINDOW(::std::ptr::null_mut())
                }, action, c_cancel.as_ptr(), gtk::ResponseType::Cancel, c_ok.as_ptr(), gtk::ResponseType::Accept, ::std::ptr::null_mut())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }
}

impl_drop!(FileChooserDialog);
impl_TraitWidget!(FileChooserDialog);

impl gtk::ContainerTrait for FileChooserDialog {}
impl gtk::BinTrait for FileChooserDialog {}
impl gtk::WindowTrait for FileChooserDialog {}
impl gtk::DialogTrait for FileChooserDialog {}
impl gtk::FileChooserTrait for FileChooserDialog {}

impl_widget_events!(FileChooserDialog);
