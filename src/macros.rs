// Copyright 2013-2015, The Rust-GNOME Project Developers.
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
                ::glib::ffi::g_object_ref(::cast::$cast_fn($tmp_pointer));
            }

            ::std::option::Option::Some($gtk_struct {
                pointer: $tmp_pointer
            })
        }
    );
);

macro_rules! struct_Widget(
    ($gtk_struct:ident) => (
        pub struct $gtk_struct {
            pointer: *mut ffi::C_GtkWidget
        }
    );
);

macro_rules! impl_TraitObject(
    ($gtk_struct:ident, $ffi_type:ident) => (
        impl ::glib::traits::FFIGObject for $gtk_struct {
            fn unwrap_gobject(&self) -> *mut ::glib::ffi::C_GObject {
                self.pointer as *mut ::glib::ffi::C_GObject
            }

            fn wrap_object(object: *mut ::glib::ffi::C_GObject) -> $gtk_struct {
                unsafe{
                    ::glib::ffi::g_object_ref(object);
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
            fn unwrap_widget(&self) -> *mut ffi::C_GtkWidget {
                self.pointer
            }

            fn wrap_widget(widget: *mut ffi::C_GtkWidget) -> $gtk_struct {
                unsafe{
                    ::glib::ffi::g_object_ref(::ffi::cast_GtkObject(widget));
                }

                $gtk_struct {
                    pointer: widget
                }
            }
        }

        impl ::WidgetTrait for $gtk_struct {}

        impl ::glib::traits::FFIGObject for $gtk_struct {
            fn unwrap_gobject(&self) -> *mut ::glib::ffi::C_GObject {
                use ::FFIWidget;
                ::cast::G_OBJECT(self.unwrap_widget())
            }

            fn wrap_object(object: *mut ::glib::ffi::C_GObject) -> $gtk_struct {
                unsafe{
                    ::glib::ffi::g_object_ref(object);
                }

                $gtk_struct {
                    pointer: object as *mut ffi::C_GtkWidget
                }
            }
        }

        impl ::GObjectTrait for $gtk_struct {}

        // impl ::glib::traits::Connect for $gtk_struct {}

    );
);

// macro_rules! impl_connect(
//     ($gtk_struct:ident -> $($signal_name:ident),*) => (
//         $(impl<'a> ::glib::traits::Connect<::signals::$signal_name<'a>> for $gtk_struct {})*

//     )
// )

macro_rules! impl_connect(
    ($gtk_struct:ident -> $($signal_name:ident),*) => (
        $(impl<'a> ::glib::traits::Connect<'a, ::signals::$signal_name<'a>> for $gtk_struct {})*
    )
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
                    ::glib::ffi::g_object_unref(self.pointer as *mut ::glib::ffi::C_GObject);
                }
            }
        }

        impl Clone for $gtk_struct {
            fn clone(&self) -> $gtk_struct {
                let pointer = unsafe {
                    ::glib::ffi::g_object_ref(self.pointer as *mut ::glib::ffi::C_GObject)
                };

                $gtk_struct {
                    pointer: ::cast::$cast_func(pointer)
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

macro_rules! impl_widget_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> ButtonPressEvent, ButtonReleaseEvent, CanActivateAccel,
                                     ChildNotify, CompositedChanged, ConfigureEvent, DamageEvent,
                                     DeleteEvent, DestroyEvent, DirectionChanged, Draw, EnterNotifyEvent,
                                     Event, EventAfter, Focus, FocusInEvent, FocusOutEvent, GrabBrokenEvent,
                                     GrabFocus, GrabNotify, Hide, KeyPressEvent,
                                     KeyReleaseEvent, KeynavFailed, LeaveNotifyEvent, Map, MapEvent,
                                     MnemonicActivate, MotionNotifyEvent, MoveFocus,
                                     PropertyNotifyEvent, ProximityInEvent, ProximityOutEvent,
                                     QueryTooltip, Realize, ScreenChanged, ScrollEvent,
                                     Show, SizeAllocate,
                                     StateFlagsChanged, StyleUpdated,
                                     TouchEvent, WindowStateEvent);


        // not implemented:
        // DragBegin, DragDataDelete, DragDataGet, DragDataReceived, DragDrop
        // DragEnd, DragFailed, DragLeave, DragMotion, HierarchyChanged, ParentSet, PopuMenu,
        // UnRealize, VisibilityNotifyEvent, UnMap, UnMapEvent, StyleSet, StateChanged, SelectionRequestEvent,
        // SelectionReceived, SelectionClearEvent, SelectionGet, SelectionNotifyEvent,
    )
);

macro_rules! impl_tree_view_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> ColumnsChanged, CursorChanged, ExpandCollapseCursorRow,
                                     MoveCursor, RowActivated, RowCollapsed, RowExpanded,
                                     SelectAll, SelectCursorParent, SelectCursorRow,
                                     StartInteractiveSearch, TestCollapseRow, TestExpandRow,
                                     ToggleCursorRow, UnselectAll);
    )
);

macro_rules! impl_button_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> Activate, Clicked, Enter, Leave, Pressed, Released);
    )
);

macro_rules! impl_range_events(
    ($gtk_struct:ident) => (
        impl_connect!($gtk_struct -> AdjustBounds, MoveSlider, ValueChanged);

    // ChangeValue
    )
);
