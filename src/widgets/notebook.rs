// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use super::widget::Widget;

use {
    PackType,
    PositionType,
};

/// A tabbed notebook container.
pub type Notebook = Object<ffi::GtkNotebook>;

unsafe impl Upcast<Widget> for Notebook { }
unsafe impl Upcast<::Container> for Notebook { }
unsafe impl Upcast<::Buildable> for Notebook { }

impl Notebook {
    pub fn new() -> Notebook {
        unsafe { Widget::from_glib_none(ffi::gtk_notebook_new()).downcast_unchecked() }
    }

    pub fn append_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> Option<u32>
    where T: Upcast<Widget>, U: Upcast<Widget> {
        unsafe { 
            from_glib(
                ffi::gtk_notebook_append_page(
                    self.to_glib_none().0,
                    child.upcast().to_glib_none().0,
                    tab_label.map(|w| w.upcast()).to_glib_none().0))
        }
    }

    pub fn append_page_menu<T, U, W>(&self, child: &T, tab_label: Option<&U>,
                                     menu_label: Option<&W>) -> Option<u32>
    where T: Upcast<Widget>, U: Upcast<Widget>, W: Upcast<Widget> {
        unsafe {
            from_glib(
                ffi::gtk_notebook_append_page_menu(
                    self.to_glib_none().0,
                    child.upcast().to_glib_none().0,
                    tab_label.map(|w| w.upcast()).to_glib_none().0,
                    menu_label.map(|w| w.upcast()).to_glib_none().0))
        }
    }

    pub fn prepend_page<T, U>(&self, child: &T, tab_label: Option<&U>) -> Option<u32>
    where T: Upcast<Widget>, U: Upcast<Widget> {
        unsafe {
            from_glib(
                ffi::gtk_notebook_prepend_page(
                    self.to_glib_none().0,
                    child.upcast().to_glib_none().0,
                    tab_label.map(|w| w.upcast()).to_glib_none().0))
        }
    }

    pub fn prepend_page_menu<T, U, W>(&self, child: &T, tab_label: Option<&U>,
                                      menu_label: Option<&W>) -> Option<u32>
    where T: Upcast<Widget>, U: Upcast<Widget>, W: Upcast<Widget> {
        unsafe {
            from_glib(
                ffi::gtk_notebook_prepend_page_menu(
                    self.to_glib_none().0,
                    child.upcast().to_glib_none().0,
                    tab_label.map(|w| w.upcast()).to_glib_none().0,
                    menu_label.map(|w| w.upcast()).to_glib_none().0))
        }
    }

    pub fn insert_page<T, U>(&self, child: &T, tab_label: Option<&U>, position: i32) -> Option<u32>
    where T: Upcast<Widget>, U: Upcast<Widget> {
        unsafe {
            from_glib(
                ffi::gtk_notebook_insert_page(
                    self.to_glib_none().0,
                    child.upcast().to_glib_none().0,
                    tab_label.map(|w| w.upcast()).to_glib_none().0,
                    position))
        }
    }

    pub fn insert_page_menu<T, U, W>(&self, child: &T, tab_label: Option<&U>,
                                     menu_label: Option<&W>, position: i32) -> Option<u32>
    where T: Upcast<Widget>, U: Upcast<Widget>, W: Upcast<Widget> {
        unsafe {
            from_glib(
                ffi::gtk_notebook_insert_page_menu(
                    self.to_glib_none().0,
                    child.upcast().to_glib_none().0,
                    tab_label.map(|w| w.upcast()).to_glib_none().0,
                    menu_label.map(|w| w.upcast()).to_glib_none().0,
                    position))
        }
    }

    pub fn remove_page(&self, page_num: i32) {
        unsafe { ffi::gtk_notebook_remove_page(self.to_glib_none().0, page_num) }
    }

