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
use glib::translate::ToGlibPtr;
use ffi;
use cast::GTK_WINDOW;
use FFIWidget;
use DialogButtons;

struct_Widget!(Dialog);

impl Dialog {
    pub fn new() -> Dialog {
        unsafe {
            ::FFIWidget::wrap_widget(
                ffi::gtk_dialog_new())
        }
    }

    pub fn with_buttons<T: DialogButtons>(title: &str, parent: Option<::Window>,
                                          flags: ::DialogFlags, buttons: T) -> Dialog {
        unsafe {
            let parent = match parent {
                Some(w) => GTK_WINDOW(w.unwrap_widget()),
                None => ptr::null_mut(),
            };
            ::FFIWidget::wrap_widget(
                buttons.invoke3(ffi::gtk_dialog_new_with_buttons,
                                title.borrow_to_glib().0,
                                parent,
                                flags))
        }
    }
}

impl_drop!(Dialog);
impl_TraitWidget!(Dialog);

impl ::ContainerTrait for Dialog {}
impl ::BinTrait for Dialog {}
impl ::WindowTrait for Dialog {}
impl ::DialogTrait for Dialog {}

impl_widget_events!(Dialog);
