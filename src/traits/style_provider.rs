// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::FFIGObject;

/// The priority used for default style information that is used in the absence of themes.
/// Note that this is not very useful for providing default styling for custom style classes,
/// themes are likely to override styling provided at this priority with catch-all * {...} rules.
pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK;

/// The priority used for style information provided by themes.
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME;

/// The priority used for style information provided via GtkSettings.
/// This priority is higher than STYLE_PROVIDER_PRIORITY_THEME to let settings override themes.
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS;

/// A priority that can be used when adding a GtkStyleProvider for application-specific style
/// information.
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION;

/// A priority that can be used when adding a GtkStyleProvider for application-specific style
/// information.
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER;

pub trait StyleProviderTrait : FFIGObject {}
