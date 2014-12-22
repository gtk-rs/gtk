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

use gtk::{mod, ffi};
use gtk::ffi::FFIWidget;
use gtk::cast::{GTK_WINDOW};

struct_Widget!(FileChooserDialog);

impl FileChooserDialog {
    pub fn new(title: &str, parent: Option<gtk::Window>, action: gtk::FileChooserAction) -> Option<FileChooserDialog> {
        let ok = "Ok";
        let cancel = "Cancel";
        let tmp_pointer = unsafe {
            title.with_c_str(|c_str|{
                ok.with_c_str(|c_ok|{
                    cancel.with_c_str(|c_cancel|{
                        ffi::gtk_file_chooser_dialog_new(c_str, match parent {
                            Some(ref p) => GTK_WINDOW(p.get_widget()),
                            None => GTK_WINDOW(::std::ptr::null_mut())
                        }, action, c_cancel, gtk::ResponseType::Cancel, c_ok, gtk::ResponseType::Accept, ::std::ptr::null_mut())
                    })
                })
            })
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
