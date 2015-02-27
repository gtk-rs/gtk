// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

#![allow(unused_imports)]

use gtk::{self, ffi};
use gtk::cast::{GTK_APP_LAUNCH_CONTEXT};
use std::str;
use std::string;

struct_Widget!(AppLaunchContext);

impl AppLaunchContext {/*
    pub fn new() -> Option<AppLaunchContext> {
        let tmp_pointer = unsafe { ffi::g_app_launch_context_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::FFIWidget::wrap_widget(tmp_pointer as *mut gtk::ffi::C_GtkWidget))
        }
    }

    pub fn setenv(&self, variable: &str, value: &str) -> () {
        unsafe {
            variable.with_c_str(|c_variable| {
                value.with_c_str(|c_value| {
                    ffi::g_app_launch_context_setenv(GTK_APP_LAUNCH_CONTEXT(self.unwrap_widget()), c_variable, c_value)
                })
            })
        }
    }

    pub fn unsetenv(&self, variable: &str) -> () {
        unsafe {
            variable.with_c_str(|c_variable| {
                ffi::g_app_launch_context_unsetenv(GTK_APP_LAUNCH_CONTEXT(self.unwrap_widget()), c_variable)
            })
        }
    }

    pub fn get_environment(&self) -> Vec<String> {
        let env = unsafe { ffi::g_app_launch_context_get_environment(GTK_APP_LAUNCH_CONTEXT(self.unwrap_widget())) };
        let mut ret = Vec::new();

        if env.is_not_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = env.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(FromCStr::from_raw_buf(*tmp));
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn get_display(&self, info: &gtk::AppInfo, files: &glib::List) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_launch_context_get_display(GTK_APP_LAUNCH_CONTEXT(self.unwrap_widget()), GTK_APP_INFO(info.unwrap_widget()), files.unwrap()) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FromCStr::from_raw_buf(tmp_pointer))
        }
    }

    pub fn get_startup_notify_id(&self, app_info: &gtk::AppInfo, files: &glib::List) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_launch_context_get_startup_notify_id(GTK_APP_LAUNCH_CONTEXT(self.unwrap_widget()), GTK_APP_INFO(app_info.unwrap_widget()), files.unwrap()) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FromCStr::from_raw_buf(tmp_pointer))
        }
    }

    pub fn launch_failed(&self, startup_notify_id: &str) -> () {
        unsafe {
            startup_notify_id.with_c_str(|c_str|{
                ffi::g_app_launch_context_launch_failed(GTK_APP_LAUNCH_CONTEXT(self.unwrap_widget()), c_str)
            })
        }
    }*/
}

impl_drop!(AppLaunchContext);
impl_TraitWidget!(AppLaunchContext);
