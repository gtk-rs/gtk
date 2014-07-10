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

#![allow(visible_private_types)]

use libc::c_int;
use gtk::ffi;
use gtk::signals::Signal;
use gtk::enums;

pub trait Widget: ffi::FFIWidget {
    fn show_all(&mut self) -> () {
        unsafe {
            ffi::gtk_widget_show_all(self.get_widget());
        }
    }

    fn show_now(&self) {
        unsafe { ffi::gtk_widget_show_now(self.get_widget()) }
    }

    fn hide(&self) {
        unsafe { ffi::gtk_widget_hide(self.get_widget()) }
    }

    fn map(&self) {
        unsafe { ffi::gtk_widget_map(self.get_widget()) }
    }

    fn unmap(&self) {
        unsafe { ffi::gtk_widget_unmap(self.get_widget()) }
    }

    fn realize(&self) {
        unsafe { ffi::gtk_widget_realize(self.get_widget()) }
    }

    fn unrealize(&self) {
        unsafe { ffi::gtk_widget_unrealize(self.get_widget()) }
    }

    fn queue_draw(&self) {
        unsafe { ffi::gtk_widget_queue_draw(self.get_widget()) }
    }

    fn queue_resize(&self) {
        unsafe { ffi::gtk_widget_queue_resize(self.get_widget()) }
    }

    fn queue_resize_no_redraw(&self) {
        unsafe { ffi::gtk_widget_queue_resize_no_redraw(self.get_widget()) }
    }

    fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_scale_factor(self.get_widget()) }
    }

    fn activate(&self) -> bool {
        match unsafe { ffi::gtk_widget_activate(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn reparent(&self, new_parent: &Widget) {
        unsafe { ffi::gtk_widget_reparent(self.get_widget(), new_parent.get_widget()) }
    }

    fn is_focus(&self) -> bool {
        match unsafe { ffi::gtk_widget_is_focus(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn grab_focus(&self) {
        unsafe { ffi::gtk_widget_grab_focus(self.get_widget()) }
    }

    fn grab_default(&self) {
        unsafe { ffi::gtk_widget_grab_default(self.get_widget()) }
    }

    fn set_name(&self, name: &str) {
        name.with_c_str(|c_str| {
            unsafe { ffi::gtk_widget_set_name(self.get_widget(), c_str) }
        })
    }

    fn get_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_widget_get_name(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::std::str::raw::from_c_str(tmp)) }
        }
    }

    fn set_state(&self, state: enums::StateType) {
        unsafe { ffi::gtk_widget_set_state(self.get_widget(), state) }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe { ffi::gtk_widget_set_sensitive(self.get_widget(), match sensitive {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn set_parent(&self, parent: &Widget) {
        unsafe { ffi::gtk_widget_set_parent(self.get_widget(), parent.get_widget()) }
    }

    /*fn set_parent_window(&self, parent: &Widget) {
        unsafe { gtk_widget_set_parent_window(self.get_widget(), parent.get_widget()) }
    }*/

    fn get_toplevel(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_toplevel(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn get_ancestor(&self, widget_type: i32) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_ancestor(self.get_widget(), widget_type) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn is_ancestor(&self, ancestor: &Widget) -> bool {
        match unsafe { ffi::gtk_widget_is_ancestor(self.get_widget(), ancestor.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn hide_on_delete(&self) -> bool {
        match unsafe { ffi::gtk_widget_hide_on_delete(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_direction(&self, dir: enums::TextDirection) {
        unsafe { ffi::gtk_widget_set_direction(self.get_widget(), dir) }
    }

    fn get_direction(&self) -> enums::TextDirection {
        unsafe { ffi::gtk_widget_get_direction(self.get_widget()) }
    }

    fn set_default_direction(dir: enums::TextDirection) {
        unsafe { ffi::gtk_widget_set_default_direction(dir) }
    }

    fn get_default_direction() -> enums::TextDirection {
        unsafe { ffi::gtk_widget_get_default_direction() }
    }

    fn in_destruction(&self) -> bool {
        match unsafe { ffi::gtk_widget_in_destruction(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn unparent(&self) {
        unsafe { ffi::gtk_widget_unparent(self.get_widget()) }
    }

    fn set_margin_right(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_right(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_left(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_left(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_top(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_top(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_bottom(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_bottom(self.get_widget(), margin as c_int)
        }
    }

    fn get_margin_right(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_right(self.get_widget()) as i32
        }
    }

    fn get_margin_left(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_left(self.get_widget()) as i32
        }
    }

    fn get_margin_top(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_top(self.get_widget()) as i32
        }
    }

    fn get_margin_bottom(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_bottom(self.get_widget()) as i32
        }
    }

    fn get_size_request(&mut self) -> (i32, i32) {
        let mut width = 0i32;
        let mut height = 0i32;
        unsafe {
            ffi::gtk_widget_get_size_request(self.get_widget(), &mut width, &mut height)
        }
        (width, height)
    }

    fn set_size_request(&mut self, width: i32, height: i32) {
        unsafe{
            ffi::gtk_widget_set_size_request(self.get_widget(), width, height)
        }
    }

    fn connect<'a>(&self, signal: Box<Signal<'a>>) -> () {
        use std::mem::transmute;

        unsafe {
            let signal_name     = signal.get_signal_name();
            let trampoline      = signal.get_trampoline();

            let user_data_ptr   = transmute(box signal);

            signal_name.replace("_", "-").with_c_str(|signal_name| {
                ffi::glue_signal_connect(
                    self.get_widget(),
                    signal_name,
                    Some(trampoline),
                    user_data_ptr
                )
            });
        }
    }

    fn get_allocated_width(&self) -> i32{
        unsafe{
            ffi::gtk_widget_get_allocated_width(self.get_widget()) as i32
        }
    }

    fn get_allocated_height(&self) -> i32{
        unsafe{
            ffi::gtk_widget_get_allocated_height(self.get_widget()) as i32
        }
    }
}