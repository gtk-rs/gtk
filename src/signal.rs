// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::mem::transmute;

use glib::translate::*;

use glib_ffi::{self, gboolean, gpointer};
use gdk::ModifierType;

use {
    CellEditable,
    CellRenderer,
    Continue,
    Rectangle,
    ScrollType,
    SpinButton,
    TreeIter,
    TreeModel,
    TreePath,
    TreeViewColumn,
};

/// Whether to propagate the signal to the default handler.
///
/// Don't inhibit default handlers without a reason, they're usually helpful.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

#[doc(hidden)]
impl ToGlib for Inhibit {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

// idle_add and timeout_add fixed to the main thread

unsafe extern "C" fn trampoline(func: gpointer) -> gboolean {
    callback_guard!();
    let func: &RefCell<Box<FnMut() -> Continue + 'static>> = transmute(func);
    (&mut *func.borrow_mut())().to_glib()
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    callback_guard!();
    Box::<RefCell<Box<FnMut() -> Continue + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() -> Continue + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut() -> Continue + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// Similar to `glib::idle_add` but only callable from the main thread and
/// doesn't require `Send`.
pub fn idle_add<F>(func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        glib_ffi::g_idle_add_full(glib_ffi::G_PRIORITY_DEFAULT_IDLE, Some(trampoline),
            into_raw(func), Some(destroy_closure))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// Similar to `glib::timeout_add` but only callable from the main thread and
/// doesn't require `Send`.
pub fn timeout_add<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        glib_ffi::g_timeout_add_full(glib_ffi::G_PRIORITY_DEFAULT, interval, Some(trampoline),
            into_raw(func), Some(destroy_closure))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// Similar to `glib::timeout_add_seconds` but only callable from the main thread and
/// doesn't require `Send`.
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> u32
    where F: FnMut() -> Continue + 'static {
    assert_initialized_main_thread!();
    unsafe {
        glib_ffi::g_timeout_add_seconds_full(glib_ffi::G_PRIORITY_DEFAULT, interval,
            Some(trampoline), into_raw(func), Some(destroy_closure))
    }
}

pub trait CellRendererToggleSignals {
    fn connect_toggled<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> u64;
}

mod cell_renderer_toggle {
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use libc::c_char;
    use ffi::{GtkCellRendererToggle, gtk_tree_path_new_from_string};
    use {CellRendererToggle, TreePath};

    impl super::CellRendererToggleSignals for CellRendererToggle {
        fn connect_toggled<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "toggled",
                    transmute(string_path_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn string_path_trampoline(this: *mut GtkCellRendererToggle, c_str_path: *const c_char,
            f: &Box<Fn(&CellRendererToggle, &TreePath) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(c_str_path));
        f(&from_glib_none(this), &path);
    }
}

pub trait TreeViewSignals {
    fn connect_columns_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_cursor_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
    fn connect_expand_collapse_cursor_row<F: Fn(&Self, bool, bool, bool) -> bool + 'static>(&self, f: F)
        -> u64;
    fn connect_row_activated<F: Fn(&Self, &TreePath, &TreeViewColumn) + 'static>(&self, f: F) -> u64;
    fn connect_row_collapsed<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64;
    fn connect_row_expanded<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64;
    fn connect_select_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_select_cursor_parent<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_select_cursor_row<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_start_interactive_search<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_test_collapse_row<F: Fn(&Self, &TreeIter, &TreePath) -> Inhibit + 'static>(&self, f: F)
        -> u64;
    fn connect_test_expand_row<F: Fn(&Self, &TreeIter, &TreePath) -> Inhibit + 'static>(&self, f: F)
        -> u64;
    fn connect_toggle_cursor_row<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
    fn connect_unselect_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;
}

mod tree_view {
    use std::mem::transmute;
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::gboolean;
    use ffi::{GtkTreeIter, GtkTreePath, GtkTreeView, GtkTreeViewColumn};
    use super::Inhibit;
    use {TreeIter, TreePath, TreeView, TreeViewColumn};

    impl super::TreeViewSignals for TreeView {
        fn connect_columns_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "columns-changed",
                    transmute(void_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_cursor_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "cursor-changed",
                    transmute(void_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_expand_collapse_cursor_row<F: Fn(&Self, bool, bool, bool) -> bool + 'static>(&self,
                f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, bool, bool, bool) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "expand-collapse-cursor-row",
                    transmute(bool3_bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_row_activated<F: Fn(&Self, &TreePath, &TreeViewColumn) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreePath, &TreeViewColumn) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "row-activated",
                    transmute(path_column_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_row_collapsed<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "row-collapsed",
                    transmute(iter_path_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_row_expanded<F: Fn(&Self, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "row-expanded",
                    transmute(iter_path_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_select_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "select-all",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_select_cursor_parent<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "select-cursor-parent",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_select_cursor_row<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, bool) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "select-cursor-row",
                    transmute(bool_bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_start_interactive_search<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "start-interactive-search",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_test_collapse_row<F: Fn(&Self, &TreeIter, &TreePath) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) -> Inhibit + 'static>> =
                    Box::new(Box::new(f));
                connect(self.to_glib_none().0, "test-collapse-row",
                    transmute(iter_path_inhibit_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_test_expand_row<F: Fn(&Self, &TreeIter, &TreePath) -> Inhibit + 'static>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeIter, &TreePath) -> Inhibit + 'static>> =
                    Box::new(Box::new(f));
                connect(self.to_glib_none().0, "test-expand-row",
                    transmute(iter_path_inhibit_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_toggle_cursor_row<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "toggle-cursor-row",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_unselect_all<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self) -> bool + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "unselect-all",
                    transmute(bool_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn void_trampoline(this: *mut GtkTreeView, f: &Box<Fn(&TreeView) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this));
    }

    unsafe extern "C" fn bool_trampoline(this: *mut GtkTreeView, f: &Box<Fn(&TreeView) -> bool + 'static>)
            -> gboolean {
        callback_guard!();
        f(&from_glib_none(this)).to_glib()
    }

    unsafe extern "C" fn bool_bool_trampoline(this: *mut GtkTreeView, arg1: gboolean,
            f: &Box<Fn(&TreeView, bool) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), from_glib(arg1)).to_glib()
    }

    unsafe extern "C" fn bool3_bool_trampoline(this: *mut GtkTreeView, arg1: gboolean, arg2: gboolean,
            arg3: gboolean, f: &Box<Fn(&TreeView, bool, bool, bool) -> bool + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), from_glib(arg1), from_glib(arg2),
            from_glib(arg3)).to_glib()
    }

    unsafe extern "C" fn path_column_trampoline(this: *mut GtkTreeView, path: *mut GtkTreePath,
            column: *mut GtkTreeViewColumn,
            f: &Box<Fn(&TreeView, &TreePath, &TreeViewColumn) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), &from_glib_borrow(path), &from_glib_none(column));
    }

    unsafe extern "C" fn iter_path_trampoline(this: *mut GtkTreeView, iter: *mut GtkTreeIter,
            path: *mut GtkTreePath, f: &Box<Fn(&TreeView, &TreeIter, &TreePath) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path));
    }

    unsafe extern "C" fn iter_path_inhibit_trampoline(this: *mut GtkTreeView, iter: *mut GtkTreeIter,
            path: *mut GtkTreePath,
            f: &Box<Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static>) -> gboolean {
        callback_guard!();
        f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path)).to_glib()
    }
}

pub trait CellAreaSignals {
    fn connect_add_editable<F>(&self, add_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static;
    fn connect_apply_attributes<F>(&self, apply_attributes_func: F) -> u64
        where F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static;
    fn connect_focus_changed<F>(&self, focus_changed_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, TreePath) + 'static;
    fn connect_remove_editable<F>(&self, remove_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable) + 'static;
}

mod cell_area {
    use CellArea;
    use CellEditable;
    use CellRenderer;
    use TreeIter;
    use TreeModel;
    use TreePath;
    use Rectangle;
    use std::mem::transmute;
    use ffi::{GtkCellArea, GtkCellEditable, GtkCellRenderer, GtkTreeIter, GtkTreeModel};
    use ffi::gtk_tree_path_new_from_string;
    use gdk_ffi::GdkRectangle;
    use glib_ffi::gboolean;
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use IsA;
    use libc::c_char;

