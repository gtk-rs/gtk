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
#![allow(unused_variables)]

use std::mem::transmute;
use libc::c_void;
use ffi;
use gdk;
use glib;
use cairo;
use std::any::Any;

pub use glib::traits::Signal;

// The defintion of the signal macro is split in a argumentless and
// one-with-arguments case because of a limitation in the rust macro
// system.
macro_rules! signal(
    //Signals without arguments
    ($signal:ident, $class:ident () -> $ret_type:ty) => (

        //General case (see below)
        signal!($signal, $class [] -> $ret_type);

        mod $signal {
            use std::mem::transmute;
            use libc::c_void;
            use ffi;
            use gdk;
            use glib;
            use cairo;
            use std::any::Any;

            pub extern fn trampoline<'a>(widget : *mut ffi::C_GtkWidget, signal: *mut Box<super::Signal>) -> $ret_type {
                unsafe {
                    let t : &'a Box<super::Signal> = ::std::mem::transmute(signal);

                    match t.get_user_data() {
                        &Some(ref user_data) => {
                            let cb : *mut FnMut(*const c_void) -> $ret_type = transmute((*signal).fetch_cb());
                            (*cb)(::std::mem::transmute(user_data))
                        },
                        &None => {
                            let cb : *mut FnMut() -> $ret_type = transmute((*signal).fetch_cb());
                            (*cb)()
                        },
                    }
                }
            }
        }
    );

    //Signals with arguments
    ($signal:ident, $class:ident ( $($arg_name:ident : $arg_type:ty),* ) -> $ret_type:ty) => (
        //General case (see below)
        signal!($signal, $class [$(($arg_name : $arg_type)),*] -> $ret_type);

        mod $signal {
            use std::ops::Deref;
            use std::mem::transmute;
            use libc::c_void;
            use ffi;
            use gdk;
            use glib;
            use cairo;
            use std::any::Any;

            pub extern fn trampoline<'a>(widget : *mut ffi::C_GtkWidget, $($arg_name : $arg_type),* , signal: *mut Box<super::Signal>) -> $ret_type {
                unsafe {
                    let t : &'a Box<super::Signal> = ::std::mem::transmute(signal);

                    match t.get_user_data() {
                        &Some(ref user_data) => {
                            let cb : *mut FnMut($($arg_type),*, *const c_void) -> $ret_type = transmute((*signal).fetch_cb());
                            (*cb)($($arg_name),*, ::std::mem::transmute(user_data))
                        },
                        &None => {
                            let cb : *mut FnMut($($arg_type),*) -> $ret_type = transmute((*signal).fetch_cb());
                            (*cb)($($arg_name),*)
                        },
                    }
                }
            }
        }
    );

    //TODO custom trampoline
    /*($signal:ident, $class:ident ( $($arg_name:ident : $arg_type:ty),* ) -> $ret_type:ty,
                    trampoline   ( $($t_arg_nm:ident : $t_arg_ty:ty),* ) -> $t_ret_ty:ty $t_blck:expr) => (

        //General case (see below)
        signal!($signal, $class [$(($arg_name : $arg_type)),*] -> $ret_type);

        mod $signal{
            #[allow(unused_imports)]
            #[allow(unused_variables)]

            use std::mem::transmute;
            use libc::c_void;
            use gdk;
            use glib;
            use cairo;
            use std::any::Any;


            pub extern fn trampoline<'a>(widget: *mut ffi::C_GtkWidget, $($t_arg_nm : $t_arg_ty),* , signal: *mut Box<super::Signal>) -> $t_ret_ty {
                unsafe {
                    let t : &'a Box<super::Signal> = ::std::mem::transmute(signal);

                    match t.get_user_data() {
                        &Some(ref user_data) => {
                            let cb: *mut FnMut($($arg_type),*, *const c_void) -> $ret_type = transmute((*signal).fetch_cb());

                            let cont = |$($arg_name: $arg_type),*| {
                                (*cb)($($arg_name),* , ::std::mem::transmute(user_data))
                            };

                            let custom_trampoline = $t_blck;

                            custom_trampoline(cont)
                        },
                        &None => {
                            let cb : *mut FnMut($($arg_type),*) -> $ret_type = transmute((*signal).fetch_cb());

                            let cont = |$($arg_name: $arg_type),*| {
                                (*cb)($($arg_name),*)
                            };

                            let custom_trampoline = $t_blck;

                            custom_trampoline(cont)
                        },
                    }
                }
            }
        }
    );*/

    //General case
    ($signal:ident, $class:ident [ $(($arg_name:ident : $arg_type:ty)),* ] -> $ret_type:ty) => (
        pub struct $class<'a> {
            pub cb: &'a mut (FnMut($($arg_type),*) -> $ret_type + 'a),
            pub user_data: Option<Box<Any>>
        }

        impl<'a> $class<'a> {
            pub fn new (cb : &'a mut (FnMut($($arg_type),*) -> $ret_type + 'a)) -> Box<$class<'a>> {
                Box::new( $class {
                    cb: cb,
                    user_data: None
                })
            }

            //TODO: Rust lexer bug here, can't parse the middel `,` in `|$($arg_type),* , Box<Any>|`
            /*pub fn new_with_data (user_data: Box<Any>, cb: |$($arg_type),*, Box<Any>|:'a -> $ret_type) -> Box<Signal> {
                box $class{
                    cb: cb,
                    user_data: user_data
                } as Box<Signal>
            }*/
        }

        impl<'a> Signal<'a> for $class<'a> {
            fn get_signal_name(&self) -> &str {
                stringify!($signal)
            }

            fn get_trampoline(&self) -> extern "C" fn() {
                unsafe {
                    transmute(self::$signal::trampoline)
                }
            }

            fn fetch_cb(&self) -> *mut FnMut() {
                unsafe {
                    transmute(&(*self.cb))
                }
            }

            fn get_user_data(&'a self) -> &'a Option<Box<Any>> {
                &self.user_data
            }
        }
    );
);

