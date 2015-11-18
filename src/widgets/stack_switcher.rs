// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A controller for GtkStack

use ffi;
use cast::{GTK_STACK_SWITCHER, GTK_STACK};
use FFIWidget;

/// GtkStackSwitcher — A controller for GtkStack
struct_Widget!(StackSwitcher);

impl StackSwitcher {
    pub fn new() -> Option<StackSwitcher> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_stack_switcher_new() };
        check_pointer!(tmp_pointer, StackSwitcher)
    }

    pub fn set_stack(&self, stack: ::Stack) {
        unsafe {
            ffi::gtk_stack_switcher_set_stack(GTK_STACK_SWITCHER(self.pointer),
                                              GTK_STACK(stack.unwrap_widget()))
        }
    }

    pub fn get_stack(&self) -> Option<::Stack> {
        let tmp_pointer = unsafe { ffi::gtk_stack_switcher_get_stack(GTK_STACK_SWITCHER(self.pointer)) };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut _))
        }
    }
}

impl_drop!(StackSwitcher);
impl_TraitWidget!(StackSwitcher);

impl ::ContainerTrait for StackSwitcher {}
impl ::BoxTrait for StackSwitcher {}
