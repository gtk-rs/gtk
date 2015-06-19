// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! StatusIcon — Display an icon in the system tray

use glib::translate::*;
use glib::types;
use ffi;

use object::Object;

/**
* Display an icon in the system tray
*/
pub type StatusIcon = Object<ffi::GtkStatusIcon>;

impl StatusIcon {
    /// Creates an empty status icon object.
    pub fn new() -> StatusIcon {
        unsafe { from_glib_none(ffi::gtk_status_icon_new()) }
    }
    /// Creates a status icon displaying the file `filename`.
    /// The image will be scaled down to fit in the available space in the notification area,
    /// if necessary.
    pub fn new_from_file(filename: &str) -> StatusIcon {
        unsafe { from_glib_none(ffi::gtk_status_icon_new_from_file(filename.to_glib_none().0)) }
    }
    /// Makes status icon display the file `filename`. See `new_from_file` for details.
    pub fn set_from_file(&self, filename: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_file(self.to_glib_none().0,
                filename.to_glib_none().0)
        }
    }
    /// Gets the size in pixels that is available for the image.
    /// Stock icons and named icons adapt their size automatically
    /// if the size of the notification area changes. For other storage types,
    /// the size-changed signal can be used to react to size changes.
    /// Note that the returned size is only meaningful while the status icon is embedded
    /// (see `is_embedded()`).
    pub fn get_size(&self) -> i32 {
        unsafe { ffi::gtk_status_icon_get_size(self.to_glib_none().0) }
    }
    /// Sets the has-tooltip property on `StatusIcon` to `has_tooltip`.
    /// See `has-tooltip` for more information.
    pub fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe { ffi::gtk_status_icon_set_has_tooltip(self.to_glib_none().0, has_tooltip.to_glib()) }
    }
    /// Sets text as the contents of the tooltip.
    /// This function will take care of setting `has-tooltip` to `true` and
    /// of the default handler for the `query-tooltip` signal.
    /// See also the `tooltip-text` property and `gtk_tooltip_set_text()`.
    pub fn set_tooltip_text(&self, text: &str) {
        unsafe { ffi::gtk_status_icon_set_tooltip_text(self.to_glib_none().0, text.to_glib_none().0) }
    }
    /// Sets `markup` as the contents of the tooltip,
    /// which is marked up with the Pango text markup language.
    /// This function will take care of setting `has-tooltip` to `true` and
    /// of the default handler for the `query-tooltip` signal.
    /// See also the `tooltip-markup` property and `tk_tooltip_set_markup()`.
    pub fn set_tooltip_markup(&self, markup: Option<&str>) {
        unsafe { ffi::gtk_status_icon_set_tooltip_markup(self.to_glib_none().0, markup.to_glib_none().0) }
    }
    /// Sets the title of this tray icon. This should be a short, human-readable,
    /// localized string describing the tray icon.
    /// It may be used by tools like screen readers to render the tray icon.
    pub fn set_title(&self, title: &str) {
        unsafe { ffi::gtk_status_icon_set_title(self.to_glib_none().0, title.to_glib_none().0) }
    }
    /// Gets the title of this tray icon. See `set_title()`.
    pub fn get_title(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gtk_status_icon_get_title(self.to_glib_none().0)) }
    }
    pub fn set_visible(&self, visible: bool) {
        unsafe { ffi::gtk_status_icon_set_visible(self.to_glib_none().0, visible.to_glib()) }
    }
    /// Returns whether the status icon is visible or not.
    /// Note that being visible does not guarantee that the user can actually see the icon,
    /// see also `is_embedded()`.
    pub fn get_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_status_icon_get_visible(self.to_glib_none().0)) }
    }
    /// Returns whether the status icon is embedded in a notification area.
    pub fn is_embedded(&self) -> bool {
        unsafe { from_glib(ffi::gtk_status_icon_is_embedded(self.to_glib_none().0)) }
    }
    /// Returns the current value of the has-tooltip property.
    /// See `has-tooltip` for more information.
    pub fn get_has_tooltip(&self) -> bool {
        unsafe { from_glib(ffi::gtk_status_icon_get_has_tooltip(self.to_glib_none().0)) }
    }
    /// Gets the contents of the tooltip for `StatusIcon`.
    pub fn get_tooltip_text(&self) -> Option<String> {
        unsafe { from_glib_full(ffi::gtk_status_icon_get_tooltip_text(self.to_glib_none().0)) }
    }
    /// Gets the contents of the tooltip for `StatusIcon`.
    pub fn get_tooltip_markup(&self) -> Option<String> {
        unsafe { from_glib_full(ffi::gtk_status_icon_get_tooltip_markup(self.to_glib_none().0)) }
    }
}

impl types::StaticType for StatusIcon {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_status_icon_get_type()) }
    }
}