//GObject
//https://developer.gnome.org/gobject/unstable/gobject-The-Base-Object-Type.html#gobject-The-Base-Object-Type.signals
signal!(notify, Notify(g_param_spec:c_void) -> ());

//GtkWidget
//https://developer.gnome.org/gtk3/stable/GtkWidget.html#GtkWidget.signals

signal!(accel_closures_changed, AccelClosuresChanged(spec : glib::ParamSpec) -> ());
signal!(can_activate_accel,     CanActivateAccel(signal_id:usize) -> bool);
signal!(child_notify,           ChildNotify(spec : glib::ParamSpec) -> ());
signal!(composited_changed,     CompositedChanged() -> ());
signal!(destroy,                Destroy() -> ());
signal!(direction_changed,      DirectionChanged(previous_direction: ::TextDirection) -> ());
signal!(draw,                   Draw(ctx: cairo::Context) -> ());//, trampoline(ctx_raw: *mut cairo::ffi::cairo_t) -> () |cb| {
//    cb(cairo::Context::wrap(ctx_raw))
//});
signal!(focus,                  Focus(direction : ::DirectionType) -> bool);
signal!(grab_focus,             GrabFocus() -> ());
signal!(grab_notify,            GrabNotify(was_grabbed : bool) -> ());
signal!(hide,                   Hide() -> ());
signal!(keynav_failed,          KeynavFailed(direction : ::DirectionType) -> bool);
signal!(map,                    Map() -> ());
signal!(mnemonic_activate,      MnemonicActivate(arg : bool) -> bool);
signal!(move_focus,             MoveFocus(direction : ::DirectionType) -> ());
signal!(popup_menu,             PopupMenu() -> bool);
signal!(query_tooltip,          QueryTooltip(x:isize, y:isize, keyboard_mode:bool, tooltip : *mut ::Tooltip) -> bool);
signal!(realize,                Realize() -> ());
signal!(screen_changed,         ScreenChanged(previous_screen : *mut gdk::Screen) -> ());
signal!(show,                   Show() -> ());
signal!(show_help,              ShowHelp(help_type : ::WidgetHelpType) -> bool);
signal!(size_allocate,          SizeAllocate(allocation : *mut gdk::Rectangle) -> ());
signal!(state_changed,          StateChanged(state : ::StateType) -> ());
signal!(state_flags_changed,    StateFlagsChanged(flags : ::StateFlags) -> ());
signal!(style_updated,          StyleUpdated() -> ());
signal!(unmap,                  Unmap() -> ());
signal!(unrealize,              Unrealize() -> ());

