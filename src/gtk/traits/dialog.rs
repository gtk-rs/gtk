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

use std::ffi::CString;
use gtk::cast::GTK_DIALOG;
use gtk::ffi;
use gtk;

pub trait DialogTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait + gtk::WindowTrait {
    fn run(&self) -> i32 {
        unsafe { ffi::gtk_dialog_run(GTK_DIALOG(self.get_widget())) }
    }

    fn response(&self, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_response(GTK_DIALOG(self.get_widget()), response_id) }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Option<gtk::Button> {
        let tmp_pointer = unsafe {
            let c_str = CString::from_slice(button_text.as_bytes());

            ffi::gtk_dialog_add_button(GTK_DIALOG(self.get_widget()), c_str.as_ptr(), response_id)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    fn add_buttons(&self, buttons: Vec<(&str, i32)>) -> Vec<gtk::Button> {
        //unsafe { ffi::gtk_dialog_add_buttons(GTK_DIALOG(self.get_widget()), ...) }
        let mut ret = Vec::new();

        for &(button_text, response_id) in buttons.iter() {
            match self.add_button(button_text, response_id) {
                Some(b) => ret.push(b),
                None => {}
            }
        }
        ret
    }

    fn add_action_widget(&self, child: &gtk::WidgetTrait, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_add_action_widget(GTK_DIALOG(self.get_widget()), child.get_widget(), response_id) }
    }

    fn set_default_response(&self, response_id: i32) -> () {
        unsafe { ffi::gtk_dialog_set_default_response(GTK_DIALOG(self.get_widget()), response_id) }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: ffi::Gboolean) -> () {
        unsafe { ffi::gtk_dialog_set_response_sensitive(GTK_DIALOG(self.get_widget()), response_id, setting) }
    }

    fn get_response_for_widget(&self, widget: &gtk::WidgetTrait) -> Result<i32, gtk::ResponseType> {
        let tmp = unsafe { ffi::gtk_dialog_get_response_for_widget(GTK_DIALOG(self.get_widget()), widget.get_widget()) };

        if tmp < 0 {
            Err(unsafe { ::std::mem::transmute(tmp) })
        } else {
            Ok(tmp)
        }
    }

    fn get_widget_for_reponse<T: gtk::WidgetTrait>(&self, response_id: i32) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_widget_for_response(GTK_DIALOG(self.get_widget()), response_id) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    fn get_action_area<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_action_area(GTK_DIALOG(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    fn get_content_area<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_content_area(GTK_DIALOG(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    #[cfg(any(feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn get_header_bar<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_header_bar(GTK_DIALOG(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }
}
