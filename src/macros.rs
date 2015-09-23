// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![macro_use]

macro_rules! check_pointer(
    ($tmp_pointer:ident, $gtk_struct:ident) => ( check_pointer!($tmp_pointer, $gtk_struct, G_OBJECT) );
    ($tmp_pointer:ident, $gtk_struct:ident, $cast_fn:ident) => (
        if $tmp_pointer == ::std::ptr::null_mut() {
            ::std::option::Option::None
        } else {
            unsafe {
                ::gobject_ffi::g_object_ref(
                    ::cast::$cast_fn($tmp_pointer as *mut _) as *mut ::libc::c_void);
            }

            ::std::option::Option::Some($gtk_struct {
                pointer: $tmp_pointer as *mut _
            })
        }
    );
);

macro_rules! struct_Widget(
    ($gtk_struct:ident) => (
        pub struct $gtk_struct {
            pub pointer: *mut ffi::GtkWidget
        }
    );
);

macro_rules! impl_TraitObject(
    ($gtk_struct:ident, $ffi_type:ident) => (
        impl ::glib::traits::FFIGObject for $gtk_struct {
            fn unwrap_gobject(&self) -> *mut ::gobject_ffi::GObject {
                self.pointer as *mut ::gobject_ffi::GObject
            }

            fn wrap_object(object: *mut ::gobject_ffi::GObject) -> $gtk_struct {
                unsafe{
                    ::gobject_ffi::g_object_ref(object as *mut ::libc::c_void);
                }

                $gtk_struct {
                    pointer: object as *mut ffi::$ffi_type
                }
            }
        }

        impl ::GObjectTrait for $gtk_struct {}
    );
);

macro_rules! impl_TraitWidget(
    ($gtk_struct:ident) => (
        impl ::FFIWidget for $gtk_struct {
            fn unwrap_widget(&self) -> *mut ffi::GtkWidget {
                self.pointer
            }

            fn wrap_widget(widget: *mut ffi::GtkWidget) -> $gtk_struct {
                unsafe{
                    ::gobject_ffi::g_object_ref(::ffi::cast_GtkObject(widget) as *mut ::libc::c_void);
                }

                $gtk_struct {
                    pointer: widget
                }
            }
        }

        impl ::WidgetTrait for $gtk_struct {}

        impl ::glib::traits::FFIGObject for $gtk_struct {
            fn unwrap_gobject(&self) -> *mut ::gobject_ffi::GObject {
                use ::FFIWidget;
                ::cast::G_OBJECT(self.unwrap_widget())
            }

            fn wrap_object(object: *mut ::gobject_ffi::GObject) -> $gtk_struct {
                unsafe{
                    ::gobject_ffi::g_object_ref(object as *mut ::libc::c_void);
                }

                $gtk_struct {
                    pointer: object as *mut ffi::GtkWidget
                }
            }
        }

        impl ::GObjectTrait for $gtk_struct {}
    );
);

macro_rules! impl_GObjectFunctions(
    ($gtk_struct:ident, $ffi_type:ident) => (
        impl $gtk_struct {
            pub fn unwrap_pointer(&self) -> *mut ffi::$ffi_type {
                self.pointer
            }

            pub fn wrap_pointer(pointer: *mut ffi::$ffi_type) -> $gtk_struct {
                $gtk_struct {
                    pointer: pointer
                }
            }
        }
    )
);

macro_rules! impl_drop(
    ($gtk_struct:ident) => ( impl_drop!($gtk_struct, GTK_WIDGET); );
    ($gtk_struct:ident, $cast_func:ident) => (
        impl Drop for $gtk_struct {
            fn drop(&mut self) {
                unsafe {
                    ::gobject_ffi::g_object_unref(self.pointer as *mut ::libc::c_void);
                }
            }
        }

        impl Clone for $gtk_struct {
            fn clone(&self) -> $gtk_struct {
                let pointer = unsafe {
                    ::gobject_ffi::g_object_ref(self.pointer as *mut ::libc::c_void)
                };

                unsafe {
                    $gtk_struct {
                        pointer: ::cast::$cast_func(::std::mem::transmute(pointer))
                    }
                }
            }
        }
    );
);

// Useful for function wich take a valid widget or NULL for a default widget
// takes an option<&trait::Widget> and return the c widget pointer or ptr::null()
macro_rules! unwrap_widget(
    ($w:ident) => (
        match $w {
            Some(ref _w) => _w.unwrap_widget(),
            None => ::std::ptr::null_mut()
        };
    );
);