    impl super::CellAreaSignals for CellArea {
        fn connect_add_editable<F>(&self, add_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static>> =
                    Box::new(Box::new(add_editable_func));
                connect(self.to_glib_none().0, "add-editable",
                    transmute(add_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_apply_attributes<F>(&self, apply_attributes_func: F) -> u64
        where F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>> =
                    Box::new(Box::new(apply_attributes_func));
                connect(self.to_glib_none().0, "apply-attributes",
                    transmute(apply_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_focus_changed<F>(&self, focus_changed_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellRenderer, TreePath) + 'static>> =
                    Box::new(Box::new(focus_changed_func));
                connect(self.to_glib_none().0, "focus-changed",
                    transmute(focus_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_remove_editable<F>(&self, remove_editable_func: F) -> u64
        where F: Fn(&Self, &CellRenderer, &CellEditable) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellRenderer, &CellEditable) + 'static>> =
                    Box::new(Box::new(remove_editable_func));
                connect(self.to_glib_none().0, "remove-editable",
                    transmute(remove_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn add_trampoline<T>(this: *mut GtkCellArea,
                                           renderer: *mut GtkCellRenderer,
                                           editable: *mut GtkCellEditable,
                                           cell_area: *mut GdkRectangle,
                                           path: *const c_char,
                                           f: &Box<Fn(&T, &CellRenderer, &CellEditable, &Rectangle, TreePath) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &CellRenderer::from_glib_none(renderer),
          &CellEditable::from_glib_none(editable),
          &Rectangle { x: (*cell_area).x, y: (*cell_area).y, width: (*cell_area).width, height: (*cell_area).height },
          path);
    }

    unsafe extern "C" fn apply_trampoline<T>(this: *mut GtkCellArea,
                                             model: *mut GtkTreeModel,
                                             iter: *mut GtkTreeIter,
                                             is_expander: gboolean,
                                             is_expanded: gboolean,
                                             f: &Box<Fn(&T, &TreeModel, &TreeIter, bool, bool) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &TreeModel::from_glib_none(model),
          &TreeIter::from_glib_borrow(iter),
          from_glib(is_expander),
          from_glib(is_expanded));
    }

    unsafe extern "C" fn focus_trampoline<T>(this: *mut GtkCellArea,
                                             renderer: *mut GtkCellRenderer,
                                             path: *const c_char,
                                             f: &Box<Fn(&T, &CellRenderer, TreePath) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &CellRenderer::from_glib_none(renderer),
          path);
    }

    unsafe extern "C" fn remove_trampoline<T>(this: *mut GtkCellArea,
                                              renderer: *mut GtkCellRenderer,
                                              editable: *mut GtkCellEditable,
                                              f: &Box<Fn(&T, &CellRenderer, &CellEditable) + 'static>)
    where T: IsA<CellArea> {
        callback_guard!();
        f(&CellArea::from_glib_none(this).downcast_unchecked(),
          &CellRenderer::from_glib_none(renderer),
          &CellEditable::from_glib_none(editable));
    }
}

pub trait CellRendererAccelSignals {
    fn connect_accel_cleared<F>(&self, accel_cleared_func: F) -> u64
        where F: Fn(&Self, TreePath) + 'static;
    fn connect_accel_edited<F>(&self, accel_edited_func: F) -> u64
        where F: Fn(&Self, TreePath, u32, ModifierType, u32) + 'static;
}

mod cell_renderer_accel {
    use CellRendererAccel;
    use TreePath;
    use std::mem::transmute;
    use ffi::GtkCellRendererAccel;
    use glib::signal::connect;
    use glib::translate::*;
    use gdk::ModifierType;
    use ffi::gtk_tree_path_new_from_string;
    use gdk_ffi::GdkModifierType;
    use libc::{c_char, c_uint};

    impl super::CellRendererAccelSignals for CellRendererAccel {
        fn connect_accel_cleared<F>(&self, accel_cleared_func: F) -> u64
        where F: Fn(&Self, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, TreePath) + 'static>> =
                    Box::new(Box::new(accel_cleared_func));
                connect(self.to_glib_none().0, "accel-cleared",
                    transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_accel_edited<F>(&self, accel_edited_func: F) -> u64
        where F: Fn(&Self, TreePath, u32, ModifierType, u32) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, TreePath, u32, ModifierType, u32) + 'static>> =
                    Box::new(Box::new(accel_edited_func));
                connect(self.to_glib_none().0, "accel-edited",
                    transmute(e_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline(this: *mut GtkCellRendererAccel,
                                    path: *const c_char,
                                    f: &Box<Fn(&CellRendererAccel, TreePath) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRendererAccel::from_glib_none(this), path);
    }

    unsafe extern "C" fn e_trampoline(this: *mut GtkCellRendererAccel,
                                      path: *const c_char,
                                      accel_key: c_uint,
                                      accel_mods: GdkModifierType,
                                      hardware_keycode: c_uint,
                                      f: &Box<Fn(&CellRendererAccel, TreePath, u32, ModifierType, u32) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRendererAccel::from_glib_none(this),
           path,
           accel_key,
           from_glib(accel_mods),
           hardware_keycode);
    }
}

pub trait CellRendererComboSignals {
    fn connect_changed<F>(&self, changed_func: F) -> u64
        where F: Fn(&Self, TreePath, &TreeIter) + 'static;
}

mod cell_renderer_combo {
    use CellRendererCombo;
    use TreeIter;
    use TreePath;
    use std::mem::transmute;
    use ffi::{GtkCellRendererCombo, GtkTreeIter};
    use glib::signal::connect;
    use glib::translate::*;
    use ffi::gtk_tree_path_new_from_string;
    use libc::c_char;

    impl super::CellRendererComboSignals for CellRendererCombo {
        fn connect_changed<F>(&self, accel_cleared_func: F) -> u64
        where F: Fn(&Self, TreePath, &TreeIter) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, TreePath, &TreeIter) + 'static>> =
                    Box::new(Box::new(accel_cleared_func));
                connect(self.to_glib_none().0, "changed",
                    transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline(this: *mut GtkCellRendererCombo,
                                    path: *const c_char,
                                    new_iter: *mut GtkTreeIter,
                                    f: &Box<Fn(&CellRendererCombo, TreePath, &TreeIter) + 'static>) {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRendererCombo::from_glib_none(this), path, &from_glib_borrow(new_iter));
    }
}

pub trait CellRendererSignals {
    fn connect_editing_canceled<F>(&self, editing_canceled_func: F) -> u64
        where F: Fn(&Self) + 'static;
    fn connect_editing_started<F>(&self, editing_started_func: F) -> u64
        where F: Fn(&Self, &CellEditable, TreePath) + 'static;
}

mod cell_renderer {
    use CellEditable;
    use CellRenderer;
    use Object;
    use TreePath;
    use std::mem::transmute;
    use ffi::{GtkCellEditable, GtkCellRenderer};
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use IsA;
    use ffi::gtk_tree_path_new_from_string;
    use libc::c_char;

    impl<T: IsA<CellRenderer> + IsA<Object>> super::CellRendererSignals for T {
        fn connect_editing_canceled<F>(&self, editing_canceled_func: F) -> u64
        where F: Fn(&Self) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> =
                    Box::new(Box::new(editing_canceled_func));
                connect(self.to_glib_none().0, "editing-canceled",
                    transmute(trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_editing_started<F>(&self, editing_started_func: F) -> u64
        where F: Fn(&Self, &CellEditable, TreePath) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &CellEditable, TreePath) + 'static>> =
                    Box::new(Box::new(editing_started_func));
                connect(self.to_glib_none().0, "editing-started",
                    transmute(start_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline<T>(this: *mut GtkCellRenderer,
                                       f: &Box<Fn(&T) + 'static>)
    where T: IsA<CellRenderer> {
        callback_guard!();
        f(&CellRenderer::from_glib_none(this).downcast_unchecked());
    }

    unsafe extern "C" fn start_trampoline<T>(this: *mut GtkCellRenderer,
                                             editable: *mut GtkCellEditable,
                                             path: *const c_char,
                                             f: &Box<Fn(&T, &CellEditable, TreePath) + 'static>)
    where T: IsA<CellRenderer> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        f(&CellRenderer::from_glib_none(this).downcast_unchecked(),
          &CellEditable::from_glib_none(editable),
          path);
    }
}

pub trait CellRendererTextSignals {
    fn connect_edited<F: Fn(&Self, &TreePath, &str) + 'static>(&self, f: F) -> u64;
}

mod cell_renderer_text {
    use Object;
    use std::mem::transmute;
    use std::str;
    use glib::signal::connect;
    use glib::translate::*;
    use libc::c_char;
    use std::ffi::CStr;
    use glib::object::Downcast;
    use IsA;
    use ffi::{GtkCellRendererText, gtk_tree_path_new_from_string};
    use {CellRendererText, TreePath};

    impl<T: IsA<CellRendererText> + IsA<Object>> super::CellRendererTextSignals for T {
        fn connect_edited<F: Fn(&Self, &TreePath, &str) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(&Self, &TreePath, &str) + 'static>> = Box::new(Box::new(f));
                connect(self.to_glib_none().0, "edited",
                    transmute(trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline<T>(this: *mut GtkCellRendererText, path: *const c_char,
        new_text: *const c_char, f: &Box<Fn(&T, &TreePath, &str) + 'static>)
    where T: IsA<CellRendererText> {
        callback_guard!();
        let path = from_glib_full(gtk_tree_path_new_from_string(path));
        let buf = CStr::from_ptr(new_text).to_bytes();
        let new_text = str::from_utf8(buf).unwrap();
        f(&CellRendererText::from_glib_none(this).downcast_unchecked(), &path, new_text);
    }
}

pub trait EditableSignals {
    fn connect_changed<F>(&self, changed_func: F) -> u64
        where F: Fn(&Self) + 'static;
    fn connect_delete_text<F>(&self, delete_text_func: F) -> u64
        where F: Fn(&Self, i32, i32) + 'static;
    fn connect_insert_text<F>(&self, insert_text_func: F) -> u64
        where F: Fn(&Self, &str, &mut i32) + 'static;
}

mod editable {
    use Editable;
    use Object;
    use std::mem::transmute;
    use ffi::GtkEditable;
    use glib::signal::connect;
    use glib::translate::*;
    use IsA;
    use libc::{c_char, c_int, c_uchar};
    use std::ffi::CStr;
    use std::str;
    use glib::object::Downcast;
    use std::slice;

    impl<T: IsA<Editable> + IsA<Object>> super::EditableSignals for T {
        fn connect_changed<F>(&self, changed_func: F) -> u64
        where F: Fn(&Self) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self) + 'static>> =
                    Box::new(Box::new(changed_func));
                connect(self.to_glib_none().0, "changed",
                    transmute(trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_delete_text<F>(&self, delete_text_func: F) -> u64
        where F: Fn(&Self, i32, i32) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, i32, i32) + 'static>> =
                    Box::new(Box::new(delete_text_func));
                connect(self.to_glib_none().0, "delete-text",
                    transmute(delete_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_insert_text<F>(&self, insert_text_func: F) -> u64
        where F: Fn(&Self, &str, &mut i32) + 'static {
            unsafe {
                let f: Box<Box<Fn(&Self, &str, &mut i32) + 'static>> =
                    Box::new(Box::new(insert_text_func));
                connect(self.to_glib_none().0, "insert-text",
                    transmute(insert_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn trampoline<T>(this: *mut GtkEditable,
                                       f: &Box<Fn(&T) + 'static>)
    where T: IsA<Editable> {
        callback_guard!();
        f(&Editable::from_glib_none(this).downcast_unchecked());
    }

    unsafe extern "C" fn delete_trampoline<T>(this: *mut GtkEditable,
                                              start_pos: c_int,
                                              end_pos: c_int,
                                              f: &Box<Fn(&T, i32, i32) + 'static>)
    where T: IsA<Editable> {
        callback_guard!();
        f(&Editable::from_glib_none(this).downcast_unchecked(), start_pos, end_pos);
    }

    unsafe extern "C" fn insert_trampoline<T>(this: *mut GtkEditable,
                                              new_text: *mut c_char,
                                              new_text_length: c_int,
                                              position: *mut c_int,
                                              f: &Box<Fn(&T, &str, &mut i32) + 'static>)
    where T: IsA<Editable> {
        callback_guard!();
        let buf = if new_text_length != -1 {
            slice::from_raw_parts(new_text as *mut c_uchar,
                                  new_text_length as usize)
        } else {
            CStr::from_ptr(new_text).to_bytes()
        };
        let string = str::from_utf8(buf).unwrap();
        f(&Editable::from_glib_none(this).downcast_unchecked(),
          string,
          transmute(position));
    }
}

pub trait SpinButtonSignals {
    fn connect_change_value<F>(&self, change_value_func: F) -> u64
        where F: Fn(&SpinButton, ScrollType) + 'static;
    fn connect_input<F>(&self, input_func: F) -> u64
        where F: Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static;
    fn connect_output<F>(&self, output_func: F) -> u64
        where F: Fn(&SpinButton) -> Inhibit + 'static;
    fn connect_value_changed<F>(&self, value_changed_func: F) -> u64
        where F: Fn(&SpinButton) + 'static;
    fn connect_wrapped<F>(&self, wrapped_func: F) -> u64
        where F: Fn(&SpinButton) + 'static;
}

mod spin_button {
    use Inhibit;
    use SpinButton;
    use ScrollType;
    use ffi::{GTK_INPUT_ERROR, GtkSpinButton};
    use glib::signal::connect;
    use glib::translate::*;
    use glib_ffi::{GTRUE, GFALSE};
    use libc::{c_int, c_double};
    use std::boxed::Box as Box_;
    use std::mem::transmute;
    use glib_ffi::gboolean;

    impl ::SpinButtonSignals for SpinButton {
        fn connect_change_value<F>(&self, change_value_func: F) -> u64
        where F: Fn(&SpinButton, ScrollType) + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton, ScrollType) + 'static>> =
                    Box::new(Box::new(change_value_func));
                connect(self.to_glib_none().0, "change_value",
                        transmute(change_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_input<F>(&self, f: F) -> u64
        where F: Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static {
            unsafe {
                let f: Box_<Box_<Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static>> = Box_::new(Box_::new(f));
                connect(self.to_glib_none().0, "input",
                        transmute(input_trampoline as usize), Box_::into_raw(f) as *mut _)
            }
        }

        fn connect_output<F>(&self, output_func: F) -> u64
        where F: Fn(&SpinButton) -> Inhibit + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton) -> Inhibit + 'static>> =
                    Box::new(Box::new(output_func));
                connect(self.to_glib_none().0, "output",
                        transmute(output_trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_value_changed<F>(&self, value_changed_func: F) -> u64
        where F: Fn(&SpinButton) + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton) + 'static>> =
                    Box::new(Box::new(value_changed_func));
                connect(self.to_glib_none().0, "value-changed",
                        transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }

        fn connect_wrapped<F>(&self, wrapped_func: F) -> u64
        where F: Fn(&SpinButton) + 'static {
            unsafe {
                let f: Box<Box<Fn(&SpinButton) + 'static>> =
                    Box::new(Box::new(wrapped_func));
                connect(self.to_glib_none().0, "wrapped",
                        transmute(trampoline as usize), Box::into_raw(f) as *mut _)
            }
        }
    }

    unsafe extern "C" fn change_trampoline(this: *mut GtkSpinButton,
                                           scroll: ScrollType,
                                           f: &Box<Fn(&SpinButton, ScrollType) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this), scroll)
    }

    unsafe extern "C" fn input_trampoline(this: *mut GtkSpinButton,
                                          new_value: *mut c_double,
                                          f: &Box_<Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static>)
                                          -> c_int {
        callback_guard!();
        match f(&from_glib_none(this)) {
            Some(Ok(v)) => {
                *new_value = v;
                GTRUE
            }
            Some(Err(_)) => GTK_INPUT_ERROR,
            None => GFALSE,
        }
    }

    unsafe extern "C" fn output_trampoline(this: *mut GtkSpinButton,
                                           f: &Box<Fn(&SpinButton) -> Inhibit + 'static>)
                                           -> gboolean {
        callback_guard!();
        f(&from_glib_none(this)).to_glib()
    }

    unsafe extern "C" fn trampoline(this: *mut GtkSpinButton,
                                    f: &Box<Fn(&SpinButton) + 'static>) {
        callback_guard!();
        f(&from_glib_none(this))
    }
}
