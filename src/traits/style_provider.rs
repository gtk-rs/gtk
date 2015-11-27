// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::FFIGObject;

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK;

pub const STYLE_PROVIDER_PRIORITY_THEME: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME;

pub const STYLE_PROVIDER_PRIORITY_SETTINGS: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS;

pub const STYLE_PROVIDER_PRIORITY_APPLICATION: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION;

pub const STYLE_PROVIDER_PRIORITY_USER: i32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER;

pub trait StyleProviderTrait : FFIGObject {}
