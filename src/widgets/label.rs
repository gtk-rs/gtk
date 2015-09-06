// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A widget that displays a small to medium amount of text

use glib::translate::ToGlibPtr;
use ffi;

/// Label — A widget that displays a small to medium amount of text
/*
* # Available signals:
* * `activate-current-link` : Action
* * `activate-link` : Run Last
* * `copy-clipboard` : Action
* * `move-cursor` : Action
* * `populate-popup` : Run Last
*/
struct_Widget!(Label);

impl Label {
    pub fn new(text: &str) -> Option<Label> {
        let tmp_pointer = unsafe {
            ffi::gtk_label_new(text.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Label)
    }

    pub fn new_with_mnemonic(text: &str) -> Option<Label> {
        let tmp_pointer = unsafe {
            ffi::gtk_label_new_with_mnemonic(text.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, Label)
    }
}

impl_drop!(Label);
impl_TraitWidget!(Label);

impl ::MiscTrait for Label {}
impl ::LabelTrait for Label {}
