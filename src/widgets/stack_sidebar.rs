// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A controller for StackSidebar

use ffi;
use cast::{GTK_STACK_SIDEBAR, GTK_STACK};
use FFIWidget;
use widgets::Stack;

/// An automatic sidebar widget
struct_Widget!(StackSidebar);

impl StackSidebar {
    pub fn new() -> Option<StackSidebar> {
        let tmp_pointer = unsafe { ffi::gtk_stack_sidebar_new() };
        check_pointer!(tmp_pointer, StackSidebar)
    }

    pub fn set_stack(&self, stack:Stack) {
        unsafe {
            ffi::gtk_stack_sidebar_set_stack(GTK_STACK_SIDEBAR(self.pointer),
                                             GTK_STACK(stack.unwrap_widget()))
        }
    }

    pub fn get_stack(&self) -> Option<Stack> {
        let tmp_pointer = unsafe {
            ffi::gtk_stack_sidebar_get_stack(GTK_STACK_SIDEBAR(self.pointer))
        };

        check_pointer!(tmp_pointer, Stack)
    }
}

impl_drop!(StackSidebar);
impl_TraitWidget!(StackSidebar);

impl ::ContainerTrait for StackSidebar {}
impl ::BinTrait for StackSidebar {}
