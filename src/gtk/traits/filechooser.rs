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

use gtk::traits;
use gtk::cast::{GTK_FILE_CHOOSER, GTK_FILE_FILTER};
use gtk::ffi;
use gtk::ffi::FFIWidget;
use gtk;
use glib;
use std::str;
use libc::c_char;
use glib::GlibContainer;

pub trait FileChooser: traits::Widget {
    fn set_action(&self, action: gtk::FileChooserAction) -> () {
        unsafe { ffi::gtk_file_chooser_set_action(GTK_FILE_CHOOSER(self.get_widget()), action) }
    }

    fn get_action(&self) -> gtk::FileChooserAction {
        unsafe { ffi::gtk_file_chooser_get_action(GTK_FILE_CHOOSER(self.get_widget())) }
    }

    fn set_local_only(&self, local_only: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_local_only(GTK_FILE_CHOOSER(self.get_widget()), match local_only {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_local_only(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_local_only(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_select_multiple(GTK_FILE_CHOOSER(self.get_widget()), match select_multiple {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_select_multiple(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_select_multiple(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_show_hidden(&self, show_hidden: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_show_hidden(GTK_FILE_CHOOSER(self.get_widget()), match show_hidden {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_show_hidden(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_show_hidden(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_do_overwrite_confirmation(GTK_FILE_CHOOSER(self.get_widget()), match do_overwrite_confirmation {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_do_overwrite_confirmation(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_do_overwrite_confirmation(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_create_folders(&self, create_folders: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_create_folders(GTK_FILE_CHOOSER(self.get_widget()), match create_folders {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_create_folders(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_create_folders(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_current_name(&self, name: &str) -> () {
        unsafe {
            name.with_c_str(|c_str| {
                ffi::gtk_file_chooser_set_current_name(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        }
    }

    fn get_current_name(&self) -> Option<String> {
        let name = unsafe { ffi::gtk_file_chooser_get_current_name(GTK_FILE_CHOOSER(self.get_widget())) };

        if name.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(name) })
        }
    }

    fn set_filename(&self, filename: &str) -> bool {
        match unsafe {
            filename.with_c_str(|c_str| {
                ffi::gtk_file_chooser_set_filename(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_filename(&self) -> Option<String> {
        let filename = unsafe { ffi::gtk_file_chooser_get_filename(GTK_FILE_CHOOSER(self.get_widget())) };

        if filename.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(filename) })
        }
    }

    fn select_filename(&self, filename: &str) -> bool {
        match unsafe {
            filename.with_c_str(|c_str| {
                ffi::gtk_file_chooser_select_filename(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn unselect_filename(&self, filename: &str) -> () {
        unsafe {
            filename.with_c_str(|c_str| {
                ffi::gtk_file_chooser_unselect_filename(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        }
    }

    fn select_all(&self) -> () {
        unsafe { ffi::gtk_file_chooser_select_all(GTK_FILE_CHOOSER(self.get_widget())) }
    }

    fn unselect_all(&self) -> () {
        unsafe { ffi::gtk_file_chooser_unselect_all(GTK_FILE_CHOOSER(self.get_widget())) }
    }

    fn get_filenames(&self) -> glib::SList<String> {
        let tmp_pointer = unsafe { ffi::gtk_file_chooser_get_filenames(GTK_FILE_CHOOSER(self.get_widget())) };

        if tmp_pointer.is_null() {
            glib::SList::new()
        } else {
            let old_list : glib::SList<*const c_char> = glib::GlibContainer::wrap(tmp_pointer);
            let mut tmp_vec = glib::SList::new();

            for it in old_list.iter() {
                unsafe {
                    tmp_vec.append(str::raw::from_c_str(*it));
                }
            }
            tmp_vec
        }
    }

    fn set_current_folder(&self, filename: &str) -> bool {
        match unsafe {
            filename.with_c_str(|c_str| {
                ffi::gtk_file_chooser_set_current_folder(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_current_folder(&self) -> Option<String> {
        let filename = unsafe { ffi::gtk_file_chooser_get_current_folder(GTK_FILE_CHOOSER(self.get_widget())) };

        if filename.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(filename) })
        }
    }

    fn set_uri(&self, uri: &str) -> bool {
        match unsafe {
            uri.with_c_str(|c_str| {
                ffi::gtk_file_chooser_set_uri(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_uri(&self) -> Option<String> {
        let uri = unsafe { ffi::gtk_file_chooser_get_uri(GTK_FILE_CHOOSER(self.get_widget())) };

        if uri.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(uri) })
        }
    }

    fn select_uri(&self, uri: &str) -> bool {
        match unsafe {
            uri.with_c_str(|c_str| {
                ffi::gtk_file_chooser_select_uri(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn unselect_uri(&self, uri: &str) -> () {
        unsafe {
            uri.with_c_str(|c_str| {
                ffi::gtk_file_chooser_unselect_uri(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        }
    }

    fn get_uris(&self) -> glib::SList<String> {
        let tmp_pointer = unsafe { ffi::gtk_file_chooser_get_uris(GTK_FILE_CHOOSER(self.get_widget())) };

        if tmp_pointer.is_null() {
            glib::SList::new()
        } else {
            let old_list : glib::SList<*const c_char> = glib::GlibContainer::wrap(tmp_pointer);
            let mut tmp_vec = glib::SList::new();

            for it in old_list.iter() {
                unsafe {
                    tmp_vec.append(str::raw::from_c_str(*it));
                }
            }
            tmp_vec
        }
    }

    fn set_current_folder_uri(&self, uri: &str) -> bool {
        match unsafe {
            uri.with_c_str(|c_str| {
                ffi::gtk_file_chooser_set_current_folder_uri(GTK_FILE_CHOOSER(self.get_widget()), c_str)
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_current_folder_uri(&self) -> Option<String> {
        let uri = unsafe { ffi::gtk_file_chooser_get_current_folder_uri(GTK_FILE_CHOOSER(self.get_widget())) };

        if uri.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(uri) })
        }
    }

    fn set_preview_widget(&self, preview_widget: &gtk::Widget) -> () {
        unsafe { ffi::gtk_file_chooser_set_preview_widget(GTK_FILE_CHOOSER(self.get_widget()), preview_widget.get_widget()) }
    }

    fn get_preview_widget(&self) -> Option<gtk::Widget> {
        let tmp_pointer = unsafe { ffi::gtk_file_chooser_get_preview_widget(GTK_FILE_CHOOSER(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::Widget::wrap(tmp_pointer))
        }
    }

    fn set_preview_widget_active(&self, preview_widget_active: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_preview_widget_active(GTK_FILE_CHOOSER(self.get_widget()), match preview_widget_active {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_preview_widget_active(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_preview_widget_active(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_use_preview_label(&self, use_label: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_use_preview_label(GTK_FILE_CHOOSER(self.get_widget()), match use_label {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn get_use_preview_label(&self) -> bool {
        match unsafe { ffi::gtk_file_chooser_get_use_preview_label(GTK_FILE_CHOOSER(self.get_widget())) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_preview_filename(&self) -> Option<String> {
        let filename = unsafe { ffi::gtk_file_chooser_get_preview_filename(GTK_FILE_CHOOSER(self.get_widget())) };

        if filename.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(filename) })
        }
    }

    fn get_preview_uri(&self) -> Option<String> {
        let uri = unsafe { ffi::gtk_file_chooser_get_preview_uri(GTK_FILE_CHOOSER(self.get_widget())) };

        if uri.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(uri) })
        }
    }

    fn set_extra_widget(&self, extra_widget: &gtk::Widget) -> () {
        unsafe { ffi::gtk_file_chooser_set_extra_widget(GTK_FILE_CHOOSER(self.get_widget()), extra_widget.get_widget()) }
    }

    fn get_extra_widget(&self) -> Option<gtk::Widget> {
        let tmp = unsafe { ffi::gtk_file_chooser_get_extra_widget(GTK_FILE_CHOOSER(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(gtk::Widget::wrap(tmp))
        }
    }

    fn add_filter(&self, filter: &gtk::FileFilter) -> () {
        unsafe { ffi::gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(self.get_widget()), GTK_FILE_FILTER(filter.get_widget())) }
    }

    fn remove_filter(&self, filter: &gtk::FileFilter) -> () {
        unsafe { ffi::gtk_file_chooser_remove_filter(GTK_FILE_CHOOSER(self.get_widget()), GTK_FILE_FILTER(filter.get_widget())) }
    }

    fn set_filter(&self, filter: &gtk::FileFilter) -> () {
        unsafe { ffi::gtk_file_chooser_set_filter(GTK_FILE_CHOOSER(self.get_widget()), GTK_FILE_FILTER(filter.get_widget())) }
    }

    fn get_filter(&self) -> Option<gtk::FileFilter> {
        let tmp = unsafe { ffi::gtk_file_chooser_get_filter(GTK_FILE_CHOOSER(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp as *mut ffi::C_GtkWidget))
        }
    }

    fn add_shortcut_folder(&self, folder: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            folder.with_c_str(|c_str| {
                ffi::gtk_file_chooser_add_shortcut_folder(GTK_FILE_CHOOSER(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn remove_shortcut_folder(&self, folder: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            folder.with_c_str(|c_str| {
                ffi::gtk_file_chooser_remove_shortcut_folder(GTK_FILE_CHOOSER(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn add_shortcut_folder_uri(&self, uri: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            uri.with_c_str(|c_str| {
                ffi::gtk_file_chooser_add_shortcut_folder(GTK_FILE_CHOOSER(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn remove_shortcut_folder_uri(&self, uri: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            uri.with_c_str(|c_str| {
                ffi::gtk_file_chooser_remove_shortcut_folder(GTK_FILE_CHOOSER(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::Gtrue => true,
            _ => false
        }
    }
}