/*
signal!(hierarchy_changed, HierarchyChanged(previous_toplevel : *::Widget) -> (),
    trampoline (previous_toplevel : *mut ffi::C_GtkWidget) -> Box<::Widget> {
        cb(previous_toplevel)
    }
)
signal!(parent_set,    ParentSet(old_parent : *::Widget) -> ())
*/

//GtkWidget: GDK events
signal!(button_press_event,     ButtonPressEvent(event : *mut gdk::EventButton) -> bool);
signal!(button_release_event,   ButtonReleaseEvent(event : *mut gdk::EventButton) -> bool);
signal!(configure_event,        ConfigureEvent(event : *mut gdk::EventConfigure) -> bool);
signal!(damage_event,           DamageEvent(event : *mut gdk::EventExpose) -> bool);
signal!(delete_event,           DeleteEvent(event : *mut gdk::EventAny) -> bool);
signal!(destroy_event,          DestroyEvent(event : *mut gdk::EventAny) -> bool);
signal!(enter_notify_event,     EnterNotifyEvent(event : *mut gdk::EventCrossing) -> bool);
signal!(leave_notify_event,     LeaveNotifyEvent(event : *mut gdk::EventCrossing) -> bool);
signal!(event,                  Event(event : *mut gdk::EventAny) -> bool);
signal!(event_after,            EventAfter(event : *mut gdk::EventAny) -> bool);
signal!(focus_in_event,         FocusInEvent(event : *mut gdk::EventFocus) -> bool);
signal!(focus_out_event,        FocusOutEvent(event : *mut gdk::EventFocus) -> bool);
signal!(grab_broken_event,      GrabBrokenEvent(event : *mut gdk::EventGrabBroken) -> bool);
signal!(key_press_event,        KeyPressEvent(event : *mut gdk::EventKey) -> bool);
signal!(key_release_event,      KeyReleaseEvent(event : *mut gdk::EventKey) -> bool);
signal!(map_event,              MapEvent(event : *mut gdk::EventAny) -> bool);
signal!(motion_notify_event,    MotionNotifyEvent(event : *mut gdk::EventMotion) -> bool);
signal!(property_notify_event,  PropertyNotifyEvent(event : *mut gdk::EventProperty) -> bool);
signal!(proximity_in_event,     ProximityInEvent(event : *mut gdk::EventProximity) -> bool);
signal!(proximity_out_event,    ProximityOutEvent(event : *mut gdk::EventProximity) -> bool);
signal!(scroll_event,           ScrollEvent(event : *mut gdk::EventScroll) -> bool);
signal!(touch_event,            TouchEvent(event : *mut gdk::EventTouch) -> bool);
signal!(unmap_event,            UnmapEvent(event : *mut gdk::EventAny) -> bool);
signal!(window_state_event,     WindowStateEvent(event : *mut gdk::EventWindowState) -> bool);

