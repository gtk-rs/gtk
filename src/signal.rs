// Copyright 2015, The Rust-gnome Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

use std::boxed::into_raw;
use std::mem::transmute;

use glib::signal::connect;
use glib::translate::*;
use glib::{ParamSpec};

use ffi::{Gboolean, C_GtkAdjustment};
use gdk::{EventAny, EventButton, EventKey};
use cairo::Context;

use traits::FFIWidget;
use {Adjustment, Button, ToolButton, Widget};

/// Whether to propagate the signal to other handlers
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

impl ToGlib for Inhibit {
    type GlibType = Gboolean;

    #[inline]
    fn to_glib(&self) -> Gboolean {
        self.0.to_glib()
    }
}

pub trait WidgetSignals: FFIWidget {
    fn connect_notify<F: Fn(Widget, &ParamSpec)>(&self, f: F) -> u64;
    fn connect_button_press_event<F: Fn(Widget, &EventButton) -> Inhibit>(&self, f: F) -> u64;
    fn connect_button_release_event<F: Fn(Widget, &EventButton) -> Inhibit>(&self, f: F) -> u64;
    fn connect_delete_event<F: Fn(Widget, &EventAny) -> Inhibit>(&self, f: F) -> u64;
    fn connect_draw<F: Fn(Widget, Context) -> Inhibit>(&self, f: F) -> u64;
    fn connect_key_press_event<F: Fn(Widget, &EventKey) -> Inhibit>(&self, f: F) -> u64;
    fn connect_key_release_event<F: Fn(Widget, &EventKey) -> Inhibit>(&self, f: F) -> u64;
    fn connect_popup_menu<F: Fn(Widget) -> bool>(&self, f: F) -> u64;
}

pub trait ButtonSignals: FFIWidget {
    fn connect_activate<F: Fn(Button)>(&self, f: F) -> u64;
    fn connect_clicked<F: Fn(Button)>(&self, f: F) -> u64;
}

pub trait ToolButtonSignals: FFIWidget {
    fn connect_clicked<F: Fn(ToolButton)>(&self, f: F) -> u64;
}

mod widget {
    use std::boxed::into_raw;
    use std::mem::transmute;
    use glib::{ParamSpec};
    use glib::signal::connect;
    use glib::translate::ToGlib;
    use gdk::{EventAny, EventButton, EventKey};
    use cairo_ffi::cairo_t;
    use cairo::Context;
    use traits::{FFIWidget, WidgetTrait};
    use ffi::{Gboolean, C_GtkWidget};
    use Widget;
    use super::Inhibit;

    impl <T: WidgetTrait> super::WidgetSignals for T {
        // this is a GObject signal actually
        fn connect_notify<F: Fn(Widget, &ParamSpec)>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &ParamSpec)>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "notify",
                    transmute(notify_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_button_press_event<F: Fn(Widget, &EventButton) -> Inhibit>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventButton) -> Inhibit>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "button-press-event",
                    transmute(button_event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_button_release_event<F: Fn(Widget, &EventButton) -> Inhibit>(&self, f: F)
                -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventButton) -> Inhibit>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "button-release-event",
                    transmute(button_event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_delete_event<F: Fn(Widget, &EventAny) -> Inhibit>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventAny) -> Inhibit>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "delete-event",
                    transmute(any_event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_draw<F: Fn(Widget, Context) -> Inhibit>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, Context) -> Inhibit>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "draw",
                    transmute(draw_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_key_press_event<F: Fn(Widget, &EventKey) -> Inhibit>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventKey) -> Inhibit>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "key-press-event",
                    transmute(key_event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_key_release_event<F: Fn(Widget, &EventKey) -> Inhibit>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget, &EventKey) -> Inhibit>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "key-release-event",
                    transmute(key_event_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_popup_menu<F: Fn(Widget) -> bool>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) -> bool>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "popup-menu",
                    transmute(bool_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    fn void_trampoline(this: *mut C_GtkWidget, f: &Box<Fn(Widget)>) {
        f(FFIWidget::wrap_widget(this));
    }

    fn bool_trampoline(this: *mut C_GtkWidget, f: &Box<Fn(Widget) -> bool>) -> Gboolean {
        f(FFIWidget::wrap_widget(this)).to_glib()
    }

    fn any_event_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventAny) -> Inhibit>) -> Gboolean {
        unsafe {
            f(FFIWidget::wrap_widget(this), transmute(event)).to_glib()
        }
    }

    fn button_event_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventButton) -> Inhibit>) -> Gboolean {
        unsafe {
            f(FFIWidget::wrap_widget(this), transmute(event)).to_glib()
        }
    }

    fn draw_trampoline(this: *mut C_GtkWidget, cr: *mut cairo_t,
            f: &Box<Fn(Widget, Context) -> Inhibit>) -> Gboolean {
        f(FFIWidget::wrap_widget(this), Context::wrap(cr)).to_glib()
    }

    fn key_event_trampoline(this: *mut C_GtkWidget, event: *mut EventAny,
            f: &Box<Fn(Widget, &EventKey) -> Inhibit>) -> Gboolean {
        unsafe {
            f(FFIWidget::wrap_widget(this), transmute(event)).to_glib()
        }
    }

    fn notify_trampoline(this: *mut C_GtkWidget, pspec: *mut ParamSpec,
            f: &Box<Fn(Widget, &ParamSpec)>) {
        unsafe {
            f(FFIWidget::wrap_widget(this), transmute(pspec));
        }
    }
}

mod button {
    use std::boxed::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use traits::{FFIWidget, ButtonTrait};
    use ffi::C_GtkButton;
    use Button;

    impl <T: ButtonTrait> super::ButtonSignals for T {
        fn connect_activate<F: Fn(Button)>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Button)>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "activate",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }

        fn connect_clicked<F: Fn(Button)>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Button)>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "clicked",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    fn void_trampoline(this: *mut C_GtkButton, f: &Box<Fn(Button)>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }
}

mod tool_button {
    use std::boxed::into_raw;
    use std::mem::transmute;
    use glib::signal::connect;
    use traits::{FFIWidget, ToolButtonTrait};
    use ffi::C_GtkToolButton;
    use ToolButton;

    impl <T: ToolButtonTrait> super::ToolButtonSignals for T {
        fn connect_clicked<F: Fn(ToolButton)>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(ToolButton)>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _, "clicked",
                    transmute(void_trampoline), into_raw(f) as *mut _)
            }
        }
    }

    fn void_trampoline(this: *mut C_GtkToolButton, f: &Box<Fn(ToolButton)>) {
        f(FFIWidget::wrap_widget(this as *mut _));
    }
}

// Adjustment doesn't implement any traits at all

impl Adjustment {
    pub fn connect_value_changed<F: Fn(Adjustment)>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(Adjustment)>> = Box::new(Box::new(f));
            connect(self.unwrap_pointer() as *mut _, "value-changed",
                transmute(adjustment_trampoline), into_raw(f) as *mut _)
        }
    }
}

extern "C" fn adjustment_trampoline(this: *mut C_GtkAdjustment, f: &Box<Fn(Adjustment)>) {
    f(Adjustment::wrap_pointer(this))
}
