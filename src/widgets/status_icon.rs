// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;
use cairo::RectangleInt;
use gdk;
use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlib, ToGlibPtr};
use ffi;

use Orientation;


glib_wrapper! {
    pub struct StatusIcon(Object<ffi::GtkStatusIcon>);

    match fn {
        get_type => || ffi::gtk_status_icon_get_type(),
    }
}


impl StatusIcon {
    pub fn new() -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new())
        }
    }

    pub fn new_from_file(filename: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_file(filename.to_glib_none().0))
        }
    }

    pub fn new_from_icon_name(icon_name: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_icon_name(icon_name.to_glib_none().0))
        }
    }

    pub fn new_from_pixbuf(pixbuf: &gdk::pixbuf::Pixbuf) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_pixbuf(pixbuf.to_glib_none().0))
        }
    }

    pub fn new_from_stock(stock_id: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_stock(stock_id.to_glib_none().0))
        }
    }

    pub fn get_geometry(&self) -> Option<(gdk::Screen, RectangleInt, Orientation)> {
        unsafe {
            let mut res = mem::uninitialized();
            let mut screen = mem::uninitialized();
            let mut area = mem::uninitialized();
            if from_glib(ffi::gtk_status_icon_get_geometry(
                self.to_glib_none().0,
                &mut screen,
                &mut area,
                &mut res)) {
                Some((from_glib_none(screen), area, res))
            }
            else {
                None
            }
        }
    }

    // pub fn get_gicon(&self) -> *mut gio_ffi::GIcon {
    //     unsafe {
    //         from_glib_none(ffi::gtk_status_icon_get_gicon(self.to_glib_none().0))
    //     }
    // }

    pub fn get_has_tooltip(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_has_tooltip(self.to_glib_none().0))
        }
    }

    pub fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_icon_name(self.to_glib_none().0))
        }
    }

    pub fn get_pixbuf(&self) -> Option<gdk::pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_screen(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_status_icon_get_size(self.to_glib_none().0)
        }
    }

    pub fn get_stock(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_stock(self.to_glib_none().0))
        }
    }

    pub fn get_storage_type(&self) -> ffi::GtkImageType {
        unsafe {
            ffi::gtk_status_icon_get_storage_type(self.to_glib_none().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_markup(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_text(self.to_glib_none().0))
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_visible(self.to_glib_none().0))
        }
    }

    pub fn get_x11_window_id(&self) -> u32 {
        unsafe {
            ffi::gtk_status_icon_get_x11_window_id(self.to_glib_none().0)
        }
    }

    pub fn is_embedded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_is_embedded(self.to_glib_none().0))
        }
    }

    pub fn set_from_file(&self, filename: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_file(self.to_glib_none().0, filename.to_glib_none().0);
        }
    }

    //pub fn set_from_gicon(&self, icon: Unknown rust type: "Icon") {
    //    unsafe { TODO: call ffi:gtk_status_icon_set_from_gicon() }
    //}

    pub fn set_from_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    pub fn set_from_pixbuf(&self, pixbuf: Option<&gdk::pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_status_icon_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    pub fn set_from_stock(&self, stock_id: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    pub fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe {
            ffi::gtk_status_icon_set_has_tooltip(self.to_glib_none().0, has_tooltip.to_glib());
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_status_icon_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_status_icon_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_tooltip_markup(&self, markup: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    pub fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_status_icon_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    // pub fn position_menu(menu: &::Menu, x: &mut i32, y: &mut i32, user_data: &StatusIcon) -> bool {
    //     unsafe {
    //         let mut push_in = mem::uninitialized();
    //         ffi::gtk_status_icon_position_menu(GTK_MENU(menu.unwrap_widget()), x, y, &mut push_in, user_data.to_glib_none().0);
    //         from_glib(push_in)
    //     }
    // }
}