//GtkWidget: Drag-drop
/*
signal!(drag_begin,         DragBegin(context : *mut gdk::DragContext) -> ())
signal!(drag_data_delete,   DragDataDelete(context : *mut gdk::DragContext) -> ())
signal!(drag_data_get,      DragDataGet(context : *mut gdk::DragContext,
                                        data : *::SelectionData,
                                        info: usize,
                                        time: usize) -> ())
signal!(drag_data_received, DragDataReceived(context : *mut gdk::DragContext,
                                             x: isize,
                                             y: isize,
                                             data: *::SelectionData,
                                             info: usize,
                                             time: usize) -> ())
signal!(drag_drop,          DragDrop(context : *mut gdk::DragContext,
                                     x: isize,
                                     y: isize,
                                     time: usize) -> ())
signal!(drag_end,           DragEnd(context : *mut gdk::DragContext) -> ())
signal!(drag_failed,        DragFailed(context : *mut gdk::DragContext, result: ::DragResult) -> ())
signal!(drag_leave,         DragLeave(context : *mut gdk::DragContext, time : usize) -> ())
signal!(drag_motion,        DragMotion(context : *mut gdk::DragContext,
                                       x : isize,
                                       y : isize,
                                       time : usize) -> bool)

//GtkWidget: Selection
signal!(selection_get,          SelectionGet(data : *::SelectionData, info: usize, time: usize) -> ())
signal!(selection_received,     SelectionReceived(data : *::SelectionData, time: usize) -> ())

signal!(selection_clear_event,  SelectionClearEvent(event : *mut gdk::EventSelection) -> bool)
signal!(selection_request_event,SelectionRequestEvent(event : *mut gdk::EventSelection) -> bool)
signal!(selection_notify_event, SelectionNotifyEvent(event : *mut gdk::EventSelection) -> bool)
*/

//GtkTreeSelection
signal!(changed,            Changed() -> ());

//GtkTreeView
signal!(columns_changed,            ColumnsChanged() -> ());
signal!(cursor_changed,             CursorChanged() -> ());
signal!(expand_collapse_cursor_row, ExpandCollapseCursorRow(arg1: ffi::Gboolean, arg2: ffi::Gboolean, arg3: ffi::Gboolean) -> ffi::Gboolean);
signal!(move_cursor,                MoveCursor(step: ffi::C_GtkMovementStep, direction: i32) -> ffi::Gboolean);
signal!(row_activated,              RowActivated(path: *mut ffi::C_GtkTreePath, column: *mut ffi::C_GtkTreeViewColumn) -> ());
signal!(row_collapsed,              RowCollapsed(iter: *mut ffi::C_GtkTreeIter, path: *mut ffi::C_GtkTreePath) -> ());
signal!(row_expanded,               RowExpanded(iter: *mut ffi::C_GtkTreeIter, path: *mut ffi::C_GtkTreePath) -> ());
signal!(select_all,                 SelectAll() -> ffi::Gboolean);
signal!(select_cursor_parent,       SelectCursorParent() -> ffi::Gboolean);
signal!(select_cursor_row,          SelectCursorRow(arg1: ffi::Gboolean) -> ffi::Gboolean);
signal!(start_interactive_search,   StartInteractiveSearch() -> ffi::Gboolean);
signal!(test_collapse_row,          TestCollapseRow(iter: *mut ffi::C_GtkTreeIter, path: *mut ffi::C_GtkTreePath) -> ffi::Gboolean);
signal!(test_expand_row,            TestExpandRow(iter: *mut ffi::C_GtkTreeIter, path: *mut ffi::C_GtkTreePath) -> ffi::Gboolean);
signal!(toggle_cursor_row,          ToggleCursorRow() -> ffi::Gboolean);
signal!(unselect_all,               UnselectAll() -> ffi::Gboolean);

//GtkContainer
//signal!(add,                Add(widget: *::Widget) -> ())
signal!(check_resize,       CheckResize() -> ());
//signal!(remove,             Remove(widget: *::Widget) -> ())
//signal!(set_focus_child,    SetFocusChild(widget: *::Widget) -> ())

//GtkButton
signal!(activate,           Activate() -> ());
signal!(clicked,            Clicked() -> ());
signal!(enter,              Enter() -> ());
signal!(leave,              Leave() -> ());
signal!(pressed,            Pressed() -> ());
signal!(released,           Released() -> ());
//GtkDialog
signal!(response,           Response(response_id : isize) -> ());

//GtkAdjustment
//https://developer.gnome.org/gtk3/stable/GtkAdjustment.html#GtkAdjustment.signals
signal!(value_changed,      ValueChanged() -> ());

// SpinButton
signal!(changed_value, ChangedValue() -> ());
signal!(wrapped, Wrapped() -> ());
//gint    input   Run Last
//gboolean    output  Run Last

// Range
signal!(adjust_bounds, AdjustBounds() -> ());
signal!(move_slider, MoveSlider() -> ());
