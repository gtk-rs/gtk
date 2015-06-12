// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::FFIGObject;

pub const PRIORITY_FALLBACK: u32 = 1;
pub const PRIORITY_THEME: u32 = 200;
pub const PRIORITY_SETTINGS: u32 = 400;
pub const PRIORITY_APPLICATION: u32 = 600;
pub const PRIORITY_USER: u32 = 800;

pub trait StyleProviderTrait : FFIGObject {}
