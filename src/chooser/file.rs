// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_char;

use glib::glib_container::GlibContainer;
use glib::translate::*;
use glib::types;
use glib::Error;
use ffi;

use object::{Object, Upcast};
use widgets::widget::Widget;

use {
    FileChooserAction,
};

//////////////////////////////////////////////////////////////////////////////

pub type FileFilter = Object<ffi::GtkFileFilter>;

impl FileFilter {
    pub fn new() -> FileFilter {
        unsafe { from_glib_full(ffi::gtk_file_filter_new()) }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_file_filter_set_name(self.to_glib_none().0, name.to_glib_none().0)
        };
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_filter_get_name(self.to_glib_none().0))
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_file_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn add_pixbuf_formats(&self) {
        unsafe { ffi::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0) }
    }
}

impl types::StaticType for FileFilter {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_file_filter_get_type()) }
    }
}

unsafe impl Upcast<::Buildable> for FileFilter { }

//////////////////////////////////////////////////////////////////////////////

pub type FileChooser = Object<ffi::GtkFileChooser>;

impl types::StaticType for FileChooser {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_file_chooser_get_type()) }
    }
}

unsafe impl Upcast<Widget> for FileChooser { }

pub trait FileChooserExt {
    fn set_action(&self, action: FileChooserAction);
    fn get_action(&self) -> FileChooserAction;
    fn set_local_only(&self, local_only: bool);
    fn get_local_only(&self) -> bool;
    fn set_select_multiple(&self, select_multiple: bool);
    fn get_select_multiple(&self) -> bool;
    fn set_show_hidden(&self, show_hidden: bool);
    fn get_show_hidden(&self) -> bool;
    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool);
    fn get_do_overwrite_confirmation(&self) -> bool;
    fn set_create_folders(&self, create_folders: bool);
    fn get_create_folders(&self) -> bool;
    fn set_current_name(&self, name: &str);
    #[cfg(gtk_3_10)]
    fn get_current_name(&self) -> Option<String>;
    fn set_filename(&self, filename: &str);
    fn get_filename(&self) -> Option<String>;
    fn select_filename(&self, filename: &str) -> bool;
    fn unselect_filename(&self, filename: &str);
    fn select_all(&self);
    fn unselect_all(&self);
    fn get_filenames(&self) -> Vec<String>;
    fn set_current_folder(&self, filename: &str);
    fn get_current_folder(&self) -> Option<String>;
    fn set_uri(&self, uri: &str);
    fn get_uri(&self) -> Option<String>;
    fn select_uri(&self, uri: &str);
    fn unselect_uri(&self, uri: &str);
    fn get_uris(&self) -> Vec<String>;
    fn set_current_folder_uri(&self, uri: &str) -> bool;
    fn get_current_folder_uri(&self) -> Option<String>;
    fn set_preview_widget<T: Upcast<Widget>>(&self, preview_widget: &T);
    fn get_preview_widget(&self) -> Option<Widget>;
    fn set_preview_widget_active(&self, preview_widget_active: bool);
    fn get_preview_widget_active(&self) -> bool;
    fn set_use_preview_label(&self, use_label: bool);
    fn get_use_preview_label(&self) -> bool;
    fn get_preview_filename(&self) -> Option<String>;
    fn get_preview_uri(&self) -> Option<String>;
    fn set_extra_widget<T: Upcast<Widget>>(&self, extra_widget: &T);
    fn get_extra_widget(&self) -> Option<Widget>;
    fn add_filter(&self, filter: &FileFilter);
    fn remove_filter(&self, filter: &FileFilter);
    fn list_filters(&self) -> Vec<FileFilter>;
    fn set_filter(&self, filter: &FileFilter);
    fn get_filter(&self) -> Option<FileFilter>;
    fn add_shortcut_folder(&self, folder: &str, error: &mut Error) -> bool;
    fn remove_shortcut_folder(&self, folder: &str, error: &mut Error) -> bool;
    fn add_shortcut_folder_uri(&self, uri: &str, error: &mut Error) -> bool;
    fn remove_shortcut_folder_uri(&self, uri: &str, error: &mut Error) -> bool;
}

impl<O: Upcast<FileChooser>> FileChooserExt for O {
    fn set_action(&self, action: FileChooserAction) {
        unsafe { ffi::gtk_file_chooser_set_action(self.upcast().to_glib_none().0, action) }
    }

