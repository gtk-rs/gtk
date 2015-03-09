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

use glib::translate::{ToGlibPtr, ToTmp};
use gtk::{self, ffi};
use gtk::FFIWidget;
use gtk::cast::{GTK_WINDOW};
use gtk::DialogButtons;

struct_Widget!(FileChooserDialog);

impl FileChooserDialog {
    pub fn new<T: DialogButtons>(title: &str, parent: Option<gtk::Window>,
                                 action: gtk::FileChooserAction, buttons: T) -> FileChooserDialog {
        unsafe {
            let mut tmp_title = title.to_tmp_for_borrow();
            let parent = match parent {
                Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                None => GTK_WINDOW(::std::ptr::null_mut())
            };

            gtk::FFIWidget::wrap_widget(
                buttons.invoke3(
                    ffi::gtk_file_chooser_dialog_new,
                    tmp_title.to_glib_ptr(),
                    parent,
                    action))

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