    pub fn set_group_name(&self, group_name: &str) {
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.to_glib_none().0)
        }
    }

    pub fn get_group_name(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0)) }
    }

    pub fn get_current_page(&self) -> Option<u32> {
        unsafe { from_glib( ffi::gtk_notebook_get_current_page(self.to_glib_none().0)) }
    }

    pub fn get_nth_page(&self, page_num: i32) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_notebook_get_nth_page(self.to_glib_none().0, page_num)) }
    }

    pub fn get_n_pages(&self) -> i32 {
        unsafe { ffi::gtk_notebook_get_n_pages(self.to_glib_none().0) }
    }

    pub fn page_num<T: Upcast<Widget>>(&self, child: &T) -> Option<u32> {
        unsafe {
            from_glib(
                ffi::gtk_notebook_page_num(self.to_glib_none().0, child.upcast().to_glib_none().0))
        }
    }

    pub fn set_current_page(&self, page_num: i32) {
        unsafe { ffi::gtk_notebook_set_current_page(self.to_glib_none().0, page_num) }
    }

    pub fn next_page(&self) {
        unsafe { ffi::gtk_notebook_next_page(self.to_glib_none().0) }
    }

    pub fn prev_page(&self) {
        unsafe { ffi::gtk_notebook_prev_page(self.to_glib_none().0) }
    }

    pub fn set_show_border(&self, show_border: bool) {
        unsafe { ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.to_glib()) }
    }

    pub fn get_show_border(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0)) }
    }

    pub fn set_show_tabs(&self, show_tabs: bool) {
        unsafe { ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.to_glib()) }
    }

    pub fn get_show_tabs(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0)) }
    }

    pub fn set_tab_pos(&self, pos: PositionType) {
        unsafe { ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos) }
    }

    pub fn get_tab_pos(&self) -> PositionType {
        unsafe { ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0) }
    }

    pub fn set_scrollable(&self, scrollable: bool) {
        unsafe { ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.to_glib()) }
    }

    pub fn is_scrollable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0)) }
    }

    pub fn get_tab_hborder(&self) -> u16 {
        unsafe { ffi::gtk_notebook_get_tab_hborder(self.to_glib_none().0) }
    }

    pub fn get_tab_vborder(&self) -> u16 {
        unsafe { ffi::gtk_notebook_get_tab_vborder(self.to_glib_none().0) }
    }

    pub fn popup_enable(&self) {
        unsafe { ffi::gtk_notebook_popup_enable(self.to_glib_none().0) }
    }

    pub fn popup_disable(&self) {
        unsafe { ffi::gtk_notebook_popup_disable(self.to_glib_none().0) }
    }

    pub fn get_tab_label<T: Upcast<Widget>>(&self, child: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(
                ffi::gtk_notebook_get_tab_label(self.to_glib_none().0,
                    child.upcast().to_glib_none().0))
        }
    }

    pub fn set_tab_label<T, U>(&self, child: &T, tab_label: Option<&U>)
    where T: Upcast<Widget>, U: Upcast<Widget> {
        unsafe {
            ffi::gtk_notebook_set_tab_label(self.to_glib_none().0,
                                            child.upcast().to_glib_none().0,
                                            tab_label.map(|w| w.upcast()).to_glib_none().0)
        }
    }

    pub fn set_tab_label_text<T: Upcast<Widget>>(&self, child: &T, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(self.to_glib_none().0,
                                                 child.upcast().to_glib_none().0,
                                                 tab_text.to_glib_none().0)
        }
    }

    pub fn get_tab_label_text<T: Upcast<Widget>>(&self, child: &T) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_notebook_get_tab_label_text(self.to_glib_none().0,
                                                     child.upcast().to_glib_none().0))
        }
    }

    pub fn get_menu_label<T: Upcast<Widget>>(&self, child: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(
                ffi::gtk_notebook_get_menu_label(self.to_glib_none().0,
                                                 child.upcast().to_glib_none().0))
        }
    }

    pub fn set_menu_label<T, U>(&self, child: &T, menu_label: Option<&U>)
    where T: Upcast<Widget>, U: Upcast<Widget> {
        unsafe {
            ffi::gtk_notebook_set_menu_label(self.to_glib_none().0,
                                             child.upcast().to_glib_none().0,
                                             menu_label.map(|w| w.upcast()).to_glib_none().0)
        }
    }

    pub fn set_menu_label_text<T: Upcast<Widget>>(&self, child: &T, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(self.to_glib_none().0,
                                                  child.upcast().to_glib_none().0,
                                                  tab_text.to_glib_none().0)
        }
    }

    pub fn get_menu_label_text<T: Upcast<Widget>>(&self, child: &T) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_notebook_get_menu_label_text(self.to_glib_none().0,
                                                      child.upcast().to_glib_none().0))
        }
    }

    pub fn reorder_child<T: Upcast<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_notebook_reorder_child(self.to_glib_none().0,
                                            child.upcast().to_glib_none().0,
                                            position)
        }
    }

    pub fn is_tab_reorderable<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_notebook_get_tab_reorderable(self.to_glib_none().0,
                                                      child.upcast().to_glib_none().0))
        }
    }

    pub fn set_tab_reorderable<T: Upcast<Widget>>(&self, child: &T, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(self.to_glib_none().0,
                                                  child.upcast().to_glib_none().0,
                                                  reorderable.to_glib())
        }
    }

    pub fn is_tab_detachable<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_notebook_get_tab_detachable(self.to_glib_none().0,
                                                     child.upcast().to_glib_none().0))
        }
    }

    pub fn set_tab_detachable<T: Upcast<Widget>>(&self, child: &T, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(self.to_glib_none().0,
                                                child.upcast().to_glib_none().0,
                                                detachable.to_glib())
        }
    }

    pub fn get_action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(
                ffi::gtk_notebook_get_action_widget(self.to_glib_none().0, pack_type))
        }
    }

    pub fn set_action_widget<T: Upcast<Widget>>(&self, child: &T, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(self.to_glib_none().0,
                                                child.upcast().to_glib_none().0,
                                                pack_type)
        }
    }
}

impl types::StaticType for Notebook {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_notebook_get_type()) }
    }
}