    fn get_action(&self) -> FileChooserAction {
        unsafe { ffi::gtk_file_chooser_get_action(self.upcast().to_glib_none().0) }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_local_only(self.upcast().to_glib_none().0,
                local_only.to_glib())
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe { from_glib(ffi::gtk_file_chooser_get_local_only(self.upcast().to_glib_none().0)) }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_select_multiple(self.upcast().to_glib_none().0,
                select_multiple.to_glib())
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_select_multiple(self.upcast().to_glib_none().0))
        }
    }

    fn set_show_hidden(&self, show_hidden: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_show_hidden(self.upcast().to_glib_none().0,
                show_hidden.to_glib())
        }
    }

    fn get_show_hidden(&self) -> bool {
        unsafe { from_glib(ffi::gtk_file_chooser_get_show_hidden(self.upcast().to_glib_none().0)) }
    }

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_do_overwrite_confirmation(self.upcast().to_glib_none().0,
                do_overwrite_confirmation.to_glib())
        }
    }

    fn get_do_overwrite_confirmation(&self) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_get_do_overwrite_confirmation(self.upcast().to_glib_none().0))
        }
    }

    fn set_create_folders(&self, create_folders: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_create_folders(self.upcast().to_glib_none().0,
                create_folders.to_glib())
        }
    }

    fn get_create_folders(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_create_folders(self.upcast().to_glib_none().0))
        }
    }

    fn set_current_name(&self, name: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_current_name(self.upcast().to_glib_none().0,
                name.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_10)]
    fn get_current_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_current_name(self.upcast().to_glib_none().0))
        }
    }

    fn set_filename(&self, filename: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_filename(self.upcast().to_glib_none().0,
                filename.to_glib_none().0);
        }
    }

    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_filename(self.upcast().to_glib_none().0))
        }
    }

    fn select_filename(&self, filename: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_select_filename(self.upcast().to_glib_none().0,
                    filename.to_glib_none().0))
        }
    }

    fn unselect_filename(&self, filename: &str) {
        unsafe {
            ffi::gtk_file_chooser_unselect_filename(self.upcast().to_glib_none().0,
                filename.to_glib_none().0)
        }
    }

    fn select_all(&self) {
        unsafe { ffi::gtk_file_chooser_select_all(self.upcast().to_glib_none().0) }
    }

    fn unselect_all(&self) {
        unsafe { ffi::gtk_file_chooser_unselect_all(self.upcast().to_glib_none().0) }
    }

    fn get_filenames(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::<*const c_char, _>::from_glib_full(
                ffi::gtk_file_chooser_get_filenames(self.upcast().to_glib_none().0))
        }
    }

    fn set_current_folder(&self, filename: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_current_folder(self.upcast().to_glib_none().0,
                filename.to_glib_none().0);
        }
    }

    fn get_current_folder(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_current_folder(self.upcast().to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_uri(self.upcast().to_glib_none().0,
                uri.to_glib_none().0);
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_uri(self.upcast().to_glib_none().0))
        }
    }

    fn select_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_file_chooser_select_uri(self.upcast().to_glib_none().0,
                uri.to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_file_chooser_unselect_uri(self.upcast().to_glib_none().0,
                uri.to_glib_none().0)
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::<*const c_char, _>::from_glib_full(
                ffi::gtk_file_chooser_get_uris(self.upcast().to_glib_none().0))
        }
    }

    fn set_current_folder_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_set_current_folder_uri(self.upcast().to_glib_none().0,
                    uri.to_glib_none().0))
        }
    }

    fn get_current_folder_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_current_folder_uri(self.upcast().to_glib_none().0))
        }
    }

    fn set_preview_widget<T: Upcast<Widget>>(&self, preview_widget: &T) {
        unsafe {
            ffi::gtk_file_chooser_set_preview_widget(self.upcast().to_glib_none().0,
                preview_widget.upcast().to_glib_none().0)
        }
    }

    fn get_preview_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_preview_widget(self.upcast().to_glib_none().0))
        }
    }

    fn set_preview_widget_active(&self, preview_widget_active: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_preview_widget_active(self.upcast().to_glib_none().0,
                preview_widget_active.to_glib())
        }
    }

    fn get_preview_widget_active(&self) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_get_preview_widget_active(self.upcast().to_glib_none().0))
        }
    }

    fn set_use_preview_label(&self, use_label: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_use_preview_label(self.upcast().to_glib_none().0,
                use_label.to_glib())
        }
    }

    fn get_use_preview_label(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_use_preview_label(self.upcast().to_glib_none().0))
        }
    }

    fn get_preview_filename(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_preview_filename(self.upcast().to_glib_none().0))
        }
    }

    fn get_preview_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(
                ffi::gtk_file_chooser_get_preview_uri(self.upcast().to_glib_none().0))
        }
    }

    fn set_extra_widget<T: Upcast<Widget>>(&self, extra_widget: &T) {
        unsafe {
            ffi::gtk_file_chooser_set_extra_widget(self.upcast().to_glib_none().0,
                extra_widget.upcast().to_glib_none().0)
        }
    }

    fn get_extra_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_extra_widget(self.upcast().to_glib_none().0))
        }
    }

    fn add_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_add_filter(self.upcast().to_glib_none().0,
                filter.to_glib_none().0)
        }
    }

    fn remove_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_remove_filter(self.upcast().to_glib_none().0,
                filter.to_glib_none().0)
        }
    }

    fn list_filters(&self) -> Vec<FileFilter> {
        unsafe {
            Vec::from_glib_container(
                ffi::gtk_file_chooser_list_filters(self.upcast().to_glib_none().0))
        }
    }

    fn set_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_set_filter(self.upcast().to_glib_none().0,
                filter.to_glib_none().0)
        }
    }

    fn get_filter(&self) -> Option<FileFilter> {
        unsafe { from_glib_none(ffi::gtk_file_chooser_get_filter(self.upcast().to_glib_none().0)) }
    }

    fn add_shortcut_folder(&self, folder: &str, error: &mut Error) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_add_shortcut_folder(self.upcast().to_glib_none().0,
                    folder.to_glib_none().0, &mut error.unwrap()))
        }
    }

    fn remove_shortcut_folder(&self, folder: &str, error: &mut Error) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_remove_shortcut_folder(self.upcast().to_glib_none().0,
                    folder.to_glib_none().0, &mut error.unwrap()))
        }
    }

    fn add_shortcut_folder_uri(&self, uri: &str, error: &mut Error) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_add_shortcut_folder(self.upcast().to_glib_none().0,
                    uri.to_glib_none().0, &mut error.unwrap()))
        }
    }

    fn remove_shortcut_folder_uri(&self, uri: &str, error: &mut Error) -> bool {
        unsafe {
            from_glib(
                ffi::gtk_file_chooser_remove_shortcut_folder(self.upcast().to_glib_none().0,
                    uri.to_glib_none().0, &mut error.unwrap()))
        }
    }
}
