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

use std::mem;
use libc::c_float;

use gtk::traits::{Widget, Container};
use gtk::enums::{ReliefStyle, PositionType};
use gtk::cast::GTK_BUTTON;
use gtk::ffi;
use gtk;
use std::string;

pub trait Button: Widget + Container {
    fn pressed(&self) -> () {
        unsafe {
            ffi::gtk_button_pressed(GTK_BUTTON(self.get_widget()));
        }
    }

    fn released(&self) -> () {
        unsafe {
            ffi::gtk_button_released(GTK_BUTTON(self.get_widget()));
        }
    }

    fn clicked(&self) -> () {
        unsafe {
            ffi::gtk_button_clicked(GTK_BUTTON(self.get_widget()));
        }
    }

    fn enter(&self) -> () {
        unsafe {
            ffi::gtk_button_enter(GTK_BUTTON(self.get_widget()));
        }
    }

    fn leave(&self) -> () {
        unsafe {
            ffi::gtk_button_leave(GTK_BUTTON(self.get_widget()));
        }
    }

    fn set_relief(&mut self, new_style: ReliefStyle) -> () {
        unsafe {
            ffi::gtk_button_set_relief(GTK_BUTTON(self.get_widget()), new_style);
        }
    }

    fn get_relief(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_button_get_relief(GTK_BUTTON(self.get_widget()))
        }
    }

    fn get_label(&self) -> Option<String> {
        let c_str = unsafe { ffi::gtk_button_get_label(GTK_BUTTON(self.get_widget())) };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe { string::raw::from_buf(c_str as *const u8) })
        }
    }

    fn set_label(&mut self, label: &str) -> () {
        unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_button_set_label(GTK_BUTTON(self.get_widget()), c_str)
            });
        }
    }

    fn get_use_stock(&self) -> bool {
        match unsafe { ffi::gtk_button_get_use_stock(GTK_BUTTON(self.get_widget())) } {
            0i32 => false,
            _ => true
        }
    }

    fn set_use_stock(&mut self, use_stock: bool) -> () {
        match use_stock {
            true    => unsafe { ffi::gtk_button_set_use_stock(GTK_BUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_button_set_use_stock(GTK_BUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_button_get_use_underline(GTK_BUTTON(self.get_widget())) } {
            0i32 => false,
            _ => true
        }
    }

    fn set_use_underline(&mut self, use_underline: bool) -> () {
        match use_underline {
            true    => unsafe { ffi::gtk_button_set_use_underline(GTK_BUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_button_set_use_underline(GTK_BUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn set_focus_on_click(&mut self, focus_on_click: bool) -> () {
        match focus_on_click {
            true    => unsafe { ffi::gtk_button_set_focus_on_click(GTK_BUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_button_set_focus_on_click(GTK_BUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn get_focus_on_click(&self) -> bool {
        match unsafe { ffi::gtk_button_get_focus_on_click(GTK_BUTTON(self.get_widget())) } {
            0i32 => false,
            _ => true
        }
    }

    fn set_alignment(&mut self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_button_set_alignment(GTK_BUTTON(self.get_widget()), x_align as c_float, y_align as c_float)
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let mut x_align = 0.1;
        let mut y_align = 0.1;
        unsafe {
            ffi::gtk_button_get_alignment(GTK_BUTTON(self.get_widget()), &mut x_align, &mut y_align);
        }
        (x_align as f32, y_align as f32)
    }

    fn set_image<T: Widget>(&mut self, image: &T) -> () {
        unsafe {
            ffi::gtk_button_set_image(GTK_BUTTON(self.get_widget()), image.get_widget());
        }
    }

    fn set_image_position(&mut self, position: PositionType) -> () {
        unsafe {
            ffi::gtk_button_set_image_position(GTK_BUTTON(self.get_widget()), position);
        }
    }

    fn get_image_position(&self) -> PositionType {
        unsafe {
            ffi::gtk_button_get_image_position(GTK_BUTTON(self.get_widget()))
        }
    }

    #[cfg(any(GTK_3_6, GTK_3_8, GTK_3_10, GTK_3_12))]
    fn set_always_show_image(&mut self, always_show: bool) -> () {
        match always_show {
            true    => unsafe { ffi::gtk_button_set_always_show_image(GTK_BUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_button_set_always_show_image(GTK_BUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    #[cfg(any(GTK_3_6, GTK_3_8, GTK_3_10, GTK_3_12))]
    fn get_always_show_image(&self) -> bool {
        match unsafe { ffi::gtk_button_get_always_show_image(GTK_BUTTON(self.get_widget())) } {
            0i32 => false,
            _ => true
        }
    }

    fn connect_clicked_signal(&self, handler: Box<ButtonClickedHandler>) {
        let data = unsafe { mem::transmute::<Box<Box<ButtonClickedHandler>>, ffi::gpointer>(box handler) };
        "clicked".with_c_str(|cstr| {
            unsafe {
                ffi::g_signal_connect_data(self.get_widget() as ffi::gpointer,
                    cstr,
                    Some(mem::transmute(widget_destroy_callback)),
                    data,
                    Some(drop_widget_destroy_handler),
                    0);
            }
        });
    }
}


pub trait ButtonClickedHandler {
    fn callback(&mut self, button: &mut gtk::Button);
}

extern "C" fn widget_destroy_callback(object: *mut ffi::C_GtkWidget, user_data: ffi::gpointer) {
    let mut handler = unsafe { mem::transmute::<ffi::gpointer, Box<Box<ButtonClickedHandler>>>(user_data) };

    // let mut window = check_pointer!(object, Window).unwrap();
    // window.can_drop = false;
    let mut button: gtk::Button = ffi::FFIWidget::wrap(object);
    handler.callback(&mut button);

    unsafe {
        mem::forget(handler);
    }
}

extern "C" fn drop_widget_destroy_handler(data: ffi::gpointer, _closure: *const ffi::C_GClosure) {
    unsafe {
        let handler = mem::transmute::<ffi::gpointer, Box<Box<ButtonClickedHandler>>>(data);
        drop(handler);
    }
}
