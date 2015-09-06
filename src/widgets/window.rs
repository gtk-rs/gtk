// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Toplevel which can contain other widgets

use ffi;
use WindowType;

/**
* Window — Toplevel which can contain other widgets
*
* # Available signals:
* * `activate-default` : Action
* * `activate-focus` : Action
* * `keys-changed` : Run First
* * `set-focus` : Run Last
*/
struct_Widget!(Window);

impl Window {
    pub fn new(window_type: WindowType) -> Option<Window> {
        let tmp_pointer = unsafe { ffi::gtk_window_new(window_type) };
        check_pointer!(tmp_pointer, Window)
    }
}

impl_drop!(Window);
impl_TraitWidget!(Window);

impl ::ContainerTrait for Window {}
impl ::WindowTrait for Window {}
impl ::BinTrait for Window {}
