// Copyright 2015, The Rust-gnome Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

//! A widget

use ffi;

/// A widget
struct_Widget!(Widget);

impl_drop!(Widget);
impl_TraitWidget!(Widget);
