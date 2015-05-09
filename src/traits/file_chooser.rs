// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_char;
use glib::translate::{from_glib_none, FromGlibPtrContainer, ToGlibPtr};
use FFIWidget;
use cast::GTK_FILE_CHOOSER;
use ffi;
use glib::{to_bool, to_gboolean};
use glib::{self, GlibContainer};

pub trait FileChooserTrait: ::WidgetTrait {
    fn set_action(&self, action: ::FileChooserAction) -> () {
        unsafe { ffi::gtk_file_chooser_set_action(GTK_FILE_CHOOSER(self.unwrap_widget()), action) }
    }

    fn get_action(&self) -> ::FileChooserAction {
        unsafe { ffi::gtk_file_chooser_get_action(GTK_FILE_CHOOSER(self.unwrap_widget())) }
    }

    fn set_local_only(&self, local_only: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_local_only(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(local_only)) }
    }

    fn get_local_only(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_local_only(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_select_multiple(&self, select_multiple: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_select_multiple(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(select_multiple)) }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_select_multiple(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_show_hidden(&self, show_hidden: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_show_hidden(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(show_hidden)) }
    }

    fn get_show_hidden(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_show_hidden(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_do_overwrite_confirmation(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(do_overwrite_confirmation)) }
    }

    fn get_do_overwrite_confirmation(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_do_overwrite_confirmation(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_create_folders(&self, create_folders: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_create_folders(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(create_folders)) }
    }

    fn get_create_folders(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_create_folders(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_current_name(&self, name: &str) -> () {
        unsafe {
            ffi::gtk_file_chooser_set_current_name(GTK_FILE_CHOOSER(self.unwrap_widget()), name.to_glib_none().0)
        }
    }

    fn get_current_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_current_name(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_filename(&self, filename: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_set_filename(GTK_FILE_CHOOSER(self.unwrap_widget()), filename.to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_filename(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn select_filename(&self, filename: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_select_filename(GTK_FILE_CHOOSER(self.unwrap_widget()), filename.to_glib_none().0))
        }
    }

    fn unselect_filename(&self, filename: &str) -> () {
        unsafe {
            ffi::gtk_file_chooser_unselect_filename(GTK_FILE_CHOOSER(self.unwrap_widget()), filename.to_glib_none().0)
        }
    }

    fn select_all(&self) -> () {
        unsafe { ffi::gtk_file_chooser_select_all(GTK_FILE_CHOOSER(self.unwrap_widget())) }
    }

    fn unselect_all(&self) -> () {
        unsafe { ffi::gtk_file_chooser_unselect_all(GTK_FILE_CHOOSER(self.unwrap_widget())) }
    }

    fn get_filenames(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::<*const c_char, _>::from_glib_full(
                ffi::gtk_file_chooser_get_filenames(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_current_folder(&self, filename: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_set_current_folder(GTK_FILE_CHOOSER(self.unwrap_widget()), filename.to_glib_none().0))
        }
    }

    fn get_current_folder(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_current_folder(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_uri(&self, uri: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_set_uri(GTK_FILE_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_uri(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn select_uri(&self, uri: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_select_uri(GTK_FILE_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0))
        }
    }

    fn unselect_uri(&self, uri: &str) -> () {
        unsafe {
            ffi::gtk_file_chooser_unselect_uri(GTK_FILE_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0)
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::<*const c_char, _>::from_glib_full(
                ffi::gtk_file_chooser_get_uris(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_current_folder_uri(&self, uri: &str) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_set_current_folder_uri(GTK_FILE_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0))
        }
    }

    fn get_current_folder_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_current_folder_uri(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_preview_widget<T: ::WidgetTrait>(&self, preview_widget: &T) -> () {
        unsafe { ffi::gtk_file_chooser_set_preview_widget(GTK_FILE_CHOOSER(self.unwrap_widget()), preview_widget.unwrap_widget()) }
    }

    fn get_preview_widget<T: ::WidgetTrait>(&self) -> Option<T> {
        unsafe {
            let tmp_pointer = ffi::gtk_file_chooser_get_preview_widget(GTK_FILE_CHOOSER(self.unwrap_widget()));

            if tmp_pointer.is_null() {
                None
            } else {
                Some(::FFIWidget::wrap_widget(tmp_pointer))
            }
        }
    }

    fn set_preview_widget_active(&self, preview_widget_active: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_preview_widget_active(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(preview_widget_active)) }
    }

    fn get_preview_widget_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_preview_widget_active(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn set_use_preview_label(&self, use_label: bool) -> () {
        unsafe { ffi::gtk_file_chooser_set_use_preview_label(GTK_FILE_CHOOSER(self.unwrap_widget()), to_gboolean(use_label)) }
    }

    fn get_use_preview_label(&self) -> bool {
        unsafe { to_bool(ffi::gtk_file_chooser_get_use_preview_label(GTK_FILE_CHOOSER(self.unwrap_widget()))) }
    }

    fn get_preview_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_preview_filename(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn get_preview_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_file_chooser_get_preview_uri(GTK_FILE_CHOOSER(self.unwrap_widget())))
        }
    }

    fn set_extra_widget<T: ::WidgetTrait>(&self, extra_widget: &T) -> () {
        unsafe { ffi::gtk_file_chooser_set_extra_widget(GTK_FILE_CHOOSER(self.unwrap_widget()), extra_widget.unwrap_widget()) }
    }

    fn get_extra_widget<T: ::WidgetTrait>(&self) -> Option<T> {
        unsafe {
            let tmp = ffi::gtk_file_chooser_get_extra_widget(GTK_FILE_CHOOSER(self.unwrap_widget()));

            if tmp.is_null() {
                None
            } else {
                Some(::FFIWidget::wrap_widget(tmp))
            }
        }
    }

    fn add_filter(&self, filter: &::FileFilter) -> () {
        unsafe { ffi::gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(self.unwrap_widget()), filter.unwrap_pointer()) }
    }

    fn remove_filter(&self, filter: &::FileFilter) -> () {
        unsafe { ffi::gtk_file_chooser_remove_filter(GTK_FILE_CHOOSER(self.unwrap_widget()), filter.unwrap_pointer()) }
    }

    fn set_filter(&self, filter: &::FileFilter) -> () {
        unsafe { ffi::gtk_file_chooser_set_filter(GTK_FILE_CHOOSER(self.unwrap_widget()), filter.unwrap_pointer()) }
    }

    fn get_filter(&self) -> Option<::FileFilter> {
        let tmp = unsafe { ffi::gtk_file_chooser_get_filter(GTK_FILE_CHOOSER(self.unwrap_widget())) };

        ::FileFilter::wrap(tmp)
    }

    fn add_shortcut_folder(&self, folder: &str, error: &mut glib::Error) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_add_shortcut_folder(GTK_FILE_CHOOSER(self.unwrap_widget()), folder.to_glib_none().0, &mut error.unwrap()))
        }
    }

    fn remove_shortcut_folder(&self, folder: &str, error: &mut glib::Error) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_remove_shortcut_folder(GTK_FILE_CHOOSER(self.unwrap_widget()), folder.to_glib_none().0, &mut error.unwrap()))
        }
    }

    fn add_shortcut_folder_uri(&self, uri: &str, error: &mut glib::Error) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_add_shortcut_folder(GTK_FILE_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0, &mut error.unwrap()))
        }
    }

    fn remove_shortcut_folder_uri(&self, uri: &str, error: &mut glib::Error) -> bool {
        unsafe {
            to_bool(ffi::gtk_file_chooser_remove_shortcut_folder(GTK_FILE_CHOOSER(self.unwrap_widget()), uri.to_glib_none().0, &mut error.unwrap()))
        }
    }
}
